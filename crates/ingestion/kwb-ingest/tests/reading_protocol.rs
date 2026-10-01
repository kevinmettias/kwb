//! Under what instructions a source was read, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/extraction.rs`
//!
//! `Test_Changing_The_Protocol_Should_Not_Redefine_What_A_Claim_Is` is the test that turns on
//! this type, and it never names it. The unit this check reads is the test file's own stem, so
//! `src/readings/reading_protocol.rs` is covered by a file called `reading_protocol.rs` and by
//! nothing else — and a change to what a protocol *is* would have arrived under the name of a
//! test about a claim, which is the one test that must keep passing unchanged.

use kwb_ingest::ReadingProtocol;

/// How many spellings of one protocol the fixture holds, every difference between them
/// whitespace.
///
/// Named rather than written in the return type, because the number is a fact about the fixture
/// and a reader should not have to count it to read the assertion stated over all three.
const SPELLINGS_OF_ONE_PROTOCOL: usize = 3;

/// One protocol named three ways, every difference between them whitespace.
///
/// Whitespace and not punctuation, deliberately: `kwb-model`'s normalization collapses runs of
/// whitespace and touches nothing else, so a fixture whose spellings differed in their hyphens
/// would be asserting that one protocol is two.
fn One_Protocol_Spelled_Three_Ways() -> [&'static str; SPELLINGS_OF_ONE_PROTOCOL]
{
    return ["  read-the-text-v1  ", "read-the-text-v1", "read-the-text-v1\n"];
}

/// What one spelling normalizes to, so that the loop below names the transformation rather than
/// the constructor.
fn Normalized_Protocol(spelling: &str) -> String
{
    return ReadingProtocol::Named(spelling).Text().to_owned();
}

#[test]
fn Test_Named_Should_Normalize_A_Protocol_So_One_Protocol_Is_One()
{
    // Normalized, and this is the reason stated as a test: two runs that spell one protocol
    // differently are one protocol. A lineage that recorded the spelling would make a re-read
    // under the same instructions look like a re-read under new ones — which is precisely the
    // reading that must *not* move, because it is the one the claim-identity test is stated over.
    let spellings = One_Protocol_Spelled_Three_Ways();
    let tight = Normalized_Protocol(spellings.first().copied().expect("the spellings"));

    for spelling in spellings
    {
        assert_eq!(
            Normalized_Protocol(spelling),
            tight,
            "two spellings of one protocol are two protocols, so a re-read under one prompt \
             looks like a re-read under another"
        );
    }
}

#[test]
fn Test_Text_Should_Answer_With_The_Protocol_That_Was_Carried()
{
    // What it answers with is the normalized protocol, so a value held by one caller compares
    // equal to a value held by another however each of them spelled it.
    assert_eq!(
        ReadingProtocol::Named("  read-the-text-v1  ").Text(),
        "read-the-text-v1",
        "a protocol came back exactly as it was handed in, so nothing was normalized"
    );

    // Case is deliberately **not** folded — `kwb-model`'s own normalization says so — because a
    // protocol is a name a repository keeps, and two names differing in case are two names it
    // kept. Folding them would make a renamed protocol look like the one it replaced.
    assert_ne!(
        ReadingProtocol::Named("read-the-text-v1").Text(),
        ReadingProtocol::Named("Read-The-Text-V1").Text(),
        "case was folded, so a protocol a repository renamed looks like the one it replaced"
    );
}
