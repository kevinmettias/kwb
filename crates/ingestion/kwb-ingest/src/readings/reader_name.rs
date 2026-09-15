//! Who or what read a source.

/// The name of whoever or whatever read a source — a person as readily as a model.
///
/// # Why it is not an [`ExtractionStrategy`]
///
/// An `ExtractionStrategy` is *how* a reading happens: it is handed bytes and returns
/// proposals. A `ReaderName` is *who* did it, recorded as a name so that a later session can
/// tell one reading apparatus from another. The corpus's admission rule turns on whether a
/// feature still makes sense with a perfect human annotator, and this type is what makes that a
/// fact the lineage states rather than one a reader has to infer from which strategy ran.
///
/// # Why this is a type and not the `&str` it carries
///
/// The partner of [`ReadingProtocol`] in one constructor, and two adjacent `&str`s are two
/// positions distinguishable only by order — so a reader could be filed under a protocol it
/// never read under. It is normalized for the reason that type gives: one reader named two ways
/// is one reader.
///
/// [`ExtractionStrategy`]: crate::ExtractionStrategy
/// [`ReadingProtocol`]: crate::ReadingProtocol
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReaderName(String);

impl ReaderName
{
    /// A reader as the caller names it.
    #[must_use]
    pub fn Named(reader: &str) -> Self
    {
        return Self(kwb_model::Normalize_Text(reader));
    }

    /// The name.
    #[must_use]
    pub fn Text(&self) -> &str
    {
        return &self.0;
    }
}
