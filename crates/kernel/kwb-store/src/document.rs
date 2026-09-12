//! What a document is to this crate: its octets, and the address they give it.

use kwb_model::ContentIdentity;
use kwb_model::Derivation;

/// The kind a document's identity is derived under, so that a document and a claim
/// carrying identical bytes are not the same artifact.
const DOCUMENT: &str = "document";

/// The one field a document's identity is derived from.
const CONTENT: &str = "content";

/// A document, and the identity its content gives it.
///
/// The two are not separable and that is the point. There is no constructor taking an
/// identity and some bytes, so no value of this type can claim an address that is not its
/// own, and [`Written`] — which is built from one of these — inherits that guarantee
/// rather than restating it.
///
/// # What is deliberately not here
///
/// A **kind**. A document in this crate is opaque: bytes that something else will
/// interpret. Naming what sort of thing it is would be the first row of the universal
/// epistemic type kernel, which `KWB-3` owns and no document in this repository has
/// designed yet; inventing one here so the store could sort its contents is exactly the
/// architectural concretization `D-004` holds until its inputs stabilise.
///
/// **Anything about where the bytes came from.** A source is excluded from the identity
/// deliberately and the exclusion is recorded, because two references carrying the same
/// passage must become one document with two citations. That is the mechanism the whole
/// workspace rests on, stated again here because this is the second place it has to hold.
///
/// [`Written`]: crate::Written
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document
{
    identity: ContentIdentity,
    content: Vec<u8>,
}

impl Document
{
    /// Address a document by its content.
    ///
    /// This is the only way to obtain one, and it derives rather than accepts the identity.
    #[must_use]
    pub fn Of(content: Vec<u8>) -> Self
    {
        let identity = Derivation::Of(DOCUMENT)
            .With_Bytes(CONTENT, &content)
            .Excluding(
                "source",
                "two references carrying one passage must become one document with two citations",
            )
            .Excluding(
                "received",
                "when a document arrived is a fact about this run, not about the document",
            )
            .Seal()
            .Identity();

        return Self { identity, content };
    }

    /// The address these bytes have.
    #[must_use]
    pub const fn Identity(&self) -> ContentIdentity
    {
        return self.identity;
    }

    /// The bytes themselves, unmodified in every respect.
    #[must_use]
    pub fn Content(&self) -> &[u8]
    {
        return &self.content;
    }

    /// How many bytes the document is.
    #[must_use]
    pub fn Length(&self) -> usize
    {
        return self.content.len();
    }

    /// Whether the document carries no bytes at all.
    #[must_use]
    pub fn Is_Empty(&self) -> bool
    {
        return self.content.is_empty();
    }
}
