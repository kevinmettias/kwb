//! Versioned knowledge, and two reads that cannot be confused for one another.

use std::collections::hash_map::RandomState;

use kwb_model::ContentIdentity;
use kwb_platform_xvpe::VersionedMap;

use crate::Assertion;
use crate::Claim;
use crate::Concept;
use crate::Standing;

/// A map from an address to what is held there, whose previous versions stay valid.
type Held<Value> = VersionedMap<ContentIdentity, Versioned<Value>, RandomState>;

/// Anything the graph holds, together with where it stands.
///
/// One type for concepts, claims and assertions, because standing means the same thing for
/// all three, and a second copy of that idea would be a second place for it to drift.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Versioned<Value>
{
    value: Value,
    standing: Standing,
}

impl<Value> Versioned<Value>
{
    /// As asserted.
    #[must_use]
    pub const fn Asserted(value: Value) -> Self
    {
        return Self {
            value,
            standing: Standing::Asserted,
        };
    }

    /// What is held.
    #[must_use]
    pub const fn Value(&self) -> &Value
    {
        return &self.value;
    }

    /// Where it stands.
    #[must_use]
    pub const fn Standing(&self) -> Standing
    {
        return self.standing;
    }
}

impl<Value: Clone> Versioned<Value>
{
    /// The same thing, closed.
    #[must_use]
    pub fn Closed(&self, standing: Standing) -> Self
    {
        return Self {
            value: self.value.clone(),
            standing,
        };
    }
}

/// Every version of everything, as one value.
///
/// # A version is a value, which is why there is no filter to forget
///
/// Publishing returns a **new graph** sharing most of its structure with this one; this one is
/// unchanged and stays queryable. So *the current graph* and *the graph before that merge* are
/// two values a caller holds, not two views over one mutable store.
///
/// `D19-B` is the incident that makes this a requirement rather than a nicety. A global query
/// filter rewrote every query, including ones written by someone who had never heard of it, and
/// `merge-audit` consequently resolved **none** of the merge log's identifiers, printed
/// *"nothing has been merged away"* and exited `0`. The prototype's answer was two interfaces
/// and a discipline about which one you depend on. A persistent structure makes the discipline
/// unnecessary: there is no filter to bypass, and no flag to forget.
///
/// The mechanism is adopted rather than written — `D-012`, through `kwb-platform-xvpe`.
#[derive(Clone, Debug, Default)]
pub struct KnowledgeGraph
{
    concepts: Option<Held<Concept>>,
    claims: Option<Held<Claim>>,
    assertions: Option<Held<Assertion>>,
}

impl KnowledgeGraph
{
    /// A graph holding nothing.
    #[must_use]
    pub fn Empty() -> Self
    {
        return Self::default();
    }

    /// Publish a concept, returning the new graph and leaving this one valid.
    #[must_use]
    pub fn With_Concept(&self, held: Versioned<Concept>) -> Self
    {
        let identity = held.Value().Identity();
        return Self {
            concepts: Some(Inserted(self.concepts.as_ref(), identity, held)),
            claims: self.claims.clone(),
            assertions: self.assertions.clone(),
        };
    }

    /// Publish a claim.
    #[must_use]
    pub fn With_Claim(&self, held: Versioned<Claim>) -> Self
    {
        let identity = held.Value().Identity();
        return Self {
            concepts: self.concepts.clone(),
            claims: Some(Inserted(self.claims.as_ref(), identity, held)),
            assertions: self.assertions.clone(),
        };
    }

    /// Publish an assertion.
    #[must_use]
    pub fn With_Assertion(&self, held: Versioned<Assertion>) -> Self
    {
        let identity = held.Value().Identity();
        return Self {
            concepts: self.concepts.clone(),
            claims: self.claims.clone(),
            assertions: Some(Inserted(self.assertions.as_ref(), identity, held)),
        };
    }

    /// What is current.
    ///
    /// A **different type** from [`Every_Version`], not the same type with a flag. A caller
    /// holding this one cannot reach a merge loser by forgetting anything, and a caller that
    /// needs merge losers has had to name the other type — visibly, at the call site, which is
    /// what `D19-B` means by *"say which world you read"*.
    ///
    /// [`Every_Version`]: Self::Every_Version
    #[must_use]
    pub const fn Current(&self) -> CurrentKnowledge<'_>
    {
        return CurrentKnowledge { graph: self };
    }

    /// Everything, including what is closed.
    #[must_use]
    pub const fn Every_Version(&self) -> EveryVersion<'_>
    {
        return EveryVersion { graph: self };
    }

    /// Whether a concept's standing is current. Absent counts as not current.
    fn Concept_Is_Current(&self, identity: ContentIdentity) -> bool
    {
        return self
            .concepts
            .as_ref()
            .and_then(|held| return held.Get(&identity))
            .is_some_and(|held| return held.Standing().Is_Current());
    }
}

/// One insertion into a map that may not exist yet.
fn Inserted<Value: Clone>(
    held: Option<&Held<Value>>,
    identity: ContentIdentity,
    value: Versioned<Value>,
) -> Held<Value>
{
    return match held
    {
        Some(held) => held.Insert(identity, value),
        None => Held::<Value>::with_hasher(RandomState::new()).Insert(identity, value),
    };
}

/// Everything a map holds, sorted by address.
///
/// Sorted rather than taken in the map's own order. The map is a hash trie, so its iteration
/// order is the hasher's business — and a listing that changed between runs would make
/// anything derived from it change for no reason anybody could account for. Determinism is
/// enforced here rather than obtained by picking a hasher, so no later change of hasher can
/// take it away.
fn Ordered<Value: Clone>(held: Option<&Held<Value>>) -> Vec<&Versioned<Value>>
{
    let Some(held) = held
    else
    {
        return Vec::new();
    };

    let mut entries: Vec<(ContentIdentity, &Versioned<Value>)> = held
        .iter()
        .map(|(identity, value)| return (*identity, value))
        .collect();
    entries.sort_by_key(|(identity, _)| return *identity);
    return entries.into_iter().map(|(_, value)| return value).collect();
}

/// The current read.
#[derive(Clone, Copy, Debug)]
pub struct CurrentKnowledge<'graph>
{
    graph: &'graph KnowledgeGraph,
}

impl<'graph> CurrentKnowledge<'graph>
{
    /// The concepts that are current.
    #[must_use]
    pub fn Concepts(&self) -> Vec<&'graph Concept>
    {
        return Ordered(self.graph.concepts.as_ref())
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
        return Ordered(self.graph.claims.as_ref())
            .into_iter()
            .filter(|held| {
                return held.Standing().Is_Current()
                    && self.graph.Concept_Is_Current(held.Value().Concept());
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

        return Ordered(self.graph.assertions.as_ref())
            .into_iter()
            .filter(|held| {
                return held.Standing().Is_Current() && current.contains(&held.Value().Claim());
            })
            .map(Versioned::Value)
            .collect();
    }
}

/// The all-versions read.
#[derive(Clone, Copy, Debug)]
pub struct EveryVersion<'graph>
{
    graph: &'graph KnowledgeGraph,
}

impl<'graph> EveryVersion<'graph>
{
    /// Every concept held, whatever its standing.
    #[must_use]
    pub fn Concepts(&self) -> Vec<&'graph Versioned<Concept>>
    {
        return Ordered(self.graph.concepts.as_ref());
    }

    /// Every claim held, whatever its standing and whatever its concept's.
    #[must_use]
    pub fn Claims(&self) -> Vec<&'graph Versioned<Claim>>
    {
        return Ordered(self.graph.claims.as_ref());
    }

    /// Every assertion held.
    #[must_use]
    pub fn Assertions(&self) -> Vec<&'graph Versioned<Assertion>>
    {
        return Ordered(self.graph.assertions.as_ref());
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
