//! What stage two produced, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/normalization.rs`
//!
//! That file asserts the algorithmic property this stage depends on — grouping is a transitive
//! closure, so handing it a relation that is not transitive is `D18` — and it reaches every
//! function here on the way. What it does not do is name any of them, and its stem is
//! `normalization` rather than `normalized`, so `src/concepts/normalized.rs` is covered by a file
//! called `normalized.rs` and by nothing else.
//!
//! `Normalize_Concepts` itself is not here. It is `pub(crate)`, so a file under `tests/` cannot
//! call it; its own contract is asserted in the inline module beside it.

use kwb_domain::Scope;
use kwb_ingest::Admit_Source;
use kwb_ingest::ClaimText;
use kwb_ingest::ConceptName;
use kwb_ingest::Extraction;
use kwb_ingest::ExtractionLineage;
use kwb_ingest::Normalized;
use kwb_ingest::ReaderName;
use kwb_ingest::ReadingKind;
use kwb_ingest::ReadingProtocol;
use kwb_ingest::SourceLocation;
use kwb_ingest::Stated;
use kwb_store::DocumentStore;

/// Two passages naming one concept: one content identity, two mentions, so a merge is a number
/// this fixture can state.
fn Mentions_Of_One_Concept() -> Vec<Extraction>
{
    return vec![
        An_Extraction("entropy", "It is non-decreasing."),
        An_Extraction("entropy", "It is extensive."),
    ];
}

/// Two concepts that are not one concept, which is the control: without it a stage that grouped
/// everything would satisfy the assertions below.
fn Distinct_Mentions() -> Vec<Extraction>
{
    return vec![
        An_Extraction("entropy", "It is non-decreasing."),
        An_Extraction("enthalpy", "It is extensive."),
    ];
}

fn An_Extraction(concept: &str, claim: &str) -> Extraction
{
    return Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));
}

/// Stage two's own output, reached through the door every caller has to use.
fn Normalized_From(statements: &[Extraction]) -> Normalized
{
    let mut store = DocumentStore::Empty();
    let reader = Stated_By_A_Person(statements);

    return Admit_Source(b"a source".to_vec(), Some(&reader), ReadingKind::Text, &mut store)
        .expect("a source of bytes is admitted even when its reading does not happen")
        .Normalized()
        .clone();
}

fn Stated_By_A_Person(statements: &[Extraction]) -> Stated
{
    return Stated::Of(
        statements.to_vec(),
        SourceLocation::Named("throughout"),
        ExtractionLineage::Of(
            ReadingProtocol::Named("stated-by-a-person"),
            ReaderName::Named("a test"),
        ),
        Scope::Unstated(),
    )
    .expect("a person who stated something is a reader");
}

#[test]
fn Test_Concepts_Should_Be_One_Per_Content_Identity()
{
    // What the stage is: grouping over identity, and over nothing else. Two mentions of one name
    // are one concept; two mentions of two names are two. The second is the control, and it is the
    // one `D18` says to keep — a stage that collapsed everything would satisfy the first half.
    let merged = Normalized_From(&Mentions_Of_One_Concept());
    let distinct = Distinct_Mentions();
    let kept = Normalized_From(&distinct);

    assert_eq!(
        merged.Concepts().len(),
        1,
        "two mentions of one name stayed two concepts, so a re-read book is two books' worth of \
         knowledge"
    );
    assert_eq!(
        kept.Concepts().len(),
        distinct.len(),
        "two concepts that are not one concept were grouped, which is the relation this stage \
         refuses to be given a looser version of"
    );
}

#[test]
fn Test_Linked_Should_Come_Back_Unchanged_Because_Grouping_Removes_No_Claim()
{
    // Normalization removes no claim, and this is how a caller checks that rather than trusting it:
    // everything linking produced comes back, so a merge cannot have quietly dropped the loser's
    // claims — a destructive edit with nothing authorising it, and `D17`'s rule arriving one stage
    // early.
    let mentions = Mentions_Of_One_Concept();
    let normalized = Normalized_From(&mentions);

    assert_eq!(
        normalized.Linked().Claims().len(),
        mentions.len(),
        "grouping dropped claims belonging to the concept it folded away"
    );
    assert_eq!(
        normalized.Linked().Refused(),
        0,
        "normalization changed what linking refused, so the two stages disagree about the input"
    );
}

#[test]
fn Test_Claims_Held_Should_Count_Every_Claim_Grouping_Kept()
{
    // The count a caller reads to check that a merge removed nothing. The two extractions name one
    // concept and assert two different things, so a merge that dropped the loser's claim would move
    // this number while `Concepts` still said one — which is precisely the failure the two
    // assertions below are drawn apart to catch.
    let mentions = Mentions_Of_One_Concept();
    let normalized = Normalized_From(&mentions);

    assert_eq!(normalized.Concepts().len(), 1, "the fixture must merge, or this proves nothing");
    assert_eq!(
        normalized.Claims_Held(),
        mentions.len(),
        "the stage reports fewer claims than it was handed, so a merge lost one"
    );
}

#[test]
fn Test_Merged_Should_Count_How_Many_Mentions_Folded_In()
{
    // The number the transitivity test turns on. A count of concepts would not have found `D18`,
    // because the defect was in the structure consuming the relation rather than in the relation —
    // and a merge count is the only number that says how much of the input was decided to be a
    // duplicate. Both answers, because a counter stuck at zero satisfies the second.
    assert_eq!(
        Normalized_From(&Mentions_Of_One_Concept()).Merged(),
        1,
        "the second mention of one concept was not counted as folded in"
    );
    assert_eq!(
        Normalized_From(&Distinct_Mentions()).Merged(),
        0,
        "a concept with no duplicate was counted as merged, so the number does not say what was \
         grouped"
    );
}
