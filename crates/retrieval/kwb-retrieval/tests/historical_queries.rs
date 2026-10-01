//! What every version answers, in the unit that file owns — including what a merge carried.
//!
//! # Why this exists beside `read_only.rs`
//!
//! The same reason `tests/current_queries.rs` does: `read_only` is no source file's stem, so the
//! historical queries it drives were, to this rule, a surface no test named. This is the
//! companion of `src/historical_queries.rs`.
//!
//! The one question with no home anywhere else is `Neighbourhood_Of`'s: `Merge_Losers` says
//! *which* concept was closed and what authorised it, and this says what it **carried**. That
//! was the measured gap before `KWB-65` — the claim was in the graph, kept by `Versioned`, and
//! no query could name it — so it is asserted here on the composed liveness as well as on the
//! reach, because the failure mode was a closed concept's claim reporting itself live.

use kwb_domain::{Assertion, Claim, Concept, KnowledgeGraph, Scope, Standing, Versioned};
use kwb_retrieval::{CurrentQueries, HeldNeighbourhood, HistoricalQueries};

/// How many concepts the corpus below publishes: the successor and the one it superseded. Every
/// version reaches both, and the current read reaches only the first, which is the difference this
/// test is about.
const EVERY_CONCEPT_IN_THE_CORPUS: usize = 2;

/// A merge: one concept superseded by another, each carrying a claim, one of them cited.
struct Corpus
{
    /// The graph.
    graph: KnowledgeGraph,

    /// The concept that was closed, which is the one the historical world can reach and the
    /// current world cannot.
    loser: Concept,

    /// The claim the loser carried, which nothing could reach before `KWB-65`.
    lost: Claim,
}

/// The corpus above, assembled.
fn Corpus() -> Corpus
{
    let loser = Concept::Named("phlogiston");
    let winner = Concept::Named("oxidation");
    let lost = Claim::About(&loser, "It is released in combustion.");
    let kept = Claim::About(&winner, "It is combination with oxygen.");
    let cited = Assertion::By("Stahl 1703", &lost, Scope::Named("chemistry").expect("a named scope"));

    let graph = KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(winner.clone()))
        .With_Claim(Versioned::Asserted(kept))
        .With_Claim(Versioned::Asserted(lost.clone()))
        .With_Assertion(Versioned::Asserted(cited))
        .With_Concept(Versioned::Asserted(loser.clone()).Closed(Standing::Superseded {
            by: winner.Identity(),
            because: "superseded by oxidation theory".to_owned(),
        }));

    return Corpus { graph, loser, lost };
}

#[test]
fn Test_Over_Should_Ask_Of_Every_Version_Of_The_Graph_It_Was_Handed()
{
    let corpus = Corpus();

    assert_eq!(HistoricalQueries::Over(&corpus.graph).Merge_Losers().len(), 1);
    assert!(
        HistoricalQueries::Over(&KnowledgeGraph::Empty())
            .Merge_Losers()
            .is_empty(),
        "a query answered about a graph other than the one it was handed"
    );
}

#[test]
fn Test_Claims_Matching_Should_Find_A_Claim_Under_A_Concept_That_Is_No_Longer_Current()
{
    // The whole difference between the two types, on one query: the current world answers
    // nothing for this claim, and a search that quietly agreed with it would leave the record
    // of what was merged away unreachable by the only surface built to reach it.
    let corpus = Corpus();

    let found = HistoricalQueries::Over(&corpus.graph).Claims_Matching("combustion");

    assert_eq!(found.len(), 1);
    assert_eq!(
        found.first().map(|held| return held.Value().Identity()),
        Some(corpus.lost.Identity()),
        "every version did not find the claim the merge closed"
    );
    assert!(
        CurrentQueries::Over(&corpus.graph)
            .Claims_Matching("combustion")
            .is_empty(),
        "the same query is not a current one, and the two types must not agree here"
    );
}

#[test]
fn Test_Merge_Losers_Should_Name_Every_Concept_Closed_Against_A_Successor()
{
    let corpus = Corpus();

    let losers = HistoricalQueries::Over(&corpus.graph).Merge_Losers();

    assert_eq!(
        losers.first().map(|held| return held.Value().Canonical_Name()),
        Some("phlogiston")
    );
}

#[test]
fn Test_Merge_Losers_Should_Answer_Nothing_When_Nothing_Was_Merged()
{
    // The control. A `Merge_Losers` that returned every concept would satisfy the test above
    // and tell an audit that everything had been merged away — the mirror of the answer
    // `D19-B` records, and just as wrong.
    let plain = KnowledgeGraph::Empty().With_Concept(Versioned::Asserted(Concept::Named("entropy")));

    assert!(HistoricalQueries::Over(&plain).Merge_Losers().is_empty());
}

#[test]
fn Test_Neighbourhood_Of_Should_Reach_What_A_Closed_Concept_Carried()
{
    let corpus = Corpus();

    let held = Held_Neighbourhood_Of_The_Merge_Loser(&corpus);

    assert_eq!(held.concept.Value().Identity(), corpus.loser.Identity());
    assert_eq!(
        held.claims.first().map(|claim| return claim.held.Value().Identity()),
        Some(corpus.lost.Identity()),
        "the claim the merged concept carried is still unreachable"
    );
    assert!(
        !held.claims.first().expect("the claim was listed immediately above").current,
        "a claim under a superseded concept was reported current, which is `D19-B`'s confusion \
         inside the query built to end it"
    );
    assert_eq!(
        held.assertions.len(),
        1,
        "the citation of a claim under a merged concept is not reachable, so the evidence for \
         what was lost is not either"
    );
}

/// The historical world's answer about the concept the merge closed, reached by the identity the
/// merge named and asked through the query built for versions that are no longer current — the
/// only surface that can reach this concept at all, since the current world answers nothing about
/// it.
///
/// The `expect` is part of the property under test rather than a convenience. The concept was
/// published whatever became of it, so a query that declined would be the failure this file exists
/// to name and not a detail of the call; a helper returning an `Option` would move that decision
/// out to each caller, where the same message would have to be written again and could be dropped.
fn Held_Neighbourhood_Of_The_Merge_Loser(corpus: &Corpus) -> HeldNeighbourhood<'_>
{
    return HistoricalQueries::Over(&corpus.graph)
        .Neighbourhood_Of(corpus.loser.Identity())
        .expect("the concept was published, whatever became of it");
}

#[test]
fn Test_Concept_Count_Should_Count_Every_Version_And_Not_Only_What_Is_Current()
{
    let corpus = Corpus();

    assert_eq!(
        HistoricalQueries::Over(&corpus.graph).Concept_Count(),
        EVERY_CONCEPT_IN_THE_CORPUS
    );
    assert_eq!(
        CurrentQueries::Over(&corpus.graph).Concept_Count(),
        1,
        "the loser is held here and is not current, which is the difference the two types are for"
    );
}
