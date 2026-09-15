//! The MCP host. Composition root only.
//!
//! # The contract this preserves, and the protocol it does not
//!
//! The .NET prototype's `Mcp/KnowledgeTools.cs` names nine read-only tools — `search`,
//! `get_concept`, `neighbours`, `path`, `proofs_for`, `claims_for`, `code_for`,
//! `gaps_in_source`, `list_connection_hypotheses` — with mutation excluded from the surface
//! architecturally rather than by convention. That exclusion is the part worth preserving
//! independently of the protocol carrying it, which is why this file wires **tools against
//! query types** and names no protocol library.
//!
//! `kwb-retrieval`'s `tests/read_only.rs` is what makes the exclusion real: the types every
//! handler below is built against have no method that can change anything, and a test fails
//! if one appears. A handler here therefore cannot write by mistake, because there is nothing
//! for it to call.
//!
//! # What it serves
//!
//! `kwb-mcp <store-dir>` replays that store's publication log and reports over the result — the
//! same fold `kwb admit` does, so a host and a command line cannot disagree about what is
//! known. With no argument it lists the surface over an empty graph and says so.
//!
//! # Four tools, not nine, and the five are named
//!
//! `search`, `get_concept`, `neighbours` and `merge_losers` are wired. The prototype's other
//! five want things this workspace does not have: `path` and `list_connection_hypotheses`
//! need typed relations between concepts, which `D-011` holds; `proofs_for` and `code_for`
//! need artifact kinds the type kernel has not declared; `gaps_in_source` needs coverage
//! recorded per source, which `KWB-4`'s `Coverage` can express and nothing yet stores.
//!
//! Naming them is deliberate. A surface that quietly shipped four tools where a reader
//! expected nine would leave them to discover the gap by its absence.

use kwb_domain::{Assertion, Claim, KnowledgeGraph, Replay_Records};
use kwb_platform::RecordLogStrategy;
use kwb_platform_std::FileRecordLog;
use kwb_retrieval::{CurrentQueries, HeldAssertion, HeldClaim, HistoricalQueries};

mod store;
mod tool;
mod tool_name;
mod world;

pub use store::Store;
pub use tool::Tool;
pub use tool_name::ToolName;
pub use world::World;

/// The corpus a host serves: what earlier runs published, replayed.
///
/// The same replay `kwb admit` does, for the same reason `D-014` gives — a graph is a fold over
/// its publications, so the log *is* the graph and there is no second representation to load.
/// An MCP host and a CLI therefore cannot disagree about what is known, because they are not
/// two readers of one store; they are the same fold over the same sequence.
///
/// # Errors
///
/// A description of what went wrong if the log cannot be opened, read, or replayed. A log that
/// will not replay is refused rather than served partially, because a corpus missing the
/// records after the first bad one would answer confidently about knowledge it does not have.
pub fn Corpus_At(root: Option<&str>) -> Result<KnowledgeGraph, String>
{
    let Some(root) = root
    else
    {
        return Ok(KnowledgeGraph::Empty());
    };

    let log = FileRecordLog::At(std::path::Path::new(root).join("publications.log"))
        .map_err(|cause| return format!("cannot open the publication log: {cause}"))?;
    let records = log
        .Records()
        .map_err(|cause| return format!("cannot read the publication log: {cause}"))?;
    return Replay_Records(&records)
        .map_err(|cause| return format!("the publication log cannot be replayed: {cause}"));
}

/// The read-only tool surface, in declaration order.
pub const TOOLS: [Tool; 5] = [
    Tool {
        name: "search",
        world: World::Current,
        summary: "claims whose text contains every word of a query",
    },
    Tool {
        name: "get_concept",
        world: World::Current,
        summary: "concepts whose canonical name contains every word of a query",
    },
    Tool {
        name: "neighbours",
        world: World::Current,
        summary: "a concept with its claims and their assertions",
    },
    Tool {
        name: "merge_losers",
        world: World::Historical,
        summary: "concepts closed against a successor -- the question an audit needs",
    },
    Tool {
        name: "held_neighbours",
        world: World::Historical,
        summary: "what a concept carried, live or closed -- what a merge loser said",
    },
];

/// Answer one tool call over a corpus.
///
/// The dispatcher every entry in [`TOOLS`] must have, and what makes that table a promise
/// rather than a description. `None` for a name no tool declares, so an unknown call is refused
/// rather than silently answering nothing.
///
/// Each arm reads the world its tool declares, and nothing here takes a world as an argument —
/// so no caller can ask the historical question of the current graph. `D19-B` is the incident
/// where exactly that happened and the answer was "nothing has been merged away".
#[must_use]
pub fn Answer_Tool_Call(graph: &KnowledgeGraph, tool: ToolName<'_>, argument: &str) -> Option<Vec<String>>
{
    let current = CurrentQueries::Over(graph);
    let historical = HistoricalQueries::Over(graph);

    return match tool.Text()
    {
        "search" => Some(
            current
                .Claims_Matching(argument)
                .iter()
                .map(|claim| return claim.Text().to_owned())
                .collect(),
        ),
        "get_concept" => Some(
            current
                .Concepts_Matching(argument)
                .iter()
                .map(|concept| return concept.Canonical_Name().to_owned())
                .collect(),
        ),
        "neighbours" => Some(Neighbourhood_Lines(current, argument)),
        "merge_losers" => Some(
            historical
                .Merge_Losers()
                .iter()
                .map(|held| return Merge_Loser_Line(held))
                .collect(),
        ),
        "held_neighbours" => Some(Held_Neighbourhood_Lines(historical, argument)),
        _ => None,
    };
}

/// A concept's neighbourhood, rendered.
///
/// Resolved by name, because a person at a terminal has a name and an agent that called
/// `get_concept` has whatever that returned, which is also a name.
fn Neighbourhood_Lines(current: CurrentQueries<'_>, name: &str) -> Vec<String>
{
    let Some(concept) = current
        .Concepts_Matching(name)
        .into_iter()
        .find(|candidate| return candidate.Canonical_Name() == name)
    else
    {
        return Vec::new();
    };

    let Some(neighbourhood) = current.Neighbourhood_Of(concept.Identity())
    else
    {
        return Vec::new();
    };

    let mut lines = vec![format!("concept  {}", neighbourhood.concept.Canonical_Name())];
    lines.extend(Claim_Lines(&neighbourhood.claims));
    lines.extend(Citation_Lines(&neighbourhood.assertions));
    return lines;
}

/// One line per claim of a current neighbourhood.
fn Claim_Lines(claims: &[&Claim]) -> Vec<String>
{
    return claims
        .iter()
        .map(|claim| return format!("claim    {}", claim.Text()))
        .collect();
}

/// One line per assertion, saying what it cites and the scope it was read under.
///
/// An unstated scope is rendered in words rather than omitted, which is `D-010` applied to a
/// listing: a source that did not say how far it meant has not said the narrowest thing, so a
/// citation whose scope quietly vanished would read as one scoped to everything.
fn Citation_Lines(assertions: &[&Assertion]) -> Vec<String>
{
    return assertions
        .iter()
        .map(|assertion| {
            let scope = if assertion.Scope().Is_Unstated()
            {
                "scope unstated"
            }
            else
            {
                assertion.Scope().Name()
            };
            return format!("cited    {} [{scope}]", assertion.Source());
        })
        .collect();
}

/// One merge loser: what it was, what it became, and what authorised that.
///
/// The reason is rendered because a merge log an audit cannot read the reasons out of is the
/// log the prototype had before `merge-audit` needed one — `D17`.
fn Merge_Loser_Line(held: &kwb_domain::Versioned<kwb_domain::Concept>) -> String
{
    let into = held
        .Standing()
        .Superseded_By()
        .map_or_else(|| return "?".to_owned(), |by| return by.Render());

    return format!(
        "{} -> {} ({})",
        held.Value().Canonical_Name(),
        into,
        held.Standing().Because().unwrap_or("no reason recorded")
    );
}

/// Everything a concept ever carried, each line saying what became of it.
///
/// # Why every line carries a standing and the current tool's lines do not
///
/// `neighbours` reads the current world, where everything it can reach is current by
/// construction, so saying so on every line would be noise. Here nothing is: a merge loser is
/// closed, and the claims it carried may be closed with it or still live under another concept.
/// A reader who could not tell those apart has `D19-B`'s confusion, which is what this tool
/// exists to end rather than to reproduce one layer up.
///
/// The concept is found by name against **every** version, because a merge loser is exactly the
/// concept the current world cannot find — that is what made it unreachable before `KWB-65`.
fn Held_Neighbourhood_Lines(historical: HistoricalQueries<'_>, name: &str) -> Vec<String>
{
    let Some(concept) = historical
        .Neighbourhood_Of(kwb_domain::Concept::Named(name).Identity())
    else
    {
        return Vec::new();
    };

    let mut lines = vec![format!(
        "concept  {} [{}]",
        concept.concept.Value().Canonical_Name(),
        Standing_Of(concept.concept.Standing())
    )];
    lines.extend(Held_Claim_Lines(&concept.claims));
    lines.extend(Held_Assertion_Lines(&concept.assertions));

    return lines;
}

/// One line per claim a held neighbourhood carried, each saying what became of it.
fn Held_Claim_Lines(claims: &[HeldClaim<'_>]) -> Vec<String>
{
    return claims
        .iter()
        .map(|claim| {
            // The composed answer, and the reason when the two disagree.
            //
            // A claim under a superseded concept is not current however its own standing reads, and
            // saying so needs both facts: the claim itself was never retired, and it is not live
            // because what it was about is not. Rendering only its own standing says "current" and
            // reproduces `D19-B` inside the tool built to end it -- which this line did twice, once
            // by reading the standing directly and once by falling back to it.
            let standing = match (claim.current, claim.held.Standing().Is_Current())
            {
                (true, _) => "current".to_owned(),
                (false, true) => "not current: its concept is not".to_owned(),
                (false, false) => Standing_Of(claim.held.Standing()),
            };
            return format!("claim    {} [{}]", claim.held.Value().Text(), standing);
        })
        .collect();
}

/// One line per citation a held neighbourhood carried, in the same composed sense.
fn Held_Assertion_Lines(assertions: &[HeldAssertion<'_>]) -> Vec<String>
{
    return assertions
        .iter()
        .map(|assertion| {
            let standing = match (assertion.current, assertion.held.Standing().Is_Current())
            {
                (true, _) => "current".to_owned(),
                (false, true) => "not current: what it cites is not".to_owned(),
                (false, false) => Standing_Of(assertion.held.Standing()),
            };
            return format!("cited    {} [{}]", assertion.held.Value().Source(), standing);
        })
        .collect();
}

/// The tool listing, and what the corpus holds.
pub fn Print_Surface(graph: &KnowledgeGraph, store: Store)
{
    let current = CurrentQueries::Over(graph);
    let historical = HistoricalQueries::Over(graph);

    Print_Tools();
    Print_Corpus(current, historical, store);
    Print_Absences();
}

/// The header and the tool table itself, in declaration order.
fn Print_Tools()
{
    println!("kwb-mcp: {} read-only tools", TOOLS.len());
    for tool in TOOLS
    {
        println!(
            "  {:<14} [{}]  {}",
            tool.name,
            tool.world.Name(),
            tool.summary
        );
    }
}

/// The corpus line, how to ask a question of it, and what an empty listing means.
fn Print_Corpus(
    current: CurrentQueries<'_>,
    historical: HistoricalQueries<'_>,
    store: Store,
)
{
    println!();
    println!("corpus     {} current, {} held", current.Concept_Count(), historical.Concept_Count());
    println!();
    println!("usage: kwb-mcp <store-dir> [<tool> [<argument>]]");
    println!();
    if store.Is_Absent()
    {
        println!("Pass a store directory to serve a real corpus; this listing is over an");
        println!("empty graph.");
        println!();
    }
}

/// What this surface does not have, said rather than left to be discovered by its absence.
fn Print_Absences()
{
    println!("No transport is wired. Every tool above reads a type with no write path, which");
    println!("kwb-retrieval's own tests check; mutation is excluded by the types rather than");
    println!("by this file remembering to exclude it.");
    println!();
    println!("Five of the prototype's nine tools are absent on purpose: path and");
    println!("list_connection_hypotheses need typed relations (D-011 holds them), proofs_for");
    println!("and code_for need artifact kinds the kernel has not declared, and gaps_in_source");
    println!("needs coverage stored per source.");
}

/// What became of something, with its reason when it has one.
///
/// # Why it is derived from `Is_Current` and not from a name on the type
///
/// `Standing::Is_Current` is **the** liveness expression in this workspace, guarded by a test
/// that fails on a second definition. Asking it here means this surface cannot drift from what
/// the domain means by live — which is the failure `D-012` describes as the prototype's index
/// enforcing half the rule for three weeks with nothing failing.
///
/// The reason is rendered rather than summarised, for `D17`: an audit that can see a concept was
/// closed and not why has the merge log the prototype had before `merge-audit` needed one.
///
/// # Why `Merge_Loser_Line` is not this
///
/// That renders a *relation* — what merged into what — and this renders a *state*, what became
/// of one thing. They overlap in the superseded case and answer different questions, and
/// collapsing them would make the merge listing carry standings nobody asked it for.
fn Standing_Of(standing: &kwb_domain::Standing) -> String
{
    if standing.Is_Current()
    {
        return "current".to_owned();
    }

    let became = match standing.Superseded_By()
    {
        Some(into) => format!("superseded by {}", into.Render()),
        None => "retired".to_owned(),
    };

    return match standing.Because()
    {
        Some(because) => format!("{became}: {because}"),
        None => became,
    };
}
