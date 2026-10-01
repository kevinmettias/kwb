//! The `kwb-mcp` binary: two doors onto the read-only tool surface.
//!
//! The surface itself is `kwb_mcp`, this crate's library, so that a test can call a tool. A
//! dispatcher nothing could reach from a test would be a promise nobody checks, which is the
//! defect the surface exists to prevent.
//!
//! # One corpus, two doors
//!
//! `kwb-mcp <store-dir> [<tool> [<argument>]]` answers once and exits; `kwb-mcp <store-dir>
//! --serve` holds a session on this process's own standard input and output until the client
//! closes its side. Both fold the store once, through the same `Corpus_At`, and both answer
//! through the same `Answer_Tool_Call` and the same `Rendered_Answer` — so the two cannot
//! disagree about what is known or about what a tool said. What differs is only who is
//! listening and in what framing.

use std::process::ExitCode;

use kwb_domain::KnowledgeGraph;
use kwb_mcp::{Answer_Tool_Call, Catalogue, Corpus_At, Print_Surface, Rendered_Answer, Store, ToolName};
use kwb_platform_xvpe::remote_call::Serve_Tools;

/// A wrong command line, which is not the same as a run that failed.
const USAGE_EXIT: u8 = 2;

/// A run that reached the corpus and could not serve it.
const FAILURE_EXIT: u8 = 1;

/// The word that asks for a session rather than an answer.
///
/// A word and not a positional argument, so it can sit anywhere after the store directory and
/// cannot be mistaken for a tool name: no entry in `TOOLS` begins with a hyphen.
const SERVE: &str = "--serve";

fn main() -> ExitCode
{
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let serving = arguments.iter().any(|argument| return argument == SERVE);
    let named: Vec<&str> = arguments
        .iter()
        .filter(|argument| return argument.as_str() != SERVE)
        .map(String::as_str)
        .collect();

    let graph = match Corpus_At(named.first().copied())
    {
        Ok(graph) => graph,
        Err(complaint) =>
        {
            eprintln!("kwb-mcp: {complaint}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    if serving
    {
        return Serve_Session(&graph);
    }

    return Serve_Command_Line(
        &graph,
        named.get(1..).unwrap_or_default(),
        Store::Of(named.first().copied()),
    );
}

/// Serve a tool session over this process's own pipes, until the client closes its side.
///
/// The framing, the handshake and every refusal the protocol has are the adopted backend's, so
/// this is the whole of what a host has to say about serving: build the catalogue, hand it the
/// pipes, and report a broken pipe rather than exiting silently. Reaching the end of the input
/// is how a session ends and is not a failure.
fn Serve_Session(graph: &KnowledgeGraph) -> ExitCode
{
    let catalogue = Catalogue::Over(graph);
    let input = std::io::stdin();
    let output = std::io::stdout();

    if let Err(cause) = Serve_Tools(&catalogue, input.lock(), output.lock())
    {
        eprintln!("kwb-mcp: the session ended badly: {cause}");
        return ExitCode::from(FAILURE_EXIT);
    }

    return ExitCode::SUCCESS;
}

/// Answer the tool the command line named, or list the surface when it named none.
///
/// The store directory is taken off the front before this is called, so the first element here
/// is the tool and the rest is its argument. Which store was named travels separately because it
/// is a fact about the command line rather than about what is left of it: a run given a store and
/// no tool has named no tool and is still not the empty listing.
fn Serve_Command_Line(graph: &KnowledgeGraph, arguments: &[&str], store: Store) -> ExitCode
{
    let Some((tool, rest)) = arguments.split_first()
    else
    {
        Print_Surface(graph, store);
        return ExitCode::SUCCESS;
    };

    let argument = rest.first().copied().unwrap_or_default();
    let Some(answers) = Answer_Tool_Call(graph, ToolName::Named(tool), argument)
    else
    {
        eprintln!("kwb-mcp: no tool named {tool}");
        Print_Surface(graph, Store::Named);
        return ExitCode::from(USAGE_EXIT);
    };

    Print_Answers(&answers);
    return ExitCode::SUCCESS;
}

/// Every answer, rendered by the one function both doors render an answer with.
///
/// That function is the library's, and this is deliberately not a second rendering of it: a
/// session is asserted to answer exactly what this command line prints, and that assertion is
/// only worth making while there is one renderer to be true of.
fn Print_Answers(answers: &[String])
{
    println!("{}", Rendered_Answer(answers));
}
