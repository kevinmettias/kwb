//! Invalid closing acts preserve the log, its prior evidence, and the current graph.

use kwb_domain::{Concept, KnowledgeGraph, Replay_Records};
use kwb_mcp::{Answer_Tool_Call, ToolName};
use std::path::PathBuf;
use std::process::{Command, Output};

const KWB: &str = env!("CARGO_BIN_EXE_kwb");
const REASON: &str = "The evidence establishes beta as the successor.";

struct Fixture
{
    root: PathBuf,
    store: String,
}

impl Fixture
{
    fn Of(test: &str) -> Self
    {
        let root = std::env::temp_dir().join(format!("kwb-closing-preconditions-{}-{test}", std::process::id()));
        std::fs::create_dir(&root).expect("an unused temporary directory");
        let source = root.join("source.txt");
        std::fs::write(&source, "Alpha and beta are recorded here.").expect("a writable source");
        let fixture = Self { store: root.join("store").display().to_string(), root };
        Succeeded(&Ran(&["admit", &source.display().to_string(), "--store", &fixture.store,
            "--says", "alpha", "Alpha is an earlier explanation.", "--says", "beta", "Beta is its successor."]));
        return fixture;
    }

    fn Close(&self, name: &str, successor: Option<&str>) -> Output
    {
        let mut args = vec![if successor.is_some() { "supersede" } else { "retire" }, name];
        if let Some(successor) = successor
        {
            args.extend_from_slice(&["--into", successor]);
        }
        args.extend_from_slice(&["--store", &self.store, "--because", REASON]);
        return Ran(&args);
    }

    fn Log(&self) -> Vec<u8>
    {
        return std::fs::read(self.root.join("store/publications.log")).expect("the recorded publications");
    }

    fn Graph(&self) -> KnowledgeGraph
    {
        let log = String::from_utf8(self.Log()).expect("a UTF-8 log");
        let records = log.lines().map(str::to_owned).collect::<Vec<_>>();
        return Replay_Records(&records).expect("the durable log must remain replayable");
    }

    fn Refused(&self, name: &str, successor: Option<&str>, standing: &str)
    {
        let before = self.Log();
        let output = self.Close(name, successor);
        assert_eq!(output.status.code(), Some(1), "invalid closure must be refused: {}", String::from_utf8_lossy(&output.stderr));
        assert_eq!(self.Log(), before, "a refused act changed the publication log");
        assert!(String::from_utf8_lossy(&output.stderr).contains(standing), "the refusal must explain the precondition: {}", String::from_utf8_lossy(&output.stderr));
    }

    fn Assert_Current(&self, name: &str, count: usize)
    {
        let graph = self.Graph();
        assert!(graph.Current().Concepts().iter().any(|held| return held.Canonical_Name() == name));
        assert_eq!(graph.Current().Concepts().len(), count);
        let history = Ran(&["history", "--store", &self.store]);
        Succeeded(&history);
        let text = String::from_utf8(history.stdout).expect("a UTF-8 history report");
        let reported = text.lines().find_map(|line| return line.strip_prefix("concepts")).expect("history reports concepts").trim();
        assert_eq!(reported, count.to_string(), "the fresh CLI replay must agree with the durable graph");
    }

    fn Assert_Merge_Evidence(&self)
    {
        let lines = Answer_Tool_Call(&self.Graph(), ToolName::Named("merge_losers"), "").expect("the actual merge query");
        assert_eq!(lines, vec![format!("alpha -> {} ({REASON})", Concept::Named("beta").Identity().Render())], "the prior successor and reason must survive the refusal");
    }
}

impl Drop for Fixture
{
    fn drop(&mut self)
    {
        std::fs::remove_dir_all(&self.root).expect("a removable temporary fixture");
    }
}

fn Ran(arguments: &[&str]) -> Output
{
    return Command::new(KWB).args(arguments).output().expect("the built CLI should run");
}

fn Succeeded(output: &Output)
{
    assert!(output.status.success(), "command failed: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn Test_Closing_Should_Refuse_A_Concept_Nobody_Published()
{
    let fixture = Fixture::Of("unpublished");
    for successor in [None, Some("beta")]
    {
        fixture.Refused("unknown", successor, "nothing published");
    }
    fixture.Assert_Current("alpha", 2);
}

#[test]
fn Test_Closing_Should_Refuse_A_Retired_Concept()
{
    let fixture = Fixture::Of("retired");
    Succeeded(&fixture.Close("alpha", None));
    for successor in [None, Some("beta")]
    {
        fixture.Refused("alpha", successor, "retired");
    }
    fixture.Assert_Current("beta", 1);
}

#[test]
fn Test_Retiring_A_Merge_Loser_Should_Preserve_Its_Successor_And_Reason()
{
    let fixture = Fixture::Of("merge-loser");
    Succeeded(&fixture.Close("alpha", Some("beta")));
    fixture.Refused("alpha", None, "superseded");
    fixture.Refused("alpha", Some("beta"), "superseded");
    fixture.Assert_Merge_Evidence();
    fixture.Assert_Current("beta", 1);
}

#[test]
fn Test_Supersession_Should_Refuse_A_Cycle_And_Keep_The_Current_Successor()
{
    let fixture = Fixture::Of("cycle");
    Succeeded(&fixture.Close("alpha", Some("beta")));
    fixture.Refused("beta", Some("alpha"), "superseded");
    fixture.Assert_Merge_Evidence();
    fixture.Assert_Current("beta", 1);
}

#[test]
fn Test_Supersession_Should_Refuse_A_Retired_Successor()
{
    let fixture = Fixture::Of("retired-successor");
    Succeeded(&fixture.Close("beta", None));
    fixture.Refused("alpha", Some("beta"), "retired");
    fixture.Assert_Current("alpha", 1);
}

#[test]
fn Test_Supersession_Should_Still_Refuse_An_Unpublished_Successor()
{
    let fixture = Fixture::Of("unknown-successor");
    fixture.Refused("alpha", Some("unknown"), "nothing published");
    fixture.Assert_Current("alpha", 2);
}