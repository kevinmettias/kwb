//! The `kwb-mcp` binary: a command line over the read-only tool surface.
//!
//! The surface itself is `kwb_mcp`, this crate's library, so that a test can call a tool. A
//! dispatcher nothing could reach from a test would be a promise nobody checks, which is the
//! defect the surface exists to prevent.

use std::process::ExitCode;

use kwb_mcp::{Answer, Corpus_At, Print_Surface};

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
            return ExitCode::from(1);
        }
    };

    let Some(tool) = borrowed.get(1)
    else
    {
        Print_Surface(&graph, borrowed.is_empty());
        return ExitCode::SUCCESS;
    };

    let argument = borrowed.get(2).copied().unwrap_or_default();
    let Some(answers) = Answer(&graph, tool, argument)
    else
    {
        eprintln!("kwb-mcp: no tool named {tool}");
        Print_Surface(&graph, false);
        return ExitCode::from(2);
    };

    if answers.is_empty()
    {
        println!("(nothing)");
    }
    for answer in answers
    {
        println!("{answer}");
    }
    return ExitCode::SUCCESS;
}
