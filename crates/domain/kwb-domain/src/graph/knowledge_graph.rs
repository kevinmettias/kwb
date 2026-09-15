//! Versioned knowledge, and the two reads it hands out.
//!
//! The reads are different types rather than one type with a flag — [`CurrentKnowledge`] and
//! [`EveryVersion`] — because `D19-B` is what a forgettable filter costs.
//!
//! [`CurrentKnowledge`]: crate::CurrentKnowledge
//! [`EveryVersion`]: crate::EveryVersion

use std::collections::hash_map::RandomState;

use kwb_model::ContentIdentity;
use kwb_platform_xvpe::VersionedMap;

use crate::Assertion;
use crate::Claim;
use crate::Concept;
use crate::CurrentKnowledge;
use crate::EveryVersion;
use crate::Versioned;

/// A map from an address to what is held there, whose previous versions stay valid.
///
/// Crate-visible rather than private because the two reads over a graph iterate it, which is
/// the one thing about the map they need and the reason they can be separate files.
pub(crate) type Held<Value> = VersionedMap<ContentIdentity, Versioned<Value>, RandomState>;

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
            concepts: Some(Inserted_Value(self.concepts.as_ref(), identity, held)),
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
            claims: Some(Inserted_Value(self.claims.as_ref(), identity, held)),
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
            assertions: Some(Inserted_Value(self.assertions.as_ref(), identity, held)),
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
        return CurrentKnowledge::Of(self);
    }

    /// Everything, including what is closed.
    #[must_use]
    pub const fn Every_Version(&self) -> EveryVersion<'_>
    {
        return EveryVersion::Of(self);
    }

    /// The map the concepts are held in, for the reads that walk it.
    pub(crate) fn Held_Concepts(&self) -> Option<&Held<Concept>>
    {
        return self.concepts.as_ref();
    }

    /// The map the claims are held in, for the reads that walk it.
    pub(crate) fn Held_Claims(&self) -> Option<&Held<Claim>>
    {
        return self.claims.as_ref();
    }

    /// The map the assertions are held in, for the reads that walk it.
    pub(crate) fn Held_Assertions(&self) -> Option<&Held<Assertion>>
    {
        return self.assertions.as_ref();
    }

    /// Whether a concept's standing is current. Absent counts as not current.
    pub(crate) fn Is_Concept_Current(&self, identity: ContentIdentity) -> bool
    {
        return self
            .concepts
            .as_ref()
            .and_then(|held| return held.Get(&identity))
            .is_some_and(|held| return held.Standing().Is_Current());
    }
}

/// One insertion into a map that may not exist yet.
fn Inserted_Value<Value: Clone>(
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
///
/// Crate-visible because both reads over a graph are sorted by it, and one sorting rule is
/// what keeps them from disagreeing about the order two addresses come in.
pub(crate) fn Ordered_Entries<Value: Clone>(held: Option<&Held<Value>>) -> Vec<&Versioned<Value>>
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
