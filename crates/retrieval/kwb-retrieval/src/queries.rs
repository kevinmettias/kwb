//! Two query surfaces, one per world, neither of which can write.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::KnowledgeGraph;
use kwb_domain::Versioned;
use kwb_model::ContentIdentity;
use kwb_model::Normalize;

/// Everything known about one concept: the concept, its claims, and who asserted them.
///
/// A **neighbourhood** in the structural sense — concept to claim to assertion — and not in
/// the relation sense. There are no typed relations between concepts in this workspace yet,
/// and `D-011` is why: the universal relation vocabulary is held, is not mutually exclusive,
/// and cannot yet commit a kind to transitivity in both directions. Traversing edges that do
/// not exist would be inventing the thing that is held.
#[derive(Clone, Debug)]
pub struct Neighbourhood<'graph>
{
    /// The concept at the centre.
    pub concept: &'graph Concept,

    /// The claims made about it.
    pub claims: Vec<&'graph Claim>,

    /// The assertions of those claims.
    pub assertions: Vec<&'graph Assertion>,
}

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

        let claims: Vec<&Claim> = current
            .Claims()
            .into_iter()
            .filter(|claim| return claim.Concept() == concept)
            .collect();
        let addresses: Vec<ContentIdentity> =
            claims.iter().map(|claim| return claim.Identity()).collect();
        let assertions = current
            .Assertions()
            .into_iter()
            .filter(|assertion| return addresses.contains(&assertion.Claim()))
            .collect();

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

/// The same questions, asked of every version.
///
/// Separate from [`CurrentQueries`] for the reason that type documents, and carrying the one
/// question the current world cannot answer: what was merged away.
#[derive(Clone, Copy, Debug)]
pub struct HistoricalQueries<'graph>
{
    graph: &'graph KnowledgeGraph,
}

impl<'graph> HistoricalQueries<'graph>
{
    /// Ask every version.
    #[must_use]
    pub const fn Over(graph: &'graph KnowledgeGraph) -> Self
    {
        return Self { graph };
    }

    /// Every claim whose text contains every word of the query, whatever its standing.
    #[must_use]
    pub fn Claims_Matching(&self, query: &str) -> Vec<&'graph Versioned<Claim>>
    {
        let words = Words_Of(query);
        if words.is_empty()
        {
            return Vec::new();
        }

        return self
            .graph
            .Every_Version()
            .Claims()
            .into_iter()
            .filter(|held| return Contains_All(held.Value().Text(), &words))
            .collect();
    }

    /// Every concept closed against a successor.
    ///
    /// The query `merge-audit` needed and asked of the wrong world. An audit needs an
    /// independent expectation, and this is the half retrieval can supply.
    #[must_use]
    pub fn Merge_Losers(&self) -> Vec<&'graph Versioned<Concept>>
    {
        return self.graph.Every_Version().Merge_Losers();
    }

    /// How many concepts are held, current or not.
    #[must_use]
    pub fn Concept_Count(&self) -> usize
    {
        return self.graph.Every_Version().Concepts().len();
    }
}

/// The query's words, normalized the way a claim's own text was.
fn Words_Of(query: &str) -> Vec<String>
{
    return Normalize(query)
        .split_whitespace()
        .map(str::to_owned)
        .collect();
}

/// Whether `text` contains every word, compared after the same normalization.
fn Contains_All(text: &str, words: &[String]) -> bool
{
    let normalized = Normalize(text);
    return words.iter().all(|word| return normalized.contains(word.as_str()));
}
