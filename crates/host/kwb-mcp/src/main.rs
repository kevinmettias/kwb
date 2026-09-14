//! The `kwb-mcp` binary: a command line over the read-only tool surface.
//!
//! The surface itself is `kwb_mcp`, this crate's library, so that a test can call a tool. A
//! dispatcher nothing could reach from a test would be a promise nobody checks, which is the
//! defect the surface exists to prevent.

use std::process::ExitCode;

use kwb_domain::KnowledgeGraph;
use kwb_mcp::{Answer, Corpus_At, Print_Surface};

/// A wrong command line, which is not the same as a run that failed.
const USAGE_EXIT: u8 = 2;

/// A run that reached the corpus and could not serve it.
const FAILURE_EXIT: u8 = 1;

fn main() -> ExitCode
{
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();

    let graph = match Corpus_At(borrowed.first().copied())
    {
        Ok(graph) => graph,
        Err(complaint) =>
        {
            eprintln!("kwb-mcp: {complaint}");
            return ExitCode::from(FAILURE_EXIT);
        }
    };

    return Serve(&graph, borrowed.get(1..).unwrap_or_default(), borrowed.is_empty());
}

/// Answer the tool the command line named, or list the surface when it named none.
///
/// The store directory is taken off the front before this is called, so the first element here
/// is the tool and the rest is its argument. `without_a_store` travels separately because it is
/// a fact about the command line rather than about what is left of it: a run given a store and
/// no tool has named no tool and is still not the empty listing.
fn Serve(graph: &KnowledgeGraph, arguments: &[&str], without_a_store: bool) -> ExitCode
{
    let Some((tool, rest)) = arguments.split_first()
    else
    {
        Print_Surface(graph, without_a_store);
        return ExitCode::SUCCESS;
    };

    let argument = rest.first().copied().unwrap_or_default();
    let Some(answers) = Answer(graph, tool, argument)
    else
    {
        eprintln!("kwb-mcp: no tool named {tool}");
        Print_Surface(graph, false);
        return ExitCode::from(USAGE_EXIT);
    };

    Print_Answers(answers);
    return ExitCode::SUCCESS;
}

/// Every answer on its own line, with finding nothing said rather than left blank.
fn Print_Answers(answers: Vec<String>)
{
    if answers.is_empty()
    {
        println!("(nothing)");
    }
    for answer in answers
    {
        println!("{answer}");
    }
}
