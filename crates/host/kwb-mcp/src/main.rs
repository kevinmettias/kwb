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

use std::process::ExitCode;

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
fn Corpus_At(root: Option<&str>) -> Result<KnowledgeGraph, String>
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

fn main() -> ExitCode
{
    let root: Vec<String> = std::env::args().skip(1).collect();
    let graph = match Corpus_At(root.first().map(String::as_str))
    {
        Ok(graph) => graph,
        Err(complaint) =>
        {
            eprintln!("kwb-mcp: {complaint}");
            return ExitCode::from(1);
        }
    };
    let current = CurrentQueries::Over(&graph);
    let historical = HistoricalQueries::Over(&graph);

    println!("kwb-mcp: {} read-only tools", TOOLS.len());
    for tool in TOOLS
    {
        println!("  {:<14} [{}]  {}", tool.name, tool.world.Name(), tool.summary);
    }

    println!();
    println!("corpus     {} current, {} held", current.Concept_Count(), historical.Concept_Count());
    println!();
    if root.is_empty()
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

    return ExitCode::SUCCESS;
}
