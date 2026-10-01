//! What produced an extraction, and under what protocol.

use crate::ReaderName;
use crate::ReadingProtocol;

/// What produced an extraction, and under what protocol.
///
/// # Why lineage travels with the output and not with the claim
///
/// A claim's identity is derived from its content with the source excluded, so two books
/// asserting one thing are one claim (`D-002`). Lineage is a fact about *this reading*, not
/// about the proposition, and putting it in the claim would make one proposition read twice
/// into two claims — the contamination `D-010` prevents, arriving through the extractor.
///
/// It travels here so that *changing the model or the prompt does not silently redefine KWB
/// semantic identity*: a re-read under a new protocol produces a new extraction with new
/// lineage, and if it proposes the same proposition it lands on the same claim.
///
/// It is filed apart from [`ProposedReading`] because it is a fact about the reading apparatus
/// rather than about what was read, and a caller reporting which protocol produced a corpus
/// reaches for it by name.
///
/// [`ProposedReading`]: crate::ProposedReading
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtractionLineage
{
    protocol: ReadingProtocol,
    reader: ReaderName,
}

impl ExtractionLineage
{
    /// The protocol that governed the reading, and who or what did it.
    ///
    /// `reader` is a person as readily as a model. The corpus's admission rule turns on whether
    /// a feature still makes sense with a perfect human annotator, and this type answers yes by
    /// construction — a person is a reader with a protocol, exactly like a model.
    ///
    /// The two are separate types rather than two `&str`s so that a caller cannot file a reader
    /// under a protocol it did not read under; see [`ReadingProtocol`] and [`ReaderName`], which
    /// carry the normalization both take.
    #[must_use]
    pub fn Of(protocol: ReadingProtocol, reader: ReaderName) -> Self
    {
        return Self { protocol, reader };
    }

    /// The protocol under which the source was read.
    #[must_use]
    pub fn Protocol(&self) -> &str
    {
        return self.protocol.Text();
    }

    /// Who or what read it.
    #[must_use]
    pub fn Reader(&self) -> &str
    {
        return self.reader.Text();
    }
}
