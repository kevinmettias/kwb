//! What produced an extraction, and under what protocol, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/extraction.rs`
//!
//! The claim-identity test builds a lineage in each hand and names none of these functions. The
//! unit this check reads is the test file's own stem, so `src/extractor/extraction_lineage.rs` is
//! covered by a file called `extraction_lineage.rs` and by nothing else — and a change to how a
//! lineage reports itself would have arrived under the name of a test about a claim.

use kwb_ingest::ExtractionLineage;
use kwb_ingest::ReaderName;
use kwb_ingest::ReadingProtocol;

/// One reading: a protocol and the reader that read under it.
///
/// The two are named apart from each other in the fixture, so that a `Protocol` test and a
/// `Reader` test cannot both pass by reading the same string.
fn A_Reading() -> ExtractionLineage
{
    return ExtractionLineage::Of(
        ReadingProtocol::Named("read-the-text-v1"),
        ReaderName::Named("a text extractor"),
    );
}

#[test]
fn Test_Of_Should_Keep_The_Two_Names_Apart_By_Type()
{
    // What the two types are for. This constructor used to take two adjacent `&str`s, so the
    // protocol and the reader could be handed over the wrong way round — filing a person under a
    // protocol nobody read under — and the compiler would accept it. There is no expression that
    // writes the swap, and the pair below is what says the two arrive where they were put rather
    // than merely arriving.
    let lineage = A_Reading();

    assert_eq!(lineage.Protocol(), "read-the-text-v1");
    assert_eq!(lineage.Reader(), "a text extractor");
}

#[test]
fn Test_Protocol_Should_Answer_The_Protocol_The_Reading_Was_Taken_Under()
{
    // The half a caller reporting which prompt produced a corpus reaches for. It answers the
    // protocol and not the reader, which the assertion below states by holding the reader fixed
    // while nothing but the protocol moves.
    let other = ExtractionLineage::Of(
        ReadingProtocol::Named("read-the-text-v2"),
        ReaderName::Named("a text extractor"),
    );

    assert_eq!(A_Reading().Protocol(), "read-the-text-v1");
    assert_ne!(
        A_Reading().Protocol(),
        other.Protocol(),
        "two protocols under one name, so a corpus read under a new prompt is reported as having \
         been read under the old one"
    );
}

#[test]
fn Test_Reader_Should_Answer_Who_Or_What_Did_The_Reading()
{
    // A person as readily as a model: the corpus's admission rule turns on whether a feature
    // still makes sense with a perfect human annotator, and this is what makes that a fact the
    // lineage states rather than one a reader has to infer from which strategy ran.
    let by_a_person = ExtractionLineage::Of(
        ReadingProtocol::Named("read-the-text-v1"),
        ReaderName::Named("a person"),
    );

    assert_eq!(A_Reading().Reader(), "a text extractor");
    assert_ne!(
        A_Reading().Reader(),
        by_a_person.Reader(),
        "two readers under one name, so a re-read by a model is reported as the reading a person \
         gave"
    );
}
