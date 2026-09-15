//! One section per stage, and each names the algorithmic property that stage's design
//! depends on rather than its typical-case output.
//!
//! `D18` is why the file is organised that way. Its defect survived **1,878 passing tests**
//! because all of them asserted properties of a predicate while the fault was in the
//! structure that consumed it. A test of what a stage usually returns would not have found
//! it, and would not find its successor either.
//!
//! # Why the sections are separate modules
//!
//! One section per stage, and one module per section, so that a stage's tests are held apart
//! from its neighbours' the same way the stages themselves are. The alternative — 950 lines in
//! one file — is a reading that ends in the middle, and what a reader loses is exactly the
//! comparison `D18` says to make: that a property is asserted of the structure, and not only of
//! what the structure usually produces. The shared builders below are what the sections have in
//! common, and they are here rather than copied because a second copy of `Said` is a second
//! place the reading seam can be entered differently.
//!
//! `linking` is stage one, `normalization` stage two, `admission` stage three and its report,
//! and `extraction` the seam in front of all of them — numbered the way the crate's own module
//! list numbers them.

use kwb_domain::KnowledgeGraph;
use kwb_domain::Scope;
use kwb_domain::Standing;
use kwb_store::Document;
use kwb_store::DocumentStore;
use kwb_store::StoreError;

use super::*;

mod admission;
mod extraction;
mod linking;
mod normalization;

/// A scope a source did not state. `D-010`: an answer, not a default.
fn Unstated() -> Scope
{
    return Scope::Unstated();
}

/// A reading taken under a named protocol by a named reader.
///
/// Two arguments rather than one, so a caller that needs both names binds one of these to a
/// local and passes the local on: a two-argument call handed straight to another call is the
/// nesting the readability rule asks to be named.
fn A_Reading_Under(protocol: &str, reader: &str) -> ExtractionLineage
{
    return ExtractionLineage::Of(ReadingProtocol::Named(protocol), ReaderName::Named(reader));
}

/// A reading taken by a person, which is the one protocol a `Stated` places its statements under.
fn A_Reading(reader: &str) -> ExtractionLineage
{
    return A_Reading_Under("stated-by-a-person", reader);
}

fn Offered(concept: &str, claim: &str) -> Extraction
{
    return Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));
}

/// A person stating what a source says, which is what `--says` is and what `Stated` names.
///
/// Every admission test goes through this rather than through a prepared list, because that is
/// now the only way in — and so these tests exercise the same path the command line does.
fn Said(statements: &[Extraction], scope: &Scope) -> Stated
{
    return Stated::Of(
        statements.to_vec(),
        SourceLocation::Named("throughout"),
        A_Reading("a test"),
        scope.clone(),
    )
    .expect("a test that states nothing is testing the wrong thing -- use `Silent`");
}

/// One source admitted, read by whatever reader was offered at the kind of reading asked for.
///
/// The source is written either way: a reader that refuses, or that answers about a document it
/// was not given, leaves an admission that reports the outcome rather than an error.
fn Admitted_By(
    bytes: &[u8],
    reader: &dyn ExtractionStrategy,
    needed: ReadingKind,
    store: &mut DocumentStore,
) -> AdmissionReport
{
    return Admit_Source(bytes.to_vec(), Some(reader), needed, store)
        .expect("a source is admitted even when its reading does not happen");
}

/// One source admitted, read by a person who stated these extractions.
fn Admitted_From(
    bytes: &[u8],
    statements: &[Extraction],
    store: &mut DocumentStore,
) -> AdmissionReport
{
    let reader = Said(statements, &Unstated());

    return Admitted_By(bytes, &reader, ReadingKind::Text, store);
}

/// Assert that a report records a prerequisite that was never met, and records it as **not**
/// evidence that the source is empty.
///
/// The two are asserted together because either half alone passes for the wrong reason. A report
/// that said `unmet` for everything would satisfy the first; a report that said `barren` for
/// everything is the 1,367-row incident, where a partial reading was recorded as a complete one.
fn Assert_Unmet_And_Not_Evidence_Of_Absence(report: &AdmissionReport, what_happened: &str)
{
    assert_eq!(report.Coverage().Name(), "unmet");
    assert!(
        !report.Coverage().Is_Evidence_Of_Absence(),
        "{what_happened} was recorded as evidence the source is empty"
    );
}

/// A reader that is asked and does not answer. The point of the tests that use it is that this is
/// reached through the same trait a working reader is, so a caller cannot tell them apart by
/// shape and must handle the refusal.
///
/// Here rather than beside the other two readers, because it is the one reader two sections need:
/// `admission` asks it for a report that says `unmet`, and `extraction` asks it directly for a
/// refusal. A copy in each would be two readers a caller could tell apart after all.
struct Silent;

impl ExtractionStrategy for Silent
{
    fn Read(
        &self,
        _source: kwb_model::ContentIdentity,
        _content: &[u8],
        _needed: ReadingKind,
    ) -> Result<Vec<ProposedReading>, ExtractionError>
    {
        return Err(ExtractionError::ReaderFailed {
            cause: "the answer did not parse".to_owned(),
        });
    }

    fn Scope(&self) -> Scope
    {
        return Unstated();
    }
}
