//! The boundary between this crate and `kwb-model`, asserted from outside both.
//!
//! # What the two crates agree about
//!
//! This crate answers questions *about* an address and *by* words, and it decides neither. The
//! address a concept is looked up by is the identity `kwb-model` derives, and the words a query is
//! matched on are the output of `kwb_model::Normalize_Text`. `CurrentQueries` and
//! `HistoricalQueries` both take a `kwb_model::ContentIdentity`, so a caller has to name the model
//! to ask a question at all.
//!
//! That agreement is invisible from inside this crate. `src/current_queries.rs` imports both and
//! uses them correctly by construction; no test in `src/` can show that the *public* surface still
//! speaks the model's types and the model's rule, because a test in `src/` would be reading the
//! same two lines the implementation reads. From out here the question is answerable: an address
//! minted where the model mints them must be what the surface accepts, and a query must be judged
//! by the model's normalizer rather than by a second one grown here.
//!
//! # Why a stem that names no source file is the right name
//!
//! `check-test-coverage` reads a test file's own stem as the unit it covers, so a file called
//! `one_vocabulary.rs` addresses no source file and covers nothing. That is correct here for the
//! reason `kwb-domain`'s seam suites give about theirs: the property belongs to the boundary
//! between two crates and to neither side's files. The name is a claim about the boundary — there
//! is **one** vocabulary for addresses and for words, and it is not this crate's.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::KnowledgeGraph;
use kwb_domain::Scope;
use kwb_domain::Standing;
use kwb_domain::Versioned;
use kwb_model::Normalize_Text;
use kwb_retrieval::CurrentQueries;
use kwb_retrieval::HistoricalQueries;

/// The one claim every query below is asked about.
const CLAIM_TEXT: &str = "It is non-decreasing in an isolated system.";

/// What the concept holding the claim is called, with the interior run of spaces a reader who
/// retyped the name might produce.
const CONCEPT_NAME: &str = "thermodynamic entropy";
const CONCEPT_NAME_REFLOWED: &str = "thermodynamic   entropy";

/// The one claim above, as a document that was wrapped for a page and indented. The break and the
/// indent are the whole point: neither is in the words a person would search for, and neither is
/// an edit -- `Normalize_Text` collapses the run.
const REFLOWED_CLAIM_TEXT: &str = "It is non-decreasing in an\n    isolated system.";

/// Queries for the one claim above, chosen for what each says about whose normalization is in
/// force. The expected answer is not written beside them — see the test — so this table states
/// only what is asked.
const QUERIES: &[&str] = &[
    // The claim's own two words.
    "isolated system",
    // The same two words with the spacing a person types them with. `Normalize_Text` collapses the
    // run and trims both ends, so this is the same query.
    "  isolated   system  ",
    // A no-break space is whitespace and not a control character, so it collapses to a space like
    // any other run and the words are the same words.
    "isolated\u{A0}system",
    // A tab is a control character, and `Normalize_Text` removes control characters outright
    // rather than reading them as spaces -- so this is ONE word, and no claim says it.
    "isolated\tsystem",
    // One word of the query missing: every word has to match, so this is not a match.
    "isolated unicorn",
    // Reordered. Word order does not decide a match.
    "system isolated",
];

/// A graph holding the one concept and the one claim every test here asks about.
fn Corpus() -> KnowledgeGraph
{
    let entropy = Concept::Named(CONCEPT_NAME);
    let claim = Claim::About(&entropy, CLAIM_TEXT);
    let assertion = Assertion::By(
        "Callen 1985",
        &claim,
        Scope::Named("physical theory").expect("a named scope"),
    );

    return KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(entropy))
        .With_Claim(Versioned::Asserted(claim))
        .With_Assertion(Versioned::Asserted(assertion));
}

#[test]
fn Test_A_Concept_Should_Be_Looked_Up_By_The_Address_The_Model_Derives()
{
    // The address is named as `kwb_model::ContentIdentity` and not as whatever `Concept::Identity`
    // returns, because that is the whole point: the type this crate's signatures take is the
    // model's, and a caller reaching the retrieval surface has to reach the model to ask anything.
    // The compiler would refuse a mismatch, so what is asserted is the other half -- that the
    // address a caller mints elsewhere resolves *here*, to this concept.
    let graph = Corpus();
    let address: kwb_model::ContentIdentity = Concept::Named(CONCEPT_NAME).Identity();

    let reached = CurrentQueries::Over(&graph)
        .Neighbourhood_Of(address)
        .expect("the concept the address names is in the graph");

    assert_eq!(
        reached.concept.Identity(),
        address,
        "the retrieval surface answered about a concept other than the addressing one"
    );
}

#[test]
fn Test_Two_Spellings_Of_One_Name_Should_Address_One_Concept_Through_The_Search()
{
    // The consequence of the address being the model's rather than a string this crate compares.
    // A second spelling of one name is one address, so a caller who reaches for the concept by
    // either spelling is handed the same concept -- which is the deduplication the whole workspace
    // rests on, arriving through the query surface rather than through the graph directly.
    let graph = Corpus();

    let reflowed = CurrentQueries::Over(&graph)
        .Neighbourhood_Of(Concept::Named(CONCEPT_NAME_REFLOWED).Identity())
        .expect("a reflowed spelling of a name is the same name");

    assert_eq!(
        reflowed.claims.len(),
        1,
        "a reflowed spelling reached a different concept, so the search has an address of its own"
    );
}

#[test]
fn Test_A_Query_Should_Be_Matched_By_The_Models_Own_Normalization()
{
    // Written as a comparison rather than as a list of expected answers, so that the expectation
    // comes from `kwb_model::Normalize_Text` -- the rule the claim's identity was derived under --
    // and not from a second statement of that rule typed out here. If this crate ever grew a
    // normalizer of its own, the two would separate and this test would be the one that says so.
    let graph = Corpus();
    let searches = CurrentQueries::Over(&graph);
    let text = Normalize_Text(CLAIM_TEXT);

    for query in QUERIES
    {
        let normalized = Normalize_Text(query);
        let expected = !normalized.is_empty()
            && normalized
                .split_whitespace()
                .all(|word| return text.contains(word));

        assert_eq!(
            searches.Claims_Matching(query).len() == 1,
            expected,
            "the search disagreed with the model's normalization about {query:?}: \
             the model reads it as {normalized:?}"
        );
    }
}

#[test]
fn Test_A_Control_Character_In_A_Query_Should_Be_Stripped_And_Not_Read_As_A_Space()
{
    // The two halves held apart on purpose, because this is the pair a normalizer written here
    // would get wrong in the same direction. Splitting the raw query on whitespace -- the obvious
    // implementation, and the one that looks more forgiving -- would treat a tab as a break and
    // match, and would treat a no-break space as one too, so both of these would come out the same
    // way. They do not, and the whole reason the search reaches for `Normalize_Text` rather than
    // doing its own splitting is that a query must be read the way the claim's text was read.
    //
    // The tab case is the surprising one and it is asserted rather than excused: a tab between two
    // words makes ONE word, because `Normalize_Text` removes control characters outright so that a
    // value cannot forge a field boundary. A search that matched here would be answering about
    // text that no claim's identity was ever derived from.
    let graph = Corpus();
    let searches = CurrentQueries::Over(&graph);

    assert!(
        searches.Claims_Matching("isolated\u{A0}system").len() == 1,
        "a no-break space is whitespace, so this is the claim's own two words"
    );
    assert!(
        searches.Claims_Matching("isolated\tsystem").is_empty(),
        "a tab was read as a space, so the search is splitting the query itself instead of \
         normalizing it the way the claim's text was normalized"
    );
}

#[test]
fn Test_A_Reflowed_Claim_Should_Still_Be_Found_By_The_Words_It_Keeps()
{
    // The other side of the same seam: the text a caller reads back off a claim is the text it was
    // built with, and the text the search compares is the model's normalization of it. So a claim
    // written with a line break and an indent in it is found by the words a person would search
    // for, even though neither the stored text nor the query contains those words as written.
    let reflowed = REFLOWED_CLAIM_TEXT;
    let entropy = Concept::Named(CONCEPT_NAME);
    let claim = Claim::About(&entropy, reflowed);
    let graph = KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(entropy.clone()))
        .With_Claim(Versioned::Asserted(claim));

    assert_eq!(
        CurrentQueries::Over(&graph).Claims_Matching("isolated system").len(),
        1,
        "a reflowed claim was not found, so a citation is broken by a difference the model's own \
         normalization says is not an edit"
    );
}

#[test]
fn Test_Both_Worlds_Should_Answer_One_Query_By_The_Same_Words()
{
    // The vocabulary is shared and not merely similar. This crate files keyword matching apart from
    // both query types precisely so the current and historical searches cannot come to disagree
    // about what a match is, and that is a claim about two public types, answerable only from out
    // here. The corpus is built so the two worlds genuinely differ -- a concept closed against a
    // successor is in the historical world and not the current one -- while the one claim the
    // query is about stays current in both. So the agreement below is about the words and not
    // about the two worlds being the same graph, which the last assertion says out loud.
    let graph = Corpus_Where_The_Two_Worlds_Differ();
    let answers = Answers_Of_Both_Worlds(&graph, "isolated system");

    assert_eq!(
        answers.current,
        [CLAIM_TEXT],
        "the current world did not answer the claim the words belong to"
    );
    assert_eq!(
        answers.historical,
        [CLAIM_TEXT],
        "the historical world answered differently about the same words, so the two searches have \
         come apart about what a match is"
    );
    assert_ne!(
        CurrentQueries::Over(&graph).Concept_Count(),
        HistoricalQueries::Over(&graph).Concept_Count(),
        "the two worlds were supposed to differ, or the agreement above says nothing about the \
         words"
    );
}

/// The corpus with one further concept closed against a successor, which is what makes the two
/// worlds differ: the one claim the query below asks about stays current in both, and only the
/// version read holds the concept that was closed.
///
/// The difference is built here rather than asserted in the test, because it is the premise of the
/// agreement and not a third thing being checked — two worlds handed the same graph would agree
/// about anything, which is what the last assertion of the test says out loud.
fn Corpus_Where_The_Two_Worlds_Differ() -> KnowledgeGraph
{
    return Corpus().With_Concept(Versioned::Asserted(Concept::Named("enthalpy")).Closed(
        Standing::Superseded {
            by: Concept::Named("enthalpy, second edition").Identity(),
            because: "the two names denote one concept".to_owned(),
        },
    ));
}

/// What each of the two worlds answered the one query with, held as one value because they are one
/// question asked of two types: a difference between the fields is a difference about the words,
/// which is the whole of what the test above is about.
///
/// Named fields rather than a pair, for the reason `Corpus` gives: a pair communicates by position,
/// and `answers.1` would not say which world answered it.
struct Answers<'graph>
{
    /// What the current world answered, in the order the search returned it.
    current: Vec<&'graph str>,

    /// What the historical world answered the same query with.
    historical: Vec<&'graph str>,
}

/// Both answers to one query, gathered through the two public types.
///
/// Gathered together rather than read once per world at the call site, so that the two results are
/// unwrapped the same way: a second unwrapping written beside the first is exactly where the two
/// searches would come to be compared as two different readings of what a match is.
fn Answers_Of_Both_Worlds<'graph>(graph: &'graph KnowledgeGraph, query: &str) -> Answers<'graph>
{
    let current: Vec<&str> = CurrentQueries::Over(graph)
        .Claims_Matching(query)
        .into_iter()
        .map(|found| return found.Text())
        .collect();
    let historical: Vec<&str> = HistoricalQueries::Over(graph)
        .Claims_Matching(query)
        .into_iter()
        .map(|found| return found.Value().Text())
        .collect();

    return Answers { current, historical };
}
