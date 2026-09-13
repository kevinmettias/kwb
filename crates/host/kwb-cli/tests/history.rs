//! `kwb history`, exercised as a person runs it.
//!
//! # Why this drives the binary
//!
//! `kwb-cli` is a composition root, so there is no library to call: the thing under test is the
//! command line. Driving the built binary is also the only way to check what a reader is
//! actually told, which is half of what this command is for.

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

/// The binary this test was built alongside.
const KWB: &str = env!("CARGO_BIN_EXE_kwb");

/// A store nobody else is using, named for the test that made it.
fn Store_For(test: &str) -> PathBuf
{
    let root = std::env::temp_dir().join(format!("kwb-history-{test}"));
    if root.exists()
    {
        std::fs::remove_dir_all(&root).expect("a removable temporary store");
    }
    return root;
}

fn Run(arguments: &[&str]) -> Output
{
    return Command::new(KWB)
        .args(arguments)
        .output()
        .expect("the binary this test was built alongside should run");
}

fn Stdout(output: &Output) -> String
{
    return String::from_utf8_lossy(&output.stdout).into_owned();
}

/// The value of a reported line, by its label.
fn Reported(output: &str, label: &str) -> String
{
    return output
        .lines()
        .find_map(|line| return line.strip_prefix(label))
        .unwrap_or_default()
        .trim()
        .to_owned();
}

/// Admit a source saying one thing about one concept.
fn Admit(store: &Path, file: &Path, contents: &str, concept: &str, claim: &str)
{
    std::fs::write(file, contents).expect("a writable temporary source");
    let output = Run(&[
        "admit",
        &file.display().to_string(),
        "--store",
        &store.display().to_string(),
        "--says",
        concept,
        claim,
    ]);
    assert!(output.status.success(), "admit failed: {}", Stdout(&output));
}

#[test]
fn Test_A_Prefix_Should_Give_The_Graph_Before_A_Later_Publication()
{
    // The property `D-012` recorded as met and nothing could reach until `KWB-60`: the graph at
    // a point is the fold over the publications up to it. The corpus is chosen so the two
    // answers differ by a concept that exists at one point and not the other -- if the prefix
    // were ignored and the whole log replayed, both would report two concepts and this test
    // would fail rather than pass for the wrong reason.
    let store = Store_For("prefix");
    let directory = store.join("sources");
    std::fs::create_dir_all(&directory).expect("a writable temporary directory");

    Admit(&store, &directory.join("one.txt"), "the first source", "entropy", "It does not fall.");
    let after_first = Stdout(&Run(&["history", "--store", &store.display().to_string()]));
    let first_length: usize = Reported(&after_first, "through")
        .split_whitespace()
        .next()
        .and_then(|count| return count.parse().ok())
        .expect("history reports how many publications it replayed");

    Admit(&store, &directory.join("two.txt"), "the second source", "enthalpy", "It is a potential.");
    let now = Stdout(&Run(&["history", "--store", &store.display().to_string()]));
    assert_eq!(Reported(&now, "concepts"), "2", "the second admission did not land: {now}");

    let earlier = Stdout(&Run(&[
        "history",
        "--store",
        &store.display().to_string(),
        "--through",
        &first_length.to_string(),
    ]));

    assert_eq!(
        Reported(&earlier, "concepts"),
        "1",
        "replaying a prefix gave the whole log, so the graph as of a point is the graph as of \
         now and the capability is not there: {earlier}"
    );
}

#[test]
fn Test_Asking_For_More_History_Than_Exists_Should_Be_Refused()
{
    // Refused rather than clamped. A run given everything when it asked for more would be told
    // the corpus is older than it is, and could not tell that from a corpus that really is that
    // old -- the shape `Coverage` exists to keep apart, one layer up.
    let store = Store_For("overrun");
    let directory = store.join("sources");
    std::fs::create_dir_all(&directory).expect("a writable temporary directory");
    Admit(&store, &directory.join("one.txt"), "the only source", "entropy", "It does not fall.");

    let output = Run(&[
        "history",
        "--store",
        &store.display().to_string(),
        "--through",
        "9999",
    ]);

    assert!(!output.status.success(), "an impossible count was accepted: {}", Stdout(&output));
    let complaint = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(
        complaint.contains("9999"),
        "the refusal does not say what was asked for: {complaint}"
    );
}

#[test]
fn Test_History_Without_A_Store_Should_Say_Why_Rather_Than_Report_An_Empty_Graph()
{
    // Without `--store` there is no log, and a graph replayed from no log is empty. Reporting
    // zero concepts would be true of the value and false about the corpus, which is the
    // distinction this repository spends most of its types on.
    let output = Run(&["history"]);

    assert!(!output.status.success(), "a history with no log reported success");
    let complaint = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(
        complaint.contains("--store"),
        "the refusal does not say what is missing: {complaint}"
    );
}
