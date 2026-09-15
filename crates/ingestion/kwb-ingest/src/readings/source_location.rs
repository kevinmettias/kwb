//! Where in a source something was found.
//!
//! It is filed apart from [`ProposedReading`] because it is the part of a reading that has to
//! keep being right for every medium, while the reading itself is about one act of reading
//! one source.
//!
//! [`ProposedReading`]: crate::ProposedReading

/// Where in a source something was found.
///
/// Deliberately opaque text rather than a page, an offset or a span. What locates a passage
/// differs by medium, this repository reads one medium, and a structured location invented for
/// one medium becomes the structure every later medium has to pretend to fit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceLocation(String);

impl SourceLocation
{
    /// A location as the extractor describes it.
    #[must_use]
    pub fn Named(description: &str) -> Self
    {
        return Self(kwb_model::Normalize_Text(description));
    }

    /// The description.
    #[must_use]
    pub fn Description(&self) -> &str
    {
        return &self.0;
    }
}
