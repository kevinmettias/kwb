//! Closing publications must occur at their recorded time in historical queries.

use kwb_domain::Published_At;
use kwb_platform_xvpe::{PublicationClock as _, SystemClock};
use std::path::PathBuf;
use std::process::{Command, Output};

const KWB: &str = env!("CARGO_BIN_EXE_kwb");

struct Fixture
{
    root: PathBuf,
    store: String,
}

impl Fixture
{
    fn Of(test: &str) -> Self
    {
        let root = std::env::temp_dir().join(format!("kwb-closing-time-{}-{test}", std::process::id()));
        std::fs::create_dir(&root).expect("an unused temporary directory");
        let source = root.join("source.txt");
        std::fs::write(&source, "Entropy and enthalpy are properties.").expect("a writable source");
        let fixture = Self { store: root.join("store").display().to_string(), root };
        Ran(&["admit", &source.display().to_string(), "--store", &fixture.store, "--says", "entropy", "It does not fall.", "--says", "enthalpy", "It is a potential."]);
        return fixture;
    }

    fn Close(&self, verb: &str)
    {
        let mut arguments = vec![verb, "entropy"];
        if verb == "supersede"
        {
            arguments.extend_from_slice(&["--into", "enthalpy"]);
        }
        arguments.extend_from_slice(&["--store", &self.store, "--because", "The reference revised this concept."]);
        Ran(&arguments);
    }

    fn Records(&self) -> Vec<String>
    {
        return std::fs::read_to_string(self.root.join("store/publications.log")).expect("the published log").lines().map(str::to_owned).collect();
    }

    fn Closure_Time(&self) -> i64
    {
        let records = self.Records();
        let closure = records.last().expect("the closure should have a record");
        return Published_At(closure).expect("every closure should carry its publication time");
    }

    fn Current_Concepts_At(&self, at: i64) -> usize
    {
        let output = Ran(&["history", "--store", &self.store, "--as-of", &at.to_string()]);
        let text = String::from_utf8(output.stdout).expect("a UTF-8 report");
        return text.lines().find_map(|line| return line.strip_prefix("concepts")).expect("history reports current concepts").trim().parse().expect("a count");
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
    let output = Command::new(KWB).args(arguments).output().expect("the built CLI should run");
    assert!(output.status.success(), "command failed: {}", String::from_utf8_lossy(&output.stderr));
    return output;
}

#[test]
fn Test_Each_Closure_Should_Carry_An_Instant_Inside_Its_Command_Window()
{
    for verb in ["retire", "supersede"]
    {
        let fixture = Fixture::Of(verb);
        let before = SystemClock.Now().Unix_Seconds();
        fixture.Close(verb);
        let after = SystemClock.Now().Unix_Seconds();
        let at = fixture.Closure_Time();
        assert!(before <= at && at <= after, "closure time {at} outside {before}..={after}");
    }
}

#[test]
fn Test_A_Supersession_Should_Not_Apply_Before_Its_Publication_Time()
{
    let fixture = Fixture::Of("historical");
    let records = fixture.Records();
    let admission = records.last().and_then(|record| return Published_At(record)).expect("a timestamped admission");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    fixture.Close("supersede");
    let closure = fixture.Closure_Time();
    assert!(closure > admission, "the fixture must separate the two events in time");
    assert_eq!(fixture.Current_Concepts_At(admission), 2, "a future supersession changed the past");
    assert_eq!(fixture.Current_Concepts_At(closure), 1, "the supersession did not close its concept");
}
