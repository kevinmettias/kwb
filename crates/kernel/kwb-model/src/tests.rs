//! Tests for the identity discipline.
//!
//! Each names the property it defends rather than the function it calls, because the
//! defect these exist to catch is a caller reaching an identity another way — not a
//! function returning the wrong bytes.

use super::*;

/// The layout a claim is addressed by, written once so every test derives the same way and
/// a change to the layout is a change in one place.
///
/// The source is excluded and the exclusion is recorded, which is the mechanism the whole
/// workspace rests on. It is taken as a parameter and deliberately never written, because a
/// test that never passes a source could not demonstrate that passing a different one
/// changes nothing.
fn Claim(concept: &str, text: &str, scope: Option<&str>, source: &str) -> Sealed
{
    let _ = source;

    let derivation = Derivation::Of("claim")
        .With_Text("concept", concept)
        .With_Text("text", text)
        .Excluding(
            "source",
            "two books asserting one claim must become one claim with two citations",
        );

    return match scope
    {
        Some(scope) => derivation.With_Text("scope", scope).Seal(),
        None => derivation.With_Absent("scope").Seal(),
    };
}

// ---- the property KWB-1 exists for ----

#[test]
fn Test_Two_Claims_Differing_Only_By_Source_Should_Have_One_Identity()
{
    let first = Claim("entropy", "Entropy is non-decreasing in an isolated system.", None, "Callen 1985");
    let second = Claim("entropy", "Entropy is non-decreasing in an isolated system.", None, "Kittel 1980");

    assert_eq!(
        first.Identity(),
        second.Identity(),
        "the source is excluded, so two sources asserting one claim must produce one identity"
    );
}

#[test]
fn Test_The_Source_Exclusion_Should_Be_Recorded_With_Its_Reason()
{
    let claim = Claim("entropy", "Anything.", None, "Callen 1985");

    let excluded = claim.Excluded();
    assert_eq!(excluded.len(), 1);

    let exclusion = excluded.first().copied().expect("one exclusion was recorded");
    assert_eq!(exclusion.field, "source");
    assert!(
        !exclusion.because.is_empty(),
        "an exclusion with no reason is indistinguishable from an oversight"
    );
}

#[test]
fn Test_A_Changed_Claim_Should_Not_Keep_Its_Identity()
{
    let before = Claim("entropy", "Entropy is non-decreasing in an isolated system.", None, "Callen 1985");
    let after = Claim("entropy", "Entropy is non-increasing in an isolated system.", None, "Callen 1985");

    assert_ne!(
        before.Identity(),
        after.Identity(),
        "a citation must fail loudly rather than resolve to text that no longer says what was cited"
    );
}

#[test]
fn Test_Included_Fields_Should_Be_Reported_In_Order()
{
    let claim = Claim("entropy", "Anything.", Some("classical thermodynamics"), "Callen 1985");

    assert_eq!(claim.Included(), ["concept", "text", "scope"]);
}

// ---- normalization: what is an edit and what is not ----

#[test]
fn Test_Reflowed_Text_Should_Not_Change_Identity()
{
    let flowed = Claim("entropy", "Entropy is non-decreasing\r\n  in an isolated   system.", None, "x");
    let plain = Claim("entropy", "Entropy is non-decreasing in an isolated system.", None, "x");

    assert_eq!(flowed.Identity(), plain.Identity());
}

#[test]
fn Test_Changed_Capitalisation_Should_Change_Identity()
{
    let upper = Claim("notation", "Polish notation", None, "x");
    let lower = Claim("notation", "polish notation", None, "x");

    assert_ne!(
        upper.Identity(),
        lower.Identity(),
        "folding case would make two different claims one; case is deliberately not normalized"
    );
}

#[test]
fn Test_Normalize_Should_Strip_Control_Characters()
{
    assert_eq!(Normalize_Text("a\u{1F}b"), "ab");
    assert_eq!(Normalize_Text("a\u{0}b"), "ab");
    assert_eq!(Normalize_Text("  a   b  "), "a b");
}

// ---- the layout cannot be forged or confused ----

#[test]
fn Test_A_Value_Should_Not_Be_Able_To_Forge_A_Field_Boundary()
{
    // Rust does not treat the unit separator as whitespace, so if normalization did not
    // strip controls this value would inject a delimiter and impersonate a second field.
    let forged = Derivation::Of("claim")
        .With_Text("concept", "entropy\u{1F}text\u{1F}Anything.")
        .With_Text("text", "")
        .Seal();
    let honest = Derivation::Of("claim")
        .With_Text("concept", "entropy")
        .With_Text("text", "Anything.")
        .Seal();

    assert_ne!(forged.Identity(), honest.Identity());
}

#[test]
fn Test_An_Absent_Field_Should_Hold_Its_Place()
{
    let absent_then_valued = Derivation::Of("claim")
        .With_Absent("scope")
        .With_Text("context", "c")
        .Seal();
    let valued_then_absent = Derivation::Of("claim")
        .With_Text("scope", "c")
        .With_Absent("context")
        .Seal();

    assert_ne!(
        absent_then_valued.Identity(),
        valued_then_absent.Identity(),
        "(a, -, b) must not collide with (a, b, -)"
    );
}

#[test]
fn Test_A_Field_Name_Should_Participate()
{
    let scope = Derivation::Of("claim").With_Text("scope", "c").Seal();
    let context = Derivation::Of("claim").With_Text("context", "c").Seal();

    assert_ne!(scope.Identity(), context.Identity());
}

#[test]
fn Test_The_Kind_Should_Participate()
{
    let claim = Derivation::Of("claim").With_Text("name", "entropy").Seal();
    let concept = Derivation::Of("concept").With_Text("name", "entropy").Seal();

    assert_ne!(
        claim.Identity(),
        concept.Identity(),
        "a claim and a concept carrying identical text are not the same artifact"
    );
}

#[test]
fn Test_Field_Order_Should_Be_Significant()
{
    let one = Derivation::Of("claim").With_Text("a", "1").With_Text("b", "2").Seal();
    let other = Derivation::Of("claim").With_Text("b", "2").With_Text("a", "1").Seal();

    assert_ne!(one.Identity(), other.Identity());
}

#[test]
fn Test_An_Identity_Field_Should_Not_Collide_With_Its_Own_Rendering()
{
    let input = Derivation::Of("chunk").With_Text("text", "anything").Seal().Identity();

    let referenced = Derivation::Of("synthesis").With_Identity("input", &input).Seal();
    let spelled = Derivation::Of("synthesis").With_Text("input", &input.Render()).Seal();

    assert_ne!(
        referenced.Identity(),
        spelled.Identity(),
        "a text field holding a rendered identity is not a reference to that artifact"
    );
}

// ---- D-006: the same input identity must be reproducible, or staleness is undecidable ----

#[test]
fn Test_A_Rederived_Input_Should_Carry_The_Same_Identity()
{
    let once = Derivation::Of("chunk").With_Text("text", "a passage").Seal().Identity();
    let twice = Derivation::Of("chunk").With_Text("text", "a passage").Seal().Identity();

    assert_eq!(
        once, twice,
        "D-006 compares a recorded input identity against a recomputed one; if derivation \
         were not reproducible, everything would read as stale"
    );
}

#[test]
fn Test_A_Derived_Artifact_Should_Change_When_An_Input_Changes()
{
    let before = Derivation::Of("chunk").With_Text("text", "a passage").Seal().Identity();
    let after = Derivation::Of("chunk").With_Text("text", "a revised passage").Seal().Identity();

    let from_before = Derivation::Of("synthesis").With_Identity("input", &before).Seal();
    let from_after = Derivation::Of("synthesis").With_Identity("input", &after).Seal();

    assert_ne!(from_before.Identity(), from_after.Identity());
}

// ---- rendering and recognition ----

/// A SHA-256 digest is 32 bytes and hexadecimal renders each byte as two characters, so the
/// full digest occupies this many characters.
///
/// It is written out rather than derived from `IDENTITY_CHARACTERS` because it is the claim
/// that test makes: `rendered.len() == IDENTITY_CHARACTERS` holds for any truncated
/// rendering too, and only a fixed count says the rendering is of the whole digest.
const SHA256_RENDERED_CHARACTERS: usize = 64;

#[test]
fn Test_Render_Should_Produce_Lowercase_Hexadecimal_Of_The_Full_Digest()
{
    let rendered = Derivation::Of("claim").With_Text("text", "x").Seal().Identity().Render();

    assert_eq!(rendered.len(), IDENTITY_CHARACTERS);
    assert_eq!(IDENTITY_CHARACTERS, SHA256_RENDERED_CHARACTERS);
    assert!(rendered.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
}

#[test]
fn Test_Parse_Should_Recognize_What_Render_Produced()
{
    let identity = Derivation::Of("claim").With_Text("text", "x").Seal().Identity();

    assert_eq!(ContentIdentity::Parse(&identity.Render()), Ok(identity));
}

#[test]
fn Test_Parse_Should_Refuse_A_Wrong_Length()
{
    let too_short = "abc";

    assert_eq!(
        ContentIdentity::Parse(too_short),
        Err(IdentityError::WrongLength { found: too_short.len() })
    );
}

#[test]
fn Test_Parse_Should_Refuse_Uppercase_So_One_Identity_Has_One_Rendering()
{
    let identity = Derivation::Of("claim").With_Text("text", "x").Seal().Identity();
    let shouted = identity.Render().to_uppercase();

    assert!(
        matches!(
            ContentIdentity::Parse(&shouted),
            Err(IdentityError::NotHexadecimal { .. })
        ),
        "accepting-and-normalizing would give one identity two renderings, and a string \
         comparison elsewhere would then disagree with this type"
    );
}

// ---- KWB-15: the opaque door, and the normalization it deliberately does not do ----

#[test]
fn Test_Two_Byte_Strings_Differing_Only_By_Whitespace_Should_Have_Different_Identities()
{
    let spaced = Derivation::Of("document").With_Bytes("content", b"fn main()  {}").Seal();
    let tight = Derivation::Of("document").With_Bytes("content", b"fn main() {}").Seal();

    assert_ne!(
        spaced.Identity(),
        tight.Identity(),
        "a stored document is its octets; if a reflow derived one identity, the second write \
         would find the first already present, report success, and lose the bytes -- which is \
         D17 exactly"
    );
}

#[test]
fn Test_The_Same_Bytes_Should_Derive_One_Identity()
{
    let once = Derivation::Of("document").With_Bytes("content", b"a passage").Seal();
    let twice = Derivation::Of("document").With_Bytes("content", b"a passage").Seal();

    assert_eq!(
        once.Identity(),
        twice.Identity(),
        "content addressing is the dedup mechanism; re-offering one document must be one row"
    );
}

#[test]
fn Test_A_Byte_Field_Should_Not_Collide_With_A_Text_Field_Of_The_Same_Content()
{
    let opaque = Derivation::Of("document").With_Bytes("content", b"a passage").Seal();
    let text = Derivation::Of("document").With_Text("content", "a passage").Seal();

    assert_ne!(
        opaque.Identity(),
        text.Identity(),
        "normalized and not-normalized are different claims about the same octets, and a \
         caller that picked the wrong door must not silently land on the right identity"
    );
}

#[test]
fn Test_A_Byte_Field_Should_Not_Collide_With_An_Identity_Field()
{
    let referenced = Derivation::Of("chunk").With_Text("text", "anything").Seal().Identity();

    let as_reference = Derivation::Of("synthesis").With_Identity("input", &referenced).Seal();
    let as_bytes = Derivation::Of("synthesis").With_Bytes("input", referenced.As_Bytes()).Seal();

    assert_ne!(
        as_reference.Identity(),
        as_bytes.Identity(),
        "a field holding an artifact's identity bytes is not a reference to that artifact"
    );
}

#[test]
fn Test_Bytes_Should_Not_Be_Able_To_Forge_A_Field_Boundary()
{
    let forged = Derivation::Of("document")
        .With_Bytes("content", b"anextz")
        .Seal();
    let honest = Derivation::Of("document")
        .With_Bytes("content", b"a")
        .With_Text("next", "z")
        .Seal();

    assert_ne!(
        forged.Identity(),
        honest.Identity(),
        "the outer layout separates its parts with a byte a raw value can contain, so the \
         value participates by a fixed-width digest instead"
    );
}

#[test]
fn Test_A_Byte_Field_Name_Should_Participate()
{
    let content = Derivation::Of("document").With_Bytes("content", b"z").Seal();
    let preview = Derivation::Of("document").With_Bytes("preview", b"z").Seal();

    assert_ne!(content.Identity(), preview.Identity());
}

#[test]
fn Test_An_Empty_Byte_Field_Should_Not_Be_An_Absent_One()
{
    let empty = Derivation::Of("document").With_Bytes("content", b"").Seal();
    let absent = Derivation::Of("document").With_Absent("content").Seal();

    assert_ne!(
        empty.Identity(),
        absent.Identity(),
        "a document that is zero bytes long is not a document that was never supplied"
    );
}

#[test]
fn Test_An_Absent_Field_Should_Still_Occupy_Its_Place_Among_Byte_Fields()
{
    let absent_then_valued = Derivation::Of("document")
        .With_Absent("a")
        .With_Bytes("b", b"1")
        .Seal();
    let valued_then_absent = Derivation::Of("document")
        .With_Bytes("a", b"1")
        .With_Absent("b")
        .Seal();

    assert_ne!(absent_then_valued.Identity(), valued_then_absent.Identity());
}

#[test]
fn Test_A_Byte_Field_Should_Be_Recorded_As_Included()
{
    let sealed = Derivation::Of("document").With_Bytes("content", b"z").Seal();

    assert_eq!(sealed.Included(), ["content"]);
}
