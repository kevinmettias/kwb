//! Stage three: the report counts only what it can see, what it published reaches the graph, and
//! what a claim is anchored to once it is there.
//!
//! Three sections, and they are one module because they are the same question asked at three
//! distances: what the report says about itself, what the graph holds after it, and what a person
//! following a citation out of the graph would find. The `D19` rule — a report must not outrun the
//! work — is what a reader is checking in all three.

use super::*;

#[test]
fn Test_Admitting_Nothing_Should_Be_Unmet_Rather_Than_Barren()
{
    // The route to this outcome changed when `Admit_Source` began taking a reader. It used to be
    // reached by handing in an empty list of extractions; that state is now unconstructible,
    // because a reader who said nothing is not a reader at all -- see the seam's own test. What
    // is left is the case that matters and always did: the reader was asked and did not answer.
    let mut store = DocumentStore::Empty();

    let report = Admit_Source(b"a source".to_vec(), Some(&Silent), ReadingKind::Text, &mut store)
        .expect("the source is admitted even though the reading of it did not happen");

    Assert_Unmet_And_Not_Evidence_Of_Absence(&report, "a reading that never happened");
    assert!(
        !report.Coverage().Has_Run(),
        "a reading that never happened was recorded as having run"
    );
    assert!(
        report.Source().is_some(),
        "the source went through the write door regardless -- only the reading failed"
    );
}

#[test]
fn Test_Examining_Extractions_And_Finding_None_Admissible_Should_Be_Barren()
{
    let mut store = DocumentStore::Empty();

    let report = Admit_Source(
        b"a source".to_vec(),
        Some(&Stated_By_A_Person(&[An_Extraction("entropy", "  "), An_Extraction("", "orphaned")], &Unstated())),
        ReadingKind::Text,
        &mut store,)
    .expect("the source is non-empty, so admission runs");

    assert_eq!(report.Coverage().Name(), "barren");
    assert!(
        report.Coverage().Is_Evidence_Of_Absence(),
        "this run did look, and did find nothing"
    );
}

#[test]
fn Test_The_Report_Should_Not_Count_More_Than_It_Holds()
{
    let mut store = DocumentStore::Empty();

    let report = Admit_Source(
        b"a source".to_vec(),
        Some(&Stated_By_A_Person(&[An_Extraction("entropy", "one"), An_Extraction("enthalpy", "two")], &Unstated())),
        ReadingKind::Text,
        &mut store,)
    .expect("the source is non-empty, so admission runs");

    assert_eq!(
        report.Coverage().Findings(),
        report.Normalized().Claims_Held(),
        "the count and the claims it describes disagree, which is the shape of a run \
         reporting chunks admitted it did not admit"
    );
    assert_eq!(report.Coverage().Name(), "yielded");
}

#[test]
fn Test_An_Admitted_Source_Should_Be_Readable_Back_From_The_Store()
{
    let mut store = DocumentStore::Empty();
    let bytes = b"a source document".to_vec();

    let report = Admit_Source(bytes.clone(), Some(&Stated_By_A_Person(&[An_Extraction("entropy", "one")], &Unstated())), ReadingKind::Text, &mut store).expect("the source is non-empty, so admission runs");

    let written = report.Source().expect("a source was written");
    assert_eq!(store.Read(written.Identity()).expect("the address came from the write just above").Content(), bytes);
}

#[test]
fn Test_Admitting_The_Same_Source_Twice_Should_Not_Duplicate_It()
{
    let mut store = DocumentStore::Empty();
    let extractions = [An_Extraction("entropy", "one")];

    let first = Admit_Source(b"a source".to_vec(), Some(&Stated_By_A_Person(&extractions, &Unstated())), ReadingKind::Text, &mut store).expect("the source is non-empty, so admission runs");
    let second = Admit_Source(b"a source".to_vec(), Some(&Stated_By_A_Person(&extractions, &Unstated())), ReadingKind::Text, &mut store).expect("admits again");

    assert_eq!(
        first.Source().expect("the source went through the write door, so it is present").Identity(),
        second.Source().expect("the source went through the write door, so it is present").Identity()
    );
    assert!(!second.Source().expect("the source went through the write door, so it is present").Has_Stored());
    assert_eq!(store.Length(), 1);
}

#[test]
fn Test_A_Source_Of_No_Bytes_Should_Be_Refused_Before_Anything_Is_Admitted()
{
    let mut store = DocumentStore::Empty();

    let refusal = Admit_Source(Vec::new(), Some(&Stated_By_A_Person(&[An_Extraction("entropy", "one")], &Unstated())), ReadingKind::Text, &mut store)
        .expect_err("must refuse");

    assert_eq!(refusal, StoreError::Vacuous);
    assert!(store.Is_Empty());
}

#[test]
fn Test_Admission_Should_Queue_Nothing_For_A_Consumer_That_Does_Not_Exist()
{
    // D19 asserted the only way an absence can be: the claims come back to the caller, in
    // the same call, and the store holds the source alone. If a later change queues claims
    // for a worker that has not been written, this count moves.
    let mut store = DocumentStore::Empty();
    let says = [An_Extraction("entropy", "one"), An_Extraction("enthalpy", "two")];

    let report = Admitted_From(b"a source", &says, &mut store);

    assert_eq!(report.Normalized().Claims_Held(), says.len(), "the claims were handed back");
    assert_eq!(store.Length(), 1, "the store holds the source and nothing else");
    assert!(store.Read(Document::Of(b"a source".to_vec()).Identity()).is_ok());
}

// ---- KWB-25: what admission reports is what the graph holds ----

#[test]
fn Test_What_Admission_Reports_Should_Be_What_The_Graph_Holds()
{
    // The connection, asserted rather than assumed. A pipeline whose output reached nothing
    // is the defect this item exists to close, and the only way to see it is to compare the
    // report against the thing it was supposed to fill.
    let mut store = DocumentStore::Empty();
    let says = [An_Extraction("entropy", "one"), An_Extraction("enthalpy", "two")];
    let report = Admitted_From(b"a source", &says, &mut store);

    let graph = report.Published_Into(&KnowledgeGraph::Empty());

    assert_eq!(graph.Current().Claims().len(), report.Coverage().Findings());
    assert_eq!(graph.Current().Concepts().len(), says.len());
}

#[test]
fn Test_Publishing_Should_Leave_The_Graph_It_Was_Given_Unchanged()
{
    let mut store = DocumentStore::Empty();
    let report = Admit_Source(b"a source".to_vec(), Some(&Stated_By_A_Person(&[An_Extraction("entropy", "one")], &Unstated())), ReadingKind::Text, &mut store)
        .expect("the source is non-empty, so admission runs");
    let before = KnowledgeGraph::Empty();

    let after = report.Published_Into(&before);

    assert!(
        before.Current().Concepts().is_empty(),
        "the caller still holds what it passed in, which is what makes the previous version \
         a thing that was kept rather than overwritten"
    );
    assert_eq!(after.Current().Concepts().len(), 1);
}

#[test]
fn Test_Re_Admitting_A_Source_Should_Not_Duplicate_Its_Concepts()
{
    let mut store = DocumentStore::Empty();
    let extractions = [An_Extraction("entropy", "one"), An_Extraction("entropy", "two")];

    let first = Admit_Source(b"a source".to_vec(), Some(&Stated_By_A_Person(&extractions, &Unstated())), ReadingKind::Text, &mut store).expect("the source is non-empty, so admission runs");
    let graph = first.Published_Into(&KnowledgeGraph::Empty());
    let second = Admit_Source(b"a source".to_vec(), Some(&Stated_By_A_Person(&extractions, &Unstated())), ReadingKind::Text, &mut store).expect("admits again");
    let graph = second.Published_Into(&graph);

    assert_eq!(
        graph.Current().Concepts().len(),
        1,
        "content addressing means a re-read book is the same concept, not a second one"
    );
    assert_eq!(graph.Current().Claims().len(), extractions.len());
}

#[test]
fn Test_A_Claim_About_A_Retired_Concept_Should_Not_Be_Current()
{
    // One liveness rule applied twice rather than two rules. Nothing else in this workspace
    // would have said that a claim about a retired concept is not current knowledge.
    let mut store = DocumentStore::Empty();
    let report = Admit_Source(b"a source".to_vec(), Some(&Stated_By_A_Person(&[An_Extraction("phlogiston", "It is released in combustion.")], &Unstated())), ReadingKind::Text, &mut store)
        .expect("the source is non-empty, so admission runs");
    let graph = report.Published_Into(&KnowledgeGraph::Empty());
    let held = graph.Every_Version().Concepts();
    let concept = (*held.first().expect("the graph holds the concepts just published")).clone();

    let retired = graph.With_Concept(concept.Closed(Standing::Retired {
            because: "the concept was withdrawn by its source".to_owned(),
        }));

    assert!(retired.Current().Claims().is_empty(), "the claim is not current knowledge");
    assert_eq!(
        retired.Every_Version().Claims().len(),
        1,
        "and it is still there, because D17 says nothing is destroyed without evidence"
    );
    assert!(graph.Current().Claims().len() == 1, "and the earlier graph is unchanged");
}

#[test]
fn Test_A_Published_Concept_Should_Be_Addressed_By_Its_Content()
{
    let mut store = DocumentStore::Empty();
    let report = Admit_Source(b"a source".to_vec(), Some(&Stated_By_A_Person(&[An_Extraction("entropy", "one")], &Unstated())), ReadingKind::Text, &mut store)
        .expect("the source is non-empty, so admission runs");

    let graph = report.Published_Into(&KnowledgeGraph::Empty());

    let held = graph.Current().Concepts();
    let concept = held.first().expect("the graph holds the concepts just published");
    assert_eq!(concept.Canonical_Name(), "entropy");
    assert_eq!(
        Some(concept.Identity()),
        report.Normalized().Concepts().first().map(|c| return c.Identity())
    );
}

// ---- KWB-36: the citation, which is what all of this was for ----

/// How many documents the corroboration setup admits.
///
/// Two is the smallest number that can show the thing being tested: one document cannot
/// corroborate anything, and a claim that holds two citations is the whole mechanism `D-002`
/// excluding the source from a claim's derivation exists to produce.
const SOURCES_IN_CORROBORATION: usize = 2;

/// Every document a graph's current assertions name, as the addresses they carry.
fn Cited_Documents(graph: &KnowledgeGraph) -> Vec<&str>
{
    return graph
        .Current()
        .Assertions()
        .iter()
        .map(|assertion| return assertion.Source())
        .collect();
}

/// Assert that one claim is current and carries a citation per source.
///
/// The two together, because a graph that held one claim with no citations satisfies the first
/// half alone — and a claim nobody can cite is the thing a citation exists to prevent.
fn Assert_Corroborated_By(graph: &KnowledgeGraph, sources: usize)
{
    assert_eq!(graph.Current().Claims().len(), 1, "the source is excluded, so this is one claim");
    assert_eq!(
        graph.Current().Assertions().len(),
        sources,
        "a claim must carry a citation per source, or corroboration cannot be counted"
    );
}

#[test]
fn Test_Two_Sources_Asserting_One_Claim_Should_Be_One_Claim_With_Two_Citations()
{
    // The mechanism every identity decision in this workspace was made to support, end to end
    // for the first time. Two different documents, the same claim text: one claim, two
    // assertions, each naming the address of the document it was read out of.
    let mut store = DocumentStore::Empty();
    let says = [An_Extraction("entropy", "It is non-decreasing.")];
    let callen = Admitted_From(b"Callen, Thermodynamics", &says, &mut store);
    let kittel = Admitted_From(b"Kittel, Thermal Physics", &says, &mut store);

    let graph = kittel.Published_Into(&callen.Published_Into(&KnowledgeGraph::Empty()));

    Assert_Corroborated_By(&graph, SOURCES_IN_CORROBORATION);
    let cited = Cited_Documents(&graph);
    assert!(
        cited.contains(&callen.Source().expect("the source went through the write door, so it is present").Identity().Render().as_str())
            && cited.contains(&kittel.Source().expect("the source went through the write door, so it is present").Identity().Render().as_str()),
        "a citation must name the document it was read out of: {cited:?}"
    );
}

#[test]
fn Test_A_Citation_Should_Resolve_To_The_Bytes_The_Claim_Was_Read_Out_Of()
{
    // What makes the address a citation rather than a label: following it returns the exact
    // content, or fails loudly because it is gone.
    let mut store = DocumentStore::Empty();
    let bytes = b"Entropy is non-decreasing in an isolated system.".to_vec();
    let report = Admit_Source(
        bytes.clone(),
        Some(&Stated_By_A_Person(&[An_Extraction("entropy", "It is non-decreasing.")], &Unstated())),
        ReadingKind::Text,
        &mut store,)
    .expect("the source is non-empty, so admission runs");

    let assertion = report.Assertions().first().expect("one assertion");
    let address = kwb_model::ContentIdentity::Parse(assertion.Source()).expect("the source was rendered from a content identity");

    assert_eq!(store.Read(address).expect("the citation names a document in this store").Content(), bytes);
}

#[test]
fn Test_An_Unstated_Scope_Should_Stay_Unstated_Rather_Than_Become_The_Narrowest()
{
    let mut store = DocumentStore::Empty();
    let report = Admit_Source(
        b"a source".to_vec(),
        Some(&Stated_By_A_Person(&[An_Extraction("entropy", "It is non-decreasing.")], &Unstated())),
        ReadingKind::Text,
        &mut store,)
    .expect("the source is non-empty, so admission runs");

    assert!(
        report.Assertions().first().expect("the run recorded one assertion").Scope().Is_Unstated(),
        "a source that did not say how far it meant has not said the narrowest thing"
    );
}

#[test]
fn Test_A_Stated_Scope_Should_Reach_The_Assertion()
{
    let mut store = DocumentStore::Empty();
    let report = Admit_Source(
        b"a source".to_vec(),
        Some(&Stated_By_A_Person(&[An_Extraction("entropy", "It is non-decreasing.")], &Scope::Named("physical theory").expect("a named scope"))),
        ReadingKind::Text,
        &mut store,)
    .expect("the source is non-empty, so admission runs");

    assert_eq!(report.Assertions().first().expect("the run recorded one assertion").Scope().Name(), "physical theory");
}

#[test]
fn Test_Publications_Should_Order_Assertions_After_The_Claims_They_Name()
{
    // Replay refuses an assertion naming a claim no earlier record published, so this ordering
    // is a property of the method rather than an accident of iteration.
    let mut store = DocumentStore::Empty();
    let says = [An_Extraction("entropy", "one"), An_Extraction("enthalpy", "two")];
    let report = Admitted_From(b"a source", &says, &mut store);

    let records: Vec<String> = report
        .Publications()
        .iter()
        .map(|publication| return publication.Record(None))
        .collect();

    let replayed = kwb_domain::Replay_Records(&records).expect("a run's own publications must replay");
    assert_eq!(replayed.Current().Assertions().len(), says.len());
    assert_eq!(replayed.Current().Claims().len(), says.len());
}
