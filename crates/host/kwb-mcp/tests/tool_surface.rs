//! The property a declared surface must have: every tool it names can be called.

use kwb_domain::{Claim, Concept, KnowledgeGraph, Standing, Versioned};
use kwb_domain::{Assertion, Scope};
use kwb_mcp::{Answer_Tool_Call, ToolName, TOOLS};

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
/// So every declared tool must dispatch. Adding a row to `TOOLS` without an arm in `Answer_Tool_Call`
/// fails here rather than at whatever asks for it later.
#[test]
fn Test_Every_Declared_Tool_Should_Be_Answerable()
{
    let graph = Corpus();

    let undispatched: Vec<&str> = TOOLS
        .iter()
        .filter(|tool| return Answer_Tool_Call(&graph, ToolName::Named(tool.name), "entropy").is_none())
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

    assert!(Answer_Tool_Call(&graph, ToolName::Named("delete_everything"), "").is_none());
    assert_eq!(
        Answer_Tool_Call(&graph, ToolName::Named("search"), "unicorn"),
        Some(Vec::new()),
        "a known tool that found nothing answers with nothing, which is not the same as \
         having no such tool"
    );
}

#[test]
fn Test_Search_Should_Find_A_Claim_By_Its_Words()
{
    let answers = Answer_Tool_Call(&Corpus(), ToolName::Named("search"), "isolated system").expect("a declared tool");

    assert_eq!(answers, ["It is non-decreasing in an isolated system."]);
}

#[test]
fn Test_Neighbours_Should_Reach_The_Claims_And_Their_Citations()
{
    let answers = Answer_Tool_Call(&Corpus(), ToolName::Named("neighbours"), "entropy").expect("a declared tool");

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
    let answers = Answer_Tool_Call(&Corpus(), ToolName::Named("merge_losers"), "").expect("a declared tool");

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

    assert_eq!(Answer_Tool_Call(&graph, ToolName::Named("get_concept"), "C").expect("declared"), ["C++"]);
    assert_eq!(Answer_Tool_Call(&graph, ToolName::Named("merge_losers"), "").expect("declared").len(), 1);
}

// ---- KWB-65: what a merge loser carried ----

/// A corpus with a merge in it: one concept superseded by another, each carrying a claim.
fn After_A_Merge() -> KnowledgeGraph
{
    let loser = Concept::Named("phlogiston");
    let winner = Concept::Named("oxidation");
    let lost = Claim::About(&loser, "It is released in combustion.");
    let kept = Claim::About(&winner, "It is combination with oxygen.");
    let cited = Assertion::By("Stahl 1703", &lost, Scope::Named("chemistry").expect("the name is not blank, so Named answers Some"));

    let graph = KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(winner.clone()))
        .With_Claim(Versioned::Asserted(kept))
        .With_Claim(Versioned::Asserted(lost))
        .With_Assertion(Versioned::Asserted(cited));

    return graph.With_Concept(Versioned::Asserted(loser).Closed(Standing::Superseded {
        by: winner.Identity(),
        because: "superseded by oxidation theory".to_owned(),
    }));
}

#[test]
fn Test_A_Merge_Loser_Should_Say_What_It_Carried()
{
    // The gap measured before this tool existed: `merge_losers` reported the loser and its
    // reason, and `neighbours` reported nothing, so an audit could establish that something was
    // merged and not what was lost. `D17` is that destruction requires evidence, and evidence a
    // reader cannot reach is an obligation that looks handled because nothing complains.
    let graph = After_A_Merge();

    let answered = Answer_Tool_Call(&graph, ToolName::Named("held_neighbours"), "phlogiston").expect("a declared tool");
    let joined = answered.join("\n");

    assert!(
        joined.contains("It is released in combustion."),
        "the claim the merged concept carried is still unreachable: {joined}"
    );
    assert!(
        joined.contains("superseded by oxidation theory"),
        "the reason that authorised the merge is not rendered: {joined}"
    );
}

/// Assert that `line` -- the one a tool rendered for `what` -- is reported as not current.
///
/// The claim under a superseded concept and the citation of it are two lines of one answer and
/// one check, so it is written once: a test that spelled it out twice could have the two drift,
/// and it is the pair together that says nothing under a merged concept is live.
fn Assert_Reported_Not_Current(line: &str, what: &str)
{
    assert!(line.contains("not current"), "{what} is reported as live: {line}");
}

#[test]
fn Test_Nothing_Under_A_Merged_Concept_Should_Be_Reported_As_Current()
{
    // A claim is current when its own standing is current **and its concept's is**, which is
    // what `CurrentKnowledge::Claims` composes. This listing got that wrong twice while it was
    // being written -- once by reading the claim's own standing, and once by falling back to it
    // -- and both times it rendered a claim under a superseded concept as live, which is
    // `D19-B`'s confusion inside the tool built to end it.
    let graph = After_A_Merge();
    let answered = Answer_Tool_Call(&graph, ToolName::Named("held_neighbours"), "phlogiston").expect("a declared tool");

    let lost = answered
        .iter()
        .find(|line| return line.contains("It is released in combustion."))
        .expect("the claim is listed");
    let cited = answered
        .iter()
        .find(|line| return line.starts_with("cited"))
        .expect("the citation is listed");

    Assert_Reported_Not_Current(lost, "a claim under a superseded concept");
    Assert_Reported_Not_Current(cited, "a citation of a claim that is not current");
}

#[test]
fn Test_A_Concept_That_Is_Current_Should_Still_Read_As_Current()
{
    // The control. Without it the two tests above pass against a listing that calls everything
    // not current, which would be a tool that has stopped answering rather than one that
    // answers carefully.
    let graph = After_A_Merge();

    let answered = Answer_Tool_Call(&graph, ToolName::Named("held_neighbours"), "oxidation").expect("a declared tool");
    let joined = answered.join("\n");

    assert!(
        joined.contains("It is combination with oxygen."),
        "the surviving concept's claim is missing: {joined}"
    );
    assert!(
        !joined.contains("not current"),
        "the surviving concept's neighbourhood is reported as closed: {joined}"
    );
}
