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
    assert_eq!(Normalize("a\u{1F}b"), "ab");
    assert_eq!(Normalize("a\u{0}b"), "ab");
    assert_eq!(Normalize("  a   b  "), "a b");
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

#[test]
fn Test_Render_Should_Produce_Lowercase_Hexadecimal_Of_The_Full_Digest()
{
    let rendered = Derivation::Of("claim").With_Text("text", "x").Seal().Identity().Render();

    assert_eq!(rendered.len(), IDENTITY_CHARACTERS);
    assert_eq!(IDENTITY_CHARACTERS, 64);
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
    assert_eq!(
        ContentIdentity::Parse("abc"),
        Err(IdentityError::WrongLength { found: 3 })
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
