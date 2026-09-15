//! The reader the corpus's admission rule is written about, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/extraction.rs`
//!
//! `Test_A_Reader_Who_Stated_Nothing_Should_Not_Become_A_Reader` is the test that turns on this
//! constructor, and it never names it. The unit this check reads is the test file's own stem, so
//! `src/extractor/stated.rs` is covered by a file called `stated.rs` and by nothing else — and a
//! change to when a person counts as a reader would have arrived under the name of a test about
//! the seam.
//!
//! `Read` and `Scope` are not here. They are this type's implementation of the seam, and the seam
//! is what `src/tests/extraction.rs` is named for and already asserts; what this file gives an
//! address is the constructor that decides whether there is a reader at all.

use kwb_domain::Scope;
use kwb_ingest::ClaimText;
use kwb_ingest::ConceptName;
use kwb_ingest::Extraction;
use kwb_ingest::ExtractionLineage;
use kwb_ingest::ReaderName;
use kwb_ingest::ReadingProtocol;
use kwb_ingest::SourceLocation;
use kwb_ingest::Stated;

/// What a person says a source asserts, when they say something.
fn Statements() -> Vec<Extraction>
{
    return vec![An_Extraction("entropy", "It is non-decreasing in an isolated system.")];
}

fn An_Extraction(concept: &str, claim: &str) -> Extraction
{
    return Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));
}

fn A_Location() -> SourceLocation
{
    return SourceLocation::Named("throughout");
}

fn A_Lineage() -> ExtractionLineage
{
    return ExtractionLineage::Of(
        ReadingProtocol::Named("stated-by-a-person"),
        ReaderName::Named("a person"),
    );
}

#[test]
fn Test_Of_Should_Answer_None_When_The_Person_Said_Nothing()
{
    // A reading that examined material and found none is `Barren`, which is **evidence of
    // absence** and the only outcome that licenses a caller to conclude there is nothing there. A
    // person who supplied no statements has not examined anything, so admitting them as a reader
    // would turn *nobody said anything* into *there is nothing to say* — the 1,367-row incident
    // arriving through the front door.
    assert!(
        Stated::Of(Vec::new(), A_Location(), A_Lineage(), Scope::Unstated()).is_none(),
        "silence was admitted as a reader, so an empty reading becomes evidence of absence"
    );

    // And the other half, because a constructor that refused everything would satisfy the first
    // on its own — a reader that never exists is a reader nothing can be admitted through.
    assert!(
        Stated::Of(Statements(), A_Location(), A_Lineage(), Scope::Unstated()).is_some(),
        "a person who stated something is not a reader, so `--says` admits nothing"
    );
}
