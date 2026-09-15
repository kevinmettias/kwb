//! An identity's own three public doors: what it renders to, what it recognizes, and what
//! bytes it answers with.
//!
//! # Why this is here and not in `src/tests.rs`
//!
//! The unit is the file, and the file stem is what names the test. `src/tests.rs` is the
//! whole crate's suite and its unit is `tests`, which is no source file's stem — so `Render`,
//! `Parse` and `As_Bytes` are driven from there and *named* nowhere the rule can read. These
//! are the three functions' own contracts, asserted where a `pub` function's test belongs.
//!
//! `From_Digest` is the fourth door and it is `pub(crate)`, so its assertion lives in the file
//! itself; see the module there. What is asserted here is what a *caller* can reach.

use kwb_model::ContentIdentity;
use kwb_model::IDENTITY_BYTES;
use kwb_model::IDENTITY_CHARACTERS;
use kwb_model::IdentityError;

#[test]
fn Test_Render_Should_Spell_A_Leading_Zero_Byte_As_Two_Digits()
{
    // `{byte:x}` and `{byte:02x}` agree on every byte above `0x0F`, so a rendering that dropped
    // the zero-padding looks right on almost every identity there is. It is wrong in two ways
    // that matter: a rendering whose first byte is zero comes out one character short, and two
    // byte strings that differ only in a leading zero render to the same text.
    let text = format!("00{}", "ff".repeat(31));
    let identity = ContentIdentity::Parse(&text).expect("sixty-four lowercase hexadecimal characters");

    assert_eq!(
        identity.Render(),
        text,
        "a byte below ten was rendered with fewer than two digits, so the text form of an \
         identity is not always the length the constant says it is"
    );
    assert_eq!(identity.Render().len(), IDENTITY_CHARACTERS);
}

#[test]
fn Test_Parse_Should_Name_The_Character_It_Could_Not_Read_As_Hexadecimal()
{
    // The refusal is what a caller loading a citation sees when a stored identity is damaged,
    // and it has to name the character rather than the position: a person hunting one bad byte
    // in sixty-four needs the value, not the index. A refusal that said only "not hexadecimal"
    // would send them to the source of the string instead of to the byte inside it.
    let text = format!("0z{}", "ff".repeat(31));

    assert_eq!(
        ContentIdentity::Parse(&text),
        Err(IdentityError::NotHexadecimal { found: 'z' }),
        "the refusal does not say which character it could not read"
    );
}

#[test]
fn Test_As_Bytes_Should_Answer_The_Thirty_Two_Bytes_A_Parsed_Identity_Carries()
{
    // `As_Bytes` is the door for a caller writing an identity somewhere that is not text, and
    // what it must hand over is the digest the rendering decodes to -- not the rendering, and
    // not a prefix of it. Anything else and the two doors disagree about the same value, which
    // is how a record and the citation that points at it come apart.
    let identity = ContentIdentity::Parse("abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789")
        .expect("sixty-four lowercase hexadecimal characters");
    let bytes = identity.As_Bytes();

    assert_eq!(bytes.len(), IDENTITY_BYTES, "the bytes are not the whole digest");
    assert_eq!(
        bytes.first().copied(),
        Some(0xAB),
        "the first byte does not decode the first two characters of the rendering"
    );
    assert_eq!(
        bytes.last().copied(),
        Some(0x89),
        "the last byte does not decode the last two characters of the rendering"
    );
}
