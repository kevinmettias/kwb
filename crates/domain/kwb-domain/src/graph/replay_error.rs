//! Why a publication record could not be read back.
//!
//! It is filed apart from [`Replay_Records`] because it is the refusal half of the reader rather than
//! part of what a record is: a caller that reports a log which would not replay carries one of
//! these, and has no reason to carry the record format — the separator, the arities, the field
//! order — across that boundary to reach it.
//!
//! [`Replay_Records`]: crate::Replay_Records

use kwb_model::IdentityError;

/// Why a record could not be read back.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReplayError
{
    /// The record's shape is not one this reader knows.
    Malformed
    {
        /// The record, as read.
        record: String,
    },

    /// A claim or assertion named something the records had not published yet.
    ///
    /// Records are a sequence and their order is part of their meaning: a claim is about a
    /// concept, so the concept's record comes first. Out of order is refused rather than
    /// guessed at, because guessing would mean constructing a claim about a concept nobody
    /// recorded.
    OutOfOrder
    {
        /// What was named.
        missing: String,
    },

    /// A field that had to name something by address did not.
    ///
    /// The record's shape read — the kind, the arity, the standing — so this is not
    /// [`Malformed`]: a claim names its concept by address and an assertion names its claim by
    /// one, and text that is not an address names nothing.
    ///
    /// [`IdentityError`] is carried rather than replaced, because it is the only thing that says
    /// *how* the text was wrong — the wrong length, or a character outside the alphabet — and
    /// whoever is repairing a log is looking at exactly that.
    ///
    /// The record itself is not carried. The complaint is about one field and not about the
    /// record's shape, so the field is what it names and the line is the one that carries that
    /// field — which [`Malformed`] cannot rely on and carries the whole record for.
    ///
    /// [`Malformed`]: ReplayError::Malformed
    Unaddressed
    {
        /// The field, as it was written.
        field: String,

        /// Why the identity reader refused it.
        cause: IdentityError,
    },
}

impl core::fmt::Display for ReplayError
{
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        return match self
        {
            Self::Malformed { record } => write!(formatter, "not a publication record: {record:?}"),
            Self::OutOfOrder { missing } => write!(
                formatter,
                "a record names {missing}, which no earlier record published. Refusing rather \
                 than guessing: a claim about a concept nobody recorded is not a claim"
            ),
            Self::Unaddressed { field, cause } => write!(
                formatter,
                "the record names {field:?} where an address belongs: {cause}"
            ),
        };
    }
}

impl core::error::Error for ReplayError
{
}
