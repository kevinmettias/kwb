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
//! **Nothing survives the process.** The store and the graph are in memory. `D-012` deferred
//! durability and named the condition that would settle it — a consumer that actually loses
//! something when the process ends — and this is now that consumer. It does not answer the
//! question; it is the thing whose absence made it unanswerable.

use std::process::ExitCode;

use kwb_domain::KnowledgeGraph;
use kwb_ingest::{Admit, AdmissionReport, Extraction};
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

    let mut store = DocumentStore::Empty();
    let report = match Admit(bytes, &extractions, &mut store)
    {
        Ok(report) => report,
        Err(refusal) =>
        {
            eprintln!("kwb admit: {refusal}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    let graph = report.Published_Into(&KnowledgeGraph::Empty());

    // The library's own vocabulary, not a second one. `Name` is the same string a journal
    // would carry, so a person reading a terminal and a report reading a file are told the
    // same thing about the same run.
    println!("source     {}", Address_Of(&report));
    println!("coverage   {}", report.Coverage().Name());
    println!("concepts   {}", graph.Current().Concepts().len());
    println!("claims     {}", graph.Current().Claims().len());
    println!("refused    {}", report.Normalized().Linked().Refused());

    if !report.Coverage().Was_Run()
    {
        println!();
        println!("Nothing was examined. Pass --says to supply what a passage asserts;");
        println!("this tool does not read the document and decide for itself.");
    }

    return ExitCode::SUCCESS;
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
    eprintln!("usage: kwb admit <file> [--says <concept> <claim>]...");
    eprintln!();
    eprintln!("  Admits a file: the bytes are written to the content-addressed store and");
    eprintln!("  whatever is supplied by --says is linked, normalized and published.");
    eprintln!();
    eprintln!("  There is no extractor. Nothing here reads the document and decides what it");
    eprintln!("  says, so --says is how a passage's claims get in. A run given none reports");
    eprintln!("  coverage `unmet` -- it was never given anything to examine -- which is not");
    eprintln!("  the same as `barren`, which means it looked and found nothing.");
    eprintln!();
    eprintln!("  Nothing survives the process. Durability is undecided; see D-012.");
}
