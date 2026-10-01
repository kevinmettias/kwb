//! What `kwb help` prints, and what every refusal shows under its own name.
//!
//! # Why the table is rendered and the prose is written out
//!
//! [`Print_Usage`] loops over [`VERBS`] rather than listing the verbs, because `KWB-73` measured
//! what two lists cost: `history` was dispatched while the help named it nowhere, and the help
//! went on saying *there is no extractor* after `KWB-66` built one. The loop is the one thing
//! this module knows that the table does not — the first usage line carries the `usage:` label
//! and the rest align under it, which is a fact about layout rather than about the binary.
//!
//! Everything under it is prose a reader reads once. It is written out in helpers so that the
//! renderer reads as what it is: the table, plus a list of the sections shown after it.
//!
//! [`VERBS`]: crate::VERBS

/// What this tool does, including the half it does not have.
pub(crate) fn Print_Usage()
{
    use crate::VERBS;

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
