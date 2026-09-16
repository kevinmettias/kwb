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

/// The two things only this file can assert, because [`Link_Concepts`] is `pub(crate)`.
///
/// A file under `tests/` compiles as its own package and sees only the `pub` surface, so the stage
/// itself cannot be called from there. What is reachable from outside is the [`Linked`] it returns,
/// and that is asserted in `tests/linked.rs`. What is left for here is what the stage guarantees
/// about the input it was handed — that it infers nothing, and that it refuses rather than admits a
/// blank half — which is a fact about this function rather than about the value it produces.
#[cfg(test)]
mod tests
{
    use crate::ClaimText;
    use crate::ConceptName;
    use crate::Extraction;
    use crate::Link_Concepts;

    #[test]
    fn Test_Link_Concepts_Should_Refuse_An_Extraction_Missing_A_Half()
    {
        // An extraction missing either side is refused rather than admitted with an empty field,
        // which is the shape of the prototype's `ReplaceAllAsync` incident: an empty-but-valid
        // reply erased a concept's entire synthesis because empty was indistinguishable from
        // answered. The complete extraction beside it is the control — a stage that refused
        // everything would satisfy the count below on its own, and would admit nothing at all.
        let half = One_Complete_And_One_Half_Stated();
        let linked = Link_Concepts(&half);

        assert_eq!(
            linked.Refused(),
            1,
            "an extraction missing a half was admitted, so a claim with a blank side reaches the \
             graph"
        );
        assert_eq!(
            linked.Concepts().len(),
            1,
            "the refused extraction produced a concept anyway, so the count of what was admitted \
             and the count of what was refused disagree"
        );
        assert_eq!(linked.Claims().len(), 1, "the refused extraction produced a claim");
    }

    /// One complete extraction and one missing a half, which is the pair the refusal is stated
    /// over: a stage that refused everything would satisfy the refusal half on its own.
    ///
    /// Directly under the refusal test, because that test is the only one that asks for it.
    fn One_Complete_And_One_Half_Stated() -> Vec<Extraction>
    {
        return vec![
            An_Extraction("entropy", "It is non-decreasing."),
            An_Extraction("entropy", " \t "),
        ];
    }

    #[test]
    fn Test_Link_Concepts_Should_Attach_A_Claim_To_The_Concept_Its_Own_Extraction_Named()
    {
        // Inference is what this stage refuses to do, and `D18` is why: `AliasDerivation.IsVariantOf`
        // was a correct predicate, and the harm came from a stage consuming its answers with a
        // structure assuming a property the predicate did not have. The cheapest way for this stage
        // not to repeat that is to produce no relation beyond *this extraction named that concept*,
        // which is what the pairing below asserts — a claim attached to any concept other than the
        // one its own extraction named would be a judgement this stage is not entitled to make.
        let linked = Link_Concepts(&Extractions_Naming_Two_Concepts());

        let named: Vec<&str> = linked
            .Concepts()
            .iter()
            .map(|concept| return concept.Canonical_Name())
            .collect();

        assert_eq!(
            named.as_slice(),
            Two_Concepts().as_slice(),
            "a concept was named other than by the extraction that produced it"
        );

        for (index, claim) in linked.Claims().iter().enumerate()
        {
            assert_eq!(
                claim.Concept(),
                linked
                    .Concepts()
                    .get(index)
                    .expect("one concept per complete extraction")
                    .Identity(),
                "a claim is attached to a concept other than the one its own extraction named"
            );
        }
    }

    /// One extraction per concept, each asserting something.
    ///
    /// Under the test that asks for it, and above the pair it is built from, so the shared region
    /// below reads from the general down to the specific.
    fn Extractions_Naming_Two_Concepts() -> Vec<Extraction>
    {
        return Two_Concepts()
            .iter()
            .map(|name| return An_Extraction(name, "It is extensive."))
            .collect();
    }

    /// Two concepts, so that *the concept its own extraction named* is a distinction rather than a
    /// single value every answer would satisfy.
    ///
    /// Both tests name both halves, so this belongs under neither of them and sits in the shared
    /// region with `An_Extraction` below it.
    fn Two_Concepts() -> [&'static str; 2]
    {
        return ["entropy", "enthalpy"];
    }

    fn An_Extraction(concept: &str, claim: &str) -> Extraction
    {
        return Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));
    }
}
