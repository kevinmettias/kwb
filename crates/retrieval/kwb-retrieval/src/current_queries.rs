//! Questions asked of the current world.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::CurrentKnowledge;
use kwb_domain::KnowledgeGraph;
use kwb_model::ContentIdentity;

use crate::matching::Contains_All;
use crate::matching::Words_Of;
use crate::Neighbourhood;

/// Questions asked of the current world.
///
/// # This type cannot write, and that is checked rather than promised
///
/// The prototype's `D10` excluded mutation from its read surface architecturally rather than
/// by convention, and `KWB-6` asks for the same: *the type a tool handler is built against
/// should not have a mutating method to call by mistake.* Nothing here takes `&mut self` and
/// `tests/read_only.rs` asserts that against this crate's own source, so a mutating method
/// added later fails a test rather than passing review.
///
/// # Why there is a second type rather than a flag
///
/// [`HistoricalQueries`] answers the same questions of every version. They are separate types
/// because `D19-B` is what a forgettable filter costs: `merge-audit` asked the current world a
/// question about merge losers, resolved none, printed *"nothing has been merged away"* and
/// exited `0`. A caller here has had to name the world it wants, at the call site, and cannot
/// arrive in the wrong one by omitting an argument — there is no argument to omit.
///
/// [`HistoricalQueries`]: crate::HistoricalQueries
#[derive(Clone, Copy, Debug)]
pub struct CurrentQueries<'graph>
{
    graph: &'graph KnowledgeGraph,
}

impl<'graph> CurrentQueries<'graph>
{
    /// Ask the current world.
    #[must_use]
    pub const fn Over(graph: &'graph KnowledgeGraph) -> Self
    {
        return Self { graph };
    }

    /// Claims whose text contains every word of the query.
    ///
    /// Words rather than a substring, so that word order and spacing do not decide a match,
    /// and normalized through `kwb-model` so the query is compared the way the claim's own
    /// identity was derived. **Case is significant**, consistently with everything else here:
    /// `BVH` and `bvh` are different text, and folding case in one place and not another is
    /// how two searches come to disagree.
    ///
    /// Deliberately not ranked. `D-008`'s seventh requirement is that ranked retrieval ranks
    /// from a **stored** vector and never derives one per query — the prototype measured 38 ms
    /// against 1.3 ms for the derived form, on byte-identical rows, so no correctness test
    /// could ever have seen it. There is no stored vector here, so there is no ranking here.
    #[must_use]
    pub fn Claims_Matching(&self, query: &str) -> Vec<&'graph Claim>
    {
        let words = Words_Of(query);
        if words.is_empty()
        {
            return Vec::new();
        }

        return self
            .graph
            .Current()
            .Claims()
            .into_iter()
            .filter(|claim| return Contains_All(claim.Text(), &words))
            .collect();
    }

    /// Concepts whose canonical name contains every word of the query.
    #[must_use]
    pub fn Concepts_Matching(&self, query: &str) -> Vec<&'graph Concept>
    {
        let words = Words_Of(query);
        if words.is_empty()
        {
            return Vec::new();
        }

        return self
            .graph
            .Current()
            .Concepts()
            .into_iter()
            .filter(|concept| return Contains_All(concept.Canonical_Name(), &words))
            .collect();
    }

    /// Everything current that is attached to one concept.
    ///
    /// Absent when the concept is not current — which includes a merge loser. A caller that
    /// wants a retired concept's neighbourhood is asking the other world, and has to say so.
    #[must_use]
    pub fn Neighbourhood_Of(&self, concept: ContentIdentity) -> Option<Neighbourhood<'graph>>
    {
        let current = self.graph.Current();
        let found = current
            .Concepts()
            .into_iter()
            .find(|held| return held.Identity() == concept)?;

        let claims = Claims_About(current, concept);
        let assertions = Assertions_Citing(current, &claims);

        return Some(Neighbourhood {
            concept: found,
            claims,
            assertions,
        });
    }

    /// How many concepts are current.
    #[must_use]
    pub fn Concept_Count(&self) -> usize
    {
        return self.graph.Current().Concepts().len();
    }
}

/// The current claims made about one concept.
///
/// A neighbourhood is reached in two hops, and this is the first of them. An assertion cites a
/// claim rather than a concept, so the claims have to be found before anything can say which
/// assertions are attached.
fn Claims_About(current: CurrentKnowledge<'_>, concept: ContentIdentity) -> Vec<&Claim>
{
    return current
        .Claims()
        .into_iter()
        .filter(|claim| return claim.Concept() == concept)
        .collect();
}

/// The current assertions citing any of `claims`.
///
/// The second hop, and the one that has to go *through* the claims: an assertion names no
/// concept of its own, so the citation is the whole of what attaches it to a neighbourhood.
/// Filtering assertions by concept instead would find none of them.
fn Assertions_Citing<'graph>(
    current: CurrentKnowledge<'graph>,
    claims: &[&'graph Claim],
) -> Vec<&'graph Assertion>
{
    let addresses: Vec<ContentIdentity> =
        claims.iter().map(|claim| return claim.Identity()).collect();
    return current
        .Assertions()
        .into_iter()
        .filter(|assertion| return addresses.contains(&assertion.Claim()))
        .collect();
}
