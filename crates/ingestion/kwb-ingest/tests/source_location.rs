//! Where in a source something was found, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/extraction.rs`
//!
//! Every reading in this workspace carries a location and none of those tests names this type.
//! The unit this check reads is the test file's own stem, so `src/readings/source_location.rs` is
//! covered by a file called `source_location.rs` and by nothing else — and a change to how a
//! passage is located would have arrived under the name of a test about a claim.

use kwb_ingest::SourceLocation;

/// One location named three ways, every difference between them whitespace.
///
/// The medium here is a chapter, which is the point the type makes: what locates a passage
/// differs by medium, so this is text rather than a page or an offset.
fn One_Place_Spelled_Three_Ways() -> [&'static str; 3]
{
    return ["  chapter two  ", "chapter  two", "chapter two"];
}

#[test]
fn Test_Named_Should_Normalize_A_Location_So_One_Place_Is_One()
{
    // Normalized like the other reading apparatus and unlike the reading itself: where a passage
    // was found is a fact about the act of reading rather than about what was read, so a
    // reflowed description of one place is one place.
    let spellings = One_Place_Spelled_Three_Ways();
    let tight = SourceLocation::Named(spellings.last().copied().expect("the spellings")).Description().to_owned();

    for spelling in spellings
    {
        assert_eq!(
            SourceLocation::Named(spelling).Description(),
            tight.as_str(),
            "two descriptions of one place are two places, so nothing reading a location could \
             group two citations of the same passage"
        );
    }
}

#[test]
fn Test_Description_Should_Answer_With_What_Was_Carried()
{
    // What it answers with is the normalized description rather than the caller's spelling, and
    // it is deliberately opaque text: nothing here parses a page number out, because a structured
    // location invented for one medium becomes the structure every later medium pretends to fit.
    assert_eq!(
        SourceLocation::Named("  throughout  ").Description(),
        "throughout",
        "a location came back exactly as it was handed in, so nothing was normalized"
    );
    assert_eq!(SourceLocation::Named("p. 12, chapter two").Description(), "p. 12, chapter two");
}
