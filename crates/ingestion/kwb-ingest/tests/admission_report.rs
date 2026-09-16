//! What an admission reports, asserted function by function in the file named for it.
//!
//! # Why this file sits beside `src/tests/admission.rs`
//!
//! That file asserts stage three's outcomes — unmet is not barren, the report does not count more
//! than it holds, what it published is what the graph holds, and a citation resolves to the bytes
//! the claim was read out of — and it reaches every function here on the way. What it does not do
//! is name any of them, and its stem is `admission` rather than `admission_report`, so
//! `src/admission_report.rs` is covered by a file called `admission_report.rs` and by nothing
//! else. A failure in `Published_Into` arrived under the name of a test about a citation.
//!
//! `Admit_Source` is here too, because it is declared in this file rather than in `lib.rs`, and
//! because the report's own contract begins with the call that produces one.

use kwb_domain::KnowledgeGraph;
use kwb_domain::Publication;
use kwb_domain::Scope;
use kwb_ingest::AdmissionReport;
use kwb_ingest::Admit_Source;
use kwb_ingest::ClaimText;
use kwb_ingest::ConceptName;
use kwb_ingest::Extraction;
use kwb_ingest::ExtractionError;
use kwb_ingest::ExtractionLineage;
use kwb_ingest::ReaderName;
use kwb_ingest::ReadingKind;
use kwb_ingest::ReadingProtocol;
use kwb_ingest::SourceLocation;
use kwb_ingest::Stated;
use kwb_store::Document;
use kwb_store::DocumentStore;

fn An_Extraction(concept: &str, claim: &str) -> Extraction
{
    return Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));
}

/// One statement, which is the smallest run that yields anything.
fn One_Claim() -> Vec<Extraction>
{
    return vec![An_Extraction("entropy", "It is non-decreasing in an isolated system.")];
}

/// Two statements about two concepts, so that a count is asserted rather than a presence and the
/// publication ordering below has something to order.
fn Two_Claims() -> Vec<Extraction>
{
    return vec![
        An_Extraction("entropy", "It is non-decreasing in an isolated system."),
        An_Extraction("enthalpy", "It is extensive."),
    ];
}

/// A reading that happened and proposed only halves, so the run found nothing admissible.
fn Nothing_Admissible() -> Vec<Extraction>
{
    return vec![
        An_Extraction("entropy", " \t "),
        An_Extraction("   ", "orphaned"),
    ];
}

fn Stated_By_A_Person(statements: &[Extraction]) -> Stated
{
    // The lineage is named before it is passed, rather than written into the argument list it is
    // one quarter of: as an argument it is four positions to hold at once, and as a local it is one
    // name the reader has already been told the meaning of.
    let lineage = ExtractionLineage::Of(
        ReadingProtocol::Named("stated-by-a-person"),
        ReaderName::Named("a test"),
    );

    return Stated::Of(
        statements.to_vec(),
        SourceLocation::Named("throughout"),
        lineage,
        Scope::Unstated(),
    )
    .expect("a person who stated something is a reader");
}

/// One source admitted, read by a person who stated these extractions.
fn Admitted(bytes: &[u8], statements: &[Extraction]) -> AdmissionReport
{
    let mut store = DocumentStore::Empty();
    let reader = Stated_By_A_Person(statements);

    return Admit_Source(bytes.to_vec(), Some(&reader), ReadingKind::Text, &mut store)
        .expect("a source of bytes is admitted even when its reading does not happen");
}

/// One source admitted with nobody offered to read it, which is what `kwb admit` does when it is
/// given a source and no `--says`.
fn Admitted_Unread(bytes: &[u8]) -> AdmissionReport
{
    let mut store = DocumentStore::Empty();

    return Admit_Source(bytes.to_vec(), None, ReadingKind::Text, &mut store)
        .expect("a source of bytes is admitted even when its reading does not happen");
}

#[test]
fn Test_Coverage_Should_Say_What_The_Run_Did()
{
    // The outcome, handed back as a value, and the three answers are not decoration: one says a
    // prerequisite was missing, one is **evidence of absence**, and one is work done. A report that
    // could not tell the first from the second is the prototype's 1,367 rows, where a reading that
    // never happened was recorded as a source with nothing in it.
    let unread = Admitted_Unread(b"a source");
    let barren = Admitted(b"a source", &Nothing_Admissible());
    let yielded = Admitted(b"a source", &One_Claim());

    assert_eq!(unread.Coverage().Name(), "unmet");
    assert_eq!(barren.Coverage().Name(), "barren");
    assert_eq!(yielded.Coverage().Name(), "yielded");
    assert!(
        !unread.Coverage().Has_Run(),
        "a reading that never happened is recorded as having run"
    );
}

#[test]
fn Test_Normalized_Should_Hand_Back_The_Concepts_And_Claims()
{
    // Handed back rather than stored, and linked and grouped as one body before it is: what a
    // concept mentioned in two passages becomes is one concept, by the same mechanism that makes it
    // one concept across two documents. *What was admitted* and *where it goes* are different
    // decisions, and publishing is the second one.
    let statements = Two_Claims();
    let report = Admitted(b"a source", &statements);

    assert_eq!(
        report.Normalized().Concepts().len(),
        statements.len(),
        "normalization grouped two distinct concepts, so the report describes a world nobody read"
    );
    assert_eq!(
        report.Normalized().Claims_Held(),
        statements.len(),
        "the report does not hold the claims it was handed"
    );
}

#[test]
fn Test_Assertions_Should_Cite_The_Document_Each_Claim_Came_From()
{
    // One per admitted claim, sourced by the **content address of the document it came from**.
    // That is what makes a citation resolve: following it returns the exact bytes the claim was
    // read out of, or fails loudly because they are gone. `D-006`'s dependency edge, and the
    // reason `D-014` keeps documents at all. It is also where `D-010` becomes a thing the pipeline
    // does rather than a thing a record says — a claim carries no source, and two sources asserting
    // one claim are two assertions meeting at it.
    let bytes = b"a source document".to_vec();
    let report = Admitted(&bytes, &Two_Claims());
    let written = report.Source().expect("a source went through the write door");

    assert_eq!(
        report.Assertions().len(),
        report.Normalized().Claims_Held(),
        "the assertions are not one per admitted claim, so a claim nobody can cite is current \
         knowledge"
    );
    for assertion in report.Assertions()
    {
        assert_eq!(
            assertion.Source(),
            written.Identity().Render(),
            "an assertion cites something other than the document its claim was read out of"
        );
    }
}

#[test]
fn Test_Source_Should_Name_The_Document_The_Reading_Was_Anchored_To()
{
    // The source is carried as the document's own address rather than a filename or a title, so
    // following it returns the bytes that were admitted — and two documents with the same content
    // are one source, which is the same mechanism one crate down.
    let bytes = b"a source document".to_vec();

    let report = Admitted(&bytes, &One_Claim());
    let written = report.Source().expect("a source went through the write door");

    assert_eq!(
        written.Identity(),
        Document::Of(bytes.clone()).Identity(),
        "the report names a document other than the one it was handed"
    );
    assert_eq!(written.Length(), bytes.len());

    // And it is present even when nothing was learned: the source was admitted and only the
    // reading did not happen, so a report that dropped it would lose the document it could not
    // read — which is the one thing a later re-read would need.
    assert!(
        Admitted_Unread(&bytes).Source().is_some(),
        "a source whose reading never happened was not kept, so the bytes cannot be re-read"
    );
}

#[test]
fn Test_Refusal_Should_Carry_Why_The_Reading_Did_Not_Happen()
{
    // `Coverage::Unmet` says *a prerequisite was not satisfied* in a `&'static str`, which is a
    // decision written in code and deliberately not a message assembled at runtime — right for the
    // outcome and useless to the person who ran the command, who needs to know which reader gave up
    // and what it said. This carries that, and only that.
    assert_eq!(
        Admitted_Unread(b"a source").Refusal(),
        Some(&ExtractionError::NotRead),
        "a source nobody read reports no reason, so a report cannot say why nothing was claimed"
    );
    assert!(
        Admitted(b"a source", &One_Claim()).Refusal().is_none(),
        "a reading that happened was carried as a refusal, so a report blames a reader that \
         answered"
    );
}

#[test]
fn Test_Publications_Should_Express_What_Reached_The_Graph_As_Records()
{
    // The same things `Published_Into` adds, in the same order, expressed as the transitions
    // `D-014` records rather than as the graph they produce — a caller recording them does not have
    // to take a graph apart to find out what changed, which it could not do correctly anyway, since
    // a graph is a fold and a fold does not remember its inputs.
    //
    // The order is a property of the method and not an accident of iteration: replay refuses a
    // claim naming a concept no earlier record published, and an assertion naming a claim no
    // earlier record published. That is what the two `matches!` assertions below state.
    let report = Admitted(b"a source", &Two_Claims());
    let publications = report.Publications();

    Assert_One_Publication_Per_Thing_The_Report_Holds(&report);
    Assert_A_Concept_Comes_First_And_An_Assertion_Comes_Last(&publications);
}

/// One publication per thing the report holds, by kind: one concept per concept, one claim per
/// claim, one assertion per assertion.
///
/// Directly under the test that asks for it, because that test is the only one that does, and the
/// comment above it carries what the publications are for.
fn Assert_One_Publication_Per_Thing_The_Report_Holds(report: &AdmissionReport)
{
    let publications = report.Publications();

    assert_eq!(
        (
            publications
                .iter()
                .filter(|publication| return matches!(publication, Publication::Concept { .. }))
                .count(),
            publications
                .iter()
                .filter(|publication| return matches!(publication, Publication::Claim { .. }))
                .count(),
            publications
                .iter()
                .filter(|publication| return matches!(publication, Publication::Assertion { .. }))
                .count(),
        ),
        (
            report.Normalized().Concepts().len(),
            report.Normalized().Claims_Held(),
            report.Assertions().len(),
        ),
        "the publications are not one per thing the report holds, by kind"
    );
}

/// A concept comes first and an assertion comes last, which is the order replay consumes.
///
/// Directly under the test that asks for it, because that test is the only one that does, and the
/// comment above it says why the two ends are where the order is asserted.
fn Assert_A_Concept_Comes_First_And_An_Assertion_Comes_Last(publications: &[Publication])
{
    assert!(
        matches!(publications.first(), Some(Publication::Concept { .. })),
        "a publication other than a concept comes first, so replay refuses every claim after it"
    );
    assert!(
        matches!(publications.last(), Some(Publication::Assertion { .. })),
        "a publication other than an assertion comes last, so replay refuses it for naming a \
         claim no earlier record published"
    );
}

#[test]
fn Test_Published_Into_Should_Leave_The_Graph_It_Was_Handed_Unchanged()
{
    // The caller still holds the graph it passed in, unchanged and queryable. That is what makes
    // the previous version a thing that was *kept* rather than a thing that was overwritten, and it
    // is the property `D-012` adopted the persistent map for. Both halves are asserted, because an
    // implementation that mutated the graph in place would satisfy the second on its own.
    let report = Admitted(b"a source", &Two_Claims());
    let before = KnowledgeGraph::Empty();

    let after = report.Published_Into(&before);

    assert!(
        before.Current().Concepts().is_empty() && before.Current().Claims().is_empty(),
        "the graph the caller passed in was mutated, so the version it held is gone"
    );
    assert_eq!(after.Current().Concepts().len(), report.Normalized().Concepts().len());
    assert_eq!(after.Current().Claims().len(), report.Normalized().Claims_Held());
    assert_eq!(after.Current().Assertions().len(), report.Assertions().len());
}

#[test]
fn Test_Admit_Source_Should_Write_The_Source_And_Report_What_It_Saw()
{
    // The whole call, and the two things only it does.
    //
    // The source goes through `kwb-store`'s one write door, so the address the report cites is the
    // address of bytes that exist. And the claims come back to the caller **in the same call**
    // rather than being queued for a consumer: `D19` is what happens when a producer outruns its
    // consumer, and the store holding the source and nothing else is what asserts that nothing was
    // queued.
    let mut store = DocumentStore::Empty();
    let bytes = b"a source document".to_vec();
    let statements = One_Claim();
    let reader = Stated_By_A_Person(&statements);

    let report = Admit_Source(bytes.clone(), Some(&reader), ReadingKind::Text, &mut store)
        .expect("a source of bytes is admitted even when its reading does not happen");
    Assert_The_Store_Holds_The_Source_The_Report_Cited(&store, &report, &bytes);

    assert_eq!(
        report.Normalized().Claims_Held(),
        statements.len(),
        "the claims were not handed back to the caller"
    );
}

/// The source the report cites is the source the store holds, and the store holds nothing else.
///
/// Directly under the test that asks for it, because that test is the only one that does. These are
/// the two halves of one thing: the source goes through `kwb-store`'s one write door, so the address
/// the report cites is the address of bytes that exist — and the store holding one document is what
/// asserts nothing was queued for a consumer, which is the shape `D19` says a producer outrunning
/// its consumer takes.
fn Assert_The_Store_Holds_The_Source_The_Report_Cited(
    store: &DocumentStore,
    report: &AdmissionReport,
    bytes: &[u8],
)
{
    let written = report.Source().expect("a source went through the write door");

    assert_eq!(
        store
            .Read(written.Identity())
            .expect("the address came from the write just above")
            .Content(),
        bytes,
        "the source the report cites is not the source the store holds"
    );
    assert_eq!(
        store.Length(),
        1,
        "admission queued something in the store for a consumer that does not exist"
    );
}
