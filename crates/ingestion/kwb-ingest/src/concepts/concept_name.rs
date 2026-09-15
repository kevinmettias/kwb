//! The concept an extraction says a passage is about.

/// The concept an extractor said a passage is about.
///
/// # Why this is a type and not the `String` it carries
///
/// [`Extraction::New`] took two adjacent `String`s, so an extractor's concept name and its
/// claim text could be handed over in the wrong order and the compiler would accept it —
/// producing a claim whose concept is its own text, and a concept named after an assertion.
/// Neither position is interchangeable with the other, so each gets its own type and the swap
/// becomes a compile error.
///
/// Deliberately text, and deliberately not normalized. This is the boundary where a model's
/// output arrives, and nothing about it is trusted yet — so the name is carried exactly as
/// the extractor gave it. Normalizing here would derive an identity from it one stage before
/// [`Link_Concepts`] does, and would hide from that stage the difference between what was
/// said and what it normalizes to.
///
/// [`Extraction::New`]: crate::Extraction::New
/// [`Link_Concepts`]: crate::Link_Concepts
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConceptName(String);

impl ConceptName
{
    /// A name as the extractor gave it.
    #[must_use]
    pub fn Named(name: &str) -> Self
    {
        return Self(name.to_owned());
    }

    /// The name.
    #[must_use]
    pub fn Text(&self) -> &str
    {
        return &self.0;
    }
}
