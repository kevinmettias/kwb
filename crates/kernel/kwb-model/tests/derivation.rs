//! Each way a part enters a derivation, and what entering that way is worth.
//!
//! # Why this is here and not in `src/tests.rs`
//!
//! The unit is the file, and the file stem is what names the test. `src/tests.rs` is the whole
//! crate's suite and its unit is `tests`, which is no source file's stem — so the eight
//! functions below are driven from it and *named* nowhere the rule can read.
//!
//! What is asserted here is the contract of each door rather than the property the suite
//! next door defends. Those are not the same question: `src/tests.rs` asks whether two
//! derivations that should agree do, and this asks what each door alone is responsible for.
//! Where the two overlap the assertion is narrowed to the one thing the door decides — a
//! leading zero, a reparse, a single octet — because a second copy of the property would be a
//! second thing to keep in step and would check nothing the first one does not.

use kwb_model::ContentIdentity;
use kwb_model::Derivation;
use kwb_model::Exclusion;
use kwb_model::Normalize_Text;

#[test]
fn Test_Of_Should_Name_The_Kind_Without_Listing_It_Among_The_Fields()
{
    // The kind is written under its own field name before anything else, and it is not one of
    // the fields the caller supplied. A reader asking what participated is reading the caller's
    // layout, so `kind` appearing there would say the caller wrote a field called `kind` — and
    // the second assertion is what says so rather than assuming it: a derivation that spells
    // the same word as a text field is a different derivation from the one `Of` begins.
    let opened = Derivation::Of("claim").Seal();
    let spelled = Derivation::Of("claim").With_Text("kind", "claim").Seal();

    assert!(opened.Included().is_empty(), "the kind was reported as a field the caller wrote");
    assert_ne!(
        opened.Identity(),
        spelled.Identity(),
        "opening a derivation by its kind is the same as writing a field named `kind`, so the \
         header `Of` writes is not in the layout at all"
    );
}

#[test]
fn Test_With_Text_Should_Make_An_Empty_Value_Participate_Rather_Than_Vanish()
{
    // An empty string is a value somebody wrote, and the layout is self-describing enough to say
    // so. If it were folded into an absence, a claim whose scope was recorded as the empty string
    // would derive the identity of a claim whose scope was never supplied — two different
    // statements about one artifact, told apart by nothing.
    let empty = Derivation::Of("claim").With_Text("scope", "").Seal();
    let absent = Derivation::Of("claim").With_Absent("scope").Seal();

    assert_ne!(
        empty.Identity(),
        absent.Identity(),
        "an empty text field derived the identity of an absent one"
    );
    assert_eq!(empty.Included(), ["scope"]);
}

#[test]
fn Test_With_Identity_Should_Participate_By_The_Bytes_A_Reparsed_Identity_Carries()
{
    // `D-006` compares a recorded input identity against a recomputed one, and the recorded one
    // travelled as text. So the field has to participate by the bytes a rendering decodes to:
    // an identity that had been through `Parse` on the way must address the same input, or every
    // derivation reading a citation would be judged stale the moment it was written down.
    let input = Derivation::Of("chunk").With_Text("text", "a passage").Seal().Identity();
    let reparsed = ContentIdentity::Parse(&input.Render()).expect("a rendering this crate produced");

    let referenced = Derivation::Of("synthesis").With_Identity("input", &input).Seal();
    let round_tripped = Derivation::Of("synthesis").With_Identity("input", &reparsed).Seal();

    assert_eq!(
        referenced.Identity(),
        round_tripped.Identity(),
        "an identity that went to text and back addresses a different artifact than it did \
         before, so a recorded input can never match a recomputed one"
    );
}

#[test]
fn Test_With_Bytes_Should_Distinguish_Two_Values_Differing_In_One_Octet()
{
    // "Every octet is the content" is the whole of what this door claims, and one octet is the
    // smallest thing that can make it false. If it hashed anything but the value — a normalized
    // form, a rendering, a prefix — two documents a byte apart would land on one address, and the
    // second write would report the first one's bytes as already present.
    let one = Derivation::Of("document").With_Bytes("content", b"a passage").Seal();
    let other = Derivation::Of("document").With_Bytes("content", b"a passagf").Seal();

    assert_ne!(one.Identity(), other.Identity());
}

#[test]
fn Test_With_Absent_Should_Still_Report_The_Field_As_One_That_Participated()
{
    // The field occupies its place in the layout, and that is exactly why it is listed. A reader
    // asking what this identity was derived from has to see that `scope` was considered and had
    // no value; without it the only record of the layout is a digest, and a digest is not
    // something a person reads back.
    let sealed = Derivation::Of("claim").With_Absent("scope").Seal();

    assert_eq!(
        sealed.Included(),
        ["scope"],
        "a field that occupied its place in the layout is missing from the record of it"
    );
}

#[test]
fn Test_Excluding_Should_Change_No_Identity_And_Carry_Its_Reason_Verbatim()
{
    // The exclusion contributes nothing to the digest — that is what excluded means — and the
    // reason travels with it unchanged. Both halves in one reading, because either alone is the
    // wrong thing: an exclusion that changed the identity is a field, and a reason the caller
    // did not write is the omission this type exists to be told apart from.
    let because = "two books asserting one claim must become one claim with two citations";
    let plain = Derivation::Of("claim").With_Text("text", "a passage").Seal();
    let excluding = Derivation::Of("claim")
        .With_Text("text", "a passage")
        .Excluding("source", because)
        .Seal();

    assert_eq!(plain.Identity(), excluding.Identity(), "recording an exclusion changed the digest");
    assert_eq!(
        excluding.Excluded().first().copied(),
        Some(Exclusion { field: "source", because })
    );
}

#[test]
fn Test_Seal_Should_Carry_The_Fields_And_The_Exclusions_Into_One_Answer()
{
    // What a finished derivation is, in one value: the identity, the fields that participated,
    // and the fields that were deliberately kept out. Read together because they are two halves
    // of one statement — a reader who can only see that `source` was considered has no way to
    // learn it was meant not to participate, and this is the only place both are handed over.
    let sealed = Derivation::Of("claim")
        .With_Text("text", "a passage")
        .Excluding("source", "two books, one claim")
        .Seal();

    assert_eq!(sealed.Included(), ["text"]);
    assert_eq!(
        sealed.Excluded().first().map(|exclusion| return exclusion.field),
        Some("source")
    );
}

#[test]
fn Test_Normalize_Text_Should_Leave_Capitalisation_Exactly_As_It_Found_It()
{
    // The one normalization that looks obviously helpful and is not. `Polish notation` and
    // `polish notation` are two different statements, and folding case would make them one claim.
    // Asserted on the function directly rather than through the identity it produces, because a
    // reader changing the fold should see this fail rather than have to derive it.
    assert_eq!(
        Normalize_Text("  Polish   notation  BVH  "),
        "Polish notation BVH",
        "whitespace is collapsed and trimmed, and nothing else is touched"
    );
    assert_ne!(Normalize_Text("Entropy"), Normalize_Text("entropy"));
}
