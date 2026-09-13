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

fn main() -> ExitCode
{
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();

    return match borrowed.split_first()
    {
        Some((&"admit", rest)) => Admit_Command(rest),
        Some((&"retire", rest)) => Close_Command(rest, None),
        Some((&"supersede", rest)) => Close_Command(rest, Some(())),
        Some((&"history", rest)) => History_Command(rest),
        Some((&"help" | &"--help" | &"-h", _)) =>
        {
            Print_Usage();
            ExitCode::SUCCESS
        }
        _ =>
        {
            eprintln!("kwb: expected a verb");
            Print_Usage();
            ExitCode::from(USAGE_EXIT)
        }
    };
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

    let (store_root, scope, extractions) = match Arguments_Of_Admit(rest)
    {
        Ok(parsed) => parsed,
        Err(complaint) =>
        {
            eprintln!("kwb admit: {complaint}");
            Print_Usage();
            return ExitCode::from(USAGE_EXIT);
        }
    };

    let bytes = match std::fs::read(path)
    {
        Ok(bytes) => bytes,
        Err(cause) =>
        {
            eprintln!("kwb admit: cannot read {path}: {cause}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    let mut store = match Store_For(store_root)
    {
        Ok(store) => store,
        Err(complaint) =>
        {
            eprintln!("kwb admit: {complaint}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };
    let durable = store.Is_Durable();
    let log = match Log_For(store_root)
    {
        Ok(log) => log,
        Err(complaint) =>
        {
            eprintln!("kwb admit: {complaint}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };
    let known = match Known_So_Far(log.as_ref())
    {
        Ok(known) => known,
        Err(complaint) =>
        {
            eprintln!("kwb admit: {complaint}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };
    // `--says` is a person stating what a passage asserts, which is exactly what `Stated` is.
    // Routing it through the extraction seam rather than handing `Admit` a list means the
    // command line uses the same door a reading adapter will, and that the protocol and the
    // reader are recorded instead of being implied by the fact that somebody typed them.
    let said = Stated::Of(
        extractions,
        SourceLocation::Named("as stated on the command line"),
        ExtractionLineage::Of("stated-by-a-person", "the operator of kwb admit"),
        scope,
    );
    let reader = said.as_ref().map(|said| return said as &dyn ExtractionStrategy);

    // Text, because that is what a person reading a file to type `--says` was doing. A source
    // needing visual reading is a judgement no part of this command can make, and saying `Text`
    // here does not assert otherwise -- `Stated` refuses no kind, so the value reaches nothing
    // that acts on it. It becomes load-bearing when a reader that can refuse arrives.
    let report = match Admit(bytes, reader, ReadingKind::Text, &mut store)
    {
        Ok(report) => report,
        Err(refusal) =>
        {
            eprintln!("kwb admit: {refusal}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    let published = match Record_Into(log.as_ref(), &report, Some(Now()))
    {
        Ok(()) => report.Published_Into(&known),
        Err(complaint) =>
        {
            eprintln!("kwb admit: {complaint}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    // The library's own vocabulary, not a second one. `Name` is the same string a journal
    // would carry, so a person reading a terminal and a report reading a file are told the
    // same thing about the same run.
    println!("source     {}", Address_Of(&report));
    println!("coverage   {}", report.Coverage().Name());
    println!("concepts   {}", published.Current().Concepts().len());
    println!("claims     {}", published.Current().Claims().len());
    println!("citations  {}", published.Current().Assertions().len());
    println!("refused    {}", report.Normalized().Linked().Refused());
    println!("documents  {}", if durable { "kept" } else { "in memory only" });
    println!("knowledge  {}", if log.is_some() { "kept" } else { "in memory only" });

    if let Some(refusal) = report.Refusal()
    {
        // The library's own words for why, rather than one sentence covering every reason a
        // reading did not happen. A person who passed `--says` and still sees this needs to
        // know it was not their omission.
        println!();
        println!("{refusal}.");
        if matches!(refusal, ExtractionRefused::NotRead)
        {
            println!("Pass --says to supply what a passage asserts; this tool does not read");
            println!("the document and decide for itself.");
        }
    }

    return ExitCode::SUCCESS;
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
        Ok(None) =>
        {
            eprintln!("kwb history: --store names where the publication log is; there is no");
            eprintln!("history without one, because history is the log replayed.");
            return ExitCode::from(USAGE_EXIT);
        }
        Err(complaint) =>
        {
            eprintln!("kwb history: {complaint}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    let records = match log.Records()
    {
        Ok(records) => records,
        Err(cause) =>
        {
            eprintln!("kwb history: cannot read the publication log: {cause}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    let through = match Through_From(rest, &records)
    {
        Ok(through) => through,
        Err(complaint) =>
        {
            eprintln!("kwb history: {complaint}");
            return ExitCode::from(USAGE_EXIT);
        }
    };

    // The prefix, and nothing else. `D-014`: the graph at a point is a fold over the
    // publications up to it, so this is the same `Replay` every other caller uses and not a
    // second mechanism that could disagree with it.
    let Some(prefix) = records.get(..through)
    else
    {
        eprintln!("kwb history: {through} publications were asked for and the log holds");
        eprintln!("{}. Refusing rather than returning what there is.", records.len());
        return ExitCode::from(USAGE_EXIT);
    };

    let graph = match Replay(prefix)
    {
        Ok(graph) => graph,
        Err(cause) =>
        {
            eprintln!("kwb history: the publication log cannot be replayed: {cause}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    println!("through    {through} of {}", records.len());
    println!("concepts   {}", graph.Current().Concepts().len());
    println!("claims     {}", graph.Current().Claims().len());
    println!("citations  {}", graph.Current().Assertions().len());
    println!("held       {}", graph.Every_Version().Concepts().len());

    return ExitCode::SUCCESS;
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
    let held = records.len();
    let through = match arguments
    {
        [flag, count, rest @ ..] if *flag == "--through" =>
        {
            if !rest.is_empty()
            {
                return Err(format!("unexpected argument {:?}", rest.first()));
            }
            count
                .parse::<usize>()
                .map_err(|_| return format!("--through takes a count, and {count:?} is not one"))?
        }
        [flag, time, rest @ ..] if *flag == "--as-of" =>
        {
            if !rest.is_empty()
            {
                return Err(format!("unexpected argument {:?}", rest.first()));
            }
            let asked = time.parse::<i64>().map_err(|_| {
                return format!("--as-of takes a time in unix seconds, and {time:?} is not one");
            })?;
            return Through_Time(records, asked);
        }
        [] => held,
        _ => return Err(format!("unexpected argument {:?}", arguments.first())),
    };

    if through > held
    {
        return Err(format!(
            "--through {through} was asked for and the log holds {held} publications"
        ));
    }

    return Ok(through);
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
        eprintln!("kwb: expected a concept");
        Print_Usage();
        return ExitCode::from(USAGE_EXIT);
    };

    let (successor, rest) = Flag_From(rest, "--into");
    let (store_root, rest) = Store_Root_From(rest);
    let (because, rest) = Flag_From(rest, "--because");

    if !rest.is_empty()
    {
        eprintln!("kwb: unexpected argument {}", rest.first().unwrap_or(&""));
        Print_Usage();
        return ExitCode::from(USAGE_EXIT);
    }

    let Some(because) = because.filter(|reason| return !reason.trim().is_empty())
    else
    {
        eprintln!("kwb: --because is required. D17: destruction requires evidence");
        Print_Usage();
        return ExitCode::from(USAGE_EXIT);
    };

    let Some(store_root) = store_root
    else
    {
        eprintln!("kwb: --store is required; closing a concept nothing keeps changes nothing");
        Print_Usage();
        return ExitCode::from(USAGE_EXIT);
    };

    let log = match Log_For(Some(store_root))
    {
        Ok(log) => log,
        Err(complaint) =>
        {
            eprintln!("kwb: {complaint}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };
    let known = match Known_So_Far(log.as_ref())
    {
        Ok(known) => known,
        Err(complaint) =>
        {
            eprintln!("kwb: {complaint}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    let concept = Concept::Named(name);
    let standing = match Closing_Standing(merging, successor, because, &known, &concept)
    {
        Ok(standing) => standing,
        Err(complaint) =>
        {
            eprintln!("kwb: {complaint}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    // Applied to what was known, so the count reported is the corpus after this closure and
    // not this closure in isolation. The previous graph is untouched, which is what makes the
    // state before a merge a thing that was kept.
    let after = known.With_Concept(Versioned::Asserted(concept.clone()).Closed(standing.clone()));
    let publication = Publication::Concept { concept, standing };

    // Recorded before anything is printed. `D19`: a report must not outrun the work, and a
    // closure announced but not recorded is one the next run will not know about.
    if let Some(log) = log.as_ref()
    {
        if let Err(cause) = log.Append(&publication.Record(None))
        {
            eprintln!("kwb: cannot record the closure: {cause}");
            return ExitCode::from(FAILURE_EXIT);
        }
    }

    println!("closed     {name}");
    println!("current    {}", after.Current().Concepts().len());
    println!("held       {}", after.Every_Version().Concepts().len());
    return ExitCode::SUCCESS;
}

/// The standing a closure produces, refusing the merges `D17` would not authorise.
fn Closing_Standing(
    merging: Option<()>,
    successor: Option<&str>,
    because: &str,
    known: &KnowledgeGraph,
    concept: &Concept,
) -> Result<Standing, String>
{
    if merging.is_none()
    {
        return Ok(Standing::Retired {
            because: because.to_owned(),
        });
    }

    let Some(successor) = successor
    else
    {
        return Err("supersede needs --into <concept>".to_owned());
    };

    let into = Concept::Named(successor);
    if into.Identity() == concept.Identity()
    {
        return Err("a concept cannot supersede itself".to_owned());
    }

    // The successor has to be one the graph holds. A merge into something nobody published is
    // a merge whose successor cannot be resolved, which is how `merge-audit` came to resolve
    // none of the merge log and report that nothing had been merged away.
    let held = known
        .Every_Version()
        .Concepts()
        .iter()
        .any(|candidate| return candidate.Value().Identity() == into.Identity());
    if !held
    {
        return Err(format!("nothing published a concept named {successor} to merge into"));
    }

    return Ok(Standing::Superseded {
        by: into.Identity(),
        because: because.to_owned(),
    });
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

/// The graph a run starts from: what earlier runs published, replayed.
///
/// `D-014`: a graph is a fold over its publications, so replaying the log *is* loading the
/// graph. There is no second representation to keep in step with the log, which is the reason
/// transitions were recorded rather than versions.
fn Known_So_Far(log: Option<&FileRecordLog>) -> Result<KnowledgeGraph, String>
{
    let Some(log) = log
    else
    {
        return Ok(KnowledgeGraph::Empty());
    };

    let records = log
        .Records()
        .map_err(|cause| return format!("cannot read the publication log: {cause}"))?;
    return Replay(&records)
        .map_err(|cause| return format!("the publication log cannot be replayed: {cause}"));
}

/// Record what this admission published, before reporting that it did.
///
/// Before, deliberately. `D19`: a report must not outrun the work, and a run that printed its
/// counts and then failed to record them would have told a reader about knowledge the next run
/// will not have.
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

/// Read `--says <concept> <claim>` groups.
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

        let (Some(concept), Some(claim)) = (rest.first(), rest.get(1))
        else
        {
            return Err("--says takes a concept and a claim".to_owned());
        };

        extractions.push(Extraction::New((*concept).to_owned(), (*claim).to_owned()));
        remaining = rest.get(2..).unwrap_or_default();
    }

    return Ok(extractions);
}

/// What this tool does, including the half it does not have.
fn Print_Usage()
{
    eprintln!(
        "usage: kwb admit <file> [--store <dir>] [--scope <name>] [--says <concept> <claim>]..."
    );
    eprintln!("       kwb retire <concept> --store <dir> --because <reason>");
    eprintln!("       kwb supersede <concept> --into <concept> --store <dir> --because <reason>");
    eprintln!(
        "       kwb history --store <dir> [--through <count> | --as-of <unix seconds>]"
    );
    eprintln!();
    eprintln!("  history replays the publication log and reports the graph as it was. --through");
    eprintln!("  takes a count of publications, --as-of takes a time; given neither it reports");
    eprintln!("  all of it. Asking for more history than the log holds is refused, and so is");
    eprintln!("  --as-of on a log written before publications carried a time -- every answer");
    eprintln!("  would otherwise be the same answer whatever was asked.");
    eprintln!();
    eprintln!("  retire and supersede close a concept, and --because is required rather than");
    eprintln!("  optional. D17: destruction requires evidence, and the question it demands be");
    eprintln!("  answerable is what belief authorises this. The prototype found 144 of 579");
    eprintln!("  merges wrong months later, and only because it had a log to re-read.");
    eprintln!();
    eprintln!("  Admits a file: the bytes are written to the content-addressed store and");
    eprintln!("  whatever is supplied by --says is linked, normalized and published.");
    eprintln!();
    eprintln!("  This command does not read the document. An extractor exists -- kwb-extract");
    eprintln!("  reads a source and proposes what it says -- and it has no provider in this");
    eprintln!("  workspace, so there is nothing here for a flag to invoke. Until one arrives,");
    eprintln!("  --says is how a passage's claims get in.");
    eprintln!();
    eprintln!("  A run given no --says reports coverage `unmet` -- it was never given anything");
    eprintln!("  to examine -- which is not the same as `barren`, which means it looked and");
    eprintln!("  found nothing.");
    eprintln!();
    eprintln!("  --store <dir> keeps both halves: the document bytes as one file per address,");
    eprintln!("  and what was published as an append-only log. A run replays that log before");
    eprintln!("  it admits, so the counts below include what earlier runs learned.");
    eprintln!();
    eprintln!("  Without it nothing is kept and the run says so.");
    eprintln!();
    eprintln!("  Every admitted claim gets a citation naming the document's content address,");
    eprintln!("  so following it returns the exact bytes the claim was read out of. --scope");
    eprintln!("  says how far the source claims it reaches; leaving it out is an answer, not");
    eprintln!("  a default -- a source that did not say has not said the narrowest thing.");
}
