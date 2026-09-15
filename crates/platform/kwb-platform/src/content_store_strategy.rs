//! The filesystem surface a content-addressed store needs, and nothing wider.
//!
//! The refusal every method below returns is [`StorageError`], which is filed in its own
//! module because it is the shape every port onto a medium refuses with rather than a fact
//! about this one.
//!
//! [`StorageError`]: crate::StorageError

use crate::StorageError;

/// Somewhere bytes can be kept under an address and read back unchanged.
///
/// # The one thing an implementation must get right
///
/// **A write is atomic.** A reader must never see a partial value under a complete address.
///
/// `D-014` calls this out rather than leaving it to an implementer because it is the single way
/// a content-addressed store can lie. The address *is* the digest of the content, so everything
/// downstream treats reading a value at an address as proof that the value hashes to it —
/// nothing re-hashes what it reads. A half-written value therefore does not fail loudly; it
/// silently becomes content that is not what its address says it is, and every citation
/// resolving through that address inherits the lie.
///
/// The usual discharge is to write to a temporary location and rename into place, because
/// rename is atomic on the filesystems this repository runs on. An implementation that cannot
/// offer atomicity is not an implementation of this trait.
///
/// # Why this surface and not a filesystem
///
/// Five operations, all of them things a content-addressed store does. No listing, no deletion,
/// no directory walk, no metadata. A port wide enough to be a filesystem would let a caller
/// reach past the store's own discipline — `D-008` measured what a second route into storage
/// costs, and `kwb-store`'s single write door exists because of it.
///
/// Deletion is absent by construction rather than by omission: `D17` says destruction requires
/// evidence, and a content-addressed store has no overwrite to authorise. When this repository
/// acquires a reason to remove something, that reason arrives with the evidence rule attached
/// and widens this trait as its own decision.
pub trait ContentStoreStrategy
{
    /// Whether something is stored at this address.
    ///
    /// # Errors
    ///
    /// [`StorageError::Refused`] if the medium could not answer.
    fn Holds(&self, address: &str) -> Result<bool, StorageError>;

    /// Store these bytes at this address, atomically.
    ///
    /// Storing the same address twice is not an error: the content is the address, so the
    /// second write is the same bytes. An implementation may skip the write entirely.
    ///
    /// # Errors
    ///
    /// [`StorageError::Refused`] if the write could not be completed atomically.
    fn Put(&self, address: &str, content: &[u8]) -> Result<(), StorageError>;

    /// Read back what is stored at this address.
    ///
    /// # Errors
    ///
    /// [`StorageError::Absent`] if nothing is there — absent is an error rather than an empty
    /// value, because `kwb-store` refuses to write a document of no bytes and so an empty read
    /// could only ever mean a fault.
    fn Get(&self, address: &str) -> Result<Vec<u8>, StorageError>;
}
