//! The identity itself: 32 bytes, and the two ways to obtain one.

use core::fmt;

/// The number of bytes in a content identity — the full SHA-256 digest, untruncated.
pub const IDENTITY_BYTES: usize = 32;

/// The number of characters in a rendered identity.
pub const IDENTITY_CHARACTERS: usize = IDENTITY_BYTES * 2;

/// What a thing *is*, rendered as an identifier.
///
/// Two artifacts with the same content have the same identity, by construction. Two with
/// different content do not. That is the whole of it, and every property this workspace
/// wants from identity follows: a re-derived claim keeps its identity so a citation
/// survives the re-run, a changed claim gets a new one so the old citation fails loudly
/// rather than quietly resolving to text that no longer says what was cited, and a write
/// is idempotent because a re-proposed claim is the same row.
///
/// # There is no constructor from an arbitrary value
///
/// An identity is obtained in exactly two ways, and the difference between them is the
/// point:
///
/// - [`Derivation::Seal`] **mints** one, and can only be reached by handing over the
///   content it is derived from.
/// - [`ContentIdentity::Parse`] **recognizes** one that was minted earlier — the read side
///   of a recorded dependency edge (`D-006`) and of a citation a peer carries (`D-002`).
///
/// What is deliberately absent is a third way: a public constructor taking a string or a
/// byte array and asserting that it *is* the identity of something. The prototype's
/// `DeterministicId.From(params string?[])` was exactly that — a bare static any caller
/// could hand arbitrary strings — and the measured consequence was that 87 types in its
/// `Domain/` defaulted their id to a fresh random value while 9 files called the deriving
/// function at all. A discipline a caller must remember is a discipline 87 callers forgot.
///
/// [`Derivation::Seal`]: crate::Derivation::Seal
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentIdentity([u8; IDENTITY_BYTES]);

impl ContentIdentity
{
    /// Construct from a finished digest. Crate-private: the only caller is [`Derivation::Seal`],
    /// which is the only place that has seen the content.
    ///
    /// [`Derivation::Seal`]: crate::Derivation::Seal
    pub(crate) const fn From_Digest(digest: [u8; IDENTITY_BYTES]) -> Self
    {
        return Self(digest);
    }

    /// The identity as lowercase hexadecimal, always [`IDENTITY_CHARACTERS`] characters.
    ///
    /// This is the string form `D-002` promises Nomos: stable, opaque, and the same for the
    /// same content forever. Nomos does not compute, verify or interpret it, so the only
    /// thing this rendering owes is that it never changes meaning once cited.
    #[must_use]
    pub fn Render(&self) -> String
    {
        let mut rendered = String::with_capacity(IDENTITY_CHARACTERS);
        for byte in self.0
        {
            use fmt::Write as _;
            let _ = write!(rendered, "{byte:02x}");
        }
        return rendered;
    }

    /// Recognize an identity that was minted earlier.
    ///
    /// This does not mint one. It parses the rendering of one, which is what reading a
    /// recorded dependency edge or an incoming citation requires. It validates the shape
    /// and nothing else — it cannot know whether the content that produced it still exists,
    /// and pretending otherwise would be the kind of claim `D-003` exists to stop.
    ///
    /// # Errors
    ///
    /// [`IdentityError::WrongLength`] when the text is not exactly
    /// [`IDENTITY_CHARACTERS`] characters, and [`IdentityError::NotHexadecimal`] when it
    /// contains anything but lowercase hexadecimal digits. Uppercase is refused rather
    /// than accepted-and-normalized, so that one identity has exactly one rendering and a
    /// string comparison anywhere is as good as this type's own.
    pub fn Parse(text: &str) -> Result<Self, IdentityError>
    {
        if text.len() != IDENTITY_CHARACTERS
        {
            return Err(IdentityError::WrongLength { found: text.len() });
        }

        return Decode(text).map(Self);
    }

    /// The raw bytes, for a caller writing the identity somewhere that is not text.
    #[must_use]
    pub const fn As_Bytes(&self) -> &[u8; IDENTITY_BYTES]
    {
        return &self.0;
    }
}

/// The digest a rendered identity names.
///
/// The length is already known to be right, so the only way this fails is a digit that is not
/// lowercase hexadecimal — and the `else` arm exists because a total match costs nothing and an
/// argument that a branch is unreachable costs a reader something every time they check it.
fn Decode(text: &str) -> Result<[u8; IDENTITY_BYTES], IdentityError>
{
    let mut digest = [0_u8; IDENTITY_BYTES];
    let mut digits = text.bytes();

    for slot in &mut digest
    {
        let (Some(high), Some(low)) = (digits.next(), digits.next())
        else
        {
            return Err(IdentityError::WrongLength { found: text.len() });
        };

        *slot = High_Nibble(high)? | Low_Nibble(low)?;
    }

    return Ok(digest);
}

/// How many values a nibble holds, and so how many hexadecimal digits there are.
///
/// Named because it is the width of every table below and of the parameter [`Nibble_Of`] takes —
/// four places that must agree, and a `16` written at each of them is four chances to disagree.
const NIBBLE_VALUES: usize = 16;

/// The sixteen lowercase hexadecimal digits, in ascending order of the value they name.
///
/// A table rather than arithmetic. Deriving a digit's value would mean arithmetic on a parse
/// path, which this workspace denies for the reason nomos states: a panic there is a
/// determinism defect rather than a bug. A table has no arithmetic to check.
const HEXADECIMAL_DIGITS: [u8; NIBBLE_VALUES] = *b"0123456789abcdef";

/// What each digit names in the **high** nibble of a byte.
const HIGH_NIBBLE: [u8; NIBBLE_VALUES] = [
    0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0,
];

/// What each digit names in the **low** nibble of a byte.
const LOW_NIBBLE: [u8; NIBBLE_VALUES] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
];

/// One lowercase hexadecimal digit, as the high nibble of a byte.
fn High_Nibble(byte: u8) -> Result<u8, IdentityError>
{
    return Nibble_Of(byte, HIGH_NIBBLE);
}

/// One lowercase hexadecimal digit, as the low nibble of a byte.
fn Low_Nibble(byte: u8) -> Result<u8, IdentityError>
{
    return Nibble_Of(byte, LOW_NIBBLE);
}

/// What one digit names in `table`, or a refusal naming the byte that is not a digit.
///
/// The digit is FOUND rather than computed, which is what keeps an out-of-range byte a refusal
/// the caller can see instead of a computation free to run past the end of `table`.
fn Nibble_Of(byte: u8, table: [u8; NIBBLE_VALUES]) -> Result<u8, IdentityError>
{
    let Some(digit) = HEXADECIMAL_DIGITS
        .iter()
        .position(|candidate| return *candidate == byte)
    else
    {
        return Err(IdentityError::NotHexadecimal { found: char::from(byte) });
    };

    let Some(value) = table.get(digit).copied()
    else
    {
        return Err(IdentityError::NotHexadecimal { found: char::from(byte) });
    };

    return Ok(value);
}

impl fmt::Display for ContentIdentity
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        return formatter.write_str(&self.Render());
    }
}

/// Debug renders the same text as [`ContentIdentity::Render`], because a truncated or
/// byte-array debug form is the shape that makes two different identities look alike in a
/// log.
impl fmt::Debug for ContentIdentity
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        return write!(formatter, "ContentIdentity({})", self.Render());
    }
}

/// Why a rendered identity could not be recognized.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityError
{
    /// The text was not [`IDENTITY_CHARACTERS`] characters long.
    WrongLength
    {
        /// How long it actually was.
        found: usize,
    },

    /// The text contained something other than a lowercase hexadecimal digit.
    NotHexadecimal
    {
        /// The first offending character.
        found: char,
    },
}

impl fmt::Display for IdentityError
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        return match *self
        {
            Self::WrongLength { found } => write!(
                formatter,
                "a content identity renders as {IDENTITY_CHARACTERS} characters, found {found}"
            ),
            Self::NotHexadecimal { found } => write!(
                formatter,
                "a content identity renders as lowercase hexadecimal, found {found:?}"
            ),
        };
    }
}

impl core::error::Error for IdentityError {}
