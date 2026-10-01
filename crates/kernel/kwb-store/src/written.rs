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
    pub const fn Has_Stored(&self) -> bool
    {
        return matches!(self.admission, Admission::Stored);
    }
}

/// The one assertion only this file can make, because [`Written::For`] is `pub(crate)`.
///
/// A receipt is minted in exactly one place, and a file under `tests/` compiles as its own package
/// and sees only the `pub` surface — so the constructor cannot be reached from there, and its
/// contract has to be asserted here. The four readers it feeds are asserted in `tests/written.rs`,
/// which is where a `pub` method's test belongs.
#[cfg(test)]
mod tests
{
    use crate::Admission;
    use crate::Document;
    use crate::Written;

    #[test]
    fn Test_For_Should_Take_The_Receipt_Out_Of_The_Document_It_Was_Handed()
    {
        // The document is the only source of two of the three fields, and that is the whole reason
        // this constructor is crate-private: a receipt for work nobody did would be one assembled
        // from an address and a size a caller supplied. Handing over the document as a whole value
        // — rather than a pair of numbers copied out of it — is what makes that unreachable, so it
        // is what this asserts.
        //
        // The verdict is the third field, and it is the caller's: `For` carries what it was given
        // rather than deciding anything, which is `D20` read at this seam — the store derives the
        // admission from what it held, and the receipt only reports it.
        let document = Document::Of(b"a passage".to_vec());
        let stored = Written::For(&document, Admission::Stored);
        let present = Written::For(&document, Admission::AlreadyPresent);

        Assert_Receipt_Describes_The_Document(stored, present, &document);
        Assert_Receipt_Carries_The_Admission(stored, present);
    }

    /// Asserts both receipts hold the address and the size of the one document they were minted
    /// from, and that the two agree on the address.
    ///
    /// Compared against the document the test still holds rather than against a second derivation
    /// from the same octets: a receipt that recomputed the address itself would agree with one,
    /// and the claim being made here is that it carries the document's.
    ///
    /// The pair agreeing with each other is asserted for the same reason. One document producing
    /// two addresses would mean the receipt answers to something other than the document, which is
    /// the guarantee `For` taking the document whole exists to give — and a single receipt could
    /// not show it.
    fn Assert_Receipt_Describes_The_Document(stored: Written, present: Written, document: &Document)
    {
        assert_eq!(
            stored.Identity(),
            document.Identity(),
            "the receipt carries an address the document it was built from does not have"
        );
        assert_eq!(
            stored.Length(),
            document.Length(),
            "the receipt carries a size the document it was built from does not have"
        );
        assert_eq!(
            present.Identity(),
            stored.Identity(),
            "one document produced two receipts with different addresses, so the receipt depends on \
             something other than the document"
        );
    }

    /// Asserts each receipt reports the verdict the caller handed to `For`, and that the two
    /// verdicts differ.
    ///
    /// Both are asserted together because either one alone is satisfied by a receipt that reports
    /// the same verdict forever: the differing pair is what shows the field tracks what it was
    /// given rather than a constant.
    fn Assert_Receipt_Carries_The_Admission(stored: Written, present: Written)
    {
        assert_eq!(
            stored.Admission(),
            Admission::Stored,
            "the verdict the store derived was not carried onto the receipt"
        );
        assert_eq!(
            present.Admission(),
            Admission::AlreadyPresent,
            "the verdict the store derived was not carried onto the receipt"
        );
    }
}
