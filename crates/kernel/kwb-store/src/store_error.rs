//! Every way the store refuses a document.

use core::fmt;

use kwb_model::ContentIdentity;

/// Why the store would not do what was asked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreError
{
    /// A write offered no bytes.
    Vacuous,

    /// A read named an address the store does not hold.
    NoSuchDocument
    {
        /// The address that was asked for.
        document: ContentIdentity,
    },

    /// Two different byte strings derived one address.
    Collision
    {
        /// The address both claim.
        document: ContentIdentity,
    },
}

impl fmt::Display for StoreError
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        return match *self
        {
            Self::Vacuous => formatter.write_str(
                "a write offered no bytes. Refusing it, because a document of zero length is \
                 indistinguishable from a read that failed and returned nothing -- which is how \
                 the prototype's ReplaceAllAsync erased a concept's entire synthesis on an \
                 empty-but-valid reply",
            ),
            Self::NoSuchDocument { document } => write!(
                formatter,
                "no document {document}. Absent is not empty: nothing here ever wrote this"
            ),
            Self::Collision { document } => write!(
                formatter,
                "two different documents derive the address {document}. Refusing the second \
                 rather than holding one of them under an address that names both"
            ),
        };
    }
}

impl core::error::Error for StoreError {}
