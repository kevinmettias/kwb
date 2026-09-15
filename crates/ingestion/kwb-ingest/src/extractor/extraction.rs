//! What an extractor offers the pipeline, before any of it is believed.

use crate::ClaimText;
use crate::ConceptName;

/// One concept-and-claim pair as an extractor produced it.
///
/// Deliberately plain text on both sides. This is the boundary where a model's output
/// arrives, and nothing about it is trusted yet — it carries no identity, no confidence and
/// no grading, because a grade a source assigns its own output is not a measurement of that
/// output (`D-004`, on the epistemic-strength model, and the bearing measurement behind it).
///
/// The two halves are separate types rather than two `String`s, so that an extractor's
/// concept and claim cannot be offered the wrong way round — see [`ConceptName`] and
/// [`ClaimText`], which also say why neither is normalized here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Extraction
{
    /// What the extractor said the passage is about.
    pub concept_name: ConceptName,

    /// What the extractor said the passage asserts about it.
    pub claim_text: ClaimText,
}

impl Extraction
{
    /// An offered extraction.
    #[must_use]
    pub const fn New(concept_name: ConceptName, claim_text: ClaimText) -> Self
    {
        return Self {
            concept_name,
            claim_text,
        };
    }

    /// Whether this extraction carries both halves of what it claims to be.
    ///
    /// An extraction missing either side is refused rather than admitted with an empty
    /// field, which is the shape of the prototype's `ReplaceAllAsync` incident: an
    /// empty-but-valid reply erased a concept's entire synthesis because empty was
    /// indistinguishable from answered.
    #[must_use]
    pub fn Is_Complete(&self) -> bool
    {
        return !self.concept_name.Text().trim().is_empty()
            && !self.claim_text.Text().trim().is_empty();
    }
}
