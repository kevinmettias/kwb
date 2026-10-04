//! Controls-only --says must behave like a blank proposal and publish no knowledge.

use std::path::{Path, PathBuf};
use std::process::Command;

struct Fixture(PathBuf);

impl Drop for Fixture
{
    fn drop(&mut self)
    {
        std::fs::remove_dir_all(&self.0).expect("a removable fixture directory");
    }
}

fn Admitted(root: &Path, name: &str, concept: &str, claim: &str) -> String
{
    let output = Command::new(env!("CARGO_BIN_EXE_kwb"))
        .args(["admit", &root.join("source.txt").display().to_string(), "--store", &root.join(name).display().to_string(), "--says", concept, claim])
        .output().expect("the built CLI should run");
    assert!(output.status.success(), "admission failed: {}", String::from_utf8_lossy(&output.stderr));
    return String::from_utf8(output.stdout).expect("a UTF-8 report");
}

fn Reported<'a>(output: &'a str, label: &str) -> &'a str
{
    return output.lines().find_map(|line| return line.strip_prefix(label)).expect("the report should name this count").trim();
}

#[test]
fn Test_Controls_Only_Statements_Should_Be_Counted_As_Refused_Without_Publishing_Knowledge()
{
    let root = std::env::temp_dir().join(format!("kwb-control-only-says-{}", std::process::id()));
    std::fs::create_dir(&root).expect("an unused fixture directory");
    let fixture = Fixture(root);
    std::fs::write(fixture.0.join("source.txt"), "A real source.").expect("a writable fixture source");
    let blank = Admitted(&fixture.0, "blank", "", "");
    let controls = Admitted(&fixture.0, "controls", "\u{1}", "\u{7}");
    assert_eq!(Reported(&controls, "concepts"), "0");
    assert_eq!(Reported(&controls, "claims"), "0");
    assert_eq!(Reported(&controls, "refused"), "1");
    assert_eq!(Reported(&controls, "coverage"), Reported(&blank, "coverage"), "controls changed the coverage for a refused proposal");
    let log = std::fs::read_to_string(fixture.0.join("controls/publications.log")).expect("the admission log");
    assert!(!log.lines().any(|line| return line.starts_with("concept\u{1f}") || line.starts_with("claim\u{1f}")), "an empty canonical entity was published");
}
