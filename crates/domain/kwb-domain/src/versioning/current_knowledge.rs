//! The current read over a graph.
//!
//! A **different type** from [`EveryVersion`], not the same type with a flag. A caller holding
//! this one cannot reach a merge loser by forgetting anything, and a caller that needs merge
//! losers has had to name the other type — visibly, at the call site, which is what `D19-B`
//! means by *"say which world you read"*.
//!
//! [`EveryVersion`]: crate::EveryVersion

use kwb_model::ContentIdentity;

use crate::graph::knowledge_graph::Ordered_Entries;
use crate::Assertion;
use crate::Claim;
use crate::Concept;
use crate::KnowledgeGraph;
use crate::Versioned;

/// The current read.
#[derive(Clone, Copy, Debug)]
pub struct CurrentKnowledge<'graph>
{
    graph: &'graph KnowledgeGraph,
}

impl<'graph> CurrentKnowledge<'graph>
{
    /// The current read over a graph.
    ///
    /// Crate-private, and handed out by [`KnowledgeGraph::Current`] rather than constructed by
    /// a caller: the graph being read is the graph, and a reader assembled against a different
    /// one would be a second answer to a question already answered.
    ///
    /// [`KnowledgeGraph::Current`]: crate::KnowledgeGraph::Current
    pub(crate) const fn Of(graph: &'graph KnowledgeGraph) -> Self
    {
        return Self { graph };
    }

    /// The concepts that are current.
    #[must_use]
    pub fn Concepts(&self) -> Vec<&'graph Concept>
    {
        return Ordered_Entries(self.graph.Held_Concepts())
            .into_iter()
            .filter(|held| return held.Standing().Is_Current())
            .map(Versioned::Value)
            .collect();
    }

    /// The claims that are current.
    ///
    /// A claim is current when its own standing is current **and its concept is**. That is one
    /// liveness rule applied twice rather than a second rule: a claim about a concept that was
    /// retired is not a current claim, and nothing else in this workspace would have said so.
    #[must_use]
    pub fn Claims(&self) -> Vec<&'graph Claim>
    {
        return Ordered_Entries(self.graph.Held_Claims())
            .into_iter()
            .filter(|held| {
                return held.Standing().Is_Current()
                    && self.graph.Is_Concept_Current(held.Value().Concept());
            })
            .map(Versioned::Value)
            .collect();
    }

    /// The assertions that are current, by the same composition.
    ///
    /// An assertion of a claim that is not current is not a current assertion. Otherwise a
    /// retired concept's claims would keep accumulating citations that nothing could see were
    /// attached to retired knowledge.
    #[must_use]
    pub fn Assertions(&self) -> Vec<&'graph Assertion>
    {
        let current: Vec<ContentIdentity> = self
            .Claims()
            .iter()
            .map(|claim| return claim.Identity())
            .collect();

        return Ordered_Entries(self.graph.Held_Assertions())
            .into_iter()
            .filter(|held| {
                return held.Standing().Is_Current() && current.contains(&held.Value().Claim());
            })
            .map(Versioned::Value)
            .collect();
    }
}
