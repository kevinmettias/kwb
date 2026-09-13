//! One section per stage, and each names the algorithmic property that stage's design
//! depends on rather than its typical-case output.
//!
//! `D18` is why the file is organised that way. Its defect survived **1,878 passing tests**
//! because all of them asserted properties of a predicate while the fault was in the
//! structure that consumed it. A test of what a stage usually returns would not have found
//! it, and would not find its successor either.

use kwb_domain::KnowledgeGraph;
use kwb_domain::Scope;
use kwb_domain::Standing;
use kwb_store::Document;
use kwb_store::DocumentStore;
use kwb_store::StoreError;

use super::*;

fn Offered(concept: &str, claim: &str) -> Extraction
{
    return Extraction::New(concept.to_owned(), claim.to_owned());
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
        ExtractionLineage::Of("stated-by-a-person", "a test"),
        scope.clone(),
    )
    .expect("a test that states nothing is testing the wrong thing -- use `Silent`");
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
    // The route to this outcome changed when `Admit` began taking a reader. It used to be
    // reached by handing in an empty list of extractions; that state is now unconstructible,
    // because a reader who said nothing is not a reader at all -- see the seam's own test. What
    // is left is the case that matters and always did: the reader was asked and did not answer.
    let mut store = DocumentStore::Empty();

    let report = Admit(b"a source".to_vec(), Some(&Silent), ReadingKind::Text, &mut store)
        .expect("the source is admitted even though the reading of it did not happen");

    assert!(
        !report.Coverage().Is_Evidence_Of_Absence(),
        "a reading that never happened was recorded as evidence the source is empty"
    );
    assert!(
        !report.Coverage().Was_Run(),
        "a reading that never happened was recorded as having run"
    );
    assert_eq!(report.Coverage().Name(), "unmet");
    assert!(
        report.Source().is_some(),
        "the source went through the write door regardless -- only the reading failed"
    );
}

#[test]
fn Test_Examining_Extractions_And_Finding_None_Admissible_Should_Be_Barren()
{
    let mut store = DocumentStore::Empty();

    let report = Admit(
        b"a source".to_vec(),
        Some(&Said(&[Offered("entropy", "  "), Offered("", "orphaned")], &Unstated())),
        ReadingKind::Text,
        &mut store,)
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
        Some(&Said(&[Offered("entropy", "one"), Offered("enthalpy", "two")], &Unstated())),
        ReadingKind::Text,
        &mut store,)
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

    let report = Admit(bytes.clone(), Some(&Said(&[Offered("entropy", "one")], &Unstated())), ReadingKind::Text, &mut store).expect("admits");

    let written = report.Source().expect("a source was written");
    assert_eq!(store.Read(written.Identity()).expect("reads").Content(), bytes);
}

#[test]
fn Test_Admitting_The_Same_Source_Twice_Should_Not_Duplicate_It()
{
    let mut store = DocumentStore::Empty();
    let extractions = [Offered("entropy", "one")];

    let first = Admit(b"a source".to_vec(), Some(&Said(&extractions, &Unstated())), ReadingKind::Text, &mut store).expect("admits");
    let second = Admit(b"a source".to_vec(), Some(&Said(&extractions, &Unstated())), ReadingKind::Text, &mut store).expect("admits again");

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

    let refusal = Admit(Vec::new(), Some(&Said(&[Offered("entropy", "one")], &Unstated())), ReadingKind::Text, &mut store)
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
        Some(&Said(&[Offered("entropy", "one"), Offered("enthalpy", "two")], &Unstated())),
        ReadingKind::Text,
        &mut store,)
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
        Some(&Said(&[Offered("entropy", "one"), Offered("enthalpy", "two")], &Unstated())),
        ReadingKind::Text,
        &mut store,)
    .expect("admits");

    let graph = report.Published_Into(&KnowledgeGraph::Empty());

    assert_eq!(graph.Current().Claims().len(), report.Coverage().Findings());
    assert_eq!(graph.Current().Concepts().len(), 2);
}

#[test]
fn Test_Publishing_Should_Leave_The_Graph_It_Was_Given_Unchanged()
{
    let mut store = DocumentStore::Empty();
    let report = Admit(b"a source".to_vec(), Some(&Said(&[Offered("entropy", "one")], &Unstated())), ReadingKind::Text, &mut store)
        .expect("admits");
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
    let extractions = [Offered("entropy", "one"), Offered("entropy", "two")];

    let first = Admit(b"a source".to_vec(), Some(&Said(&extractions, &Unstated())), ReadingKind::Text, &mut store).expect("admits");
    let graph = first.Published_Into(&KnowledgeGraph::Empty());
    let second = Admit(b"a source".to_vec(), Some(&Said(&extractions, &Unstated())), ReadingKind::Text, &mut store).expect("admits again");
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
    let report = Admit(b"a source".to_vec(), Some(&Said(&[Offered("phlogiston", "It is released in combustion.")], &Unstated())), ReadingKind::Text, &mut store)
        .expect("admits");
    let graph = report.Published_Into(&KnowledgeGraph::Empty());
    let held = graph.Every_Version().Concepts();
    let concept = (*held.first().expect("one concept")).clone();

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
    let report = Admit(b"a source".to_vec(), Some(&Said(&[Offered("entropy", "one")], &Unstated())), ReadingKind::Text, &mut store)
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

/// A scope a source did not state. `D-010`: an answer, not a default.
fn Unstated() -> Scope
{
    return Scope::Unstated();
}

// ---- KWB-36: the citation, which is what all of this was for ----

#[test]
fn Test_Two_Sources_Asserting_One_Claim_Should_Be_One_Claim_With_Two_Citations()
{
    // The mechanism every identity decision in this workspace was made to support, end to end
    // for the first time. Two different documents, the same claim text: one claim, two
    // assertions, each naming the address of the document it was read out of.
    let mut store = DocumentStore::Empty();
    let says = [Offered("entropy", "It is non-decreasing.")];

    let callen = Admit(
        b"Callen, Thermodynamics".to_vec(),
        Some(&Said(&says, &Unstated())),
        ReadingKind::Text,
        &mut store,
    )
    .expect("admits");
    let kittel = Admit(
        b"Kittel, Thermal Physics".to_vec(),
        Some(&Said(&says, &Unstated())),
        ReadingKind::Text,
        &mut store,
    )
    .expect("admits");

    let graph = kittel.Published_Into(&callen.Published_Into(&KnowledgeGraph::Empty()));

    assert_eq!(graph.Current().Claims().len(), 1, "the source is excluded, so this is one claim");
    assert_eq!(
        graph.Current().Assertions().len(),
        2,
        "and it carries two citations, which is the whole mechanism"
    );

    let cited: Vec<&str> = graph
        .Current()
        .Assertions()
        .iter()
        .map(|assertion| return assertion.Source())
        .collect();
    assert!(
        cited.contains(&callen.Source().expect("written").Identity().Render().as_str())
            && cited.contains(&kittel.Source().expect("written").Identity().Render().as_str()),
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
    let report = Admit(
        bytes.clone(),
        Some(&Said(&[Offered("entropy", "It is non-decreasing.")], &Unstated())),
        ReadingKind::Text,
        &mut store,)
    .expect("admits");

    let assertion = report.Assertions().first().expect("one assertion");
    let address = kwb_model::ContentIdentity::Parse(assertion.Source()).expect("an address");

    assert_eq!(store.Read(address).expect("resolves").Content(), bytes);
}

#[test]
fn Test_An_Unstated_Scope_Should_Stay_Unstated_Rather_Than_Become_The_Narrowest()
{
    let mut store = DocumentStore::Empty();
    let report = Admit(
        b"a source".to_vec(),
        Some(&Said(&[Offered("entropy", "It is non-decreasing.")], &Unstated())),
        ReadingKind::Text,
        &mut store,)
    .expect("admits");

    assert!(
        report.Assertions().first().expect("one").Scope().Is_Unstated(),
        "a source that did not say how far it meant has not said the narrowest thing"
    );
}

#[test]
fn Test_A_Stated_Scope_Should_Reach_The_Assertion()
{
    let mut store = DocumentStore::Empty();
    let report = Admit(
        b"a source".to_vec(),
        Some(&Said(&[Offered("entropy", "It is non-decreasing.")], &Scope::Named("physical theory").expect("a named scope"))),
        ReadingKind::Text,
        &mut store,)
    .expect("admits");

    assert_eq!(report.Assertions().first().expect("one").Scope().Name(), "physical theory");
}

#[test]
fn Test_Publications_Should_Order_Assertions_After_The_Claims_They_Name()
{
    // Replay refuses an assertion naming a claim no earlier record published, so this ordering
    // is a property of the method rather than an accident of iteration.
    let mut store = DocumentStore::Empty();
    let report = Admit(
        b"a source".to_vec(),
        Some(&Said(&[Offered("entropy", "one"), Offered("enthalpy", "two")], &Unstated())),
        ReadingKind::Text,
        &mut store,)
    .expect("admits");

    let records: Vec<String> = report
        .Publications()
        .iter()
        .map(|publication| return publication.Record(None))
        .collect();

    let replayed = kwb_domain::Replay(&records).expect("a run's own publications must replay");
    assert_eq!(replayed.Current().Assertions().len(), 2);
    assert_eq!(replayed.Current().Claims().len(), 2);
}

// ---- Stage zero: the extraction seam, and the four things it may not do ----

fn A_Reading(reader: &str) -> ExtractionLineage
{
    return ExtractionLineage::Of("stated-by-a-person", reader);
}

#[test]
fn Test_A_Reader_Who_Stated_Nothing_Should_Not_Become_A_Reader()
{
    // The invariant: model failure — or here, silence — must not become evidence of source
    // barrenness. A `Stated` holding no statements would `Read` successfully and propose
    // nothing, which admission reports as `Barren`: evidence there is nothing in the source.
    // Nobody examined anything, so the honest outcome is `Unmet`, and this is where that is
    // decided — by the reader being unconstructible rather than by every caller remembering.
    assert!(
        Stated::Of(
            Vec::new(),
            SourceLocation::Named("throughout"),
            A_Reading("a person"),
            Unstated(),
        )
        .is_none(),
        "silence was admitted as a reading, and an empty reading is evidence of absence"
    );

    assert!(
        Stated::Of(
            vec![Offered("entropy", "It is non-decreasing.")],
            SourceLocation::Named("throughout"),
            A_Reading("a person"),
            Unstated(),
        )
        .is_some(),
        "a reader who stated something must be a reader"
    );
}

#[test]
fn Test_Changing_The_Protocol_Should_Not_Redefine_What_A_Claim_Is()
{
    // The invariant this seam exists to protect: changing the model or the prompt must not
    // silently redefine KWB semantic identity. `D-002` excludes the source from a claim's
    // derivation so that two books asserting one thing are one claim; lineage is the same
    // question one level down, and it is answered by lineage travelling on the *reading*
    // rather than on the claim.
    //
    // The reference miner is the worked example of the other answer: its own claim identity
    // absorbs the source path and the page window, so the same sentence read twice is two
    // claims. A test that only checked the types would not see the difference.
    let statements = vec![Offered("entropy", "It is non-decreasing in an isolated system.")];
    let source = Document::Of(b"a source".to_vec()).Identity();

    let first = Stated::Of(
        statements.clone(),
        SourceLocation::Named("chapter two"),
        ExtractionLineage::Of("read-once-v1", "a person"),
        Unstated(),
    )
    .expect("a reader")
    .Read(source, b"the passage", ReadingKind::Text)
    .expect("a stated reading cannot fail")
    .remove(0);

    let second = Stated::Of(
        statements,
        SourceLocation::Named("chapter two, on re-reading"),
        ExtractionLineage::Of("read-again-v2", "a model"),
        Unstated(),
    )
    .expect("a reader")
    .Read(source, b"the passage", ReadingKind::Text)
    .expect("a stated reading cannot fail")
    .remove(0);

    assert_ne!(
        first.Lineage(),
        second.Lineage(),
        "the two readings must differ, or this test proves nothing"
    );

    let of_first = Link_Concepts(first.Proposed());
    let of_second = Link_Concepts(second.Proposed());

    assert_eq!(
        of_first.Claims().first().expect("one").Identity(),
        of_second.Claims().first().expect("one").Identity(),
        "a re-read under a new protocol produced a different claim, so changing the prompt \
         silently redefined what this repository thinks a proposition is"
    );
}

/// A reader that is asked and does not answer. The point of the tests below is that this is
/// reached through the same trait a working reader is, so a caller cannot tell them apart by
/// shape and must handle the refusal.
struct Silent;

impl ExtractionStrategy for Silent
{
    fn Read(
        &self,
        _source: kwb_model::ContentIdentity,
        _content: &[u8],
        _needed: ReadingKind,
    ) -> Result<Vec<ProposedReading>, ExtractionRefused>
    {
        return Err(ExtractionRefused::ReaderFailed {
            cause: "the answer did not parse".to_owned(),
        });
    }

    fn Scope(&self) -> Scope
    {
        return Unstated();
    }
}

#[test]
fn Test_A_Refusal_Should_Not_Be_Readable_As_A_Reading_That_Found_Nothing()
{
    // `Coverage`'s 1,367-row incident, one stage earlier. A refusal carries no proposals at
    // all — not an empty list of them — so there is no value a caller can take out of a failed
    // reading and hand to `Admit` that would make it look examined-and-empty. The failure is
    // in the return type, which is why it cannot be got wrong by a caller who forgets.
    let source = Document::Of(b"a source".to_vec()).Identity();
    let refusal = Silent
        .Read(source, b"the passage", ReadingKind::Text)
        .expect_err("a silent reader must refuse rather than propose nothing");

    // And it must say why in terms a report can print, because the consequence — nothing was
    // learned about the source — is the part a reader of that report has to act on, and it is
    // the part that distinguishes this from a source that really is empty.
    let reported = refusal.to_string();
    assert!(
        reported.contains("the answer did not parse"),
        "a refusal must report what went wrong: {reported}"
    );
    assert!(
        reported.contains("Nothing was learned about the source"),
        "a refusal must report the consequence, not only the cause: {reported}"
    );
}

#[test]
fn Test_A_Reading_Should_Carry_The_Address_Of_What_Was_Read()
{
    // Every admitted claim must identify its source occurrence and the protocol it was read
    // under. The source is carried as a content address rather than a path, so it names the
    // exact bytes admitted — a renamed or moved file is the same source, and an edited one is
    // not, which a filename cannot express.
    let source = Document::Of(b"a source".to_vec()).Identity();
    let reading = Stated::Of(
        vec![Offered("entropy", "It is non-decreasing.")],
        SourceLocation::Named("chapter two"),
        A_Reading("a person"),
        Scope::Named("physical theory").expect("a named scope"),
    )
    .expect("a reader")
    .Read(source, b"the passage", ReadingKind::Text)
    .expect("a stated reading cannot fail")
    .remove(0);

    assert_eq!(reading.Source(), source);
    assert_eq!(reading.Location().Description(), "chapter two");
    assert_eq!(reading.Lineage().Protocol(), "stated-by-a-person");
    assert_eq!(reading.Lineage().Reader(), "a person");
}

/// A reader that does text and nothing else, which is every extractor this repository expects
/// to gain first. It exists to demonstrate the refusal, because `Stated` cannot: a person has
/// no capability gap, so the asymmetry needs both sides present to be a test of anything.
struct TextOnly;

impl ExtractionStrategy for TextOnly
{
    fn Read(
        &self,
        source: kwb_model::ContentIdentity,
        content: &[u8],
        needed: ReadingKind,
    ) -> Result<Vec<ProposedReading>, ExtractionRefused>
    {
        if needed != ReadingKind::Text
        {
            return Err(ExtractionRefused::CannotRead { needed });
        }

        // Deliberately trivial: what a real reader proposes is not this test's subject. What is
        // its subject is that a reader which *can* read proposes, and one which cannot refuses.
        let proposed = core::str::from_utf8(content).map_or_else(
            |_| return Vec::new(),
            |text| return vec![Offered("a concept", text)],
        );

        return Ok(vec![ProposedReading::Of(
            source,
            SourceLocation::Named("throughout"),
            proposed,
            ExtractionLineage::Of("read-the-text-v1", "a text extractor"),
        )]);
    }

    fn Scope(&self) -> Scope
    {
        return Unstated();
    }
}

#[test]
fn Test_A_Text_Reader_Should_Refuse_A_Source_That_Needs_Looking_At()
{
    // The distinction the first extractor has to be able to draw. A scanned page handed to a
    // text reader is not a page with nothing in it, and the only way to keep those apart is for
    // the reader to say it cannot do this kind of reading.
    let mut store = DocumentStore::Empty();

    let report = Admit(
        b"a scan".to_vec(),
        Some(&TextOnly),
        ReadingKind::Visual,
        &mut store,
    )
    .expect("the source is admitted even though it could not be read");

    assert_eq!(report.Coverage().Name(), "unmet");
    assert!(
        !report.Coverage().Is_Evidence_Of_Absence(),
        "a page nobody could read was recorded as a page with nothing on it"
    );
    assert_eq!(
        report.Refusal(),
        Some(&ExtractionRefused::CannotRead {
            needed: ReadingKind::Visual
        }),
        "the refusal must say which kind of reading was needed, or a report cannot say what \
         would have to change"
    );
}

#[test]
fn Test_The_Same_Reader_Should_Read_A_Source_That_Is_Already_Text()
{
    // The other half, and the reason the test above proves something: this reader refuses on
    // the kind of reading and not on everything, so the refusal is a distinction rather than a
    // reader that never works.
    let mut store = DocumentStore::Empty();

    let report = Admit(
        b"a passage".to_vec(),
        Some(&TextOnly),
        ReadingKind::Text,
        &mut store,
    )
    .expect("admits");

    assert_eq!(report.Coverage().Name(), "yielded");
    assert_eq!(report.Assertions().len(), 1);
    assert!(report.Refusal().is_none(), "a reading that happened is not a refusal");
}

/// A reader that answers about a document it was not given.
///
/// Not a hypothetical. `Admit` cites the document *it* wrote and never compared it with the one
/// the reading names, so this reader's proposals were attributed to whatever source happened to
/// be passed in, with a citation that resolved perfectly to the wrong bytes.
struct Confused;

impl ExtractionStrategy for Confused
{
    fn Read(
        &self,
        _source: kwb_model::ContentIdentity,
        _content: &[u8],
        _needed: ReadingKind,
    ) -> Result<Vec<ProposedReading>, ExtractionRefused>
    {
        return Ok(vec![ProposedReading::Of(
            Document::Of(b"some other document".to_vec()).Identity(),
            SourceLocation::Named("throughout"),
            vec![Offered("entropy", "It is non-decreasing.")],
            A_Reading("a reader with the wrong book open"),
        )]);
    }

    fn Scope(&self) -> Scope
    {
        return Unstated();
    }
}

#[test]
fn Test_A_Reading_About_Another_Document_Should_Not_Be_Cited_As_This_One()
{
    // That a citation resolves to the exact bytes a claim was read out of is the strongest
    // thing this repository claims, and `D-006`'s dependency edge rests on it. An unchecked
    // field is not provenance; it is a field.
    let mut store = DocumentStore::Empty();

    let report = Admit(
        b"a source".to_vec(),
        Some(&Confused),
        ReadingKind::Text,
        &mut store,
    )
    .expect("the source is admitted; only the reading is refused");

    assert!(
        report.Assertions().is_empty(),
        "a reading about another document was cited as this one, so the citation resolves to \
         bytes the claim was not read out of"
    );
    assert_eq!(report.Coverage().Name(), "unmet");
    assert!(
        report
            .Refusal()
            .is_some_and(|refusal| return matches!(refusal, ExtractionRefused::ReaderFailed { .. })),
        "a reader that answered about the wrong document was not reported as having failed"
    );
}

#[test]
fn Test_A_Reading_About_The_Right_Document_Should_Still_Be_Admitted()
{
    // The control. Without it the test above passes against a check that refuses every reading,
    // which would be a guard that reports the strongest possible provenance by admitting
    // nothing at all.
    let mut store = DocumentStore::Empty();

    let report = Admit(
        b"a source".to_vec(),
        Some(&Said(&[Offered("entropy", "It is non-decreasing.")], &Unstated())),
        ReadingKind::Text,
        &mut store,
    )
    .expect("admits");

    assert_eq!(report.Assertions().len(), 1);
    assert!(report.Refusal().is_none());
}
