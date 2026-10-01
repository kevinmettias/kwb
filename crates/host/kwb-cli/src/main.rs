//! The `kwb` binary. Composition root only — no domain logic lives here.
//!
//! # What this can and cannot do, said plainly
//!
//! It admits a file through the real pipeline: the bytes go through `kwb-store`'s one write
//! door, the extractions are linked and normalized, and what survives is published into a
//! knowledge graph.
//!
//! **There is no extractor.** What a passage asserts is supplied on the command line, because
//! nothing in this repository reads a document and decides what it says. That is the
//! interesting half and it is missing, and the help says so rather than leaving a reader to
//! infer it from an empty result. A tool that implied it had read the file would be the
//! prototype's `chunks admitted 0` printed under the heading *Admitted*.
//!
//! **`--store <dir>` keeps both halves.** The bytes go to a content-addressed file store and
//! the publications go to an append-only log, and a run replays that log before it admits — so
//! what the third run knows includes what the first two learned. `D-014` decided both, and this
//! file is where the implementations are named: the library names ports and never an
//! implementation of one, which is the whole reason a composition root exists.
//!
//! Without `--store`, nothing is kept and the run says so.
//!
//! # Where the rest of it is
//!
//! This file is the dispatch and the report: the verb table, `main`, and the fields a person
//! sees after an admission. Each verb's own reading, running and printing is in the module named
//! for it — `admission`, `closing`, `history` — with the flags they share in `arguments`, what a
//! run keeps in `keeping`, how a command it could not carry out is answered in `refusals`, and
//! the help in `usage`.
//!
//! The report stays here rather than moving in beside `admission`, for the same reason the verb
//! table does: it is the composition root's statement about its own run, and the README's
//! transcripts are compared against it there.

mod admission;
mod arguments;
mod closing;
mod history;
mod keeping;
mod refusals;
mod usage;

use std::process::ExitCode;

use kwb_domain::KnowledgeGraph;
use kwb_ingest::{AdmissionReport, ExtractionError};

use crate::refusals::{Complained_With_Usage, Wrong_Command_Line};

/// A wrong command line, which is not the same as a run that failed.
const USAGE_EXIT: u8 = 2;

/// A run that reached the pipeline and could not complete it.
const FAILURE_EXIT: u8 = 1;

/// One verb: what a reader types, what they are shown, and what runs.
///
/// # Why the usage line lives here and not in the help
///
/// It was in both until `KWB-73`, and they drifted: `history` was dispatched while the help
/// named it nowhere, and the help went on saying *there is no extractor* after `KWB-66` built
/// one. `KWB-67` added a guard that ran the binary and compared the two in both directions,
/// which made the drift detectable and still left two places to edit — the arrangement that
/// produced it.
///
/// One table leaves nothing to keep in step. The dispatch runs from it and `Print_Usage`
/// renders from it, so a verb added in one place cannot be missing from the other.
///
/// Only the **enumeration** is here. What a command does stays in its handler and the prose
/// explaining `D17` or coverage stays prose — `KWB-70` drew that line for the tool table and it
/// is the same line.
struct Verb
{
    /// What a reader types.
    name: &'static str,

    /// The usage line they are shown, after `kwb `.
    usage: &'static str,

    /// What runs.
    run: &'static dyn Command,
}

/// What a verb does when it runs: the one operation the verb table holds.
///
/// # Why the table holds a strategy and not a `fn` pointer
///
/// A stored `fn(&[&str]) -> ExitCode` is a collaborator with no name. It is invoked later, at a
/// time the caller cannot see, and nothing in the table said what it may do while it runs —
/// whether it reads the filesystem, writes to the store, or ends the process outright. A named
/// operation is where that is said, and it is what lets a verb be answered by a test double
/// that reads as an implementation rather than as a lambda.
///
/// Holding a `&'static dyn Command` rather than a closure loses nothing here: no verb captures
/// anything, because everything each one needs arrives in `arguments`.
trait Command
{
    /// Run this verb over the arguments that followed its name.
    fn Run_Command_Line(&self, arguments: &[&str]) -> ExitCode;
}

/// Every verb this binary answers, in the order a reader is shown them.
///
/// `help` and its aliases are deliberately absent: they are how a reader finds the rest rather
/// than a capability to be listed, which is the same reason `KWB-67`'s guard exempts them.
const VERBS: [Verb; 4] = [
    Verb {
        name: "admit",
        usage: "admit <file> [--store <dir>] [--scope <name>] [--says <concept> <claim>]...",
        run: &Admit,
    },
    Verb {
        name: "retire",
        usage: "retire <concept> --store <dir> --because <reason>",
        run: &Retire,
    },
    Verb {
        name: "supersede",
        usage: "supersede <concept> --into <concept> --store <dir> --because <reason>",
        run: &Supersede,
    },
    Verb {
        name: "history",
        usage: "history --store <dir> [--through <count> | --as-of <unix seconds>]",
        run: &History,
    },
];

/// `kwb retire <concept> ...`.
///
/// What separates it from [`Supersede`] is the whole of what differs between the two closings,
/// and it is said by which of them a run names rather than by an argument.
struct Retire;

impl Command for Retire
{
    fn Run_Command_Line(&self, arguments: &[&str]) -> ExitCode
    {
        use crate::closing::Close_Command;

        return Close_Command(arguments, None);
    }
}

/// `kwb supersede <concept> --into <concept> ...`
struct Supersede;

impl Command for Supersede
{
    fn Run_Command_Line(&self, arguments: &[&str]) -> ExitCode
    {
        use crate::closing::Close_Command;

        return Close_Command(arguments, Some(()));
    }
}

/// `kwb history --store <dir> ...`
struct History;

impl Command for History
{
    fn Run_Command_Line(&self, arguments: &[&str]) -> ExitCode
    {
        use crate::history::History_Command;

        return History_Command(arguments);
    }
}

fn main() -> ExitCode
{
    use crate::usage::Print_Usage;

    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();

    let Some((verb, rest)) = borrowed.split_first()
    else
    {
        return Wrong_Command_Line("expected a verb");
    };

    if Is_Help(verb)
    {
        Print_Usage();
        return ExitCode::SUCCESS;
    }

    return Dispatch_Verb(verb, rest);
}

/// Whether a reader asked for the help rather than for a verb.
///
/// Named because the help is not a capability: it is how a reader finds the rest, which is why the
/// verb table omits it and why this is asked before the table is consulted at all.
fn Is_Help(verb: &str) -> bool
{
    return matches!(verb, "help" | "--help" | "-h");
}

/// Run the verb a reader named, over the arguments that followed it.
///
/// The one place the verb table is consulted to dispatch, and the whole reason `main` reads as
/// orchestration: the lookup, the refusal of a verb the table does not name, and the call are one
/// operation with one name. It takes the borrowed remainder rather than the process arguments,
/// because a verb is handed its own arguments and never the ones that named it.
fn Dispatch_Verb(verb: &str, rest: &[&str]) -> ExitCode
{
    let Some(known) = VERBS.iter().find(|known| return known.name == verb)
    else
    {
        return Wrong_Command_Line("expected a verb");
    };

    return known.run.Run_Command_Line(rest);
}

/// `kwb admit <file> [--says <concept> <claim>]...`
struct Admit;

impl Command for Admit
{
    fn Run_Command_Line(&self, arguments: &[&str]) -> ExitCode
    {
        use crate::admission::Run_Admission;

        let Some((path, rest)) = arguments.split_first()
        else
        {
            return Complained_With_Usage("kwb admit", "expected a file to admit");
        };

        return Run_Admission(path, rest);
    }
}

/// Whether what a run kept outlived it.
///
/// The report says this twice about two different things — the bytes a source was kept as, and
/// the publications recording what was learned — and prints the same words for both. At a call
/// site a bare `true` says neither which of the two it is nor what it means, and the two
/// positions swap without the compiler objecting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Persistence
{
    /// On disk, under the store the run was given.
    Kept,

    /// Built and answered from, and gone when the process ends.
    InMemoryOnly,
}

impl Persistence
{
    /// Whether a store that reports itself durable kept what it was given.
    ///
    /// The conversion sits here rather than at each call site because the two stores report the
    /// same fact in their own vocabularies — a directory that exists, a log that was opened —
    /// and the report is where those become the one vocabulary it prints in.
    pub(crate) fn Of(durable: bool) -> Self
    {
        if durable
        {
            return Self::Kept;
        }

        return Self::InMemoryOnly;
    }

    /// The words the report prints for this state.
    fn Name(self) -> &'static str
    {
        return match self
        {
            Self::Kept => "kept",
            Self::InMemoryOnly => "in memory only",
        };
    }
}

/// What one admission found, in the library's own vocabulary.
///
/// `Name` is the same string a journal would carry, so a person reading a terminal and a report
/// reading a file are told the same thing about the same run.
fn Report_Admission(
    report: &AdmissionReport,
    published: &KnowledgeGraph,
    documents: Persistence,
    knowledge: Persistence,
)
{
    println!("source     {}", Address_Of(report));
    println!("coverage   {}", report.Coverage().Name());
    println!("concepts   {}", published.Current().Concepts().len());
    println!("claims     {}", published.Current().Claims().len());
    println!("citations  {}", published.Current().Assertions().len());
    println!("refused    {}", report.Normalized().Linked().Refused());
    println!("documents  {}", documents.Name());
    println!("knowledge  {}", knowledge.Name());

    if let Some(refusal) = report.Refusal()
    {
        Report_Refusal(refusal);
    }
}

/// The library's own words for why a reading did not happen, rather than one sentence covering
/// every reason a reading did not happen.
///
/// A person who passed `--says` and still sees this needs to know it was not their omission,
/// and a single sentence for every cause could not tell them that.
fn Report_Refusal(refusal: &ExtractionError)
{
    println!();
    println!("{refusal}.");
    if matches!(refusal, ExtractionError::NotRead)
    {
        println!("Pass --says to supply what a passage asserts; this tool does not read");
        println!("the document and decide for itself.");
    }
}

/// The address the source was written under, rendered.
fn Address_Of(report: &AdmissionReport) -> String
{
    return report
        .Source()
        .map_or_else(|| return "-".to_owned(), |written| return written.Identity().Render());
}
