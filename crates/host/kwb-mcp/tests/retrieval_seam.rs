//! The boundary between this host and `kwb-retrieval`, asserted from outside both.
//!
//! # What this host does not decide
//!
//! The rest of this crate's suites cover what the host decides for itself: which store it was
//! named, which world a tool declares, whether every declared tool dispatches. This file covers
//! the opposite -- the rules the host deliberately does **not** have. Every answer
//! `Answer_Tool_Call` gives is a projection of a query's answer: which claims a word matches,
//! which concepts a name reaches, which concepts were merged away. None of that is decided here,
//! and `src/lib.rs` says so in as many words.
//!
//! That claim is not testable from inside the crate. A test in `src/` would import the same two
//! query types the dispatcher imports and read the same lines it reads. From out here the question
//! is answerable, and it is answerable the strong way: for each tool, the answer is compared
//! against the query type the tool declares, computed independently in this file from the same
//! graph. A host that grew a liveness rule, a matching rule or a name comparison of its own would
//! separate from the query's answer and fail here.
//!
//! # Why `retrieval_seam` and not a source file's stem
//!
//! The unit `check-test-coverage` reads is the test file's own stem, so this file addresses no
//! source file and covers nothing -- correctly, since its subject is the boundary between two
//! crates rather than either side's files. Its companions here are named for the stems they cover
//! (`store.rs`, `world.rs`, `tool_name.rs`) or for what they assert (`tool_surface.rs`); this one
//! is the second kind.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::KnowledgeGraph;
use kwb_domain::Scope;
use kwb_domain::Standing;
use kwb_domain::Versioned;
use kwb_mcp::Answer_Tool_Call;
use kwb_mcp::ToolName;
use kwb_retrieval::CurrentQueries;
use kwb_retrieval::HistoricalQueries;

/// The concept the current world holds.
const CURRENT_NAME: &str = "entropy";

/// What the current concept says, and the words that reach it.
const CURRENT_CLAIM: &str = "It is non-decreasing in an isolated system.";
const CURRENT_WORDS: &str = "isolated system";

/// The concept that was merged away, and the name it was merged into.
const LOSER_NAME: &str = "C";
const KEEPER_NAME: &str = "C++";

/// What the merged-away concept says, and the words that reach it. Deliberately disjoint from the
/// current claim's words, so that one query asks each world a question only that world can answer.
const LOSER_CLAIM: &str = "It has undefined behaviour.";
const LOSER_WORDS: &str = "undefined behaviour";

/// What the two concepts are about, so an assertion that fails reads as a corpus problem rather
/// than as a missing name.
const SCOPE_NAME: &str = "programming languages";
const SOURCE: &str = "K&R 1988";

/// The reason the merge is recorded with, which the merge listing is expected to render.
const BECAUSE: &str = "the two names denote one concept";

/// A corpus with a live concept and a concept that was merged away, each saying something only one
/// world answers.
///
/// Built so that the two worlds genuinely differ: the words above reach the loser's claim in the
/// historical world and nothing in the current one. Every test here leans on that difference, and
/// the last assertion in the first test says it out loud rather than assuming it.
fn Corpus() -> KnowledgeGraph
{
    let keeper = Concept::Named(KEEPER_NAME);
    let loser = Concept::Named(LOSER_NAME);
    let current = Claim::About(&Concept::Named(CURRENT_NAME), CURRENT_CLAIM);
    let closed = Claim::About(&loser, LOSER_CLAIM);
    let assertion = Assertion::By(
        SOURCE,
        &closed,
        Scope::Named(SCOPE_NAME).expect("a named scope"),
    );

    return KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(keeper.clone()))
        .With_Concept(Versioned::Asserted(Concept::Named(CURRENT_NAME)))
        .With_Claim(Versioned::Asserted(current))
        .With_Assertion(Versioned::Asserted(assertion))
        .With_Concept(Versioned::Asserted(loser).Closed(Standing::Superseded {
            by: keeper.Identity(),
            because: BECAUSE.to_owned(),
        }))
        .With_Claim(Versioned::Asserted(closed));
}

/// The lines one tool call answered, or a failure naming the call that answered nothing.
fn Answer(graph: &KnowledgeGraph, tool: &str, argument: &str) -> Vec<String>
{
    let Some(lines) = Answer_Tool_Call(graph, ToolName::Named(tool), argument)
    else
    {
        panic!("the dispatched tool {tool:?} answered nothing");
    };

    return lines;
}

/// How many lines of `answered` report a thing of `kind`.
///
/// The host opens each line with the word for what the line is about -- `claim` for a claim of a
/// concept, `cited` for the assertion of one -- so counting a kind is how a listing is held to the
/// number of parts a query says it has. Read here rather than in each test because two tests hold
/// two listings to that same shape, and a test that scraped `claim` out of the text itself would be
/// a second, quieter statement of what the host renders.
fn Count_Of_Lines_Reporting(answered: &[String], kind: &str) -> usize
{
    let reported = answered
        .iter()
        .filter(|line| return line.starts_with(kind))
        .count();

    return reported;
}

#[test]
fn Test_The_Search_Tool_Should_Answer_The_Current_Querys_Answer_And_Not_The_Historical_One()
{
    // The dispatcher's whole reason for naming a world per tool, asserted against the two query
    // types rather than against a count typed here. `D19-B` is the incident where the historical
    // question was asked of the current graph and answered "nothing has been merged away", and the
    // shape of that defect -- a tool reading the world it did not declare -- is what the helper
    // below exists to catch.
    let graph = Corpus();
    Assert_The_Search_Tool_Reads_Only_The_Current_World(&graph);

    // And where the two worlds agree, the host must be answering the query rather than a rule of
    // its own. Compared as sets of text, because the host's job is to render what the query
    // found and not to decide what it found.
    let answered_by_the_query: Vec<String> = CurrentQueries::Over(&graph)
        .Claims_Matching(CURRENT_WORDS)
        .into_iter()
        .map(|claim| return claim.Text().to_owned())
        .collect();
    assert_eq!(
        Answer(&graph, "search", CURRENT_WORDS),
        answered_by_the_query,
        "search and the current keyword query disagree about the same query over the same graph"
    );
}

/// Holds `search` to the world it declares: a word only the historical world answers is a word it
/// does not answer with.
///
/// The first assertion is about the corpus rather than about the host, and it is what makes the
/// second one mean anything: every test in this file leans on the two worlds genuinely differing,
/// and against a corpus whose worlds are the same world a search tool reading the historical one
/// passes vacuously. The second is `D19-B`'s confusion with the two worlds swapped -- a tool that
/// declares the current world and answers out of the historical one.
fn Assert_The_Search_Tool_Reads_Only_The_Current_World(graph: &KnowledgeGraph)
{
    let words_only_the_historical_world_answers =
        HistoricalQueries::Over(graph).Claims_Matching(LOSER_WORDS);
    assert!(
        !words_only_the_historical_world_answers.is_empty(),
        "the corpus was supposed to hold a claim the current world cannot reach, or this test \
         proves nothing about which world the tool reads"
    );
    assert!(
        Answer(graph, "search", LOSER_WORDS).is_empty(),
        "search answered with a claim about a concept that is no longer current, so the tool is \
         reading the historical world while declaring the current one"
    );
}

#[test]
fn Test_The_Get_Concept_Tool_Should_Answer_The_Current_Querys_Answer()
{
    // The name a caller reaches a concept by is matched by the query's rule -- every word, after
    // the model's normalization -- and the host adds nothing to it. So a query that is one word of
    // a two-word name reaches the concept, and the tool and the query name the same concepts in
    // the same order.
    let graph = Corpus();

    let answered_by_the_query: Vec<String> = CurrentQueries::Over(&graph)
        .Concepts_Matching("entropy")
        .into_iter()
        .map(|concept| return concept.Canonical_Name().to_owned())
        .collect();

    assert_eq!(
        answered_by_the_query,
        [CURRENT_NAME],
        "the corpus was supposed to hold exactly the one concept that word reaches"
    );
    assert_eq!(
        Answer(&graph, "get_concept", "entropy"),
        answered_by_the_query,
        "get_concept and the current concept query disagree about the same words"
    );
}

#[test]
fn Test_The_Neighbours_Tool_Should_Answer_The_Neighbourhood_At_The_Address_The_Model_Derives()
{
    // The address half of the seam: the neighbourhood is fetched by an identity, and the identity
    // is derived from the name by `kwb-model`. `The_Neighbourhood_The_Model_Derives_For` derives
    // that address itself and asks the query for the neighbourhood at it, so the host's answer is
    // tied to retrieval's answer *at the model's address* rather than to a neighbourhood fetched
    // some other way.
    let graph = Corpus();

    let reached = The_Neighbourhood_The_Model_Derives_For(&graph, CURRENT_NAME);
    let answered = Answer(&graph, "neighbours", CURRENT_NAME);
    assert_eq!(
        Count_Of_Lines_Reporting(&answered, "claim"),
        reached.claims.len(),
        "the host's neighbourhood reached a different number of claims than the query's does at \
         the address the model derives from the same name: {answered:?}"
    );
    assert!(
        answered
            .first()
            .is_some_and(|line| return line.contains(CURRENT_NAME)),
        "the neighbourhood does not open with the concept it is a neighbourhood of: {answered:?}"
    );
}

/// The neighbourhood the current query answers at the address `kwb-model` derives from `name`.
///
/// The address is derived here, from the name, and not taken from the host or from a position in a
/// listing: what this file asserts is that the host asks retrieval the question the query answers,
/// and an address the test chose for itself would not be the one the model derives. The half a host
/// can get wrong without noticing is this one -- a host that hashed the name itself, or looked the
/// concept up by position in a listing, would answer a different neighbourhood or none at all --
/// and it is why the derivation is a named step rather than a line inside the test.
fn The_Neighbourhood_The_Model_Derives_For<'graph>(
    graph: &'graph KnowledgeGraph,
    name: &str,
) -> kwb_retrieval::Neighbourhood<'graph>
{
    let reached = CurrentQueries::Over(graph)
        .Neighbourhood_Of(Concept::Named(name).Identity())
        .expect("the corpus holds the concept this address names");

    return reached;
}

#[test]
fn Test_The_Merge_Losers_Tool_Should_Answer_The_Historical_Querys_Answer()
{
    // The tool that exists because of `D19-B`, and the one whose wrongness is quietest: a listing
    // of merge losers that answers nothing looks exactly like a corpus nothing was merged in. The
    // expectation here is the query's own count rather than a number, so the two cannot drift.
    let graph = Corpus();
    let losers = HistoricalQueries::Over(&graph).Merge_Losers();

    assert_eq!(
        losers.len(),
        1,
        "the corpus was supposed to hold one merged-away concept"
    );

    let answered = Answer(&graph, "merge_losers", "");
    assert_eq!(
        answered.len(),
        losers.len(),
        "the merge listing and the historical query disagree about how many concepts were merged \
         away: {answered:?}"
    );
    Assert_The_Merge_Listing_Names_The_Relation_And_Its_Reason(&answered);
}

/// Holds the merge listing to the two things it must say about a concept that was merged away: what
/// it was merged into, and the reason the merge was recorded with.
///
/// What was merged *into* is rendered as the address the merge points at rather than as the
/// successor's name — the host's own decision, and the one it is held to here: the claim is that
/// the listing names the relation, not that it spells it a particular way. The reason is `D17`'s
/// requirement that a merge carry something an audit can re-read, which a successor alone is not.
fn Assert_The_Merge_Listing_Names_The_Relation_And_Its_Reason(answered: &[String])
{
    let into = Concept::Named(KEEPER_NAME).Identity().Render();
    assert!(
        answered
            .first()
            .is_some_and(|line| return line.contains(LOSER_NAME) && line.contains(&into)),
        "the listing does not say what was merged into what: {answered:?}"
    );
    assert!(
        answered.first().is_some_and(|line| return line.contains(BECAUSE)),
        "the listing drops the reason the merge was recorded with, which is `D17`: {answered:?}"
    );
}

#[test]
fn Test_The_Held_Neighbours_Tool_Should_Reach_A_Concept_The_Current_World_Can_Not()
{
    // `KWB-65`, and the property that makes the two worlds worth being two types. A merge loser is
    // precisely the concept the current neighbourhood cannot be asked about, so the same address
    // that answers nothing in one world answers everything in the other -- and the host reaches
    // that answer only through the historical query, which is what this pins.
    let graph = Corpus();
    let address = Concept::Named(LOSER_NAME).Identity();
    assert!(
        CurrentQueries::Over(&graph).Neighbourhood_Of(address).is_none(),
        "a merged-away concept is reachable from the current world, so the corpus is not the one \
         this test needs"
    );

    let held = The_Held_Neighbourhood_The_Model_Derives_For(&graph, LOSER_NAME);
    let answered = Answer(&graph, "held_neighbours", LOSER_NAME);
    assert!(
        !answered.is_empty(),
        "held_neighbours answered nothing for a concept the historical world holds, which is the \
         merge-audit symptom with the world the right way round"
    );
    Assert_The_Held_Listing_Accounts_For_The_Held_Neighbourhood(&answered, &held);
}

/// The neighbourhood the historical query answers at the address `kwb-model` derives from `name`.
///
/// The derivation the current world's helper makes, one world over, and the reason the two worlds
/// are two types rather than one: a concept that was merged away is reachable at the address
/// derived from its name through this query and through no other, the current world's listing not
/// holding it at all.
fn The_Held_Neighbourhood_The_Model_Derives_For<'graph>(
    graph: &'graph KnowledgeGraph,
    name: &str,
) -> kwb_retrieval::HeldNeighbourhood<'graph>
{
    let held = HistoricalQueries::Over(graph)
        .Neighbourhood_Of(Concept::Named(name).Identity())
        .expect("the historical world holds the concept this address names");

    return held;
}

/// Holds the held listing to the held neighbourhood it is a listing of.
///
/// Every part of what the concept carried is listed, and every line says what became of it. That is
/// why each count is checked against the query's own list rather than against a number typed here:
/// a number would agree with a listing that dropped a part and with a corpus that lost one, and the
/// two are not the same defect. The last assertion is the one the tool exists for -- the listing
/// says what the concept became -- because a merge whose evidence does not carry its conclusion is
/// `D17` unread.
fn Assert_The_Held_Listing_Accounts_For_The_Held_Neighbourhood(
    answered: &[String],
    held: &kwb_retrieval::HeldNeighbourhood<'_>,
)
{
    assert_eq!(
        Count_Of_Lines_Reporting(answered, "claim"),
        held.claims.len(),
        "the held listing and the held neighbourhood disagree about the claims: {answered:?}"
    );
    assert_eq!(
        Count_Of_Lines_Reporting(answered, "cited"),
        held.assertions.len(),
        "the held listing and the held neighbourhood disagree about the citations: {answered:?}"
    );
    assert!(
        answered
            .first()
            .is_some_and(|line| return line.contains(LOSER_NAME) && line.contains(BECAUSE)),
        "the held listing does not say what the concept became, which is the standing it exists to \
         carry: {answered:?}"
    );
}
