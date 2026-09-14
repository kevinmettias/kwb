//! Why a publication record could not be read back.
//!
//! It is filed apart from [`Replay`] because it is the refusal half of the reader rather than
//! part of what a record is: a caller that reports a log which would not replay carries one of
//! these, and has no reason to carry the record format — the separator, the arities, the field
//! order — across that boundary to reach it.
//!
//! [`Replay`]: crate::Replay

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
        };
    }
}

impl core::error::Error for ReplayError {}
