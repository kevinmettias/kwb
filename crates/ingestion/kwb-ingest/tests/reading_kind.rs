//! What a kind of reading is called, asserted in the file named for it.
//!
//! # Why this file exists at all
//!
//! `src/readings/reading_kind.rs` declares one function. `Name` was exercised the moment a
//! refusal carried a kind and printed it, and no test named it — so a change to a vocabulary
//! **both sides of the extraction seam quote** would have arrived under the name of a test about
//! something else. The unit this check reads is the test file's own stem, so a file called
//! `reading_kind.rs` is the only address that reaches it.

use kwb_ingest::ReadingKind;

/// How many kinds the vocabulary has, which is every one of them.
///
/// Named rather than written in the return type, because the number is a fact about the
/// vocabulary rather than about the test: a kind added to it is a kind this fixture must hold,
/// and a reader should not have to count the array to know whether it does.
const KINDS_IN_THE_VOCABULARY: usize = 2;

/// Every kind this vocabulary has, so that the distinctness below is asserted over all of them
/// rather than over whichever two a test happened to name.
///
/// A provider rather than a literal in the test: a third kind added here is a third kind the
/// duplicate check runs against, which is the case a hand-written pair would silently miss.
fn Kinds() -> [ReadingKind; KINDS_IN_THE_VOCABULARY]
{
    return [ReadingKind::Text, ReadingKind::Visual];
}

#[test]
fn Test_Name_Should_Be_A_Short_Stable_Word_Per_Kind()
{
    // What a report prints and what a stored journal keeps, so these are load-bearing strings
    // rather than labels: changing one silently changes what every report already written says.
    assert_eq!(ReadingKind::Text.Name(), "text");
    assert_eq!(ReadingKind::Visual.Name(), "visual");

    // And no two kinds may share one, because a refusal names the reading it could not do — and
    // two kinds under one name would leave a person reading that refusal unable to tell which
    // capability their extractor is missing.
    let mut names: Vec<&'static str> = Kinds().iter().map(|kind| return kind.Name()).collect();
    names.sort_unstable();
    names.dedup();

    assert_eq!(
        names.len(),
        Kinds().len(),
        "two kinds of reading answer one name, so a refusal for a kind an extractor cannot do \
         would not say which reading was needed"
    );
}
