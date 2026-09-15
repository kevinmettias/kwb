//! Why a durable write or read could not be done.
//!
//! It is filed apart from [`ContentStoreStrategy`] because it is shared by every port that
//! touches a medium rather than being a fact about one of them: a refusal carries what was
//! being attempted, which is the only part of an error a port is in a position to know.
//!
//! [`ContentStoreStrategy`]: crate::ContentStoreStrategy

use core::fmt;

/// Why a durable write or read could not be done.
///
/// Deliberately not a wrapper around a standard-library error type. A port trait that named
/// one would make every implementation of it an implementation over that library, which is the
/// thing a port exists to avoid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StorageError
{
    /// Nothing is stored under that address.
    Absent
    {
        /// The address asked for.
        address: String,
    },

    /// The underlying medium refused, with whatever it said.
    Refused
    {
        /// What was being attempted.
        doing: &'static str,

        /// What the medium reported.
        cause: String,
    },
}

impl fmt::Display for StorageError
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        return match self
        {
            Self::Absent { address } => write!(formatter, "nothing is stored at {address}"),
            Self::Refused { doing, cause } => write!(formatter, "{doing}: {cause}"),
        };
    }
}

impl core::error::Error for StorageError
{
}
