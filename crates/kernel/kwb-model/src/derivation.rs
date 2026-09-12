//! Deriving an identity, and recording what was deliberately left out of it.

use sha2::Digest as _;
use sha2::Sha256;

use crate::ContentIdentity;
use crate::IDENTITY_BYTES;

/// ASCII unit separator, between every part of the derived layout.
///
/// A hash of a concatenation is only as unambiguous as its delimiters: with none, a field
/// named `ab` holding `c` and a field named `a` holding `bc` hash identical bytes.
///
/// # The prototype's argument for this byte does not survive the move to Rust
///
/// The .NET prototype chose `0x1F` and reasoned that it "is whitespace, so it cannot
/// survive normalisation and be mistaken for a delimiter." That is true of .NET, whose
/// `char.IsWhiteSpace` returns `true` for `U+001C`–`U+001F`. It is **false** of Rust:
/// `char::is_whitespace` follows the Unicode `White_Space` property, which does not
/// include the separators. Carrying the constant across without carrying a replacement for
/// the guarantee would have silently lost it — the separator would have passed straight
/// through whitespace normalization and a value containing one could forge a field
/// boundary.
///
/// So the guarantee is restored explicitly rather than inherited: [`Normalize`] strips
/// every C0 and C1 control character, which includes this one, and it does so because
/// identity integrity requires it rather than as a side effect of something else.
const SEPARATOR: u8 = 0x1F;

/// Written in place of a value for a field that participates and is absent.
///
/// An absent field is not the same as a missing one. `(concept, -, scope)` and
/// `(concept, scope, -)` are different layouts and must not collide, which is why an
/// absent field still occupies its place rather than being skipped.
const ABSENT: &[u8] = b"<absent>";

/// Marks a value that is itself an identity, so that a text field holding the 64
/// characters of a rendered identity cannot collide with a field that genuinely refers to
/// that artifact.
const IDENTITY_TAG: &[u8] = b"<identity>";

/// Marks a value that is opaque bytes, so that a byte field and a text field of the same
/// name holding the same content cannot collide. `With_Text` normalizes and `With_Bytes`
/// does not, so the two are genuinely different claims about the same octets and must not
/// derive one identity.
const OPAQUE_TAG: &[u8] = b"<opaque>";

/// A field that was considered and deliberately left out of an identity, with the reason.
///
/// Recording the exclusion is the point. `D-006` and the cross-source dedup mechanism both
/// rest on one exclusion — a claim's source — and an exclusion that exists only as an
/// omission is indistinguishable from an oversight. The prototype states its own source
/// exclusion in a doc comment; a doc comment is not available to a test.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Exclusion
{
    /// The field that was left out.
    pub field: &'static str,

    /// Why leaving it out is correct, in one phrase.
    pub because: &'static str,
}

/// Accumulates the content an identity is derived from.
///
/// Every field is named as well as valued, and the name is hashed with the value, so the
/// layout is self-describing: renaming a field changes the identity, which is correct,
/// because a claim's `scope` and a claim's `context` are not the same field even when they
/// hold the same text.
///
/// Order is significant and deliberately not sorted. A derivation is written once, next to
/// the type it addresses, and reading it top to bottom should be how a reader learns what
/// participates.
#[must_use]
pub struct Derivation
{
    digest: Sha256,
    included: Vec<&'static str>,
    excluded: Vec<Exclusion>,
}

impl Derivation
{
    /// Begin deriving the identity of a thing of this kind.
    ///
    /// The kind participates, so a concept and a claim carrying identical text do not
    /// collide. It is `&'static str` rather than a parameter because a kind is a property
    /// of the code that derives, never of the data being derived from.
    pub fn Of(kind: &'static str) -> Self
    {
        let mut derivation = Self {
            digest: Sha256::new(),
            included: Vec::new(),
            excluded: Vec::new(),
        };
        derivation.Write(b"kind");
        derivation.Write(kind.as_bytes());
        return derivation;
    }

    /// A text field that participates, normalized.
    ///
    /// See [`Normalize`] for exactly what normalization does and, more importantly, what it
    /// deliberately does not do.
    pub fn With_Text(mut self, field: &'static str, value: &str) -> Self
    {
        self.included.push(field);
        self.Write(field.as_bytes());
        self.Write(Normalize(value).as_bytes());
        return self;
    }

    /// A field that participates by the identity of another artifact.
    ///
    /// This is the edge `D-006` records: a derived artifact declares the identity each of
    /// its inputs *had* when it was derived, so staleness is a comparison rather than a
    /// stored flag.
    pub fn With_Identity(mut self, field: &'static str, value: &ContentIdentity) -> Self
    {
        self.included.push(field);
        self.Write(field.as_bytes());
        self.Write(IDENTITY_TAG);
        self.Write(value.As_Bytes());
        return self;
    }

    /// A field that participates as opaque bytes, normalized in no way at all.
    ///
    /// This is the door a content-addressed **document** goes through, and it is the exact
    /// complement of [`With_Text`]: there, a reflow is not an edit, and here every octet is
    /// the content. Reaching for this when the field is text silently opts that field out of
    /// the dedup mechanism the whole workspace rests on, because two renderings of one claim
    /// would stop being one claim.
    ///
    /// # Why the value is hashed before it participates
    ///
    /// The outer layout separates its parts with [`SEPARATOR`] and relies on no part being
    /// able to contain one. [`With_Text`] earns that by construction — [`Normalize`] strips
    /// every control character — and arbitrary bytes cannot. Written raw, a value of
    /// `b"ayz"` in field `x` produces the identical byte stream to field `x` holding
    /// `b"a"` followed by a field `y` holding `z`, which is precisely the field-boundary
    /// forgery this layout exists to prevent.
    ///
    /// So the value participates by its own digest, which is a **fixed width**. A fixed-width
    /// part cannot shift a boundary no matter what it contains, which is the same argument
    /// [`With_Identity`] already rests on, and reaching the same guarantee a second way would
    /// have been two arguments to keep in step. It also means a large document contributes 32
    /// bytes to the outer digest rather than its whole length.
    ///
    /// [`With_Text`]: Self::With_Text
    /// [`With_Identity`]: Self::With_Identity
    pub fn With_Bytes(mut self, field: &'static str, value: &[u8]) -> Self
    {
        self.included.push(field);
        self.Write(field.as_bytes());
        self.Write(OPAQUE_TAG);
        self.Write(&Digest_Of(value));
        return self;
    }

    /// A field that participates and has no value in this instance.
    pub fn With_Absent(mut self, field: &'static str) -> Self
    {
        self.included.push(field);
        self.Write(field.as_bytes());
        self.Write(ABSENT);
        return self;
    }

    /// A field that was considered and deliberately left out, with the reason.
    ///
    /// This contributes nothing to the digest — that is what excluded means — and it is
    /// carried on the sealed result so a test can assert it. The exclusion this workspace
    /// exists on is a claim's source: two books asserting the same claim must produce one
    /// claim with two citations, which only holds if the source is not part of what makes
    /// two claims the same claim.
    pub fn Excluding(mut self, field: &'static str, because: &'static str) -> Self
    {
        self.excluded.push(Exclusion { field, because });
        return self;
    }

    /// Finish, producing the identity and the layout that produced it.
    #[must_use]
    pub fn Seal(mut self) -> Sealed
    {
        let digest: [u8; IDENTITY_BYTES] = self.digest.finalize_reset().into();
        return Sealed {
            identity: ContentIdentity::From_Digest(digest),
            included: self.included,
            excluded: self.excluded,
        };
    }

    /// One part, followed by the separator.
    fn Write(&mut self, part: &[u8])
    {
        self.digest.update(part);
        self.digest.update([SEPARATOR]);
    }
}

/// The digest of an opaque value, as the fixed-width stand-in that participates for it.
///
/// A separate hash state rather than the derivation's own: folding the value straight into
/// the running digest is what would let its content reach a field boundary, and that is the
/// whole reason this function exists.
fn Digest_Of(value: &[u8]) -> [u8; IDENTITY_BYTES]
{
    let mut digest = Sha256::new();
    digest.update(value);
    return digest.finalize().into();
}

/// A finished derivation: the identity, and a record of how it was reached.
#[derive(Clone, Debug)]
pub struct Sealed
{
    identity: ContentIdentity,
    included: Vec<&'static str>,
    excluded: Vec<Exclusion>,
}

impl Sealed
{
    /// The identity.
    #[must_use]
    pub const fn Identity(&self) -> ContentIdentity
    {
        return self.identity;
    }

    /// The fields that participated, in the order they were written.
    #[must_use]
    pub fn Included(&self) -> &[&'static str]
    {
        return &self.included;
    }

    /// The fields deliberately left out, with their reasons.
    #[must_use]
    pub fn Excluded(&self) -> &[Exclusion]
    {
        return &self.excluded;
    }
}

/// Whitespace-normalized, control-stripped text.
///
/// Every run of whitespace collapses to one space and the ends are trimmed, so a reflowed
/// line, a changed line ending, or a model that indents its output differently is not an
/// edit and must not break a citation.
///
/// Every C0 and C1 control character is removed. This is what keeps [`SEPARATOR`] out of a
/// value, and therefore what stops a value forging a field boundary — see that constant's
/// own documentation for why the prototype's reasoning did not carry across.
///
/// **Case is deliberately not normalized.** `BVH` and `bvh` are different text, and a claim
/// that changed its capitalisation changed what it says. This is the one normalization that
/// looks obviously helpful and is not: folding case would make `Polish notation` and
/// `polish notation` one claim.
#[must_use]
pub fn Normalize(text: &str) -> String
{
    let mut normalized = String::with_capacity(text.len());
    let mut pending_space = false;

    for character in text.chars()
    {
        if character.is_control()
        {
            continue;
        }

        if character.is_whitespace()
        {
            pending_space = !normalized.is_empty();
            continue;
        }

        if pending_space
        {
            normalized.push(' ');
            pending_space = false;
        }

        normalized.push(character);
    }

    return normalized;
}
