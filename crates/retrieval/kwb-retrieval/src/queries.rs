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

/// One claim of a held neighbourhood, and whether it is current **in the composed sense**.
///
/// # Why the flag is here and not read off the claim's own standing
///
/// A claim is current when its own standing is current **and its concept's is** — that is what
/// `CurrentKnowledge::Claims` composes, and it is the whole reason a merge closes the knowledge
/// under it rather than orphaning it.
///
/// Asking only `held.Standing().Is_Current()` gives the wrong answer for exactly the case this
/// type exists for: a claim under a superseded concept reports itself live. That was rendered
/// once, during `KWB-65`, and caught by reading the output rather than the types — the tool
/// built to end `D19-B`'s confusion reproducing it on its first run.
///
/// So the flag is computed by **asking the graph which claims are current**, not by restating
/// the rule. `Standing::Is_Current` stays the one liveness expression and this consumes it
/// through the composition that already exists.
#[derive(Debug)]
pub struct HeldClaim<'graph>
{
    /// The claim and what became of it.
    pub held: &'graph Versioned<Claim>,

    /// Whether it is current, concept and all.
    pub current: bool,
}

/// One assertion of a held neighbourhood, and whether it is current in the composed sense.
///
/// The same composition as [`HeldClaim`], one level further: *an assertion of a claim that is
/// not current is not a current assertion*, which is how the graph puts it. A citation of a
/// claim whose concept was merged away is not evidence for anything live, and a listing that
/// said otherwise would be the citation resolving perfectly to something retired.
#[derive(Debug)]
pub struct HeldAssertion<'graph>
{
    /// The assertion and what became of it.
    pub held: &'graph Versioned<Assertion>,

    /// Whether it is current, claim and concept and all.
    pub current: bool,
}

/// Everything ever attached to one concept, each with the standing it holds.
///
/// # Why this is a second type and not [`Neighbourhood`] with a flag
///
/// `D-012` decided that the two worlds are distinct *values* rather than one type a caller could
/// ask the wrong question of, and `KWB-6` asked for the same of the read surface. The difference
/// is not decoration: everything here is a [`Versioned`], because in this world a claim can be
/// retired and a reader who could not tell a live claim from a closed one would have exactly the
/// confusion `D19-B` is about — `merge-audit` asked the historical question of the current graph
/// and was told nothing had been merged away.
///
/// So the standing travels with every part. A caller cannot render one of these without having
/// been handed what it would need to say so.
#[derive(Debug)]
pub struct HeldNeighbourhood<'graph>
{
    /// The concept at the centre, and what became of it.
    pub concept: &'graph Versioned<Concept>,

    /// Every claim made about it, live or closed, each saying which it is.
    pub claims: Vec<HeldClaim<'graph>>,

    /// Every assertion of those claims, each saying whether it is current.
    pub assertions: Vec<HeldAssertion<'graph>>,
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

    /// Everything ever attached to one concept, whatever became of it.
    ///
    /// # The question a merge audit reaches second
    ///
    /// `Merge_Losers` says *which* concept was closed and what authorised it. This says what it
    /// carried. Measured before `KWB-65` existed: `merge_losers` reported `phlogiston` against
    /// its successor and a reason, and nothing could reach the claim it held — the claim was in
    /// the graph, because `Versioned` keeps it, and no query could name it.
    ///
    /// `D17` is that destruction requires evidence. Evidence a reader cannot reach is the shape
    /// of the prototype's `AdmitChunk` path disarmed by a configuration default: an obligation
    /// that looks handled because nothing complains.
    #[must_use]
    pub fn Neighbourhood_Of(&self, concept: ContentIdentity) -> Option<HeldNeighbourhood<'graph>>
    {
        let every = self.graph.Every_Version();
        let found = every
            .Concepts()
            .into_iter()
            .find(|held| return held.Value().Identity() == concept)?;

        // The claims the graph itself calls current, composed the way it composes them. Asked
        // rather than re-derived: a second liveness rule here would be the half-enforced index
        // `D-012` describes, and it would be wrong in exactly the case this query is for.
        let live: Vec<ContentIdentity> = self
            .graph
            .Current()
            .Claims()
            .into_iter()
            .map(Claim::Identity)
            .collect();

        let claims: Vec<HeldClaim<'graph>> = every
            .Claims()
            .into_iter()
            .filter(|held| return held.Value().Concept() == concept)
            .map(|held| {
                return HeldClaim {
                    held,
                    current: live.contains(&held.Value().Identity()),
                };
            })
            .collect();
        let addresses: Vec<ContentIdentity> = claims
            .iter()
            .map(|claim| return claim.held.Value().Identity())
            .collect();
        // Asked of the graph, like the claims above, and for the same reason.
        let cited: Vec<ContentIdentity> = self
            .graph
            .Current()
            .Assertions()
            .into_iter()
            .map(Assertion::Identity)
            .collect();

        let assertions = every
            .Assertions()
            .into_iter()
            .filter(|held| return addresses.contains(&held.Value().Claim()))
            .map(|held| {
                return HeldAssertion {
                    held,
                    current: cited.contains(&held.Value().Identity()),
                };
            })
            .collect();

        return Some(HeldNeighbourhood {
            concept: found,
            claims,
            assertions,
        });
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
