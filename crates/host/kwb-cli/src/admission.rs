//! `kwb admit`: one file through the pipeline, from the flags that describe it to the bytes they
//! are about.
//!
//! # Where the report is printed, and why not here
//!
//! [`Run_Admission`] ends in [`Report_Admission`], which stays in the crate root. The report is
//! the composition root's statement about what its own run did — the same eight fields the
//! README's transcripts show — and the crate root is where a reader looks for it.
//!
//! [`Report_Admission`]: crate::Report_Admission

use std::process::ExitCode;

use kwb_domain::Scope;
use kwb_ingest::{
    Admit_Source, AdmissionReport, ClaimText, ConceptName, Extraction, ExtractionLineage,
    ExtractionStrategy, ReaderName, ReadingKind, ReadingProtocol, SourceLocation, Stated,
};

use crate::arguments::{LeadingFlag, Store_Root_From_Arguments};
use crate::keeping::{Kept, Kept_At, Now, Record_Into};
use crate::refusals::{Complained_Without_Usage, Complained_With_Usage};
use crate::{Persistence, Report_Admission, FAILURE_EXIT};

/// One admission, from the flags that describe it to the bytes they are about.
///
/// Split from the dispatch that reaches it because the flags, the file and the store are three
/// ways to refuse a run — one of them a wrong command line and two of them failed runs — and a
/// reader checking that each ends the way it should had to hold all of them at once.
pub(crate) fn Run_Admission(path: &str, rest: &[&str]) -> ExitCode
{
    let (store_root, scope, extractions) = match Arguments_Of_Admit(rest)
    {
        Ok(parsed) => parsed,
        Err(complaint) => return Complained_With_Usage("kwb admit", &complaint),
    };

    let bytes = match Bytes_Of(path)
    {
        Ok(bytes) => bytes,
        Err(complaint) => return Complained_Without_Usage("kwb admit", &complaint, FAILURE_EXIT),
    };

    let mut kept = match Kept_At(store_root)
    {
        Ok(kept) => kept,
        Err(complaint) => return Complained_Without_Usage("kwb admit", &complaint, FAILURE_EXIT),
    };

    let said = Reader_For(extractions, scope);
    return Ran_Admission(bytes, said.as_ref(), &mut kept);
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
    let LeadingFlag { value: store_root, rest } = Store_Root_From_Arguments(arguments);
    let (scope, rest) = Scope_From_Arguments(rest)?;
    let extractions = Extractions_From_Arguments(rest)?;

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
fn Scope_From_Arguments<'arguments>(
    arguments: &'arguments [&'arguments str],
) -> Result<(Scope, &'arguments [&'arguments str]), String>
{
    return match arguments
    {
        [flag, name, rest @ ..] if *flag == "--scope" => match Scope::Named(name)
        {
            Some(scope) => Ok((scope, rest)),
            None => Err(format!(
                concat!(
                    "--scope was given {name:?}, which names nothing. Leave --scope out to ",
                    "record that the source did not say how far it reached; that is a real ",
                    "answer and is not the same as this"
                ),
                name = name
            )),
        },
        _ => Ok((Scope::Unstated(), arguments)),
    };
}

/// Read `--says <concept> <claim>` groups.
///
/// # Errors
///
/// An argument that is not `--says`, and a `--says` group carrying fewer than two values.
fn Extractions_From_Arguments(arguments: &[&str]) -> Result<Vec<Extraction>, String>
{
    let mut extractions = Vec::new();
    let mut remaining = arguments;

    while let Some((flag, rest)) = remaining.split_first()
    {
        if *flag != "--says"
        {
            return Err(format!("unexpected argument {flag}"));
        }

        let Some((extraction, rest)) = Said_From_Arguments(rest)
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
fn Said_From_Arguments<'arguments>(
    arguments: &'arguments [&'arguments str],
) -> Option<(Extraction, &'arguments [&'arguments str])>
{
    let (concept, rest) = arguments.split_first()?;
    let (claim, rest) = rest.split_first()?;

    return Some((
        Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim)),
        rest,
    ));
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
/// Routing it through the extraction seam rather than handing `Admit_Source` a list means the command
/// line uses the same door a reading adapter will, and that the protocol and the reader are
/// recorded instead of being implied by the fact that somebody typed them.
fn Reader_For(extractions: Vec<Extraction>, scope: Scope) -> Option<Stated>
{
    let location = SourceLocation::Named("as stated on the command line");
    let lineage = ExtractionLineage::Of(
        ReadingProtocol::Named("stated-by-a-person"),
        ReaderName::Named("the operator of kwb admit"),
    );

    return Stated::Of(extractions, location, lineage, scope);
}

/// One admission: the bytes through the pipeline, what it published recorded, the run reported.
///
/// The reading is borrowed rather than taken because it is only read here: the pipeline is
/// handed a trait object derived from it and never the value itself.
fn Ran_Admission(bytes: Vec<u8>, said: Option<&Stated>, kept: &mut Kept) -> ExitCode
{
    // The reading crosses as a trait object because `Admit_Source` is written against one: the
    // pipeline is handed a reader and never a `Stated`, so the vtable is the seam's own signature
    // rather than a decision taken here.
    let reader = said.map(|said| return said as &dyn ExtractionStrategy);

    // Text, because that is what a person reading a file to type `--says` was doing. A source
    // needing visual reading is a judgement no part of this command can make, and saying `Text`
    // here does not assert otherwise -- `Stated` refuses no kind, so the value reaches nothing
    // that acts on it. It becomes load-bearing when a reader that can refuse arrives.
    let report = match Admit_Source(bytes, reader, ReadingKind::Text, &mut kept.store)
    {
        Ok(report) => report,
        Err(refusal) => return Complained_Without_Usage("kwb admit", &refusal.to_string(), FAILURE_EXIT),
    };

    return Finished_Admission(&report, kept);
}

/// What the run published, kept and then reported.
///
/// Split from [`Ran_Admission`] because the pipeline call and the run's tail are two things a reader
/// checks separately: that the bytes went in through the one door, and that what came out was
/// recorded before the report claimed it.
fn Finished_Admission(report: &AdmissionReport, kept: &mut Kept) -> ExitCode
{
    let published = match Record_Into(kept.recorded.log.as_ref(), report, Some(Now()))
    {
        Ok(()) => report.Published_Into(&kept.recorded.known),
        Err(complaint) => return Complained_Without_Usage("kwb admit", &complaint, FAILURE_EXIT),
    };

    Report_Admission(
        report,
        &published,
        Persistence::Of(kept.store.Is_Durable()),
        Persistence::Of(kept.recorded.log.is_some()),
    );

    return ExitCode::SUCCESS;
}

/// The three ways a run can be refused, told apart by their exit codes, and the order they happen in.
///
/// `Run_Admission` prints a report and ends an exit, so the words it prints are read at the process
/// boundary. What is left inside the process is the half a caller branches on — and the half the
/// module doc above spends its length on: one wrong command line and two failed runs, each ending
/// the way it should. Those are indistinguishable in any single refusal's text and are three
/// different exit codes, which is the whole reason they travel apart.
#[cfg(test)]
mod tests
{
    use super::*;

    use crate::USAGE_EXIT;

    #[test]
    fn Test_Run_Admission_Should_Refuse_A_Path_The_Medium_Would_Not_Hand_Over()
    {
        // A failed run rather than a wrong command line: the path is well-formed and it is the
        // medium that refused, so the exit is `FAILURE_EXIT` and the usage is withheld. A run that
        // answered `USAGE_EXIT` here would send a person to re-read the help for a file that is
        // simply not there.
        assert_eq!(
            Run_Admission("no such file this run was pointed at", &[]),
            ExitCode::from(FAILURE_EXIT)
        );
    }

    #[test]
    fn Test_Run_Admission_Should_Refuse_A_Mistyped_Command_Line_Before_It_Reads_The_File()
    {
        // The path does not exist, and deliberately. What is asserted is the ORDER: the flags are
        // read before the bytes, so an argument the verb does not take is answered as a mistyped
        // command line — `USAGE_EXIT`, with the syntax shown — rather than as the missing file it
        // would also have been. Both are true of this run and only one of them is the answer.
        assert_eq!(
            Run_Admission("no such file this run was pointed at", &["--nonsense"]),
            ExitCode::from(USAGE_EXIT)
        );
    }

    #[test]
    fn Test_Run_Admission_Should_Answer_Success_For_An_Admission_With_Nothing_To_Say()
    {
        // `--says` is how a passage's claims get in, and a run given none is still a run the tool
        // documents: the bytes are admitted, the source is written, and the coverage is reported
        // `unmet` rather than refused. A version that ended on a failure here would make
        // `kwb admit <file>` — the command the help leads with — exit `1` for having been typed
        // exactly as it was written down.
        let source = std::env::temp_dir().join("kwb-cli-admission-nothing-to-say.txt");
        std::fs::write(&source, b"a passage").expect("a writable temporary source");
        let named = source.display().to_string();

        assert_eq!(Run_Admission(&named, &[]), ExitCode::SUCCESS);
    }
}
