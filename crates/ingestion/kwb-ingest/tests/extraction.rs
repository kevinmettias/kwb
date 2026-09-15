//! One concept-and-claim pair as an extractor produced it, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/extraction.rs`
//!
//! That file is named for the **seam** and asserts the four things an `ExtractionStrategy` may
//! not do; it builds an `Extraction` in every helper and names neither of its functions. The unit
//! this check reads is the test file's own stem, and both files have the same stem — so
//! `src/extractor/extraction.rs` is covered by a file called `extraction.rs`, and the tests below
//! are the ones that give `New` and `Is_Complete` an address.

use kwb_ingest::ClaimText;
use kwb_ingest::ConceptName;
use kwb_ingest::Extraction;

/// An extraction with both halves present, which every refusal below is asserted against.
fn Complete() -> Extraction
{
    return An_Extraction("entropy", "It is non-decreasing in an isolated system.");
}

/// The same extraction with its concept blank.
fn Blank_Concept() -> Extraction
{
    return An_Extraction("", "It is non-decreasing in an isolated system.");
}

/// The same extraction with its claim blank.
fn Blank_Claim() -> Extraction
{
    return An_Extraction("entropy", "");
}

/// A blank that is not empty, which is the case a check on `is_empty` alone would admit.
fn Whitespace_Only() -> Extraction
{
    return An_Extraction("entropy", " \t\n ");
}

fn An_Extraction(concept: &str, claim: &str) -> Extraction
{
    return Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));
}

#[test]
fn Test_New_Should_Keep_The_Two_Halves_In_The_Positions_They_Arrived_In()
{
    // The defect the two types exist for: this constructor used to take two adjacent `String`s,
    // so an extractor's concept name and its claim text could be handed over the wrong way round
    // and the compiler would accept it — producing a claim whose concept is its own text, and a
    // concept named after an assertion. Neither position is interchangeable with the other, so
    // what this asserts is that the two arrive where they were put.
    let extraction = Complete();

    assert_eq!(extraction.concept_name.Text(), "entropy");
    assert_eq!(
        extraction.claim_text.Text(),
        "It is non-decreasing in an isolated system.",
        "the two halves were transposed on the way in, which is the swap the types exist to \
         make a compile error"
    );
}

#[test]
fn Test_Is_Complete_Should_Refuse_An_Extraction_Missing_Either_Half()
{
    // Both sides and the blank case, because an incomplete extraction is admitted with an
    // empty field rather than refused — which is the shape of the prototype's `ReplaceAllAsync`
    // incident, where an empty-but-valid reply erased a concept's entire synthesis because
    // empty was indistinguishable from answered.
    assert!(
        Complete().Is_Complete(),
        "an extraction carrying both halves is not complete, so every claim is refused"
    );
    assert!(
        !Blank_Concept().Is_Complete(),
        "an extraction naming no concept is complete, so it is admitted with a blank concept"
    );
    assert!(
        !Blank_Claim().Is_Complete(),
        "an extraction asserting nothing is complete, so it is admitted with a blank claim"
    );
    assert!(
        !Whitespace_Only().Is_Complete(),
        "an extraction whose claim is blank is complete, so a whitespace-only answer is \
         indistinguishable from an answered one"
    );
}
