//! Stage one: link a claim to the concept its extraction named, and to nothing else.

use kwb_domain::Claim;
use kwb_domain::Concept;

use crate::Extraction;

/// What linking produced: one concept per accepted extraction, and its claim.
///
/// **Concepts are not deduplicated here, deliberately.** One extraction produces one concept,
/// so two passages naming `entropy` produce two, and grouping them is
/// [`Normalize_Concepts`][crate::Normalize_Concepts]'s job.
///
/// This was written the other way first, and a test caught it. With the deduplication here,
/// normalization had nothing left to group — a stage that could never merge anything and
/// therefore could never be wrong about merging, which is the same as not having the stage.
/// Worse, the control test meant to prove normalization *does* merge passed anyway, because
/// linking had already done it: a stage boundary drawn in the wrong place made a test vacuous
/// without making it fail. That is `D18`'s shape at one remove, and the only reason it
/// surfaced is that the transitivity test asserted the count of merges rather than the count
/// of concepts.
#[derive(Clone, Debug, Default)]
pub struct Linked
{
    concepts: Vec<Concept>,
    claims: Vec<Claim>,
    refused: usize,
}

impl Linked
{
    /// The concepts, in the order they were first named.
    #[must_use]
    pub fn Concepts(&self) -> &[Concept]
    {
        return &self.concepts;
    }

    /// The claims, in the order their extractions arrived.
    #[must_use]
    pub fn Claims(&self) -> &[Claim]
    {
        return &self.claims;
    }

    /// How many extractions were refused as incomplete.
    ///
    /// Reported rather than dropped. `D19`'s corollary is that a report may only count what
    /// it can see, and a stage that discards input without saying how much is the reason
    /// that corollary exists.
    #[must_use]
    pub const fn Refused(&self) -> usize
    {
        return self.refused;
    }
}

/// Link each extraction's claim to the concept that extraction named.
///
/// # The property this stage depends on, and the one it refuses to assume
///
/// **It infers nothing.** A claim is attached to the concept its own extraction named, and
/// to no other. There is no matching, no similarity, no alias resolution and no model in
/// this path, so there is no relation here whose algebra anything would need to check.
///
/// That is a deliberate narrowing, and `D18` is why. `AliasDerivation.IsVariantOf` was a
/// correct predicate; the harm came from a stage that consumed its answers with a structure
/// assuming a property the predicate did not have. The cheapest way for this stage not to
/// repeat that is to produce no relation at all beyond *this extraction named that concept*,
/// which is not a judgement and cannot be wrong about anything the extractor did not already
/// say.
///
/// Anything richer — a claim linked to a concept nobody named in that passage — waits on the
/// relation vocabulary `D-011` specifies, because that is the thing that would say which
/// structures may consume which relations.
#[must_use]
pub(crate) fn Link_Concepts(extractions: &[Extraction]) -> Linked
{
    let mut linked = Linked::default();

    for extraction in extractions
    {
        if !extraction.Is_Complete()
        {
            linked.refused = linked.refused.saturating_add(1);
            continue;
        }

        let concept = Concept::Named(extraction.concept_name.Text());
        let claim = Claim::About(&concept, extraction.claim_text.Text());

        linked.concepts.push(concept);
        linked.claims.push(claim);
    }

    return linked;
}
