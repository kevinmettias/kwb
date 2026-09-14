//! The same questions, asked of every version.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::EveryVersion;
use kwb_domain::KnowledgeGraph;
use kwb_domain::Versioned;
use kwb_model::ContentIdentity;

use crate::matching::Contains_All;
use crate::matching::Words_Of;
use crate::HeldAssertion;
use crate::HeldClaim;
use crate::HeldNeighbourhood;

/// The same questions, asked of every version.
///
/// Separate from [`CurrentQueries`] for the reason that type documents, and carrying the one
/// question the current world cannot answer: what was merged away.
///
/// [`CurrentQueries`]: crate::CurrentQueries
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

        let live = self.Live_Claim_Identities();
        let cited = self.Live_Assertion_Identities();

        let claims = Held_Claims_About(every, concept, &live);
        let assertions = Held_Assertions_Citing(every, &claims, &cited);

        return Some(HeldNeighbourhood {
            concept: found,
            claims,
            assertions,
        });
    }

    /// The claims the graph itself calls current, composed the way it composes them.
    ///
    /// Asked rather than re-derived: a second liveness rule here would be the half-enforced
    /// index `D-012` describes, and it would be wrong in exactly the case this query is for.
    ///
    /// Taken by value because this type is one reference and is `Copy`: asking a question of a
    /// graph should not need the asker to still be there afterwards.
    #[must_use]
    fn Live_Claim_Identities(self) -> Vec<ContentIdentity>
    {
        return self
            .graph
            .Current()
            .Claims()
            .into_iter()
            .map(Claim::Identity)
            .collect();
    }

    /// The assertions the graph itself calls current.
    ///
    /// Asked of the graph, like the claims, and for the same reason.
    #[must_use]
    fn Live_Assertion_Identities(self) -> Vec<ContentIdentity>
    {
        return self
            .graph
            .Current()
            .Assertions()
            .into_iter()
            .map(Assertion::Identity)
            .collect();
    }

    /// How many concepts are held, current or not.
    #[must_use]
    pub fn Concept_Count(&self) -> usize
    {
        return self.graph.Every_Version().Concepts().len();
    }
}

/// Every claim ever made about one concept, each told whether the graph calls it current.
///
/// `live` is passed in rather than derived here because the graph is the only thing that
/// decides what is current. A liveness rule restated in this function would be the second one
/// `D-012` is about, and the two would part company at the first merge.
fn Held_Claims_About<'graph>(
    every: EveryVersion<'graph>,
    concept: ContentIdentity,
    live: &[ContentIdentity],
) -> Vec<HeldClaim<'graph>>
{
    return every
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
}

/// Every assertion ever made of these claims, each told whether the graph calls it current.
///
/// Reached through `claims` rather than through the concept, because an assertion cites a
/// claim and names no concept of its own. `cited` is passed in for the reason `live` is.
fn Held_Assertions_Citing<'graph>(
    every: EveryVersion<'graph>,
    claims: &[HeldClaim<'graph>],
    cited: &[ContentIdentity],
) -> Vec<HeldAssertion<'graph>>
{
    let addresses: Vec<ContentIdentity> = claims
        .iter()
        .map(|claim| return claim.held.Value().Identity())
        .collect();
    return every
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
}
