//! One section per stage, and each names the algorithmic property that stage's design
//! depends on rather than its typical-case output.
//!
//! `D18` is why the file is organised that way. Its defect survived **1,878 passing tests**
//! because all of them asserted properties of a predicate while the fault was in the
//! structure that consumed it. A test of what a stage usually returns would not have found
//! it, and would not find its successor either.

use kwb_domain::KnowledgeGraph;
use kwb_domain::Standing;
use kwb_store::Document;
use kwb_store::DocumentStore;
use kwb_store::StoreError;

use super::*;

fn Offered(concept: &str, claim: &str) -> Extraction
{
    return Extraction::New(concept.to_owned(), claim.to_owned());
}

// ---- Stage one: linking infers nothing ----

#[test]
fn Test_A_Claim_Should_Be_Linked_Only_To_The_Concept_Its_Own_Extraction_Named()
{
    let linked = Link_Concepts(&[
        Offered("entropy", "It is non-decreasing in an isolated system."),
        Offered("enthalpy", "It is a thermodynamic potential."),
    ]);

    assert_eq!(linked.Concepts().len(), 2);
    assert_eq!(linked.Claims().len(), 2);
    for (claim, concept) in linked.Claims().iter().zip(linked.Concepts())
    {
        assert_eq!(
            claim.Concept(),
            concept.Identity(),
            "a claim reached a concept its extraction did not name"
        );
    }
}

#[test]
fn Test_Every_Claims_Concept_Should_Be_Among_The_Concepts()
{
    let linked = Link_Concepts(&[
        Offered("entropy", "one"),
        Offered("enthalpy", "two"),
        Offered("entropy", "three"),
    ]);

    for claim in linked.Claims()
    {
        assert!(
            linked
                .Concepts()
                .iter()
                .any(|concept| return concept.Identity() == claim.Concept()),
            "a claim points at a concept this stage did not produce"
        );
    }
}

#[test]
fn Test_An_Incomplete_Extraction_Should_Be_Refused_And_Counted()
{
    let linked = Link_Concepts(&[
        Offered("entropy", "It is non-decreasing."),
        Offered("enthalpy", "   "),
        Offered("", "orphaned text"),
    ]);

    assert_eq!(linked.Claims().len(), 1);
    assert_eq!(
        linked.Refused(),
        2,
        "a stage that discards input without saying how much is why D19's reporting rule \
         exists"
    );
}

// ---- Stage two: the structure is entitled to the closure it computes ----

/// **The test this stage exists for.**
///
/// These three names are the prototype's own counterexample. A variant relation answers true
/// for the acronym against each expansion and false between the expansions, so a structure
/// computing a transitive closure over it fuses all three. That is how `zero matrix` and
/// `zero mass` became one concept, and how 144 of 579 merges came to fuse ideas the rule
/// explicitly rejects.
///
/// This asserts the **structure**, not a predicate, which is the half `D18` shows testing
/// cannot skip.
#[test]
fn Test_Names_A_Variant_Relation_Would_Bridge_Should_Remain_Distinct()
{
    let normalized = Normalize_Concepts(Link_Concepts(&[
        Offered("z m", "an abbreviation in the text"),
        Offered("zero mass", "a particle with no rest mass"),
        Offered("zero matrix", "the additive identity of a matrix ring"),
    ]));

    assert_eq!(
        normalized.Concepts().len(),
        3,
        "a bridge term fused concepts the relation would itself reject"
    );
    assert_eq!(normalized.Merged(), 0);
}

/// The control that stops the test above from passing for the wrong reason.
///
/// A normalizer that merged *nothing* would satisfy the bridge test perfectly and be useless.
/// This is the other direction: names that genuinely are one concept must become one.
#[test]
fn Test_Repeated_Mentions_Of_One_Concept_Should_Become_One_Concept()
{
    let normalized = Normalize_Concepts(Link_Concepts(&[
        Offered("entropy", "It is non-decreasing."),
        Offered("entropy", "It has units of joules per kelvin."),
        Offered("entropy", "It is extensive."),
    ]));

    assert_eq!(
        normalized.Concepts().len(),
        1,
        "the stage merged nothing, so the bridge test above proves nothing"
    );
    assert_eq!(normalized.Claims_Held(), 3);
}

#[test]
fn Test_The_Grouping_Relation_Should_Be_Transitive_Over_A_Chain()
{
    // Closure is what the stage computes, so the relation it computes over has to be
    // transitive. Equality is, and a three-link chain is the smallest case that would
    // expose a relation that is not.
    let normalized = Normalize_Concepts(Link_Concepts(&[
        Offered("BVH", "one"),
        Offered("BVH", "two"),
        Offered("BVH", "three"),
    ]));

    assert_eq!(normalized.Concepts().len(), 1);
    assert_eq!(normalized.Merged(), 2);
}

#[test]
fn Test_Case_Differences_Should_Not_Be_Grouped()
{
    // The recorded divergence from the prototype's LOWER() index, exercised at the stage
    // where folding it would do the damage.
    let normalized =
        Normalize_Concepts(Link_Concepts(&[Offered("BVH", "one"), Offered("bvh", "two")]));

    assert_eq!(normalized.Concepts().len(), 2);
}

#[test]
fn Test_Normalization_Should_Remove_No_Claim()
{
    let linked = Link_Concepts(&[
        Offered("entropy", "one"),
        Offered("entropy", "two"),
        Offered("enthalpy", "three"),
    ]);
    let before = linked.Claims().len();

    let normalized = Normalize_Concepts(linked);

    assert_eq!(
        normalized.Claims_Held(),
        before,
        "merging concepts dropped a claim, which is a destructive edit with nothing \
         authorising it"
    );
}

// ---- Stage three: the report counts only what it can see ----

#[test]
fn Test_Admitting_Nothing_Should_Be_Unmet_Rather_Than_Barren()
{
    let mut store = DocumentStore::Empty();

    let report = Admit(b"a source".to_vec(), &[], &mut store).expect("admits");

    assert!(
        !report.Coverage().Is_Evidence_Of_Absence(),
        "a run given nothing to examine did not look at a source and find it empty"
    );
    assert_eq!(report.Coverage().Name(), "unmet");
}

#[test]
fn Test_Examining_Extractions_And_Finding_None_Admissible_Should_Be_Barren()
{
    let mut store = DocumentStore::Empty();

    let report = Admit(
        b"a source".to_vec(),
        &[Offered("entropy", "  "), Offered("", "orphaned")],
        &mut store,
    )
    .expect("admits");

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

    let report = Admit(
        b"a source".to_vec(),
        &[Offered("entropy", "one"), Offered("enthalpy", "two")],
        &mut store,
    )
    .expect("admits");

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

    let report = Admit(bytes.clone(), &[Offered("entropy", "one")], &mut store).expect("admits");

    let written = report.Source().expect("a source was written");
    assert_eq!(store.Read(written.Identity()).expect("reads").Content(), bytes);
}

#[test]
fn Test_Admitting_The_Same_Source_Twice_Should_Not_Duplicate_It()
{
    let mut store = DocumentStore::Empty();
    let extractions = [Offered("entropy", "one")];

    let first = Admit(b"a source".to_vec(), &extractions, &mut store).expect("admits");
    let second = Admit(b"a source".to_vec(), &extractions, &mut store).expect("admits again");

    assert_eq!(
        first.Source().expect("written").Identity(),
        second.Source().expect("written").Identity()
    );
    assert!(!second.Source().expect("written").Was_Stored());
    assert_eq!(store.Length(), 1);
}

#[test]
fn Test_A_Source_Of_No_Bytes_Should_Be_Refused_Before_Anything_Is_Admitted()
{
    let mut store = DocumentStore::Empty();

    let refusal = Admit(Vec::new(), &[Offered("entropy", "one")], &mut store)
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

    let report = Admit(
        b"a source".to_vec(),
        &[Offered("entropy", "one"), Offered("enthalpy", "two")],
        &mut store,
    )
    .expect("admits");

    assert_eq!(report.Normalized().Claims_Held(), 2, "the claims were handed back");
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
    let report = Admit(
        b"a source".to_vec(),
        &[Offered("entropy", "one"), Offered("enthalpy", "two")],
        &mut store,
    )
    .expect("admits");

    let graph = report.Published_Into(&KnowledgeGraph::Empty());

    assert_eq!(graph.Current().Claims().len(), report.Coverage().Findings());
    assert_eq!(graph.Current().Concepts().len(), 2);
}

#[test]
fn Test_Publishing_Should_Leave_The_Graph_It_Was_Given_Unchanged()
{
    let mut store = DocumentStore::Empty();
    let report = Admit(b"a source".to_vec(), &[Offered("entropy", "one")], &mut store)
        .expect("admits");
    let before = KnowledgeGraph::Empty();

    let after = report.Published_Into(&before);

    assert!(
        before.Current().Concepts().is_empty(),
        "the caller still holds what it passed in, which is what makes the previous version          a thing that was kept rather than overwritten"
    );
    assert_eq!(after.Current().Concepts().len(), 1);
}

#[test]
fn Test_Re_Admitting_A_Source_Should_Not_Duplicate_Its_Concepts()
{
    let mut store = DocumentStore::Empty();
    let extractions = [Offered("entropy", "one"), Offered("entropy", "two")];

    let first = Admit(b"a source".to_vec(), &extractions, &mut store).expect("admits");
    let graph = first.Published_Into(&KnowledgeGraph::Empty());
    let second = Admit(b"a source".to_vec(), &extractions, &mut store).expect("admits again");
    let graph = second.Published_Into(&graph);

    assert_eq!(
        graph.Current().Concepts().len(),
        1,
        "content addressing means a re-read book is the same concept, not a second one"
    );
    assert_eq!(graph.Current().Claims().len(), 2);
}

#[test]
fn Test_A_Claim_About_A_Retired_Concept_Should_Not_Be_Current()
{
    // One liveness rule applied twice rather than two rules. Nothing else in this workspace
    // would have said that a claim about a retired concept is not current knowledge.
    let mut store = DocumentStore::Empty();
    let report = Admit(b"a source".to_vec(), &[Offered("phlogiston", "It is released in combustion.")], &mut store)
        .expect("admits");
    let graph = report.Published_Into(&KnowledgeGraph::Empty());
    let held = graph.Every_Version().Concepts();
    let concept = (*held.first().expect("one concept")).clone();

    let retired = graph.With_Concept(concept.Closed(Standing::Retired));

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
    let report = Admit(b"a source".to_vec(), &[Offered("entropy", "one")], &mut store)
        .expect("admits");

    let graph = report.Published_Into(&KnowledgeGraph::Empty());

    let held = graph.Current().Concepts();
    let concept = held.first().expect("one concept");
    assert_eq!(concept.Canonical_Name(), "entropy");
    assert_eq!(
        Some(concept.Identity()),
        report.Normalized().Concepts().first().map(|c| return c.Identity())
    );
}
