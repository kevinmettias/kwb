//! The receipt a write returns, asserted against the write that produced it.
//!
//! # Why every receipt here comes out of a store
//!
//! [`Written`] has no public constructor, so the only way a test outside the crate can obtain one
//! is to write a document and keep what came back. That is the type's own guarantee rather than an
//! inconvenience: `D19`'s defect was a success value produced on a path that did no work, and a
//! receipt assembled from an address and a length supplied by the caller is exactly that value. The
//! test that would break it cannot be written from here, which is the point.
//!
//! `tests/one_door.rs` asserts what the door gives the **store**. This file asserts what the
//! receipt says about the write.

use kwb_store::Admission;
use kwb_store::Document;
use kwb_store::DocumentStore;
use kwb_store::Written;

/// One write of one passage, from a store made for it, answering the receipt and nothing else.
fn Written_Once(passage: &str) -> Written
{
    let mut store = DocumentStore::Empty();
    let document = Document::Of(passage.as_bytes().to_vec());

    return store
        .Write(document)
        .expect("the document carries bytes, which is all an unbacked store requires");
}

#[test]
fn Test_Identity_Should_Be_The_Address_Of_The_Document_That_Was_Written()
{
    // The receipt's address is the document's address rather than a second derivation of the same
    // octets. A second derivation is a second answer to a question already answered, and the two
    // could disagree — a caller holding both would have no way to say which was the store's. It is
    // asserted against a `Document` built from the same passage, which the receipt never saw.
    let passage = "a passage";

    let written = Written_Once(passage);

    assert_eq!(
        written.Identity(),
        Document::Of(passage.as_bytes().to_vec()).Identity(),
        "the receipt carries an address the document it describes does not have"
    );
}

#[test]
fn Test_Length_Should_Be_The_Count_Of_Bytes_The_Document_Carries()
{
    // Taken from the document rather than from anything the caller said. This is the number a
    // report totals a corpus with, and the shape `D19` records is a zero produced without the bytes
    // ever being looked at — so what is asserted is that it counts the passage that was written.
    let passage = "a passage worth counting";

    let written = Written_Once(passage);

    assert_eq!(
        written.Length(),
        passage.len(),
        "the receipt reports a length the octets it describes do not have"
    );
}

#[test]
fn Test_Admission_Should_Say_Whether_This_Write_Is_What_Stored_The_Bytes()
{
    // The one fact a caller counting admissions needs and the receipt's other fields cannot supply:
    // bytes that landed now, or bytes the store already held. Both are successes and they are not
    // the same fact, so the second write must report the second value — a run that admitted nothing
    // new and a run that admitted a corpus are otherwise one number.
    let writes = Two_Writes_Of_One_Passage();

    assert_eq!(
        writes.first.Admission(),
        Admission::Stored,
        "the first write of a passage does not report that it stored it"
    );
    assert_eq!(
        writes.second.Admission(),
        Admission::AlreadyPresent,
        "a re-offered document reports that this write stored bytes the store already held"
    );
}

/// The two receipts one passage collects from one store: the write that stored it, and the offer
/// that found it already there.
///
/// A named struct rather than a pair of receipts, because both fields have the same type and a
/// tuple would not say which half was the first write — the distinction the assertions above are
/// entirely about.
struct TwoWrites
{
    first: Written,
    second: Written,
}

/// Writes one passage to a fresh store twice, answering both receipts in the order they were made.
///
/// Both writes come from the same store, because the second verdict exists only in relation to the
/// first: a fresh store per write would report `Stored` twice, and `Stored` on its own cannot be
/// told from a store that answers every write the same way.
fn Two_Writes_Of_One_Passage() -> TwoWrites
{
    let mut store = DocumentStore::Empty();

    let first = store
        .Write(Document::Of(b"a passage".to_vec()))
        .expect("the document carries bytes, which is all an unbacked store requires");
    let second = store
        .Write(Document::Of(b"a passage".to_vec()))
        .expect("a re-offered document is answered, never refused");

    return TwoWrites { first, second };
}

#[test]
fn Test_Has_Stored_Should_Agree_With_The_Admission_It_Is_Derived_From()
{
    // The documentation says this is derived from `Admission` rather than stored beside it, so the
    // two cannot disagree. *Derived* is a claim about the code, and this is the assertion that
    // reaches it: over both kinds of write, the answer must be the `Stored` variant and nothing
    // else. What it catches is a stored boolean, or a second match written independently, that has
    // drifted away from the verdict it was supposed to mirror.
    let mut store = DocumentStore::Empty();

    let first = store
        .Write(Document::Of(b"a passage".to_vec()))
        .expect("the document carries bytes, which is all an unbacked store requires");
    let second = store
        .Write(Document::Of(b"a passage".to_vec()))
        .expect("a re-offered document is answered, never refused");

    assert_eq!(
        (first.Has_Stored(), second.Has_Stored()),
        (
            first.Admission() == Admission::Stored,
            second.Admission() == Admission::Stored
        ),
        "the receipt says it stored the bytes while reporting an admission that says otherwise"
    );
}
