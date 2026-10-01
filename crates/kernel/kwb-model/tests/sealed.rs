//! What a finished derivation answers, asked from outside the crate.
//!
//! # Why this is here and not in `src/tests.rs`
//!
//! The unit is the file, and the file stem is what names the test. `src/tests.rs` is the whole
//! crate's suite and its unit is `tests`, which is no source file's stem — so `Identity`,
//! `Included` and `Excluded` are driven from it and *named* nowhere the rule can read.
//!
//! A `Sealed` is produced only by `Derivation::Seal`, so each of these builds one through that
//! door and asks the produced value its question. `Sealed::From` — the constructor — is
//! `pub(crate)` and cannot be reached from a file under `tests/`, which is a separate package;
//! its assertion lives in the file itself.

use kwb_model::ContentIdentity;
use kwb_model::Derivation;
use kwb_model::Exclusion;

#[test]
fn Test_Identity_Should_Survive_A_Round_Trip_Through_Its_Own_Rendering()
{
    // `D-002` promises Nomos an opaque string, so the rendering is what a peer carries and what
    // comes back. An identity that could not be recognized from its own text would make every
    // citation a peer wrote down unusable — and unusable silently, since the text looks right.
    let sealed = Derivation::Of("claim").With_Text("text", "a passage").Seal();
    let recognized = ContentIdentity::Parse(&sealed.Identity().Render())
        .expect("a rendering this crate produced");

    assert_eq!(sealed.Identity(), recognized);
}

#[test]
fn Test_Included_Should_List_The_Fields_In_The_Order_They_Were_Written()
{
    // Order is the caller's and it is significant: `(a, -, b)` must not collide with `(a, b, -)`.
    // So this is a transcript rather than a set, and sorting it here would hide the very thing a
    // reader checks it for. The absent field is in the middle on purpose — that is the position
    // the layout exists to hold open.
    let sealed = Derivation::Of("claim")
        .With_Text("concept", "entropy")
        .With_Absent("scope")
        .With_Text("text", "a passage")
        .Seal();

    assert_eq!(sealed.Included(), ["concept", "scope", "text"]);
}

#[test]
fn Test_Excluded_Should_Carry_Each_Field_With_The_Reason_It_Was_Left_Out()
{
    // More than one exclusion, because the workspace has two the store rests on — a claim's
    // source and a document's arrival — and a listing that kept only the last would report a
    // layout that was never derived from. Both are asserted, so an implementation that
    // overwrote instead of accumulating is caught rather than passing on the first one.
    let sealed = Derivation::Of("document")
        .With_Bytes("content", b"a passage")
        .Excluding("source", "two references carrying one passage must become one document")
        .Excluding("received", "when a document arrived is a fact about this run")
        .Seal();

    assert_eq!(
        sealed.Excluded(),
        [
            Exclusion {
                field: "source",
                because: "two references carrying one passage must become one document",
            },
            Exclusion {
                field: "received",
                because: "when a document arrived is a fact about this run",
            },
        ],
        "an exclusion was lost, or the reason recorded for one is not the one that was given"
    );
}
