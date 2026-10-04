//! Within-run repeats count as repeats; distinct assertions and later runs stay admissible.

use std::path::PathBuf;
use std::process::Command;

struct Fixture(PathBuf);

impl Fixture
{
    fn New(name: &str) -> Self
    {
        let root = std::env::temp_dir().join(format!("kwb-repeated-assertion-{}-{name}", std::process::id()));
        std::fs::create_dir(&root).expect("an unused fixture directory");
        std::fs::write(root.join("source.txt"), "A source asserting two propositions.").expect("a writable source");
        return Self(root);
    }

    fn Admit(&self, claims: &[&str]) -> String
    {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kwb"));
        command.arg("admit").arg(self.0.join("source.txt")).arg("--store").arg(self.0.join("store"));
        for claim in claims
        {
            command.args(["--says", "entropy", claim]);
        }
        let output = command.output().expect("the built CLI should run");
        assert!(output.status.success(), "admission failed: {}", String::from_utf8_lossy(&output.stderr));
        return String::from_utf8(output.stdout).expect("a UTF-8 report");
    }

    fn Records(&self) -> Vec<String>
    {
        return std::fs::read_to_string(self.0.join("store/publications.log")).expect("a publication log")
            .lines().map(str::to_owned).collect();
    }
}

impl Drop for Fixture
{
    fn drop(&mut self)
    {
        std::fs::remove_dir_all(&self.0).expect("a removable fixture directory");
    }
}

fn Count_Of(output: &str, field: &str) -> usize
{
    return output.lines().find_map(|line| return line.strip_prefix(field)).expect("the report must print this field")
        .trim().parse().expect("a numeric report field");
}

#[test]
fn Test_One_Assertion_Repeated_Should_Append_Only_One_Claim_And_Citation_And_Count_The_Repeat()
{
    let once = Fixture::New("once");
    let twice = Fixture::New("twice");
    let once_output = once.Admit(&["It is non-decreasing."]);
    let twice_output = twice.Admit(&["It is non-decreasing.", "It is non-decreasing."]);
    assert_eq!(Count_Of(&twice_output, "repeated"), 1);
    assert_eq!(Count_Of(&once_output, "repeated"), 0);
    assert_eq!(Count_Of(&twice_output, "claims"), 1);
    assert_eq!(Count_Of(&twice_output, "citations"), 1);
    let records = twice.Records();
    assert_eq!(records.len(), once.Records().len(), "a repeat appended extra publication records");
    assert_eq!(records.iter().filter(|line| return line.starts_with("claim\u{1f}")).count(), 1);
    assert_eq!(records.iter().filter(|line| return line.starts_with("assertion\u{1f}")).count(), 1);
}

#[test]
fn Test_Two_Different_Claims_Of_One_Concept_Should_Both_Be_Published()
{
    let fixture = Fixture::New("different");
    let output = fixture.Admit(&["It is non-decreasing.", "It is extensive."]);
    assert_eq!(Count_Of(&output, "repeated"), 0);
    assert_eq!(Count_Of(&output, "claims"), 2);
    assert_eq!(Count_Of(&output, "citations"), 2);
    assert_eq!(fixture.Records().len(), 5, "one concept, two claims and two assertions must be published");
}

#[test]
fn Test_One_Assertion_Offered_In_A_Later_Run_Should_Not_Be_Refused_As_An_In_Run_Repeat()
{
    let fixture = Fixture::New("later-run");
    fixture.Admit(&["It is non-decreasing."]);
    let before = fixture.Records().len();
    let output = fixture.Admit(&["It is non-decreasing."]);
    assert_eq!(Count_Of(&output, "repeated"), 0);
    assert_eq!(fixture.Records().len(), before.checked_mul(2).expect("the fixture count should fit"));
    assert_eq!(Count_Of(&output, "claims"), 1);
    assert_eq!(Count_Of(&output, "citations"), 1, "replay folds the later publication by identity");
}
