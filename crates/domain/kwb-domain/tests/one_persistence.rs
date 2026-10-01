//! The contract between this crate's graph and `kwb-platform-xvpe`'s persistent map, asserted from
//! outside both.
//!
//! # Why this file is named for a property rather than for a type
//!
//! `D-012` decided the mutable versioned graph is **adopted rather than built**, and
//! `kwb-platform-xvpe` is the one crate in this workspace permitted to name XVPE. So the graph's
//! whole promise — that publishing returns a new value and leaves the previous one readable, and
//! that there is therefore no filter to bypass and no flag to forget (`D19-B`) — is a promise about
//! *another crate's* map. This crate cannot test that from inside itself: `Held` is `pub(crate)`,
//! so its own tests can only observe the graph's behaviour and must take the map's word for why.
//!
//! The store is therefore rebuilt here by hand, out of `kwb_platform_xvpe` as a consumer sees it —
//! the map, this process's hasher, and the domain's address as the key — and asked the questions
//! the graph's promises rest on. What this file would catch is a change on either side of that
//! seam: a map whose `Insert` mutated in place, a key that stopped being the content address, or a
//! value whose standing did not survive being stored.
//!
//! # Why a stem that names no source file is the right name for it
//!
//! The unit `check-test-coverage` reads is the test file's own stem, so a file called
//! `one_persistence.rs` addresses no source file and covers nothing. That is correct rather than a
//! gap, for the reason `tests/one_identity.rs` gives about the other seam: the property belongs to
//! the boundary between two crates and to neither side's files.

use std::collections::hash_map::RandomState;

use kwb_domain::Concept;
use kwb_domain::KnowledgeGraph;
use kwb_domain::Standing;
use kwb_domain::Versioned;
use kwb_model::ContentIdentity;
use kwb_platform_xvpe::VersionedMap;

/// Why the concept the fixtures close was withdrawn.
const BECAUSE: &str = "the concept was withdrawn by its source";

/// A store of concepts, built the way the graph builds one.
///
/// `VersionedMap` is `kwb-platform-xvpe`'s name for the persistent map, chosen there for the role
/// rather than for the mechanism. The hasher is this process's, and the key is the address
/// `kwb-model` derives — so the three type parameters here are the seam, spelled out.
type Store<Value> = VersionedMap<ContentIdentity, Versioned<Value>, RandomState>;

/// One insertion into a store that may not exist yet, which is the fold the graph performs.
///
/// Written out rather than reached for, because the agreement this file is about is *spelled*
/// rather than shared: the graph's own fold is private, and these three lines are the same three.
/// The address is handed in rather than taken from the value, exactly as the graph takes it — a
/// fold that asked the value for its own address would be a key the store had not been told how to
/// check.
fn Putting<Value: Clone>(
    store: Option<&Store<Value>>,
    identity: ContentIdentity,
    value: Versioned<Value>,
) -> Store<Value>
{
    return match store
    {
        Some(store) => store.Insert(identity, value),
        None => Store::<Value>::with_hasher(RandomState::new()).Insert(identity, value),
    };
}

/// The concepts the fixtures below publish, in the order they are published.
///
/// A provider rather than a list inside each test, so that a concept added here is published by
/// every test that folds one.
fn Concepts_To_Publish() -> Vec<Versioned<Concept>>
{
    return vec![
        Versioned::Asserted(Concept::Named("entropy")),
        Versioned::Asserted(Concept::Named("enthalpy")),
    ];
}

/// The concepts held in a graph, in the order the all-versions read returns them.
fn Graph_Holding(values: &[Versioned<Concept>]) -> KnowledgeGraph
{
    let mut graph = KnowledgeGraph::Empty();
    for value in values
    {
        graph = graph.With_Concept(value.clone());
    }

    return graph;
}

/// The concepts held in a store, folded the way the graph folds them.
fn Store_Holding(values: &[Versioned<Concept>]) -> Store<Concept>
{
    let mut store: Option<Store<Concept>> = None;
    for value in values
    {
        store = Some(Putting(store.as_ref(), value.Value().Identity(), value.clone()));
    }

    return store.expect("the fixtures publish at least one concept");
}

/// The addresses the graph reports, in the order its all-versions read returns them.
///
/// That order is the map's rather than the publication's -- `D-012`'s map is a hash trie -- which is
/// why the other read is sorted before the two of them are compared.
fn Graph_Held_Addresses(graph: &KnowledgeGraph) -> Vec<ContentIdentity>
{
    return graph
        .Every_Version()
        .Concepts()
        .into_iter()
        .map(|held| return held.Value().Identity())
        .collect();
}

/// The addresses the store holds, sorted, so that the two reads are compared as the sets they are.
fn Store_Held_Addresses(store: &Store<Concept>) -> Vec<ContentIdentity>
{
    let mut held: Vec<ContentIdentity> =
        store.iter().map(|(identity, _)| return *identity).collect();
    held.sort_unstable();

    return held;
}

#[test]
fn Test_The_Store_This_Crate_Folds_Should_Hold_What_The_Graph_Reports()
{
    // The two are built independently over the same values, and compared as *sets of addresses*
    // rather than in order: the graph's listing is sorted by address because `D-012`'s map is a
    // hash trie whose own order is the hasher's business, so an order-sensitive comparison here
    // would be asserting the map's implementation rather than the contract.
    let values = Concepts_To_Publish();
    let graph = Graph_Holding(&values);
    let store = Store_Holding(&values);

    assert_eq!(
        Graph_Held_Addresses(&graph),
        Store_Held_Addresses(&store),
        "the graph and the store it is built on do not hold the same addresses, so what a caller \
         reads off the graph is not what the store the graph claims to be is holding"
    );
}

/// A store holding one published concept, the store that retired it, and both of the values.
///
/// Four named fields rather than any arrangement of positions: two of them are stores and two are
/// the concepts that went into them, and which value each store was asked to hold is the whole of
/// what the test below asserts.
struct Succession
{
    /// The store the concept was published into, which must still hold it.
    before: Store<Concept>,

    /// The store the retirement was published into, which must hold the retired value.
    after: Store<Concept>,

    /// The concept as it was published.
    published: Versioned<Concept>,

    /// The same concept as it was retired, under the address it was published at.
    retired: Versioned<Concept>,
}

/// A store holding one published concept, and the store built by retiring that concept over it.
fn A_Retirement_Published_Over_A_Publication() -> Succession
{
    let published = Versioned::Asserted(Concept::Named("entropy"));
    let retired = published.Closed(Standing::Retired { because: BECAUSE.to_owned() });

    let before: Store<Concept> = Putting(None, published.Value().Identity(), published.clone());
    let after = Putting(Some(&before), published.Value().Identity(), retired.clone());

    return Succession { before, after, published, retired };
}

#[test]
fn Test_A_Publication_Should_Leave_The_Store_It_Was_Published_Into_Valid()
{
    // The whole of `D19-B`, stated where it actually holds. The graph's promise is that the current
    // graph and the graph before that merge are two values a caller holds — which it can only make
    // because the map's `Insert` returns a new map rather than changing this one. Asserted on the
    // store rather than on the graph, because a test that read the graph would pass on an
    // implementation that copied the whole map on every publish, and this is the crate whose
    // behaviour decides which of the two it is.
    let Succession { before, after, published, retired } = A_Retirement_Published_Over_A_Publication();

    assert_eq!(
        before.Get(&published.Value().Identity()),
        Some(&published),
        "publishing into the store changed what the store it was published into held, so the graph \
         it came from is no longer a value and every read of it silently reads a later world"
    );
    assert_eq!(
        after.Get(&retired.Value().Identity()),
        Some(&retired),
        "the store that was published into does not hold what was published into it"
    );
}

#[test]
fn Test_Two_Spellings_Of_One_Name_Should_Be_One_Entry_In_The_Store()
{
    // The key is the address `kwb-model` derives, and the equality the map uses for it is that
    // type's. So the two spellings collapse here for the same reason they collapse in the graph —
    // and if they stopped collapsing, it would be here first, before any read could notice.
    let store = Putting(
        Some(&Putting(
            None,
            Concept::Named("entropy").Identity(),
            Versioned::Asserted(Concept::Named("entropy")),
        )),
        Concept::Named("  entropy  ").Identity(),
        Versioned::Asserted(Concept::Named("  entropy  ")),
    );

    assert_eq!(
        store.iter().count(),
        1,
        "a reflowed spelling of one name took a second entry in the store, so one concept became \
         two rows that can never learn of each other"
    );
    assert!(
        store.Get(&Concept::Named("entropy").Identity()).is_some(),
        "the entry the two spellings were supposed to share is not reachable at that address"
    );
}

#[test]
fn Test_An_Address_The_Store_Has_Never_Seen_Should_Answer_Nothing()
{
    // Absent is a case rather than an error, and the graph leans on it: `Is_Concept_Current` reads
    // a missing address as *not current*, which is what keeps a claim about a concept nobody ever
    // published out of the current read. A store that answered a default here would make every
    // absent concept a current one.
    let store = Putting(
        None,
        Concept::Named("entropy").Identity(),
        Versioned::Asserted(Concept::Named("entropy")),
    );

    assert!(
        store.Get(&Concept::Named("enthalpy").Identity()).is_none(),
        "the store answered something for an address it was never given, so a read that treats \
         absent as not current would instead see a concept nobody published"
    );
}
