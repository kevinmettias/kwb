//! The receipt a write returns, and the reason it cannot be built out of nothing.

use kwb_model::ContentIdentity;

use crate::Admission;
use crate::Document;

/// A write that happened, carrying the evidence that it did.
///
/// # Why this type exists rather than a `DocumentId` or a bare `Ok(())`
///
/// `D19` in the prototype's register: `kw admit` queued every passage of every book as a
/// job of a kind nothing consumed; the scheduler's `default:` arm deleted the job and
/// returned normally, so it was marked `Succeeded`. Four green components, one destroyed
/// corpus, and a screen of zeros indistinguishable from a clean run of an empty book. The
/// shape of that defect is a success value produced on a path that did no work.
///
/// So the success value of this crate's one write path cannot be produced on such a path.
/// [`Written::For`] is crate-private and takes a [`Document`], which cannot itself be
/// constructed without the bytes it addresses — so the identity on this receipt is the
/// identity of content that exists, the length is that content's length, and neither can
/// be supplied by a caller who has nothing to write. There is no public constructor, no
/// [`Default`], and no setter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[must_use]
pub struct Written
{
    identity: ContentIdentity,
    length: usize,
    admission: Admission,
}

impl Written
{
    /// Mint the receipt for a document the store has accepted.
    ///
    /// Crate-private, and takes the document rather than its parts: a receipt assembled
    /// from an identity and a length would be a receipt a caller could write for work
    /// nobody did.
    pub(crate) fn For(document: &Document, admission: Admission) -> Self
    {
        return Self {
            identity: document.Identity(),
            length: document.Length(),
            admission,
        };
    }

    /// The address the written content has, which is the address it will always have.
    #[must_use]
    pub const fn Identity(&self) -> ContentIdentity
    {
        return self.identity;
    }

    /// How many bytes the written document is.
    #[must_use]
    pub const fn Length(&self) -> usize
    {
        return self.length;
    }

    /// What the store did: stored these bytes, or already held them.
    #[must_use]
    pub const fn Admission(&self) -> Admission
    {
        return self.admission;
    }

    /// Whether this write is what put the bytes in the store.
    ///
    /// Derived from [`Admission`] rather than stored beside it, so the two cannot disagree.
    #[must_use]
    pub const fn Was_Stored(&self) -> bool
    {
        return matches!(self.admission, Admission::Stored);
    }
}
