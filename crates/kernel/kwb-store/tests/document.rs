//! What a document is, asserted where a document is the subject.
//!
//! # Why this file sits beside `tests/one_door.rs`
//!
//! `one_door.rs` asserts properties of the **store** — one write door, a write that cannot
//! destroy, an admission decided from the medium rather than from this process — and it reaches
//! [`Document`] only as the thing a write is handed. So every function on that type was already
//! exercised and none of them was named. The unit this check reads is the test file's own stem, so
//! `src/document.rs` is covered by a file called `document.rs` and by nothing else, and a failure
//! in `Length` arrived under the name of a test about a door.
//!
//! Each test below is one function of that file, asserting the thing that function is for.

use kwb_model::Derivation;
use kwb_store::Document;

/// Octets whose identity would change if a document went through the text door.
///
/// `Normalize_Text` strips every C0 and C1 control character, and the derivation's own field
/// separator is `0x1F` — one of them. [`Derivation::With_Bytes`] is the door a document goes
/// through precisely because every octet is the content, and this is the input that tells the two
/// doors apart.
const OPAQUE_OCTETS: &[u8] = &[0x00, 0x1F, 0xFF, b'\n', b' '];

/// Text whose byte count is not its character count.
const MULTI_BYTE: &str = "caf\u{e9} \u{1F980}";

#[test]
fn Test_Of_Should_Derive_The_Address_From_The_Octets_Under_The_Document_Kind()
{
    // The derivation is written out here rather than described, because the literals are the whole
    // assertion: `DOCUMENT` and `CONTENT` are private to `src/document.rs`, so this is the only
    // place that can state the scheme independently of the code that applies it.
    //
    // A document whose identity changed its inputs would change the value every consumer already
    // holds as a `KnowledgeReferenceId`, and `AGENTS.md` says a change to that scheme is a decision
    // to record rather than a detail to change quietly. This is what makes it not quiet.
    let content = b"a passage".to_vec();

    let derived = Derivation::Of("document").With_Bytes("content", &content);

    assert_eq!(
        Document::Of(content).Identity(),
        derived.Seal().Identity(),
        "a document's address is no longer the digest of its octets under the kind `document`, so \
         every address this store has ever handed out has changed meaning"
    );
}

#[test]
fn Test_Identity_Should_Answer_Differently_For_Documents_That_Differ_In_One_Octet()
{
    // Determinism and separation, which are the two things an address has to have and the two a
    // salt, a clock or a truncated digest would take away. Two calls over the same bytes answer the
    // same address; one octet apart answers a different one, or one of the two documents is
    // unreachable behind the other's address.
    let once = Document::Of(b"a passage".to_vec()).Identity();
    let twice = Document::Of(b"a passage".to_vec()).Identity();
    let altered = Document::Of(b"b passage".to_vec()).Identity();

    assert_eq!(once, twice, "the same octets were addressed two different ways");
    assert_ne!(
        once, altered,
        "two documents an octet apart share one address, so writing the second would be reported \
         as a re-offer of the first and its bytes would never land"
    );
}

#[test]
fn Test_Content_Should_Hand_Back_Octets_That_No_Text_Door_Would_Keep()
{
    // The complement of `Of`'s test: the identity is derived opaquely and what is handed back is
    // the same octets. A control byte, a NUL and a byte that is not valid UTF-8 are all still
    // there. Had the content gone through `With_Text`, `Normalize_Text` would have removed the
    // control characters — and the document's identity would have been taken from bytes the store
    // does not hold.
    let document = Document::Of(OPAQUE_OCTETS.to_vec());

    assert_eq!(
        document.Content(),
        OPAQUE_OCTETS,
        "the store hands back something other than the octets it was given"
    );
}

#[test]
fn Test_Length_Should_Count_Bytes_And_Not_Characters()
{
    // A document is octets, so its length is a count of octets. An implementation counting
    // characters would under-report the size of anything carrying text outside ASCII — which is
    // what a caller sizing a buffer, or a report totalling a corpus, reads this for.
    assert_ne!(
        MULTI_BYTE.len(),
        MULTI_BYTE.chars().count(),
        "this fixture has as many octets as characters, so it cannot tell the two counts apart"
    );

    assert_eq!(
        Document::Of(MULTI_BYTE.as_bytes().to_vec()).Length(),
        MULTI_BYTE.len(),
        "the length of a document is not the number of octets in it"
    );
}

#[test]
fn Test_Is_Empty_Should_Answer_For_A_Document_Of_No_Bytes_At_All()
{
    // Two facts, and the second is the one worth having.
    //
    // `Of` accepts zero octets, because a document is a value and emptiness is a property one can
    // have. It is `DocumentStore::Write` that refuses to store it, which keeps
    // `StoreError::Vacuous` a fact about a write rather than about a type.
    //
    // And a document of one blank byte is **not** empty. A blank is an octet, and reading it as
    // nothing would be the text normalization a document never goes through.
    assert!(
        Document::Of(Vec::new()).Is_Empty(),
        "a document of no octets did not read as empty"
    );
    assert!(
        !Document::Of(b" ".to_vec()).Is_Empty(),
        "a document of one blank octet read as empty, which is the text door's behaviour on a \
         type that does not go through it"
    );
}
