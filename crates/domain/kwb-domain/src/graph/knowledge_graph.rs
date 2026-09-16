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

#[cfg(test)]
mod tests
{
    //! The five functions above that no caller outside this crate can reach.
    //!
    //! # Why these tests sit here rather than in `tests/knowledge_graph.rs`
    //!
    //! [`Held_Concepts`], [`Held_Claims`], [`Held_Assertions`], [`Is_Concept_Current`] and
    //! [`Ordered_Entries`] are `pub(crate)`, and a file under `tests/` compiles as its own package:
    //! it can see a crate's public surface and nothing else. The unit a test belongs to is the stem
    //! of the file it sits in, and this file's stem is `knowledge_graph`, so the module beside the
    //! functions is where their tests go. The public surface of this file is asserted from outside,
    //! in `tests/knowledge_graph.rs`; this is the half that cannot be.
    //!
    //! # What is worth asserting about the innards
    //!
    //! Three of the five are the maps the reads walk, and *absent* is a case rather than an error: a
    //! graph built by publishing only assertions has never had a concept map, and every read over it
    //! has to answer that without a branch. [`Is_Concept_Current`] is the rule the claim and
    //! assertion reads compose with, and it has to answer *not current* about a concept it has never
    //! been shown -- including one that was published and then closed, which is the case the read
    //! exists to exclude. [`Ordered_Entries`] is the whole of the ordering guarantee this crate
    //! makes: the map is a hash trie, so a listing taken in the map's own order is one that changes
    //! between runs for no reason anybody could account for.

    use super::*;
    use crate::Standing;

    /// Why the concept the fixtures close was withdrawn.
    const BECAUSE: &str = "the concept was withdrawn by its author";

    #[test]
    fn Test_Held_Concepts_Should_Be_Absent_Until_Something_Is_Published()
    {
        assert!(
            KnowledgeGraph::Empty().Held_Concepts().is_none(),
            "a graph that has published nothing hands out a concept map, so `Empty` is not the \
             zero of the fold the three `With_` functions perform"
        );

        let concept = Concept::Named("entropy");
        let graph = KnowledgeGraph::Empty().With_Concept(Versioned::Asserted(concept.clone()));

        let held = graph.Held_Concepts().expect("a published concept has a map to be held in");
        assert!(
            held.Get(&concept.Identity()).is_some(),
            "the concept map does not hold the concept that was published into it"
        );
    }

    #[test]
    fn Test_Held_Claims_Should_Be_Absent_Until_Something_Is_Published()
    {
        // Publishing a concept does not create the claim map: the three maps are separate, and a
        // graph holding one kind of thing hands out `None` for the other two.
        let concept = Concept::Named("entropy");
        let graph = KnowledgeGraph::Empty().With_Concept(Versioned::Asserted(concept.clone()));

        assert!(
            graph.Held_Claims().is_none(),
            "the claim map exists on a graph that has published no claim"
        );

        let claim = Claim::About(&concept, "It is non-decreasing in an isolated system.");
        let graph = graph.With_Claim(Versioned::Asserted(claim.clone()));

        let held = graph.Held_Claims().expect("a published claim has a map to be held in");
        assert!(
            held.Get(&claim.Identity()).is_some(),
            "the claim map does not hold the claim that was published into it"
        );
    }

    #[test]
    fn Test_Held_Assertions_Should_Be_Absent_Until_Something_Is_Published()
    {
        let concept = Concept::Named("entropy");
        let claim = Claim::About(&concept, "It is non-decreasing in an isolated system.");
        let scope = crate::Scope::Named("physical theory").expect("a named scope");

        let graph = KnowledgeGraph::Empty()
            .With_Concept(Versioned::Asserted(concept))
            .With_Claim(Versioned::Asserted(claim.clone()));

        assert!(
            graph.Held_Assertions().is_none(),
            "the assertion map exists on a graph that has published no assertion"
        );

        let assertion = Assertion::By("Callen 1985", &claim, scope);
        let graph = graph.With_Assertion(Versioned::Asserted(assertion.clone()));

        let held = graph.Held_Assertions().expect("a published assertion has a map to be held in");
        assert!(
            held.Get(&assertion.Identity()).is_some(),
            "the assertion map does not hold the assertion that was published into it"
        );
    }

    #[test]
    fn Test_Is_Concept_Current_Should_Be_False_Of_An_Absent_Concept_And_Of_A_Closed_One()
    {
        // Absent counts as not current, and so does closed. The alternative for a claim about a
        // concept nobody ever published is to treat it as current, which is how a claim outlives
        // the thing it is about.
        let concept = Concept::Named("entropy");

        assert!(
            !KnowledgeGraph::Empty().Is_Concept_Current(concept.Identity()),
            "a concept that was never published is reported as current"
        );

        let asserted = KnowledgeGraph::Empty().With_Concept(Asserted_Concept("entropy"));
        assert!(
            asserted.Is_Concept_Current(concept.Identity()),
            "an asserted and published concept is not reported as current, so every claim and \
             assertion about it would drop out of the current read"
        );

        let closed = KnowledgeGraph::Empty().With_Concept(
            Asserted_Concept("entropy")
                .Closed(Standing::Retired { because: BECAUSE.to_owned() }),
        );
        assert!(
            !closed.Is_Concept_Current(concept.Identity()),
            "a retired concept is still reported as current, so nothing about it can go stale"
        );
    }

    #[test]
    fn Test_Ordered_Entries_Should_Sort_By_Address_Rather_Than_By_The_Order_They_Were_Put_In()
    {
        // Which of two addresses is lower is the hasher's business, so the expectation is the lower
        // of the pair as the identities themselves order them, rather than a spelling that happens
        // to sort first today.
        let alpha = Concept::Named("alpha");
        let zeta = Concept::Named("zeta");
        let (low, high) = if alpha.Identity() < zeta.Identity()
        {
            (alpha.Identity(), zeta.Identity())
        }
        else
        {
            (zeta.Identity(), alpha.Identity())
        };

        // Published in the descending order, so an implementation that took the map's own order
        // rather than sorting would have to disagree with the expectation.
        let graph = KnowledgeGraph::Empty()
            .With_Concept(Asserted_Concept("zeta"))
            .With_Concept(Asserted_Concept("alpha"));

        let ordered: Vec<ContentIdentity> = Ordered_Entries(graph.Held_Concepts())
            .into_iter()
            .map(|held| return held.Value().Identity())
            .collect();

        assert_eq!(
            ordered,
            [low, high],
            "the listing is in the map's own order, so it changes between runs for no reason \
             anybody could account for"
        );
        assert!(
            Ordered_Entries::<Concept>(None).is_empty(),
            "a graph holding nothing lists something"
        );
    }

    /// A concept published as asserted, which is the ordinary case.
    ///
    /// Last rather than beside the first test that uses it, because more than one does: a helper
    /// with several callers belongs under no single one of them, so it sits in the shared region
    /// after the tests instead of interrupting the pair that read together.
    fn Asserted_Concept(name: &str) -> Versioned<Concept>
    {
        return Versioned::Asserted(Concept::Named(name));
    }
}
