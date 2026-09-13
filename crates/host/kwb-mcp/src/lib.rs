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

use kwb_domain::{KnowledgeGraph, Replay};
use kwb_platform::RecordLogStrategy;
use kwb_platform_std::FileRecordLog;
use kwb_retrieval::{CurrentQueries, HistoricalQueries};

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
    return Replay(&records)
        .map_err(|cause| return format!("the publication log cannot be replayed: {cause}"));
}

/// One tool an agent may call, and the world it reads.
///
/// # Why the world is part of the tool rather than a parameter
///
/// `D19-B`. A single `search` tool with a `include_retired` flag is the shape that let
/// `merge-audit` ask the current world a question about merge losers, resolve none, print
/// *"nothing has been merged away"* and exit `0`. Here the world a tool reads is fixed when
/// the tool is declared, so an agent choosing a tool has chosen a world, and there is no
/// argument it can omit to land in the wrong one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tool
{
    /// The name an agent calls.
    pub name: &'static str,

    /// Which world it answers from.
    pub world: World,

    /// What it does, in one line, for a tool listing.
    pub summary: &'static str,
}

/// Which set of versions a tool reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World
{
    /// What is current. Merge losers and retired concepts are not here.
    Current,

    /// Every version, including what is closed.
    Historical,
}

impl World
{
    /// The short name shown in a listing.
    #[must_use]
    pub const fn Name(self) -> &'static str
    {
        return match self
        {
            Self::Current => "current",
            Self::Historical => "historical",
        };
    }
}

/// The read-only tool surface, in declaration order.
pub const TOOLS: [Tool; 4] = [
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
pub fn Answer(graph: &KnowledgeGraph, tool: &str, argument: &str) -> Option<Vec<String>>
{
    let current = CurrentQueries::Over(graph);
    let historical = HistoricalQueries::Over(graph);

    return match tool
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
        _ => None,
    };
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
    for claim in &neighbourhood.claims
    {
        lines.push(format!("claim    {}", claim.Text()));
    }
    for assertion in &neighbourhood.assertions
    {
        let scope = if assertion.Scope().Is_Unstated()
        {
            "scope unstated"
        }
        else
        {
            assertion.Scope().Name()
        };
        lines.push(format!("cited    {} [{scope}]", assertion.Source()));
    }
    return lines;
}

/// The tool listing, and what the corpus holds.
pub fn Print_Surface(graph: &KnowledgeGraph, without_a_store: bool)
{
    let current = CurrentQueries::Over(graph);
    let historical = HistoricalQueries::Over(graph);

    println!("kwb-mcp: {} read-only tools", TOOLS.len());
    for tool in TOOLS
    {
        println!("  {:<14} [{}]  {}", tool.name, tool.world.Name(), tool.summary);
    }

    println!();
    println!("corpus     {} current, {} held", current.Concept_Count(), historical.Concept_Count());
    println!();
    println!("usage: kwb-mcp <store-dir> [<tool> [<argument>]]");
    println!();
    if without_a_store
    {
        println!("Pass a store directory to serve a real corpus; this listing is over an");
        println!("empty graph.");
        println!();
    }

    println!("No transport is wired. Every tool above reads a type with no write path, which");
    println!("kwb-retrieval's own tests check; mutation is excluded by the types rather than");
    println!("by this file remembering to exclude it.");
    println!();
    println!("Five of the prototype's nine tools are absent on purpose: path and");
    println!("list_connection_hypotheses need typed relations (D-011 holds them), proofs_for");
    println!("and code_for need artifact kinds the kernel has not declared, and gaps_in_source");
    println!("needs coverage stored per source.");
}
