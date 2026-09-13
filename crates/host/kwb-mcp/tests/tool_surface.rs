//! The property a declared surface must have: every tool it names can be called.

use kwb_domain::{Claim, Concept, KnowledgeGraph, Standing, Versioned};
use kwb_domain::{Assertion, Scope};
use kwb_mcp::{Answer, TOOLS};

/// A corpus with something to find and something merged away.
fn Corpus() -> KnowledgeGraph
{
    let entropy = Concept::Named("entropy");
    let claim = Claim::About(&entropy, "It is non-decreasing in an isolated system.");
    let assertion = Assertion::By("callen", &claim, Scope::Named("physical theory").expect("a named scope"));
    let loser = Concept::Named("C");
    let keeper = Concept::Named("C++");

    return KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(entropy))
        .With_Claim(Versioned::Asserted(claim))
        .With_Assertion(Versioned::Asserted(assertion))
        .With_Concept(Versioned::Asserted(keeper.clone()))
        .With_Concept(Versioned::Asserted(loser).Closed(Standing::Superseded {
            by: keeper.Identity(),
            because: "the two names denote one concept".to_owned(),
        }));
}

/// **The guard this file exists for.**
///
/// A table of tools and a dispatcher are two statements of one fact, and a fact stated twice
/// drifts. The prototype's universal kernel is what that looks like at scale: twelve
/// byte-identical node types, of which **six reached no consumer at all** — a surface naming
/// capabilities nothing behind it had, discoverable only by calling one and finding nothing.
///
/// So every declared tool must dispatch. Adding a row to `TOOLS` without an arm in `Answer`
/// fails here rather than at whatever asks for it later.
#[test]
fn Test_Every_Declared_Tool_Should_Be_Answerable()
{
    let graph = Corpus();

    let undispatched: Vec<&str> = TOOLS
        .iter()
        .filter(|tool| return Answer(&graph, tool.name, "entropy").is_none())
        .map(|tool| return tool.name)
        .collect();

    assert!(
        undispatched.is_empty(),
        "these tools are declared and cannot be called: {undispatched:?}"
    );
}

#[test]
fn Test_A_Tool_Nobody_Declared_Should_Be_Refused_Rather_Than_Answered_Emptily()
{
    // The other direction: an unknown name must be distinguishable from a known name that
    // found nothing, or an agent cannot tell a typo from an empty corpus.
    let graph = Corpus();

    assert!(Answer(&graph, "delete_everything", "").is_none());
    assert_eq!(
        Answer(&graph, "search", "unicorn"),
        Some(Vec::new()),
        "a known tool that found nothing answers with nothing, which is not the same as \
         having no such tool"
    );
}

#[test]
fn Test_Search_Should_Find_A_Claim_By_Its_Words()
{
    let answers = Answer(&Corpus(), "search", "isolated system").expect("a declared tool");

    assert_eq!(answers, ["It is non-decreasing in an isolated system."]);
}

#[test]
fn Test_Neighbours_Should_Reach_The_Claims_And_Their_Citations()
{
    let answers = Answer(&Corpus(), "neighbours", "entropy").expect("a declared tool");

    assert!(answers.iter().any(|line| return line.starts_with("concept  entropy")));
    assert!(answers.iter().any(|line| return line.starts_with("claim")));
    assert!(
        answers.iter().any(|line| return line.contains("physical theory")),
        "a neighbourhood without its citations is the half that says who said it: {answers:?}"
    );
}

#[test]
fn Test_Merge_Losers_Should_Report_What_Authorised_Each_Merge()
{
    // D17: an audit needs something to re-read, and a successor alone is not it.
    let answers = Answer(&Corpus(), "merge_losers", "").expect("a declared tool");

    assert_eq!(answers.len(), 1);
    assert!(
        answers.first().is_some_and(|line| return line.contains("denote one concept")),
        "a merge log an audit cannot read the reasons out of is the log the prototype had \
         before merge-audit needed one: {answers:?}"
    );
}

#[test]
fn Test_A_Merge_Loser_Should_Be_Invisible_To_The_Current_Tools()
{
    // The world each tool reads is fixed when the tool is declared, so this cannot be got
    // wrong by passing a flag -- there is no flag.
    let graph = Corpus();

    assert_eq!(Answer(&graph, "get_concept", "C").expect("declared"), ["C++"]);
    assert_eq!(Answer(&graph, "merge_losers", "").expect("declared").len(), 1);
}
