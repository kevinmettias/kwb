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

use kwb_domain::{Concept, KnowledgeGraph, Publication, Replay, Scope, Standing, Versioned};
use kwb_ingest::{Admit, AdmissionReport, Extraction};
use kwb_platform::RecordLogStrategy;
use kwb_platform_std::{DirectoryContentStore, FileRecordLog};
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

    let (store_root, rest) = Store_Root_From(rest);
    let (scope, rest) = Scope_From(rest);
    let extractions = match Extractions_From(rest)
    {
        Ok(extractions) => extractions,
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
    let report = match Admit(bytes, &extractions, &scope, &mut store)
    {
        Ok(report) => report,
        Err(refusal) =>
        {
            eprintln!("kwb admit: {refusal}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    let published = match Record_Into(log.as_ref(), &report)
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

    if !report.Coverage().Was_Run()
    {
        println!();
        println!("Nothing was examined. Pass --says to supply what a passage asserts;");
        println!("this tool does not read the document and decide for itself.");
    }

    return ExitCode::SUCCESS;
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
        if let Err(cause) = log.Append(&publication.Record())
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

/// `--scope <name>`, if it leads the remaining arguments.
///
/// An absent scope is a real answer and not a default. `D-010`: a source that did not say how
/// far it meant has not said the narrowest thing, so an unstated scope stays distinguishable
/// from every stated one rather than being filled in with a guess.
fn Scope_From<'arguments>(arguments: &'arguments [&'arguments str])
    -> (Scope, &'arguments [&'arguments str])
{
    return match arguments
    {
        [flag, name, rest @ ..] if *flag == "--scope" => (Scope::Named(name), rest),
        _ => (Scope::Named(""), arguments),
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
fn Record_Into(log: Option<&FileRecordLog>, report: &AdmissionReport) -> Result<(), String>
{
    let Some(log) = log
    else
    {
        return Ok(());
    };

    for publication in report.Publications()
    {
        log.Append(&Publication::Record(&publication))
            .map_err(|cause| return format!("cannot record a publication: {cause}"))?;
    }

    return Ok(());
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
    eprintln!();
    eprintln!("  retire and supersede close a concept, and --because is required rather than");
    eprintln!("  optional. D17: destruction requires evidence, and the question it demands be");
    eprintln!("  answerable is what belief authorises this. The prototype found 144 of 579");
    eprintln!("  merges wrong months later, and only because it had a log to re-read.");
    eprintln!();
    eprintln!("  Admits a file: the bytes are written to the content-addressed store and");
    eprintln!("  whatever is supplied by --says is linked, normalized and published.");
    eprintln!();
    eprintln!("  There is no extractor. Nothing here reads the document and decides what it");
    eprintln!("  says, so --says is how a passage's claims get in. A run given none reports");
    eprintln!("  coverage `unmet` -- it was never given anything to examine -- which is not");
    eprintln!("  the same as `barren`, which means it looked and found nothing.");
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
