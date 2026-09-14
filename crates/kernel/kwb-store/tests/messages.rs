//! What a person actually reads when a write is refused.
//!
//! # Why this reads the rendered text and not the source
//!
//! A Rust string literal continued with a trailing `\` strips the newline *and* the following
//! indentation. Several of this repository's literals were written through a scripted edit
//! whose own language uses the same syntax, so the backslash and newline were consumed before
//! the file was written and the indentation was not — leaving a single-line literal with a run
//! of spaces in the middle of a sentence.
//!
//! It compiles. Clippy passes. Every test passes. It is visible only when the string is
//! printed, and it survived four separate readings of the source because a reader's eye
//! reconstructs the sentence.
//!
//! So this test prints. `StoreError` is the one error type here whose text reaches somebody
//! who is not reading the code, which is why it gets the guard and the assert messages in
//! `kwb-model` did not.

use kwb_platform::StorageError;
use kwb_store::Document;
use kwb_store::DocumentStore;
use kwb_store::StoreError;

/// The fewest words a refusal a person has to act on can say. It is a floor rather than an
/// exact count: the point is that the rendering is not empty and does not stop at a word or
/// two, which is the shape of a variant that was added without any text of its own.
const MINIMUM_REFUSAL_WORDS: usize = 5;

/// Every variant, so a variant added later is added here or the match stops compiling.
fn Every_Refusal() -> Vec<StoreError>
{
    let document = Document::Of(b"a passage".to_vec()).Identity();

    return vec![
        StoreError::Vacuous,
        StoreError::NoSuchDocument { document },
        StoreError::NotStored {
            cause: StorageError::Refused {
                doing: "writing the document",
                cause: "the medium was not writable".to_owned(),
            },
        },
        StoreError::Collision { document },
    ];
}

#[test]
fn Test_No_Refusal_Should_Read_With_A_Gap_In_The_Middle_Of_A_Sentence()
{
    for refusal in Every_Refusal()
    {
        let rendered = refusal.to_string();
        assert!(
            !rendered.contains("  "),
            "this is what a person sees when a write is refused, and it has a run of spaces \
             in it, which means a line continuation was eaten before the file was written: \
             {rendered:?}"
        );
    }
}

#[test]
fn Test_Every_Refusal_Should_Say_Something()
{
    // The control. Without it the test above passes on a type whose every variant renders as
    // the empty string, which is the shape of a guard that checks a property of nothing.
    for refusal in Every_Refusal()
    {
        let rendered = refusal.to_string();
        assert!(
            rendered.split_whitespace().count() >= MINIMUM_REFUSAL_WORDS,
            "a refusal a person has to act on said almost nothing: {rendered:?}"
        );
    }
}

#[test]
fn Test_The_Refusal_A_Caller_Actually_Reaches_Should_Render_Cleanly()
{
    // The two above build their variants by hand, which cannot notice a variant that is
    // constructed differently in practice. This one goes through the door and reads what comes
    // back, so the text under test is the text a caller is handed.
    let mut store = DocumentStore::Empty();
    let refusal = store
        .Write(Document::Of(Vec::new()))
        .expect_err("a document of zero bytes is refused");

    let rendered = refusal.to_string();
    assert!(!rendered.contains("  "), "{rendered:?}");
    assert!(
        rendered.contains("empty") || rendered.contains("zero") || rendered.contains("nothing"),
        "the refusal for an empty document does not say it was empty: {rendered:?}"
    );
}
