//! Versioned concept state, and two reads that cannot be confused for one another.

use std::collections::hash_map::RandomState;

use kwb_model::ContentIdentity;
use kwb_platform_xvpe::VersionedMap;

use crate::Concept;
use crate::Standing;

/// A concept together with where it stands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConceptRecord
{
    concept: Concept,
    standing: Standing,
}

impl ConceptRecord
{
    /// A concept as asserted.
    #[must_use]
    pub const fn Asserted(concept: Concept) -> Self
    {
        return Self {
            concept,
            standing: Standing::Asserted,
        };
    }

    /// The same concept, closed.
    #[must_use]
    pub fn Closed(&self, standing: Standing) -> Self
    {
        return Self {
            concept: self.concept.clone(),
            standing,
        };
    }

    /// The concept.
    #[must_use]
    pub const fn Concept(&self) -> &Concept
    {
        return &self.concept;
    }

    /// Where it stands.
    #[must_use]
    pub const fn Standing(&self) -> Standing
    {
        return self.standing;
    }
}

/// Every version of every concept, as one value.
///
/// # A version is a value, which is why there is no filter to forget
///
/// [`Publish`] returns a **new graph** sharing most of its structure with this one; this one
/// is unchanged and stays queryable. So *the current graph* and *the graph before that merge*
/// are two values a caller holds, not two views over one mutable store.
///
/// `D19-B` is the incident that makes this the requirement rather than a nicety. A global
/// query filter rewrote every query, including ones written by someone who had never heard of
/// it, and `merge-audit` consequently resolved **none** of the merge log's identifiers,
/// printed *"nothing has been merged away"* and exited `0`. The prototype's answer was two
/// interfaces and a discipline about which you depend on. A persistent structure makes the
/// discipline unnecessary: there is no filter to bypass, and no flag to forget.
///
/// The mechanism is adopted rather than written — `D-012`, through `kwb-platform-xvpe`, which
/// is the only crate here permitted to name XVPE.
///
/// [`Publish`]: Self::Publish
#[derive(Clone, Debug)]
pub struct ConceptGraph
{
    records: VersionedMap<ContentIdentity, ConceptRecord, RandomState>,
}

impl ConceptGraph
{
    /// A graph holding nothing.
    #[must_use]
    pub fn Empty() -> Self
    {
        return Self {
            records: VersionedMap::with_hasher(RandomState::new()),
        };
    }

    /// Publish a record, returning the new graph and leaving this one valid.
    ///
    /// There is no in-place edit and no delete, so `D-008`'s atomicity requirement has nothing
    /// to guard: a new version exists or it does not. `ReplaceAllAsync` deleting
    /// unconditionally and writing conditionally is not a mistake this shape can make.
    #[must_use]
    pub fn Publish(&self, record: ConceptRecord) -> Self
    {
        let identity = record.Concept().Identity();
        return Self {
            records: self.records.Insert(identity, record),
        };
    }

    /// The concepts that are current.
    ///
    /// A **different type** from [`Every_Version`], not the same type with a flag. A caller
    /// holding this one cannot reach a merge loser by forgetting anything, and a caller that
    /// needs merge losers has had to name the other type — visibly, at the call site, which is
    /// what `D19-B` says *"say which world you read"* has to mean.
    ///
    /// [`Every_Version`]: Self::Every_Version
    #[must_use]
    pub const fn Current(&self) -> CurrentConcepts<'_>
    {
        return CurrentConcepts { graph: self };
    }

    /// Every version, including the ones that are closed.
    #[must_use]
    pub const fn Every_Version(&self) -> EveryVersion<'_>
    {
        return EveryVersion { graph: self };
    }

    /// Every record this graph holds, in a deterministic order.
    ///
    /// Sorted by identity rather than taken in the map's own order. The map is a hash trie, so
    /// its iteration order is the hasher's business — and a listing that changed between runs
    /// would make anything derived from it change for no reason anybody could account for.
    /// Determinism is enforced here rather than obtained by picking a hasher, so no later
    /// change of hasher can take it away.
    fn Ordered(&self) -> Vec<&ConceptRecord>
    {
        let mut records: Vec<&ConceptRecord> = self.records.Values().collect();
        records.sort_by_key(|record| return record.Concept().Identity());
        return records;
    }
}

/// The current-concepts read.
#[derive(Clone, Copy, Debug)]
pub struct CurrentConcepts<'graph>
{
    graph: &'graph ConceptGraph,
}

impl<'graph> CurrentConcepts<'graph>
{
    /// The record at this address, if it is current.
    ///
    /// A closed concept reads as absent here. That is the world this type is, and the reason
    /// it is a separate type: absent-because-closed and absent-because-never-written are the
    /// same answer *in this world*, and a caller that needs to tell them apart is asking
    /// [`ConceptGraph::Every_Version`] a question.
    #[must_use]
    pub fn Get(&self, identity: ContentIdentity) -> Option<&'graph ConceptRecord>
    {
        return self
            .graph
            .records
            .Get(&identity)
            .filter(|record| return record.Standing().Is_Current());
    }

    /// Every current record, in a deterministic order.
    #[must_use]
    pub fn Records(&self) -> Vec<&'graph ConceptRecord>
    {
        return self
            .graph
            .Ordered()
            .into_iter()
            .filter(|record| return record.Standing().Is_Current())
            .collect();
    }

    /// How many concepts are current.
    #[must_use]
    pub fn Length(&self) -> usize
    {
        return self.Records().len();
    }
}

/// The all-versions read.
#[derive(Clone, Copy, Debug)]
pub struct EveryVersion<'graph>
{
    graph: &'graph ConceptGraph,
}

impl<'graph> EveryVersion<'graph>
{
    /// The record at this address, whatever its standing.
    #[must_use]
    pub fn Get(&self, identity: ContentIdentity) -> Option<&'graph ConceptRecord>
    {
        return self.graph.records.Get(&identity);
    }

    /// Every record, in a deterministic order.
    #[must_use]
    pub fn Records(&self) -> Vec<&'graph ConceptRecord>
    {
        return self.graph.Ordered();
    }

    /// How many records the graph holds.
    #[must_use]
    pub fn Length(&self) -> usize
    {
        return self.graph.records.Length();
    }

    /// Every record closed against a successor, in a deterministic order.
    ///
    /// The query `merge-audit` needed and could not ask. An audit needs an independent
    /// expectation, and this is the half of it the graph can supply.
    #[must_use]
    pub fn Merge_Losers(&self) -> Vec<&'graph ConceptRecord>
    {
        return self
            .graph
            .Ordered()
            .into_iter()
            .filter(|record| return record.Standing().Superseded_By().is_some())
            .collect();
    }
}
