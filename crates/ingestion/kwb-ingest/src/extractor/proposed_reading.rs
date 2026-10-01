//! What one reading of one source proposed.

use kwb_model::ContentIdentity;

use crate::ExtractionLineage;
use crate::SourceLocation;
use crate::Extraction;

/// What one reading of one source proposed.
///
/// # This is not an intermediate representation, and the name is chosen to keep it that way
///
/// A canonical Knowledge IR is **undecided**, and until `KWB-62` this paragraph said `D-009`
/// records it as *stranded*. That was true when written and stopped being true eight minutes
/// later: `D-009` states its condition as `KWB-3` and `KWB-5` closed, and `KWB-5` closed 495
/// seconds after the record holding it was amended. Both records now say so.
///
/// **The risk this type is shaped against is unchanged, and the unstranding sharpens it.** The
/// corpus calls a Knowledge IR the biggest idea in the design and the prototype has zero files
/// implementing one, so the danger was always that a local representation which happened to
/// ship first quietly becomes the canonical one because nothing else existed. While the subject
/// was stranded, that could not happen by decision — only by drift. Now that it is decidable,
/// the drift is the only way it *would* happen, because a decision would be written down.
///
/// So: source-local, named for the act rather than the architecture, and carrying only what one
/// reading of one source knows. If a multi-stage representation is ever decided, this is one
/// front end feeding it, not the thing it grew out of.
///
/// # Proposed, not asserted
///
/// Nothing here is a KWB entity and nothing here has an identity. The extractor returns text
/// and where it found it; `Admit_Source` decides what becomes a concept, a claim and an assertion, and
/// `kwb-model` derives every address from content. An extractor that minted an identity would
/// be a second authority for the thing `D-002` owns — and the reference miner is the worked
/// example of why that matters, because its own claim identity absorbs the source path and page
/// window, which is the deliberate inverse of the rule that lets two books become one claim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProposedReading
{
    source: ContentIdentity,
    location: SourceLocation,
    proposed: Vec<Extraction>,
    lineage: ExtractionLineage,
}

impl ProposedReading
{
    /// What a reader proposes a passage of a source says.
    #[must_use]
    pub fn Of(
        source: ContentIdentity,
        location: SourceLocation,
        proposed: Vec<Extraction>,
        lineage: ExtractionLineage,
    ) -> Self
    {
        return Self {
            source,
            location,
            proposed,
            lineage,
        };
    }

    /// The document this was read out of, by its address.
    #[must_use]
    pub const fn Source(&self) -> ContentIdentity
    {
        return self.source;
    }

    /// Where in it.
    #[must_use]
    pub const fn Location(&self) -> &SourceLocation
    {
        return &self.location;
    }

    /// What the reading proposes the passage says. Proposals, not claims.
    #[must_use]
    pub fn Proposed(&self) -> &[Extraction]
    {
        return &self.proposed;
    }

    /// Under what protocol, and by whom.
    ///
    /// # This reaches the door and stops there
    ///
    /// `Admit_Source` reads [`Proposed`] and [`Source`] and discards this and [`Location`]. An
    /// `Assertion` holds a source, a claim and a scope, so nothing about the *reading* survives
    /// into the graph. Recorded here rather than left for a reader to discover, because a field
    /// nothing consumes reads exactly like a field something does.
    ///
    /// [`Proposed`]: Self::Proposed
    /// [`Source`]: Self::Source
    /// [`Location`]: Self::Location
    #[must_use]
    pub const fn Lineage(&self) -> &ExtractionLineage
    {
        return &self.lineage;
    }
}
