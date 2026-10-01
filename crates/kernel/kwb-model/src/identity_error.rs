//! Why a rendered identity could not be recognized.
//!
//! The refusal half of [`ContentIdentity::Parse`]. It is filed apart from the identity
//! because the two answer different questions — the identity is what a thing *is*, and this
//! is why a string did not name one — and a caller handling the refusal has no reason to
//! carry the identity's own parsing tables across the boundary to reach it.
//!
//! [`ContentIdentity::Parse`]: crate::ContentIdentity::Parse

use core::fmt;

use crate::IDENTITY_CHARACTERS;

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

impl core::error::Error for IdentityError
{
}
