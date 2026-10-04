//! Closure reasons must preserve a replayable log across separate CLI processes.

use std::path::PathBuf;
use std::process::{Command, Output};

const KWB: &str = env!("CARGO_BIN_EXE_kwb");
const REASONS: [&str; 6] = ["a\u{1f}b", "first\nsecond", "first\r\nsecond", "a\u{85}b", "\tleading", "a\u{2028}b"];
const VERBS: [&str; 2] = ["retire", "supersede"];

struct Fixture
{
    root: PathBuf,
    store: String,
    source: String,
}

impl Fixture
{
    fn Of(test: &str, verb: &str, index: usize) -> Self
    {
        let root = std::env::temp_dir().join(format!("kwb-closing-reason-{}-{test}-{verb}-{index}", std::process::id()));
        std::fs::create_dir(&root).expect("an unused temporary directory");
        let source = root.join("source.txt");
        std::fs::write(&source, "Entropy and enthalpy are properties.").expect("a writable source");
        let fixture = Self { store: root.join("store").display().to_string(), source: source.display().to_string(), root };
        Succeeded(&fixture.Admit());
        return fixture;
    }

    fn Admit(&self) -> Output
    {
        return Ran(&["admit", &self.source, "--store", &self.store, "--says", "entropy", "It does not fall.", "--says", "enthalpy", "It is a potential."]);
    }

    fn Close(&self, verb: &str, reason: &str) -> Output
    {
        let mut arguments = vec![verb, "entropy"];
        if verb == "supersede"
        {
            arguments.extend_from_slice(&["--into", "enthalpy"]);
        }
        arguments.extend_from_slice(&["--store", &self.store, "--because", reason]);
        return Ran(&arguments);
    }

    fn Log(&self) -> Vec<u8>
    {
        return std::fs::read(self.root.join("store/publications.log")).expect("the published log");
    }

    fn Admit_Another_Source(&self)
    {
        let source = self.root.join("another.txt");
        std::fs::write(&source, "The new source speaks about pressure.").expect("a writable second source");
        Succeeded(&Ran(&["admit", &source.display().to_string(), "--store", &self.store, "--says", "pressure", "It is a property."]));
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

fn Assert_Recorded_Reason(fixture: &Fixture, verb: &str, reason: &str)
{
    let bytes = fixture.Log();
    let log = String::from_utf8(bytes).expect("a UTF-8 log");
    let standing = if verb == "retire" { "retired" } else { "superseded" };
    let prefix = format!("concept\u{1f}{standing}\u{1f}");
    let record = log.lines().find(|line| return line.starts_with(&prefix)).expect("the closure should be recorded");
    assert_eq!(record.split('\u{1f}').nth(3), Some(kwb_model::Normalize_Text(reason).as_str()), "the recorded reason should be canonical");
}

#[test]
fn Test_Closures_Should_Normalize_Reasons_And_Keep_The_Log_Replayable()
{
    for verb in VERBS
    {
        for (index, reason) in REASONS.iter().enumerate()
        {
            let fixture = Fixture::Of("normalized", verb, index);
            Succeeded(&fixture.Close(verb, reason));
            Assert_Recorded_Reason(&fixture, verb, reason);
            fixture.Admit_Another_Source();
            Succeeded(&Ran(&["history", "--store", &fixture.store]));
        }
    }
}

#[test]
fn Test_Closures_Should_Refuse_Controls_Only_Reasons_Without_Changing_The_Log()
{
    for verb in VERBS
    {
        for (index, reason) in ["\u{1}", "\u{1}\u{7}"].iter().enumerate()
        {
            let fixture = Fixture::Of("empty", verb, index);
            let before = fixture.Log();
            let output = fixture.Close(verb, reason);
            assert_eq!(output.status.code(), Some(2), "a reason that normalizes to nothing must be refused");
            assert!(String::from_utf8_lossy(&output.stderr).contains("--because is required. D17: destruction requires evidence"));
            assert_eq!(fixture.Log(), before, "a refused closure changed the log");
        }
    }
}
