//! The reader the corpus's admission rule is written about: a person.

use kwb_domain::Scope;
use kwb_model::ContentIdentity;

use crate::Extraction;
use crate::ExtractionLineage;
use crate::ExtractionRefused;
use crate::ExtractionStrategy;
use crate::ProposedReading;
use crate::ReadingKind;
use crate::SourceLocation;

/// Claims a person supplied, offered through the extraction seam.
///
/// # Why the first implementation is a human and not a model
///
/// The corpus's ontological admission rule asks whether a feature would still make sense *if
/// the model were replaced by a perfect human annotator*. Yes means it is KWB; no means it is
/// an AI capability KWB happens to use. Extraction passes that test, and this type is the
/// proof: it is the whole seam, exercised, with no model anywhere near it.
///
/// It is also not new behaviour. `kwb admit --says <concept> <claim>` has always been a person
/// stating what a passage asserts; this gives that act the name it already had and routes it
/// through the same door a model-backed reader will use, so the seam has a working
/// implementation on the day it is declared rather than a promise of one.
///
/// # What this makes visible, and what it does not yet record
///
/// `--says` supplies **no location and no lineage**. It never has. Passing it through this type
/// forces both to be *said* — a person is a reader with a protocol, and this is where the
/// protocol and the person get named instead of being implied by somebody having typed them.
///
/// **They are said and not yet recorded.** `Admit` carries a reading's location and lineage as
/// far as the door and then drops both: an `Assertion` holds a source, a claim and a scope, so
/// what survives into the graph is the source address and nothing about the reading. The
/// invariant that every admitted claim identifies its source *occurrence* and its extraction
/// protocol is therefore **unmet**, and this paragraph said the opposite until `KWB-53` — in
/// the documentation of the type whose whole purpose is making provenance explicit.
///
/// Left as a stated gap rather than closed in passing, because whether an `Assertion` carries
/// them is a decision with a consequence either way. If its identity absorbed the protocol, a
/// re-read under a new prompt would become a second citation of the same source for the same
/// claim. If it carried them without identifying by them, two readings would meet at one
/// assertion holding one of their lineages, and a value would disagree with itself — which
/// `KWB-32` has already had to fix once, for `Concept` and `Claim`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stated
{
    statements: Vec<Extraction>,
    location: SourceLocation,
    lineage: ExtractionLineage,
    scope: Scope,
}

impl Stated
{
    /// What a person says a source asserts, or [`None`] if they said nothing.
    ///
    /// # Why saying nothing is not a reader
    ///
    /// A reading that examined material and found none is `Barren`, which is **evidence of
    /// absence** and the only outcome that licenses a caller to conclude there is nothing
    /// there. A person who supplied no statements has not examined anything, so admitting them
    /// as a reader would turn *nobody said anything* into *there is nothing to say* — the
    /// 1,367-row incident `Coverage` exists to prevent, arriving through the front door.
    ///
    /// Returning [`None`] puts that case where it belongs: there is no reading, so admission
    /// reports `Unmet`, which is what `kwb admit` already does for a run given no `--says`.
    #[must_use]
    pub fn Of(
        stated: Vec<Extraction>,
        location: SourceLocation,
        lineage: ExtractionLineage,
        scope: Scope,
    ) -> Option<Self>
    {
        if stated.is_empty()
        {
            return None;
        }

        return Some(Self {
            statements: stated,
            location,
            lineage,
            scope,
        });
    }
}

impl ExtractionStrategy for Stated
{
    /// Hand back what the person said.
    ///
    /// The content is not consulted, and that is the honest description of what happened: the
    /// person read the source, this code did not. What it does supply is the source's address,
    /// so the statements are anchored to the exact bytes admitted rather than to a filename.
    ///
    /// # Why this reader refuses no [`ReadingKind`]
    ///
    /// A person can look at a scanned page. That is the whole reason the corpus's admission
    /// rule is phrased around a perfect human annotator: the annotator is the reader with no
    /// capability gap, so a refusal on kind would be this type inventing a limit its subject
    /// does not have. A reader that *does* have one — every text extractor — refuses here, and
    /// that asymmetry is the contract working rather than a gap in it.
    ///
    /// # Errors
    ///
    /// Never. A [`Stated`] cannot exist without at least one statement, so there is no reading
    /// for it to fail at. The signature keeps the [`Result`] because the seam has one shape for
    /// every reader, and a reader that cannot fail is a fact about this reader.
    fn Read(
        &self,
        source: ContentIdentity,
        _content: &[u8],
        _needed: ReadingKind,
    ) -> Result<ProposedReading, ExtractionRefused>
    {
        return Ok(ProposedReading::Of(
            source,
            self.location.clone(),
            self.statements.clone(),
            self.lineage.clone(),
        ));
    }

    fn Scope(&self) -> Scope
    {
        return self.scope.clone();
    }
}
