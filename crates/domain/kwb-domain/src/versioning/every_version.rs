//! The all-versions read over a graph.
//!
//! A **different type** from [`CurrentKnowledge`], and naming it is what `D19-B` asks of a
//! caller: reaching a merge loser means naming a reader whose name says so, rather than
//! dropping a filter that was silently rewriting every query in the module.
//!
//! [`CurrentKnowledge`]: crate::CurrentKnowledge

use crate::graph::knowledge_graph::Ordered_Entries;
use crate::Assertion;
use crate::Claim;
use crate::Concept;
use crate::KnowledgeGraph;
use crate::Versioned;

/// The all-versions read.
#[derive(Clone, Copy, Debug)]
pub struct EveryVersion<'graph>
{
    graph: &'graph KnowledgeGraph,
}

impl<'graph> EveryVersion<'graph>
{
    /// The all-versions read over a graph.
    ///
    /// Crate-private, and handed out by [`KnowledgeGraph::Every_Version`] for the reason
    /// [`CurrentKnowledge::Of`] states about the other reader.
    ///
    /// [`KnowledgeGraph::Every_Version`]: crate::KnowledgeGraph::Every_Version
    /// [`CurrentKnowledge::Of`]: crate::CurrentKnowledge
    pub(crate) const fn Of(graph: &'graph KnowledgeGraph) -> Self
    {
        return Self { graph };
    }

    /// Every concept held, whatever its standing.
    #[must_use]
    pub fn Concepts(&self) -> Vec<&'graph Versioned<Concept>>
    {
        return Ordered_Entries(self.graph.Held_Concepts());
    }

    /// Every claim held, whatever its standing and whatever its concept's.
    #[must_use]
    pub fn Claims(&self) -> Vec<&'graph Versioned<Claim>>
    {
        return Ordered_Entries(self.graph.Held_Claims());
    }

    /// Every assertion held.
    #[must_use]
    pub fn Assertions(&self) -> Vec<&'graph Versioned<Assertion>>
    {
        return Ordered_Entries(self.graph.Held_Assertions());
    }

    /// Every concept closed against a successor.
    ///
    /// The query `merge-audit` needed and could not ask. An audit needs an independent
    /// expectation, and this is the half the graph can supply.
    #[must_use]
    pub fn Merge_Losers(&self) -> Vec<&'graph Versioned<Concept>>
    {
        return self
            .Concepts()
            .into_iter()
            .filter(|held| return held.Standing().Superseded_By().is_some())
            .collect();
    }
}

#[cfg(test)]
mod tests
{
    //! [`Of`], which no caller outside this crate can reach.
    //!
    //! A file under `tests/` compiles as its own package and can see the public surface only, and
    //! the unit a test belongs to is the stem of the file it sits in. This file's stem is
    //! `every_version`, so the reader's constructor is tested here; the four reads it exposes are
    //! asserted from outside, in `tests/every_version.rs`.
    //!
    //! [`Of`]: EveryVersion::Of

    use super::*;

    #[test]
    fn Test_Of_Should_Take_The_Graph_It_Walks()
    {
        let concept = Concept::Named("entropy");
        let holding = KnowledgeGraph::Empty().With_Concept(Versioned::Asserted(concept));

        assert_eq!(
            EveryVersion::Of(&holding).Concepts().len(),
            1,
            "the reader does not see the graph it was handed"
        );
        assert!(
            EveryVersion::Of(&KnowledgeGraph::Empty()).Merge_Losers().is_empty(),
            "the reader finds a merge loser in a graph holding nothing, so it is walking a graph \
             other than the one it was handed"
        );
    }
}
