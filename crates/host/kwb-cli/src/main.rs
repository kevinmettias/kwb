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

use std::process::ExitCode;

use kwb_domain::{
    Concept, KnowledgeGraph, Publication, Published_At, Replay, Scope, Standing, Versioned,
};
use kwb_ingest::{
    Admit, AdmissionReport, Extraction, ExtractionLineage, ExtractionRefused, ExtractionStrategy,
    ReadingKind, SourceLocation, Stated,
};
use kwb_platform::RecordLogStrategy;
use kwb_platform_std::{DirectoryContentStore, FileRecordLog};
use kwb_platform_xvpe::{PublicationClock, SystemClock};
use kwb_store::DocumentStore;

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
    run: fn(&[&str]) -> ExitCode,
}

/// Every verb this binary answers, in the order a reader is shown them.
///
/// `help` and its aliases are deliberately absent: they are how a reader finds the rest rather
/// than a capability to be listed, which is the same reason `KWB-67`'s guard exempts them.
const VERBS: [Verb; 4] = [
    Verb {
        name: "admit",
        usage: "admit <file> [--store <dir>] [--scope <name>] [--says <concept> <claim>]...",
        run: Admit_Command,
    },
    Verb {
        name: "retire",
        usage: "retire <concept> --store <dir> --because <reason>",
        run: Retire_Command,
    },
    Verb {
        name: "supersede",
        usage: "supersede <concept> --into <concept> --store <dir> --because <reason>",
        run: Supersede_Command,
    },
    Verb {
        name: "history",
        usage: "history --store <dir> [--through <count> | --as-of <unix seconds>]",
        run: History_Command,
    },
];

/// `kwb retire <concept> ...`, as a function the table can hold.
///
/// A named wrapper rather than a closure, because a `fn` pointer cannot capture and a table of
/// closures would need boxing for no gain. What each wrapper says is which of the two closings
/// it is, which is the whole difference between them.
fn Retire_Command(arguments: &[&str]) -> ExitCode
{
    return Close_Command(arguments, None);
}

/// `kwb supersede <concept> --into <concept> ...`
fn Supersede_Command(arguments: &[&str]) -> ExitCode
{
    return Close_Command(arguments, Some(()));
}

/// A command line this binary could not read, as the exit that answers it.
///
/// Two lines and an exit code, repeated at every refusal in this file, and `main` reaches it
/// from two directions — nothing typed at all, and a word that is not a verb — so writing it
/// out at each site left a reader comparing the copies to be sure they agreed.
fn Wrong_Command_Line(complaint: &str) -> ExitCode
{
    eprintln!("kwb: {complaint}");
    Print_Usage();
    return ExitCode::from(USAGE_EXIT);
}

/// A run that failed, under the name of the command that ran.
///
/// The same shape as [`Wrong_Command_Line`] without the usage: a person who typed a well-formed
/// command has already been shown how to type it, and printing the usage under a store error
/// would bury the one line that says what actually went wrong.
fn Complained(verb: &str, complaint: &str, code: u8) -> ExitCode
{
    eprintln!("{verb}: {complaint}");
    return ExitCode::from(code);
}

/// A command line whose **arguments** were mistyped, under the name of the command that read them.
///
/// [`Complained`] answers a run that was well-formed and could not finish, and it is right that
/// it withholds the usage. This is the other case: what was wrong is the syntax itself, so the
/// syntax is what the person is shown. The two are told apart by which of them is being
/// answered — an argument the command does not take, or a run that could not proceed — and not
/// by the exit code, because a bad `--through` count and an unreadable log both exit `2` and
/// only the first is a question about how to type the command.
fn Complained_With_Usage(verb: &str, complaint: &str) -> ExitCode
{
    eprintln!("{verb}: {complaint}");
    Print_Usage();
    return ExitCode::from(USAGE_EXIT);
}

fn main() -> ExitCode
{
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();

    let Some((verb, rest)) = borrowed.split_first()
    else
    {
        return Wrong_Command_Line("expected a verb");
    };

    if matches!(*verb, "help" | "--help" | "-h")
    {
        Print_Usage();
        return ExitCode::SUCCESS;
    }

    let Some(known) = VERBS.iter().find(|known| return known.name == *verb)
    else
    {
        return Wrong_Command_Line("expected a verb");
    };

    return (known.run)(rest);
}

/// `kwb admit <file> [--says <concept> <claim>]...`
fn Admit_Command(arguments: &[&str]) -> ExitCode
{
    let Some((path, rest)) = arguments.split_first()
    else
    {
        eprintln!("kwb admit: expected a file to admit");
        Print_Usage();
        return ExitCode::from(USAGE_EXIT);
    };

    return Run_Admission(path, rest);
}

/// One admission, from the flags that describe it to the bytes they are about.
///
/// Split from the dispatch that reaches it because the flags, the file and the store are three
/// ways to refuse a run — one of them a wrong command line and two of them failed runs — and a
/// reader checking that each ends the way it should had to hold all of them at once.
fn Run_Admission(path: &str, rest: &[&str]) -> ExitCode
{
    let (store_root, scope, extractions) = match Arguments_Of_Admit(rest)
    {
        Ok(parsed) => parsed,
        Err(complaint) => return Complained_With_Usage("kwb admit", &complaint),
    };

    let bytes = match Bytes_Of(path)
    {
        Ok(bytes) => bytes,
        Err(complaint) => return Complained("kwb admit", &complaint, FAILURE_EXIT),
    };

    let mut kept = match Kept_At(store_root)
    {
        Ok(kept) => kept,
        Err(complaint) => return Complained("kwb admit", &complaint, FAILURE_EXIT),
    };

    let said = Reader_For(extractions, scope);
    return Admitted(bytes, said.as_ref(), &mut kept);
}

/// The bytes of the file a run was pointed at.
///
/// # Errors
///
/// The file, when it could not be read. A failed run rather than a wrong command line: the path
/// was well-formed, and it is the medium that refused.
fn Bytes_Of(path: &str) -> Result<Vec<u8>, String>
{
    return std::fs::read(path).map_err(|cause| return format!("cannot read {path}: {cause}"));
}

/// The reading a command line supplies, or nothing when it supplied nothing to read.
///
/// `--says` is a person stating what a passage asserts, which is exactly what `Stated` is.
/// Routing it through the extraction seam rather than handing `Admit` a list means the command
/// line uses the same door a reading adapter will, and that the protocol and the reader are
/// recorded instead of being implied by the fact that somebody typed them.
fn Reader_For(extractions: Vec<Extraction>, scope: Scope) -> Option<Stated>
{
    let location = SourceLocation::Named("as stated on the command line");
    let lineage = ExtractionLineage::Of("stated-by-a-person", "the operator of kwb admit");

    return Stated::Of(extractions, location, lineage, scope);
}

/// One admission: the bytes through the pipeline, what it published recorded, the run reported.
///
/// The reading is borrowed rather than taken because it is only read here: the pipeline is
/// handed a trait object derived from it and never the value itself.
fn Admitted(bytes: Vec<u8>, said: Option<&Stated>, kept: &mut Kept) -> ExitCode
{
    let reader = said.map(|said| return said as &dyn ExtractionStrategy);

    // Text, because that is what a person reading a file to type `--says` was doing. A source
    // needing visual reading is a judgement no part of this command can make, and saying `Text`
    // here does not assert otherwise -- `Stated` refuses no kind, so the value reaches nothing
    // that acts on it. It becomes load-bearing when a reader that can refuse arrives.
    let report = match Admit(bytes, reader, ReadingKind::Text, &mut kept.store)
    {
        Ok(report) => report,
        Err(refusal) => return Complained("kwb admit", &refusal.to_string(), FAILURE_EXIT),
    };

    let published = match Record_Into(kept.recorded.log.as_ref(), &report, Some(Now()))
    {
        Ok(()) => report.Published_Into(&kept.recorded.known),
        Err(complaint) => return Complained("kwb admit", &complaint, FAILURE_EXIT),
    };

    Report_Admission(&report, &published, kept.store.Is_Durable(), kept.recorded.log.is_some());

    return ExitCode::SUCCESS;
}

/// What one admission found, in the library's own vocabulary.
///
/// `Name` is the same string a journal would carry, so a person reading a terminal and a report
/// reading a file are told the same thing about the same run.
fn Report_Admission(
    report: &AdmissionReport,
    published: &KnowledgeGraph,
    durable: bool,
    knowledge_kept: bool,
)
{
    println!("source     {}", Address_Of(report));
    println!("coverage   {}", report.Coverage().Name());
    println!("concepts   {}", published.Current().Concepts().len());
    println!("claims     {}", published.Current().Claims().len());
    println!("citations  {}", published.Current().Assertions().len());
    println!("refused    {}", report.Normalized().Linked().Refused());
    println!("documents  {}", if durable { "kept" } else { "in memory only" });
    println!("knowledge  {}", if knowledge_kept { "kept" } else { "in memory only" });

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
fn Report_Refusal(refusal: &ExtractionRefused)
{
    println!();
    println!("{refusal}.");
    if matches!(refusal, ExtractionRefused::NotRead)
    {
        println!("Pass --says to supply what a passage asserts; this tool does not read");
        println!("the document and decide for itself.");
    }
}

/// Everything one admission run carries: where its bytes go, and what the log already holds.
struct Kept
{
    /// Where admitted bytes are written.
    store: DocumentStore,

    /// The log beside the store, and the graph it folds into.
    recorded: Recorded,
}

/// The store a run writes through, and the log it records into.
///
/// # Errors
///
/// A root that cannot be used as a store, or a log that cannot be opened, read, or replayed.
fn Kept_At(root: Option<&str>) -> Result<Kept, String>
{
    let store = Store_For(root)?;
    let recorded = Recorded_At(root)?;

    return Ok(Kept { store, recorded });
}

/// The publication log a run records into, and the graph every earlier run left in it.
struct Recorded
{
    /// Where publications are recorded, when the run was told where.
    log: Option<FileRecordLog>,

    /// What earlier runs published, replayed.
    known: KnowledgeGraph,
}

/// The log at `root`, replayed.
///
/// # Errors
///
/// A log that cannot be opened, read, or replayed.
fn Recorded_At(root: Option<&str>) -> Result<Recorded, String>
{
    let log = Log_For(root)?;
    let known = Known_So_Far(log.as_ref())?;

    return Ok(Recorded { log, known });
}

/// The store a run writes through: durable when told where, in memory when not.
///
/// This function is the entire reason this crate exists. `kwb-store` names
/// `ContentStoreStrategy` and never an implementation of it, so somebody has to choose one, and
/// a composition root is where that choice is visible in one place rather than spread through
/// the code that writes.
fn Store_For(root: Option<&str>) -> Result<DocumentStore, String>
{
    let Some(root) = root
    else
    {
        return Ok(DocumentStore::Empty());
    };

    let durable = DirectoryContentStore::Under(root)
        .map_err(|cause| return format!("cannot use {root} as a store: {cause}"))?;
    return Ok(DocumentStore::Backed_By(Box::new(durable)));
}

/// Everything `kwb admit` reads off its command line, in the order a misplaced flag is caught.
///
/// Gathered into one function so the command has one refusal path rather than three that have
/// drifted apart -- and because each of the three ends the same way, which is the shape a
/// reader has to check three times to be sure of.
///
/// # Errors
///
/// The first complaint any of the three readers makes, in their declared order: a misplaced
/// `--store` is an unexpected argument rather than a concept named `--store`, and a `--scope`
/// that names nothing is refused rather than recorded as an unstated scope.
fn Arguments_Of_Admit<'arguments>(
    arguments: &'arguments [&'arguments str],
) -> Result<(Option<&'arguments str>, Scope, Vec<Extraction>), String>
{
    let (store_root, rest) = Store_Root_From(arguments);
    let (scope, rest) = Scope_From(rest)?;
    let extractions = Extractions_From(rest)?;

    return Ok((store_root, scope, extractions));
}

/// `--scope <name>`, if it leads the remaining arguments.
///
/// An absent scope is a real answer and not a default. `D-010`: a source that did not say how
/// far it meant has not said the narrowest thing, so an unstated scope stays distinguishable
/// from every stated one rather than being filled in with a guess.
///
/// # Errors
///
/// A `--scope` whose text names nothing. Until `KWB-50` that was accepted and recorded as an
/// *unstated* scope, byte-identical to passing no `--scope` at all — so somebody who typed a
/// scope and had it swallowed was told nothing, and the record said they had said nothing.
/// Refusing here is the fix, because this is where a person's input arrives; `Scope::Named`
/// refusing to return the unstated scope is what makes it impossible to get wrong elsewhere.
fn Scope_From<'arguments>(
    arguments: &'arguments [&'arguments str],
) -> Result<(Scope, &'arguments [&'arguments str]), String>
{
    return match arguments
    {
        [flag, name, rest @ ..] if *flag == "--scope" => match Scope::Named(name)
        {
            Some(scope) => Ok((scope, rest)),
            None => Err(format!(
                "--scope was given {name:?}, which names nothing. Leave --scope out to record \
                 that the source did not say how far it reached; that is a real answer and is \
                 not the same as this"
            )),
        },
        _ => Ok((Scope::Unstated(), arguments)),
    };
}

/// `--store <dir>`, if it leads the remaining arguments.
///
/// Read before the extractions so that a misplaced `--store` is an unexpected argument rather
/// than a concept named `--store`, which is the kind of quiet misreading a hand-written command
/// line invites.
fn Store_Root_From<'arguments>(
    arguments: &'arguments [&'arguments str],
) -> (Option<&'arguments str>, &'arguments [&'arguments str])
{
    return match arguments
    {
        [flag, root, rest @ ..] if *flag == "--store" => (Some(root), rest),
        _ => (None, arguments),
    };
}

/// A named flag's value, if the flag leads the remaining arguments.
fn Flag_From<'arguments>(
    arguments: &'arguments [&'arguments str],
    flag: &str,
) -> (Option<&'arguments str>, &'arguments [&'arguments str])
{
    return match arguments
    {
        [found, value, rest @ ..] if *found == flag => (Some(value), rest),
        _ => (None, arguments),
    };
}

/// Read `--says <concept> <claim>` groups.
///
/// # Errors
///
/// An argument that is not `--says`, and a `--says` group carrying fewer than two values.
fn Extractions_From(arguments: &[&str]) -> Result<Vec<Extraction>, String>
{
    let mut extractions = Vec::new();
    let mut remaining = arguments;

    while let Some((flag, rest)) = remaining.split_first()
    {
        if *flag != "--says"
        {
            return Err(format!("unexpected argument {flag}"));
        }

        let Some((extraction, rest)) = Said_From(rest)
        else
        {
            return Err("--says takes a concept and a claim".to_owned());
        };
        extractions.push(extraction);
        remaining = rest;
    }

    return Ok(extractions);
}

/// One `--says <concept> <claim>` group, and what follows it.
///
/// Two `split_first` calls rather than a pair of positions, because the position said `2` and
/// nothing named what the two things before it were.
///
/// Nothing rather than a complaint, because both halves fail the same way and the caller words
/// that once: a `--says` missing its claim and a `--says` missing both are one thing a reader
/// has to fix, and two copies of the sentence would be two places for it to drift.
fn Said_From<'arguments>(
    arguments: &'arguments [&'arguments str],
) -> Option<(Extraction, &'arguments [&'arguments str])>
{
    let (concept, rest) = arguments.split_first()?;
    let (claim, rest) = rest.split_first()?;

    return Some((Extraction::New((*concept).to_owned(), (*claim).to_owned()), rest));
}

/// The publication log a run records into, when it was told where.
fn Log_For(root: Option<&str>) -> Result<Option<FileRecordLog>, String>
{
    let Some(root) = root
    else
    {
        return Ok(None);
    };

    let log = FileRecordLog::At(std::path::Path::new(root).join("publications.log"))
        .map_err(|cause| return format!("cannot open the publication log: {cause}"))?;
    return Ok(Some(log));
}

/// The lines of a publication log.
///
/// # Errors
///
/// A log the medium would not hand back.
fn Records_Of(log: &FileRecordLog) -> Result<Vec<String>, String>
{
    return log
        .Records()
        .map_err(|cause| return format!("cannot read the publication log: {cause}"));
}

/// The graph a run starts from: what earlier runs published, replayed.
///
/// `D-014`: a graph is a fold over its publications, so replaying the log *is* loading the
/// graph. There is no second representation to keep in step with the log, which is the reason
/// transitions were recorded rather than versions.
///
/// # Errors
///
/// A log that cannot be read or replayed.
fn Known_So_Far(log: Option<&FileRecordLog>) -> Result<KnowledgeGraph, String>
{
    let Some(log) = log
    else
    {
        return Ok(KnowledgeGraph::Empty());
    };

    let records = Records_Of(log)?;
    return Replay(&records)
        .map_err(|cause| return format!("the publication log cannot be replayed: {cause}"));
}

/// Record what this admission published, before reporting that it did.
///
/// Before, deliberately. `D19`: a report must not outrun the work, and a run that printed its
/// counts and then failed to record them would have told a reader about knowledge the next run
/// will not have.
///
/// # Errors
///
/// A publication the medium would not take.
fn Record_Into(
    log: Option<&FileRecordLog>,
    report: &AdmissionReport,
    at: Option<i64>,
) -> Result<(), String>
{
    let Some(log) = log
    else
    {
        return Ok(());
    };

    for publication in report.Publications()
    {
        log.Append(&publication.Record(at))
            .map_err(|cause| return format!("cannot record a publication: {cause}"))?;
    }

    return Ok(());
}

/// The wall time, from the clock this composition root chose.
///
/// # Why the clock is named here and nowhere else
///
/// `kwb-platform-xvpe` adopts the port and an implementation of it; this file picks which. That
/// is what a composition root is for, and it is why `kwb-domain` takes a number rather than a
/// clock — a library that knew where the time came from would be a library with an opinion
/// about the host.
fn Now() -> i64
{
    return SystemClock.Now().Unix_Seconds();
}

/// The address the source was written under, rendered.
fn Address_Of(report: &AdmissionReport) -> String
{
    return report
        .Source()
        .map_or_else(|| return "-".to_owned(), |written| return written.Identity().Render());
}

/// `kwb history --store <dir> [--through <count>]`: the graph as of a publication count.
///
/// # Why a count and not a time
///
/// `D-012` records temporal reconstruction as met and describes it as *the version published at
/// an instant*. A publication record carries a kind, a standing, a successor, a reason and the
/// entity, and **no time at all** — so an instant cannot be asked for in any form, and what a
/// prefix of the log actually answers is *as of the first N publications*. `D-012`'s amendment
/// says so; this command is the honest version of what the mechanism supports.
///
/// # Why it is worth having as a count
///
/// `D19-B`: a global query filter rewrote every query, so `merge-audit` resolved none of the
/// merge log's identifiers and printed *"nothing has been merged away"*. `kwb-mcp` answers
/// `merge_losers`, which says what was merged; this says what the graph looked like before it.
/// Those two together are the audit that incident could not perform.
fn History_Command(arguments: &[&str]) -> ExitCode
{
    let (store_root, rest) = Store_Root_From(arguments);
    let log = match Log_For(store_root)
    {
        Ok(Some(log)) => log,
        Ok(None) => return No_Log_To_Replay(),
        Err(complaint) => return Complained("kwb history", &complaint, FAILURE_EXIT),
    };

    return match Reported_History(&log, rest)
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(exit) => exit,
    };
}

/// The refusal for a `history` run that named no log to replay.
///
/// Without `--store` there is no log, and a graph replayed from no log is empty. Reporting zero
/// concepts would be true of the value and false about the corpus, which is the distinction
/// this repository spends most of its types on.
fn No_Log_To_Replay() -> ExitCode
{
    eprintln!("kwb history: --store names where the publication log is; there is no");
    eprintln!("history without one, because history is the log replayed.");
    return ExitCode::from(USAGE_EXIT);
}

/// The graph a prefix of the log folds into, reported.
///
/// # Errors
///
/// The exit code the run ends on: a log that cannot be read or replayed, a `--through` or
/// `--as-of` this log cannot answer, or a prefix the log does not hold.
fn Reported_History(log: &FileRecordLog, rest: &[&str]) -> Result<(), ExitCode>
{
    let records = Records_Of(log)
        .map_err(|complaint| return Complained("kwb history", &complaint, FAILURE_EXIT))?;

    let through = Through_From(rest, &records)
        .map_err(|complaint| return Complained("kwb history", &complaint, USAGE_EXIT))?;

    let Some(prefix) = records.get(..through)
    else
    {
        return Err(Over_Asked(through, records.len()));
    };

    let graph = Replay(prefix)
        .map_err(|cause| return Complained("kwb history", &cause.to_string(), FAILURE_EXIT))?;

    Print_History(through, records.len(), &graph);

    return Ok(());
}

/// The refusal for a run that asked for more history than the log holds.
///
/// Refused rather than clamped, which is `KWB-60`: a run given everything when it asked for
/// more would be told the corpus is older than it is, and could not tell that from a corpus
/// that really is that old.
fn Over_Asked(through: usize, held: usize) -> ExitCode
{
    eprintln!("kwb history: {through} publications were asked for and the log holds");
    eprintln!("{held}. Refusing rather than returning what there is.");
    return ExitCode::from(USAGE_EXIT);
}

/// What a replay found, in the library's own vocabulary.
fn Print_History(through: usize, held: usize, graph: &KnowledgeGraph)
{
    println!("through    {through} of {held}");
    println!("concepts   {}", graph.Current().Concepts().len());
    println!("claims     {}", graph.Current().Claims().len());
    println!("citations  {}", graph.Current().Assertions().len());
    println!("held       {}", graph.Every_Version().Concepts().len());
}

/// `--through <count>`, defaulting to the whole log.
///
/// # Errors
///
/// A count past the end of the log. Refused rather than clamped: a run that asked for more
/// history than exists and was quietly given everything would be told the corpus is older than
/// it is, and would have no way to tell that from a corpus that really is that old.
fn Through_From(arguments: &[&str], records: &[String]) -> Result<usize, String>
{
    let through = match arguments
    {
        [] => records.len(),
        [flag, value, rest @ ..] if *flag == "--through" => Through_Count(value, rest)?,
        [flag, value, rest @ ..] if *flag == "--as-of" => As_Of_Count(value, rest, records)?,
        _ => return Err(format!("unexpected argument {:?}", arguments.first())),
    };

    if through > records.len()
    {
        return Err(format!(
            "--through {through} was asked for and the log holds {} publications",
            records.len()
        ));
    }

    return Ok(through);
}

/// The count a `--through` flag carries.
///
/// # Errors
///
/// A value that is not a count, and anything the flag left behind.
fn Through_Count(value: &str, rest: &[&str]) -> Result<usize, String>
{
    Nothing_Left(rest)?;

    return value
        .parse::<usize>()
        .map_err(|_| return format!("--through takes a count, and {value:?} is not one"));
}

/// The count a `--as-of` flag asks for, by way of the time it named.
///
/// # Errors
///
/// A value that is not unix seconds, anything the flag left behind, and a log in which nothing
/// carries a time.
fn As_Of_Count(value: &str, rest: &[&str], records: &[String]) -> Result<usize, String>
{
    Nothing_Left(rest)?;

    let asked = value.parse::<i64>().map_err(|_| {
        return format!("--as-of takes a time in unix seconds, and {value:?} is not one");
    })?;

    return Through_Time(records, asked);
}

/// Refuse anything a flag did not claim.
///
/// # Errors
///
/// The first argument no flag took. Both of this command's flags take one value, so a second
/// one is a command line whose author meant something `history` does not do.
fn Nothing_Left(rest: &[&str]) -> Result<(), String>
{
    let Some(left) = rest.first()
    else
    {
        return Ok(());
    };

    return Err(format!("unexpected argument {left}"));
}

/// How many publications had happened by a time.
///
/// # Why a log with no timestamps is refused rather than answered
///
/// A publication written before `KWB-64` carries no time, and `Published_At` reports that as
/// unknown rather than as an epoch. A log made entirely of those cannot place anything in time,
/// so every answer would be the same answer whatever was asked — which is a tool agreeing with
/// the question instead of answering it. `D-012`'s amendment is about exactly this distinction
/// between a value and the absence of one.
///
/// A log that carries *some* times answers for those, and the untimed prefix stays included:
/// those publications did happen before the first timed one, which is the only thing about them
/// that is known.
///
/// # Errors
///
/// A log in which nothing is timestamped.
fn Through_Time(records: &[String], asked: i64) -> Result<usize, String>
{
    let timed = records.iter().filter(|record| return Published_At(record).is_some()).count();
    if timed == 0
    {
        return Err(format!(
            "nothing in this log carries a time, so as-of {asked} cannot be answered. Every \
             publication here predates timestamps; --through takes a count, which this log can \
             answer"
        ));
    }

    // The log is append-only and written in order, so the publications that had happened by a
    // time are a prefix. Counting them rather than filtering keeps that true: `Replay` refuses a
    // record naming something no earlier record published, and a filter could drop a concept
    // while keeping the claim about it.
    let through = records
        .iter()
        .take_while(|record| return Published_At(record).is_none_or(|at| return at <= asked))
        .count();

    return Ok(through);
}

/// `kwb retire <concept> --store <dir> --because <reason>`
/// `kwb supersede <concept> --into <concept> --store <dir> --because <reason>`
///
/// # Why a reason is required rather than optional
///
/// `D17`: destruction requires evidence, and the question it demands be answerable before a
/// delete, deprecate, supersede or overwrite is *what belief authorises this, and what would
/// falsify it*. A closure with no recorded reason cannot answer either half, and the prototype
/// found **144 of 579 merges** wrong months later only because it had a log to re-read.
fn Close_Command(arguments: &[&str], merging: Option<()>) -> ExitCode
{
    let Some((name, rest)) = arguments.split_first()
    else
    {
        return Wrong_Command_Line("expected a concept");
    };

    let closing = match Closing_From(name, rest, merging)
    {
        Ok(closing) => closing,
        Err(usage) => return usage,
    };

    return match Closed(&closing)
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(complaint) => Complained("kwb", &complaint, FAILURE_EXIT),
    };
}

/// A closure as its command line described it: what is closed, what it is closed into, why, and
/// where that is recorded.
struct Closing<'arguments>
{
    /// The concept being closed.
    name: &'arguments str,

    /// The successor `--into` named, when the verb merges.
    successor: Option<&'arguments str>,

    /// The reason `--because` gave, which `D17` requires rather than accepts.
    because: &'arguments str,

    /// Where `--store` said the publication log is.
    store_root: &'arguments str,

    /// `Some(())` for `supersede`, `None` for `retire`.
    merging: Option<()>,
}

/// The closure a command line asks for.
///
/// # Errors
///
/// The usage for one that asks for too little, which is what [`Required_Of`] answers.
fn Closing_From<'arguments>(
    name: &'arguments str,
    arguments: &'arguments [&'arguments str],
    merging: Option<()>,
) -> Result<Closing<'arguments>, ExitCode>
{
    let (successor, rest) = Flag_From(arguments, "--into");
    let (store_root, rest) = Store_Root_From(rest);
    let (because, rest) = Flag_From(rest, "--because");

    let (because, store_root) = Required_Of(because, store_root, rest)?;

    return Ok(Closing {
        name,
        successor,
        because,
        store_root,
        merging,
    });
}

/// The reason and the place to record it, both of which a closure cannot do without.
///
/// Anything the flags left behind is refused here too, so that the three ways a closing command
/// line can be wrong are answered in one place rather than three that have drifted apart.
///
/// # Errors
///
/// The usage for a command line carrying an argument no flag claimed, no reason, or nowhere to
/// record the closure.
fn Required_Of<'arguments>(
    because: Option<&'arguments str>,
    store_root: Option<&'arguments str>,
    rest: &[&str],
) -> Result<(&'arguments str, &'arguments str), ExitCode>
{
    if let Err(complaint) = Nothing_Left(rest)
    {
        return Err(Wrong_Command_Line(&complaint));
    }

    let Some(because) = because.filter(|reason| return !reason.trim().is_empty())
    else
    {
        return Err(Wrong_Command_Line(
            "--because is required. D17: destruction requires evidence",
        ));
    };

    let Some(store_root) = store_root
    else
    {
        return Err(Wrong_Command_Line(
            "--store is required; closing a concept nothing keeps changes nothing",
        ));
    };

    return Ok((because, store_root));
}

/// A closure, applied to what was known and recorded before it is reported.
///
/// # Errors
///
/// The first complaint of the three steps that can make one, in their order: a log that cannot
/// be opened or replayed, a closure `D17` refuses, and a record the medium would not take.
fn Closed(closing: &Closing) -> Result<(), String>
{
    let recorded = Recorded_At(Some(closing.store_root))?;
    let applied = Applied_To(closing, &recorded.known)?;

    // Recorded before anything is printed. `D19`: a report must not outrun the work, and a
    // closure announced but not recorded is one the next run will not know about.
    Record_Closure(recorded.log.as_ref(), &applied.publication)?;

    Print_Closure(closing.name, &applied.after);

    return Ok(());
}

/// A closure, and the graph it produces.
struct Applied
{
    /// The closure as the transition `D-014` records, rather than as the graph it produces.
    publication: Publication,

    /// The graph after it.
    after: KnowledgeGraph,
}

/// A closure applied to what was known.
///
/// Applied to what was known, so the count reported is the corpus after this closure and not
/// this closure in isolation. The previous graph is untouched, which is what makes the state
/// before a merge a thing that was kept.
///
/// # Errors
///
/// The complaint `D17` produces when it refuses this closure.
fn Applied_To(closing: &Closing, known: &KnowledgeGraph) -> Result<Applied, String>
{
    let concept = Concept::Named(closing.name);
    let standing = Closing_Standing(closing, known, &concept)?;

    let after = known.With_Concept(Versioned::Asserted(concept.clone()).Closed(standing.clone()));
    let publication = Publication::Concept { concept, standing };

    return Ok(Applied { publication, after });
}

/// Record the closure, when there is a log to record it in.
///
/// # Errors
///
/// A log the medium refused to append to. `D19`: a report must not outrun the work, and a
/// closure announced but not recorded is one the next run will not know about.
fn Record_Closure(log: Option<&FileRecordLog>, publication: &Publication) -> Result<(), String>
{
    let Some(log) = log
    else
    {
        return Ok(());
    };

    return log
        .Append(&publication.Record(None))
        .map_err(|cause| return format!("cannot record the closure: {cause}"));
}

/// What a closure left behind, in the library's own vocabulary.
fn Print_Closure(name: &str, after: &KnowledgeGraph)
{
    println!("closed     {name}");
    println!("current    {}", after.Current().Concepts().len());
    println!("held       {}", after.Every_Version().Concepts().len());
}

/// The standing a closure produces, refusing the merges `D17` would not authorise.
///
/// # Errors
///
/// A `supersede` with no `--into`, and whatever [`Merged_Into`] refuses about the successor it
/// was given.
fn Closing_Standing(
    closing: &Closing,
    known: &KnowledgeGraph,
    concept: &Concept,
) -> Result<Standing, String>
{
    if closing.merging.is_none()
    {
        return Ok(Standing::Retired {
            because: closing.because.to_owned(),
        });
    }

    let Some(successor) = closing.successor
    else
    {
        return Err("supersede needs --into <concept>".to_owned());
    };

    return Merged_Into(successor, closing, known, concept);
}

/// The standing a supersede produces, once its successor is known to be a real one.
///
/// # Errors
///
/// A concept that would supersede itself, and a successor nothing published.
fn Merged_Into(
    successor: &str,
    closing: &Closing,
    known: &KnowledgeGraph,
    concept: &Concept,
) -> Result<Standing, String>
{
    let into = Concept::Named(successor);
    if into.Identity() == concept.Identity()
    {
        return Err("a concept cannot supersede itself".to_owned());
    }

    if !Held_By(known, &into)
    {
        return Err(format!("nothing published a concept named {successor} to merge into"));
    }

    return Ok(Standing::Superseded {
        by: into.Identity(),
        because: closing.because.to_owned(),
    });
}

/// Whether the graph holds a concept with this identity.
///
/// The successor has to be one the graph holds. A merge into something nobody published is a
/// merge whose successor cannot be resolved, which is how `merge-audit` came to resolve none of
/// the merge log and report that nothing had been merged away.
fn Held_By(known: &KnowledgeGraph, into: &Concept) -> bool
{
    return known
        .Every_Version()
        .Concepts()
        .iter()
        .any(|candidate| return candidate.Value().Identity() == into.Identity());
}

/// What this tool does, including the half it does not have.
///
/// # Why the table is rendered here and the prose is not
///
/// The loop below is the one thing this function knows that `VERBS` does not: the first usage
/// line carries the `usage:` label and the rest align under it, which is a fact about layout
/// rather than about what the binary can do. Everything under it is prose a reader reads once,
/// and it is written out in the helpers so that this function reads as what it is — a renderer
/// of the table plus a list of the sections shown after it.
fn Print_Usage()
{
    // Rendered from `VERBS`, not restated beside it. The first line carries the `usage:` label
    // and the rest align under it, which is the only thing this loop knows that the table does
    // not -- and it is a fact about layout rather than about what the binary can do.
    for (position, verb) in VERBS.iter().enumerate()
    {
        let label = if position == 0 { "usage:" } else { "      " };
        eprintln!("{label} kwb {}", verb.usage);
    }

    Print_History_Help();
    Print_Closure_Help();
    Print_Admission_Help();
    Print_Extractor_Help();
    Print_Coverage_Help();
    Print_Store_Help();
    Print_Citation_Help();
}

/// The help for `history`, which is the one verb whose flags are not visible from its table row.
fn Print_History_Help()
{
    eprintln!();
    eprintln!("  history replays the publication log and reports the graph as it was. --through");
    eprintln!("  takes a count of publications, --as-of takes a time; given neither it reports");
    eprintln!("  all of it. Asking for more history than the log holds is refused, and so is");
    eprintln!("  --as-of on a log written before publications carried a time -- every answer");
    eprintln!("  would otherwise be the same answer whatever was asked.");
}

/// The help for the two closings, where `D17` is explained rather than cited.
fn Print_Closure_Help()
{
    eprintln!();
    eprintln!("  retire and supersede close a concept, and --because is required rather than");
    eprintln!("  optional. D17: destruction requires evidence, and the question it demands be");
    eprintln!("  answerable is what belief authorises this. The prototype found 144 of 579");
    eprintln!("  merges wrong months later, and only because it had a log to re-read.");
}

/// The help for `admit`, which is the verb a reader is most likely to try first.
fn Print_Admission_Help()
{
    eprintln!();
    eprintln!("  Admits a file: the bytes are written to the content-addressed store and");
    eprintln!("  whatever is supplied by --says is linked, normalized and published.");
}

/// The help for the half of admission this workspace cannot do yet.
///
/// It said *There is no extractor* until `KWB-67`, which `KWB-66` had made false — so the two
/// surfaces contradicted each other, introduced by the session that had closed that defect five
/// times. What replaced it names the flag instead, and `tests/help.rs` asserts on that flag's
/// presence rather than on this sentence, so an honest rephrasing is not a failure.
fn Print_Extractor_Help()
{
    eprintln!();
    eprintln!("  This command does not read the document. An extractor exists -- kwb-extract");
    eprintln!("  reads a source and proposes what it says -- and it has no provider in this");
    eprintln!("  workspace, so there is nothing here for a flag to invoke. Until one arrives,");
    eprintln!("  --says is how a passage's claims get in.");
}

/// The help for what a run reports when it was given nothing to examine.
fn Print_Coverage_Help()
{
    eprintln!();
    eprintln!("  A run given no --says reports coverage `unmet` -- it was never given anything");
    eprintln!("  to examine -- which is not the same as `barren`, which means it looked and");
    eprintln!("  found nothing.");
}

/// The help for `--store`, which decides whether anything outlives the process.
fn Print_Store_Help()
{
    eprintln!();
    eprintln!("  --store <dir> keeps both halves: the document bytes as one file per address,");
    eprintln!("  and what was published as an append-only log. A run replays that log before");
    eprintln!("  it admits, so the counts below include what earlier runs learned.");
    eprintln!();
    eprintln!("  Without it nothing is kept and the run says so.");
}

/// The help for what a claim is anchored to once it is admitted.
fn Print_Citation_Help()
{
    eprintln!();
    eprintln!("  Every admitted claim gets a citation naming the document's content address,");
    eprintln!("  so following it returns the exact bytes the claim was read out of. --scope");
    eprintln!("  says how far the source claims it reaches; leaving it out is an answer, not");
    eprintln!("  a default -- a source that did not say has not said the narrowest thing.");
}
