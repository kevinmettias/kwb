//! What stage one produced, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/linking.rs`
//!
//! That file asserts the stage's algorithmic property — it infers nothing, a claim attaches to the
//! concept its own extraction named, and an incomplete extraction is refused rather than admitted
//! with a blank half — and it reaches every function here on the way. What it does not do is name
//! any of them. The unit this check reads is the test file's own stem, and `linking` is not
//! `linked`: `src/concepts/linked.rs` is covered by a file called `linked.rs` and by nothing else,
//! so a failure in `Refused` arrived under the name of a test about a concept.
//!
//! `Link_Concepts` itself is not here. It is `pub(crate)`, so a file under `tests/` compiles as its
//! own package and cannot call it; its own contract is asserted in the inline module beside it.

use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::Scope;
use kwb_ingest::Admit_Source;
use kwb_ingest::ClaimText;
use kwb_ingest::ConceptName;
use kwb_ingest::Extraction;
use kwb_ingest::ExtractionLineage;
use kwb_ingest::Linked;
use kwb_ingest::ReaderName;
use kwb_ingest::ReadingKind;
use kwb_ingest::ReadingProtocol;
use kwb_ingest::SourceLocation;
use kwb_ingest::Stated;
use kwb_store::DocumentStore;

/// The one concept every extraction below names, so that a merge is visible as a merge rather than
/// as a difference between two fixtures.
const ONE_CONCEPT: &str = "entropy";

/// Two passages naming one concept, which is what makes *linking does not deduplicate* visible.
fn Mentions_Of_One_Concept() -> Vec<Extraction>
{
    return vec![
        An_Extraction(ONE_CONCEPT, "It is non-decreasing."),
        An_Extraction(ONE_CONCEPT, "It is extensive."),
    ];
}

/// Extractions each missing a half, which linking must refuse rather than admit with a blank side.
fn Half_Stated() -> Vec<Extraction>
{
    return vec![
        An_Extraction(ONE_CONCEPT, " \t "),
        An_Extraction("   ", "orphaned"),
    ];
}

fn An_Extraction(concept: &str, claim: &str) -> Extraction
{
    return Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));
}

/// Stage one's own output, reached through the door every caller has to use.
///
/// `Link_Concepts` is `pub(crate)`, so the stage cannot be called from here. What can be called is
/// `Admit_Source`, which runs it — so the output is reached the way a caller reaches it rather than
/// through a hole cut for the test.
fn Linked_From(statements: &[Extraction]) -> Linked
{
    let mut store = DocumentStore::Empty();
    let reader = Stated_By_A_Person(statements);

    return Admit_Source(b"a source".to_vec(), Some(&reader), ReadingKind::Text, &mut store)
        .expect("a source of bytes is admitted even when its reading does not happen")
        .Normalized()
        .Linked()
        .clone();
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

#[test]
fn Test_Concepts_Should_Stay_Unmerged_Because_Grouping_Is_The_Next_Stage()
{
    // The stage boundary, asserted at the boundary. With the grouping done here, normalization
    // would have nothing left to group — a stage that could never be wrong about merging, which is
    // the same as not having the stage. Worse, it happened: the control test meant to prove
    // normalization *does* merge passed anyway, because linking had already done the merging, and
    // a stage boundary drawn in the wrong place made a test vacuous without making it fail.
    let mentions = Mentions_Of_One_Concept();
    let linked = Linked_From(&mentions);

    assert_eq!(
        linked.Concepts().len(),
        mentions.len(),
        "two mentions of one name came back as one concept, so normalization has nothing to group \
         and its own test passes for a reason nobody intended"
    );
    assert_eq!(
        linked
            .Concepts()
            .first()
            .expect("one concept per complete extraction")
            .Canonical_Name(),
        ONE_CONCEPT
    );
}

#[test]
fn Test_Claims_Should_Arrive_In_The_Order_Their_Extractions_Did()
{
    // The whole of what this stage does: a claim is attached to the concept **its own extraction
    // named**, in the order its extraction arrived, and to no other. There is no matching, no
    // similarity, no alias resolution and no model in this path, so there is no relation here
    // whose algebra anything would have to check — which is what makes it safe to hand its output
    // to a stage that closes over what it is given.
    let mentions = Mentions_Of_One_Concept();
    let linked = Linked_From(&mentions);

    let expected: Vec<Claim> = mentions
        .iter()
        .map(|extraction| {
            return Claim::About(
                &Concept::Named(extraction.concept_name.Text()),
                extraction.claim_text.Text(),
            );
        })
        .collect();

    assert_eq!(
        linked.Claims(),
        expected.as_slice(),
        "a claim is not attached to the concept its own extraction named, in the order it arrived"
    );
}

#[test]
fn Test_Refused_Should_Count_What_Linking_Threw_Away()
{
    // Reported rather than dropped. `D19`'s corollary is that a report may only count what it can
    // see, and a stage that discards input without saying how much is the reason that corollary
    // exists. Both answers are asserted, because a counter stuck at zero satisfies the second half
    // on its own.
    let half = Half_Stated();
    let complete = Mentions_Of_One_Concept();

    assert_eq!(
        Linked_From(&half).Refused(),
        half.len(),
        "an extraction missing a half was admitted rather than refused, so a claim with a blank \
         side reaches the graph"
    );
    assert_eq!(
        Linked_From(&complete).Refused(),
        0,
        "a complete extraction was counted as refused, so the number no longer says what was \
         thrown away"
    );
}
