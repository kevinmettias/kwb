//! Who or what read a source, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/extraction.rs`
//!
//! The seam's tests build a `ReaderName` in every helper they use and name none of them. The unit
//! this check reads is the test file's own stem, so `src/readings/reader_name.rs` is covered by a
//! file called `reader_name.rs` and by nothing else — and a change to how a reader is named would
//! have arrived under the name of a test about a protocol.

use kwb_ingest::ReaderName;

/// One reader named three ways: spaced, doubled, and tight. Every difference between them is
/// whitespace, so every difference between them is one this type is supposed to erase.
fn One_Reader_Spelled_Three_Ways() -> [&'static str; 3]
{
    return ["  A Model  ", "A  Model", "A Model"];
}

#[test]
fn Test_Named_Should_Normalize_A_Reader_So_One_Reader_Is_One_Reader()
{
    // Normalized, unlike the names a source is read into. A reader is a name a repository keeps
    // rather than a reading of a source, so two spellings of one reader are one reader — and a
    // lineage that recorded the spelling would make one reading apparatus look like two, which is
    // the confusion `ExtractionLineage` exists to prevent in the other direction.
    let spellings = One_Reader_Spelled_Three_Ways();
    let named = ReaderName::Named(spellings.last().copied().expect("the spellings"));

    for spelling in spellings
    {
        assert_eq!(
            ReaderName::Named(spelling).Text(),
            named.Text(),
            "two spellings of one reader are two readers, so a re-read by the same apparatus \
             looks like a re-read by a different one"
        );
    }
}

#[test]
fn Test_Text_Should_Answer_With_The_Name_That_Was_Carried()
{
    // And what it answers with is the normalized name, not the caller's spelling: a caller that
    // got its own blank padding back would have no way to tell whether the value it holds is one
    // a comparison will agree with.
    assert_eq!(
        ReaderName::Named("  a text extractor  ").Text(),
        "a text extractor",
        "a reader's name came back exactly as it was handed in, so nothing was normalized"
    );
    assert_eq!(
        ReaderName::Named("  Gemini-Pro  ").Text(),
        "Gemini-Pro",
        "the blanks were kept or the punctuation was folded, so one reader is two readers"
    );
}
