//! Stage two: group concepts that are the same concept, under a relation that is entitled
//! to be grouped by.

use kwb_domain::Concept;
use kwb_model::ContentIdentity;

use crate::Linked;

/// Concepts after grouping, with the claims unchanged.
#[derive(Clone, Debug, Default)]
pub struct Normalized
{
    concepts: Vec<Concept>,
    linked: Linked,
    merged: usize,
}

impl Normalized
{
    /// The distinct concepts.
    #[must_use]
    pub fn Concepts(&self) -> &[Concept]
    {
        return &self.concepts;
    }

    /// Everything linking produced, unchanged. Normalization removes no claim.
    #[must_use]
    pub const fn Linked(&self) -> &Linked
    {
        return &self.linked;
    }

    /// How many claims survived the stage.
    ///
    /// Grouping concepts removes no claim, and this is how a caller checks that rather than
    /// trusting it. A merge that silently dropped the loser's claims would be a destructive
    /// edit with nothing authorising it.
    #[must_use]
    pub fn Claims_Held(&self) -> usize
    {
        return self.linked.Claims().len();
    }

    /// How many concept mentions were folded into an existing concept.
    #[must_use]
    pub const fn Merged(&self) -> usize
    {
        return self.merged;
    }
}

/// Group concepts that are the same concept.
///
/// # The algorithmic property this stage depends on
///
/// Grouping is a **transitive closure**: if `a` groups with `b` and `b` with `c`, then `a`,
/// `b` and `c` are one group whether or not anything ever compared `a` to `c`. A structure
/// that computes one is entitled to be handed only a relation that **is** transitive.
///
/// `D18` is what happens otherwise, and it is worth stating in full because this is the
/// stage it happened in. `AliasDerivation.IsVariantOf` was correct and had a passing test
/// saying `C++` and `C` are not variants. Its answers went into a union-find. *Is a variant
/// of* is not transitive:
///
/// ```text
/// IsVariantOf("z m",         "zero mass")   = true     a legitimate acronym of it
/// IsVariantOf("z m",         "zero matrix") = true     and of that too
/// IsVariantOf("zero matrix", "zero mass")   = FALSE    it said no. Nobody asked it.
/// ```
///
/// Every ambiguous abbreviation became a bridge, and **144 of 579 merges — 25% — fused ideas
/// the rule explicitly rejects**, with 1,878 green tests unable to see it because all of them
/// asserted properties of the predicate while the defect was in the structure consuming it.
///
/// # So the relation here is identity, and there is no parameter for another
///
/// Two concepts group when their content identities are equal. Equality is reflexive,
/// symmetric and transitive by construction, so the closure this function computes is one it
/// is entitled to compute, and the entitlement is a fact about the relation rather than a
/// promise in a comment.
///
/// **This function takes no predicate, and that is the design.** A caller cannot supply a
/// looser notion of sameness, so a caller cannot repeat `D18` through this door. Alias and
/// variant resolution are not deferred out of caution — they are refused until the relation
/// vocabulary exists to say which relations may be closed over, which is `D-011`'s
/// requirement that a universal kind commit its specializations on transitivity in **both**
/// directions. Today it can forbid transitivity and cannot require it, so nothing can yet
/// certify a variant relation as safe to hand to this stage.
///
/// The prototype reached the same place after the incident: its post-2026-07-12 normaliser
/// buckets on canonical names with no model and no alias in the path.
#[must_use]
pub(crate) fn Normalize_Concepts(linked: Linked) -> Normalized
{
    let mut concepts: Vec<Concept> = Vec::new();
    let mut seen: Vec<ContentIdentity> = Vec::new();
    let mut merged = 0_usize;

    for concept in linked.Concepts()
    {
        if seen.contains(&concept.Identity())
        {
            merged = merged.saturating_add(1);
            continue;
        }

        seen.push(concept.Identity());
        concepts.push(concept.clone());
    }

    return Normalized {
        concepts,
        linked,
        merged,
    };
}

/// The two things only this file can assert, because [`Normalize_Concepts`] is `pub(crate)`.
///
/// A file under `tests/` compiles as its own package and sees only the `pub` surface, so the stage
/// cannot be called from there; what is reachable is the [`Normalized`] it returns, and that is
/// asserted in `tests/normalized.rs`. What is left for here is what the stage guarantees about the
/// relation it groups by, which is a fact about this function rather than about its output.
#[cfg(test)]
mod tests
{
    use crate::ClaimText;
    use crate::ConceptName;
    use crate::Extraction;
    use crate::Link_Concepts;
    use crate::Normalize_Concepts;

    /// The one thing both fixtures are built from, and the smallest piece either of them states.
    fn An_Extraction(concept: &str, claim: &str) -> Extraction
    {
        return Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));
    }

    #[test]
    fn Test_Normalize_Concepts_Should_Group_Mentions_Of_One_Concept()
    {
        // The stage's whole job, and the property that entitles it to compute a closure: two
        // mentions of one name group, two mentions of two names do not. Grouping is a transitive
        // closure, and a closure is only entitled to be computed over a relation that **is**
        // transitive — which is why this function takes no predicate and a caller cannot hand it a
        // looser notion of sameness. The second half is the control `D18` says to keep.
        let distinct = Distinct_Mentions();

        assert_eq!(
            Normalize_Concepts(Link_Concepts(&Mentions_Of_One_Concept()))
                .Concepts()
                .len(),
            1,
            "two mentions of one content identity stayed two concepts, so a re-read source is two \
             sources' worth of knowledge"
        );
        assert_eq!(
            Normalize_Concepts(Link_Concepts(&distinct)).Concepts().len(),
            distinct.len(),
            "two concepts that are not one concept were grouped"
        );
    }

    /// Two concepts that are not one concept, which is the control: without it a stage that
    /// grouped everything would satisfy the assertion above.
    ///
    /// Under the test that asks for it, which is the only one that does.
    fn Distinct_Mentions() -> Vec<Extraction>
    {
        return vec![
            An_Extraction("entropy", "It is non-decreasing."),
            An_Extraction("enthalpy", "It is extensive."),
        ];
    }

    #[test]
    fn Test_Normalize_Concepts_Should_Drop_No_Claim_When_It_Groups()
    {
        // Grouping concepts removes no claim, and this is where that is asserted as a fact about
        // the stage rather than trusted as a comment. A merge that silently dropped the loser's
        // claims would be a destructive edit with nothing authorising it — `D17` arriving one stage
        // early, and arriving invisibly, because the concept count would be exactly right.
        let mentions = Mentions_Of_One_Concept();
        let normalized = Normalize_Concepts(Link_Concepts(&mentions));

        assert_eq!(
            normalized.Concepts().len(),
            1,
            "the fixture must merge, or the claim count below proves nothing"
        );
        assert_eq!(
            normalized.Claims_Held(),
            mentions.len(),
            "grouping dropped a claim belonging to the concept it folded away"
        );
        assert_eq!(
            normalized.Merged(),
            1,
            "the mention that was folded in was not counted, so the number does not say how much \
             of the input was decided to be a duplicate"
        );
    }

    /// Two passages naming one concept: one content identity, two mentions, so that a merge is a
    /// number this fixture can state rather than a difference between two fixtures.
    ///
    /// Both tests ask for it -- the first to assert that it merges, the second to assert that
    /// merging costs no claim -- so it belongs under neither and sits here.
    fn Mentions_Of_One_Concept() -> Vec<Extraction>
    {
        return vec![
            An_Extraction("entropy", "It is non-decreasing."),
            An_Extraction("entropy", "It is extensive."),
        ];
    }
}
