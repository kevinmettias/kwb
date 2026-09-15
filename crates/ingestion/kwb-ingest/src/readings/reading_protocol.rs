//! The protocol a reading was taken under.

/// The protocol that governed a reading — the prompt, the instructions, the model version.
///
/// # Why this is a type and not the `&str` it carries
///
/// [`ExtractionLineage::Of`] took two adjacent `&str`s, so the protocol and the reader could be
/// handed over in the wrong order and the compiler would accept it — filing a person under a
/// protocol nobody read under. The two name different things: one is *under what instructions*
/// and the other is *who or what did it*.
///
/// # Why it is normalized and [`ConceptName`] is not
///
/// A protocol is a name a repository keeps, not a reading of a source: two runs that spell one
/// protocol differently are one protocol, and a lineage that recorded the spelling would make a
/// re-read under the same instructions look like a re-read under new ones. What was *read* is
/// the opposite case, which is why the names a source is read into travel unnormalized.
///
/// [`ExtractionLineage::Of`]: crate::ExtractionLineage::Of
/// [`ConceptName`]: crate::ConceptName
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadingProtocol(String);

impl ReadingProtocol
{
    /// A protocol as its author names it.
    #[must_use]
    pub fn Named(protocol: &str) -> Self
    {
        return Self(kwb_model::Normalize_Text(protocol));
    }

    /// The protocol.
    #[must_use]
    pub fn Text(&self) -> &str
    {
        return &self.0;
    }
}
