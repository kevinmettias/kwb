//! What an extraction says a passage asserts.

/// What an extractor said a passage asserts about a concept.
///
/// # Why this is a type and not the `String` it carries
///
/// The partner of [`ConceptName`], and for the same reason: two adjacent `String`s at one call
/// site are two positions a caller can only tell apart by where they sit. A claim text handed
/// to the concept position would name a concept after a whole assertion, and nothing in the
/// pipeline would object.
///
/// Deliberately text, and deliberately not normalized, for the reason [`ConceptName`] gives:
/// a reflowed assertion must not change identity, and deciding that is [`Claim`]'s job one
/// stage further in, not this type's.
///
/// [`Claim`]: kwb_domain::Claim
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimText(String);

impl ClaimText
{
    /// Text as the extractor gave it.
    #[must_use]
    pub fn Stated(text: &str) -> Self
    {
        return Self(text.to_owned());
    }

    /// The text.
    #[must_use]
    pub fn Text(&self) -> &str
    {
        return &self.0;
    }
}
