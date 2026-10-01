//! The crate root's own surface: what a store replays to, and what the listing says about it.
//!
//! # Why this is the companion of `src/lib.rs`, and why it is a suite rather than an in-file module
//!
//! The unit is the file, so this file's *name* is what addresses `Corpus_At`,
//! `Answer_Tool_Call` and `Print_Surface`. `tests/tool_surface.rs` drives more of the surface
//! than this file does and addresses none of it: its own unit is `tool_surface`, which is no
//! source file's stem, so the dispatcher it exercises from ten directions is — to a rule that
//! reads the unit — a dispatcher no test names.
//!
//! `Print_Surface` is why this is an integration suite and not an inline `#[cfg(test)] mod
//! tests`. It writes to standard output, so a test that called it in-process would have nothing
//! to assert on, and the claim worth making about a listing is about what a *reader* sees. That
//! means running the binary, and `CARGO_BIN_EXE_kwb-mcp` is set only when building an
//! integration test. `kwb-cli`'s `tests/help.rs` reads its own help through the binary for the
//! same reason, and gives the fuller argument there.

use std::path::{Path, PathBuf};
use std::process::Command;

use kwb_domain::{Concept, KnowledgeGraph, Publication, Standing, Versioned};
use kwb_mcp::{Answer_Tool_Call, Corpus_At, ToolName};
use kwb_platform::RecordLogStrategy;
use kwb_platform_std::FileRecordLog;

/// The binary this test was built alongside.
const KWB_MCP: &str = env!("CARGO_BIN_EXE_kwb-mcp");

/// A store directory nothing else is using, named for the test that asked for it.
fn Scratch_Store(name: &str) -> PathBuf
{
    let root = std::env::temp_dir().join(format!("kwb-mcp-test-{name}"));
    if let Err(error) = std::fs::remove_dir_all(&root)
    {
        assert_eq!(
            error.kind(),
            std::io::ErrorKind::NotFound,
            "the scratch store could not be cleared: {error}"
        );
    }
    return root;
}

/// The store directory as the command line spells it.
///
/// `Corpus_At` takes an `Option<&str>` because that is what a command line hands it, so the
/// scratch path has to survive the same conversion a real argument does rather than being
/// passed as a `Path`.
fn Scratch_Name(root: &Path) -> &str
{
    return root.to_str().expect("a path under the temp directory is text on every platform this builds for");
}

/// Publish one concept into a store, through the same log writer a run uses.
fn Publish_Entropy(root: &Path)
{
    let log = FileRecordLog::At(root.join("publications.log"))
        .expect("At creates the log file and its directory");
    let published = Publication::Concept {
        concept: Concept::Named("entropy"),
        standing: Standing::Asserted,
    };

    return log
        .Append(&published.Record(None))
        .expect("the log file exists and is opened for appending");
}

/// What `kwb-mcp` printed, asserting that it got far enough to print anything.
///
/// A listing runs on the success path, so a non-zero exit here is the subject failing rather
/// than the test measuring something else: read the status before the bytes.
fn Listing(arguments: &[&str]) -> String
{
    let printed = Command::new(KWB_MCP)
        .args(arguments)
        .output()
        .expect("the binary this test was built alongside should run");

    assert!(
        printed.status.success(),
        "kwb-mcp {arguments:?} exited {:?}: {}",
        printed.status.code(),
        String::from_utf8_lossy(&printed.stderr)
    );
    return String::from_utf8_lossy(&printed.stdout).into_owned();
}

#[test]
fn Test_Corpus_At_Should_Answer_An_Empty_Graph_When_No_Store_Was_Named()
{
    // Not a failure: a host started with no store serves an empty corpus and says so. Reading
    // "no store" as an error would make the no-argument listing — the one that tells a person
    // what the surface holds — unreachable.
    let graph = Corpus_At(None).expect("naming no store is an empty corpus, not a refusal");

    assert_eq!(graph.Current().Concepts().len(), 0);
    assert_eq!(graph.Every_Version().Concepts().len(), 0);
}

#[test]
fn Test_Corpus_At_Should_Replay_The_Log_Of_The_Store_It_Was_Pointed_At()
{
    let root = Scratch_Store("replays");
    Publish_Entropy(&root);

    let graph = Corpus_At(Some(Scratch_Name(&root))).expect("the publication log replays");

    let names: Vec<String> = graph
        .Current()
        .Concepts()
        .into_iter()
        .map(|concept| return concept.Canonical_Name().to_owned())
        .collect();
    assert_eq!(
        names,
        ["entropy"],
        "the concept the log published is not in the graph the store replayed to"
    );
}

#[test]
fn Test_Corpus_At_Should_Refuse_A_Log_It_Cannot_Replay_Rather_Than_Serve_Part_Of_It()
{
    // The refusal is the whole reason this returns a `Result`: a corpus missing the records
    // after the first bad one would answer confidently about knowledge it does not have, which
    // is the failure `Corpus_At`'s own doc gives for refusing rather than serving partially.
    let root = Scratch_Store("cannot-replay");
    std::fs::create_dir_all(&root).expect("the scratch store is creatable");
    std::fs::write(root.join("publications.log"), "a record of no kind this reader knows\n")
        .expect("the scratch log is writable");

    let complaint = Corpus_At(Some(Scratch_Name(&root)))
        .expect_err("a log whose first record will not replay is refused");

    assert!(
        complaint.contains("publication log cannot be replayed"),
        "the refusal does not say which part of the store failed: {complaint}"
    );
}

#[test]
fn Test_Answer_Tool_Call_Should_Refuse_A_Name_No_Tool_Declares()
{
    // `None` rather than an empty answer, so an agent can tell a typo from a corpus that holds
    // nothing — the distinction `tests/tool_surface.rs` states and this pins in the unit that
    // owns the dispatcher.
    let graph = KnowledgeGraph::Empty();

    assert!(
        Answer_Tool_Call(&graph, ToolName::Named("delete_everything"), "").is_none(),
        "an undeclared tool answered rather than being refused"
    );
}

#[test]
fn Test_Answer_Tool_Call_Should_Answer_A_Declared_Tool_Over_The_Corpus_It_Was_Given()
{
    let graph = KnowledgeGraph::Empty().With_Concept(Versioned::Asserted(Concept::Named("entropy")));

    assert_eq!(
        Answer_Tool_Call(&graph, ToolName::Named("get_concept"), "entropy"),
        Some(vec!["entropy".to_owned()]),
        "the dispatcher did not reach the corpus it was handed"
    );
}

#[test]
fn Test_Print_Surface_Should_Say_The_Corpus_Is_Empty_When_No_Store_Was_Named()
{
    let listed = Listing(&[]);

    assert!(
        listed.contains("kwb-mcp: 5 read-only tools"),
        "the listing does not say what the surface holds: {listed}"
    );
    assert!(
        listed.contains("corpus     0 current, 0 held"),
        "the listing does not count the corpus it is over: {listed}"
    );
    assert!(
        listed.contains("Pass a store directory to serve a real corpus"),
        "a listing over an empty graph does not say so, and that sentence is the only thing \
         `Store::Is_Absent` is read for: {listed}"
    );
}

#[test]
fn Test_Print_Surface_Should_Render_The_Corpus_And_Drop_The_Empty_Sentence_Once_A_Store_Was_Named()
{
    // Both halves in one reading, because they are one decision: the sentence turns on
    // `Store::Is_Absent`, and the count turns on the graph `Corpus_At` replayed. A listing that
    // printed the count and kept the sentence would tell a reader the opposite of what they
    // asked for.
    let root = Scratch_Store("named");
    Publish_Entropy(&root);

    let listed = Listing(&[Scratch_Name(&root)]);

    assert!(
        listed.contains("corpus     1 current, 1 held"),
        "the listing did not render the corpus the store replayed to: {listed}"
    );
    assert!(
        !listed.contains("Pass a store directory"),
        "a listing given a store still says the corpus is empty: {listed}"
    );
}
