//! Somewhere records can be appended and read back in order.

use crate::StorageError;

/// An append-only sequence of records.
///
/// # Why append-only, and why that is the whole surface
///
/// `D-014` decided the graph becomes durable by recording **what was published** and replaying
/// it, rather than by storing versions. A graph is a fold over its publications, so the
/// publications are the same information and smaller — and a temporal read falls out as a
/// prefix of the sequence rather than needing a second mechanism.
///
/// The prototype reached the same place and wrote it in its own `ROADMAP.md`, in the list of
/// what it deliberately would *not* build: *"Full event sourcing for every entity — an
/// append-only derivation ledger is the right scope."*
///
/// So: append, and read in order. There is no update, no delete and no seek, because a fold
/// over a sequence needs none of them and every one of them would be a way to make the record
/// disagree with what happened.
pub trait RecordLogStrategy
{
    /// Add one record to the end.
    ///
    /// # Errors
    ///
    /// [`StorageError::Refused`] if the record could not be added.
    fn Append(&self, record: &str) -> Result<(), StorageError>;

    /// Every record, oldest first.
    ///
    /// Order is the meaning, not a convenience: a claim is about a concept, so the concept's
    /// record precedes it, and a reader that received them in another order would be asked to
    /// construct a claim about a concept nobody had published.
    ///
    /// # Errors
    ///
    /// [`StorageError::Refused`] if the records could not be read.
    fn Records(&self) -> Result<Vec<String>, StorageError>;
}
