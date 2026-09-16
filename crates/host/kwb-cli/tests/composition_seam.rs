//! The boundary between this composition root and the six crates it composes, asserted from
//! outside the root.
//!
//! # What a composition root can get wrong
//!
//! `kwb-cli` decides almost nothing. `keeping.rs` says it in as many words: the names are the
//! library's — `DocumentStore`, `FileRecordLog` — and the only thing this crate adds is *which*
//! implementation of each port the binary picked. That is the claim a suite inside the crate
//! cannot check, because a test in `src/` imports the same crates the binary imports and reads the
//! same lines. It is also the claim worth checking, because a composition root that quietly grew a
//! rule of its own looks exactly like a composition root from every seat inside it.
//!
//! So this file drives the built binary — the crate is binary-only, so the command line *is* the
//! public contract, as `history.rs` and `help.rs` say — and then compares what the run left behind
//! against what the crates underneath compute for themselves. The log is read back through
//! `kwb-platform`'s port rather than by opening the file, the graph is rebuilt by replaying it in
//! `kwb-domain`, and every address is recomputed from the bytes by the crate that mints it. A
//! divergence is a failure rather than a comparison of one crate's output with a copy of it.
//!
//! # What is asserted here that is not asserted one crate down
//!
//! Each of these is a property of the *arrangement*, not of either side. `kwb-store` asserts that
//! its own store is content-addressed; this asserts that the binary's choice of store, made in
//! `Store_For` and visible nowhere else, is what a reader of the log finds. `kwb-domain` asserts
//! that a record replays; this asserts that the records the binary wrote are the ones it replays.
//! `kwb-platform-xvpe` asserts that the clock reports a wall time; this asserts that the instants
//! standing in the log are the ones *that* clock reported during *this* run.
//!
//! # Why `composition_seam` and not a source file's stem
//!
//! The unit `check-test-coverage` reads is the test file's own stem, so this file addresses no
//! source file and covers nothing — correctly, since its subject is the boundary between this
//! crate and the ones below it rather than either side's files. It follows `kwb-extract`'s
//! `tests/admission_seam.rs` and `kwb-mcp`'s `tests/retrieval_seam.rs`, which are the same shape
//! for the same reason.

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

use kwb_domain::KnowledgeGraph;
use kwb_domain::Replay_Records;
use kwb_domain::Scope;
use kwb_ingest::Admit_Source;
use kwb_ingest::ClaimText;
use kwb_ingest::ConceptName;
use kwb_ingest::Extraction;
use kwb_ingest::ExtractionLineage;
use kwb_ingest::ExtractionStrategy;
use kwb_ingest::ReaderName;
use kwb_ingest::ReadingKind;
use kwb_ingest::ReadingProtocol;
use kwb_ingest::SourceLocation;
use kwb_ingest::Stated;
use kwb_platform::RecordLogStrategy;
use kwb_platform_std::FileRecordLog;
use kwb_platform_xvpe::PublicationClock;
use kwb_platform_xvpe::PublicationTime;
use kwb_platform_xvpe::SystemClock;
use kwb_store::Document;
use kwb_store::DocumentStore;

/// The binary this test was built alongside.
const KWB: &str = env!("CARGO_BIN_EXE_kwb");

/// The name of the log file a store keeps, which is the one entry under a store root that is not a
/// content address.
const LOG_FILE: &str = "publications.log";

/// The passage every run below is handed.
const PASSAGE: &str = "Entropy does not decrease in an isolated system.";

/// The one thing a person says the passage asserts, about the one concept it is about.
const CONCEPT_NAME: &str = "entropy";
const CLAIM_TEXT: &str = "It does not fall.";

/// The scope one run states, and which a second run declines to state.
const SCOPE_NAME: &str = "physical theory";

/// Where each field of an assertion record sits.
///
/// Named rather than counted at the call site, because a record is positional and a `5` in an
/// assertion tells a reader nothing about which field it is claiming to be.
const CITED_CLAIM: usize = 4;
const CITED_SOURCE: usize = 5;
const CITED_SCOPE: usize = 6;

/// A store nobody else is using, named for the test that made it.
///
/// Not created: the run under test is what creates it, which is how a test can later say the binary
/// put something there rather than that the test did.
fn Store_For(test: &str) -> PathBuf
{
    let root = std::env::temp_dir().join(format!("kwb-cli-seam-{test}"));
    if root.exists()
    {
        std::fs::remove_dir_all(&root).expect("a removable temporary store");
    }
    return root;
}

/// A directory of sources, and an empty one, both named for the test that made them.
///
/// The sources are kept outside the store deliberately, so that everything under the store root is
/// something the run put there and a count of what is under it means what it says.
fn Directory_For(test: &str) -> PathBuf
{
    let root = std::env::temp_dir().join(format!("kwb-cli-seam-{test}-world"));
    if root.exists()
    {
        std::fs::remove_dir_all(&root).expect("a removable temporary directory");
    }
    std::fs::create_dir_all(&root).expect("a writable temporary directory");
    return root;
}

fn Run(arguments: &[&str]) -> Output
{
    return Command::new(KWB)
        .args(arguments)
        .output()
        .expect("the binary this test was built alongside should run");
}

fn Run_In(directory: &Path, arguments: &[&str]) -> Output
{
    return Command::new(KWB)
        .current_dir(directory)
        .args(arguments)
        .output()
        .expect("the binary this test was built alongside should run");
}

fn Stdout_Text(output: &Output) -> String
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

/// A source file holding this file's passage, written where the run can be pointed at it.
fn Source_File(world: &Path, name: &str) -> PathBuf
{
    let path = world.join(name);
    std::fs::write(&path, PASSAGE).expect("a writable temporary source");
    return path;
}

/// Admit one source into a store, through the built binary.
fn Admit_Into(store: &Path, source: &Path, flags: &[&str]) -> Output
{
    let source = source.display().to_string();
    let root = store.display().to_string();
    let mut arguments = vec!["admit", source.as_str(), "--store", root.as_str()];
    arguments.extend_from_slice(flags);

    return Run(&arguments);
}

/// The records a store's log holds, read through the port rather than off the disk.
///
/// `FileRecordLog::Records` is a method of `kwb-platform`'s trait, so reading through this is also
/// what puts the port in scope: the binary picks the implementation in `Log_For`, and this is the
/// half that says the log it wrote is one the port can read.
fn Records_At(store: &Path) -> Vec<String>
{
    let log = FileRecordLog::At(store.join(LOG_FILE)).expect("a log the binary's own run opened");
    return log.Records().expect("a log the port can read back");
}

/// One field of a record, by its position.
fn Field(record: &str, index: usize) -> String
{
    return record
        .split('\u{1F}')
        .nth(index)
        .expect("a record carries every field its kind is written with")
        .to_owned();
}

/// The instant a record was written at, which is its last field.
fn Instant_Of(record: &str) -> i64
{
    return record
        .rsplit('\u{1F}')
        .next()
        .and_then(|field| return field.parse().ok())
        .expect("every record this repository writes now ends in the instant it happened");
}

/// The addresses of a graph's current concepts, and the texts of its current claims.
///
/// Addressed rather than counted, because a count is satisfied by a different concept of the same
/// size. The address is what the whole workspace uses to say *this is the same thing*, so a test
/// comparing two graphs should say it in that vocabulary.
fn Concepts_Of(graph: &KnowledgeGraph) -> Vec<String>
{
    return graph
        .Current()
        .Concepts()
        .into_iter()
        .map(|concept| return concept.Identity().Render())
        .collect();
}

fn Claims_Of(graph: &KnowledgeGraph) -> Vec<String>
{
    return graph
        .Current()
        .Claims()
        .into_iter()
        .map(|claim| return claim.Text().to_owned())
        .collect();
}

/// The one assertion record a store's log holds, which is the citation every other assertion here
/// is about.
fn Citation_In(store: &Path) -> String
{
    return Records_At(store)
        .into_iter()
        .find(|record| return record.starts_with("assertion"))
        .expect("an admission that yielded a claim cites the source it was read out of");
}

/// The content addresses a store root holds: every entry that is not the publication log.
fn Held_Addresses(store: &Path) -> Vec<String>
{
    let mut addresses: Vec<String> = std::fs::read_dir(store)
        .expect("a store the run created")
        .map(|entry| {
            return entry
                .expect("a readable store entry")
                .file_name()
                .to_string_lossy()
                .into_owned();
        })
        .filter(|name| return name != LOG_FILE)
        .collect();
    addresses.sort();

    return addresses;
}

#[test]
fn Test_The_Composition_Root_Should_Add_No_Rule_To_The_Admission_It_Drives()
{
    // The claim `keeping.rs` makes about this whole crate, checked where a reader of the binary
    // cannot check it. The same bytes and the same reading go through `kwb-ingest` a second time,
    // in process, built from the library's own names; what the binary wrote to its log and what the
    // pipeline publishes must be the same concepts and the same claims.
    //
    // The location, the protocol and the reader name below are deliberately *not* the ones
    // `admission.rs` uses. That is the point rather than carelessness: those literals describe who
    // read the source, and this asserts that they decide nothing about what the graph holds — so
    // the test cannot pass merely by restating the binary's own strings, and it will not fail
    // because somebody renames the reader.
    let store = Store_For("no-rule");
    let world = Directory_For("no-rule");
    let source = Source_File(&world, "one.txt");

    let run = Admit_Into(
        &store,
        &source,
        &["--scope", SCOPE_NAME, "--says", CONCEPT_NAME, CLAIM_TEXT],
    );
    assert!(run.status.success(), "admit failed: {}", Stdout_Text(&run));

    let lineage = ExtractionLineage::Of(
        ReadingProtocol::Named("a protocol this file made up"),
        ReaderName::Named("a reader this file made up"),
    );
    let said = Stated::Of(
        vec![Extraction::New(
            ConceptName::Named(CONCEPT_NAME),
            ClaimText::Stated(CLAIM_TEXT),
        )],
        SourceLocation::Named("a place this file made up"),
        lineage,
        Scope::Named(SCOPE_NAME).expect("a named scope"),
    )
    .expect("a person who said something is a reader");

    let mut documents = DocumentStore::Empty();
    let report = Admit_Source(
        PASSAGE.as_bytes().to_vec(),
        Some(&said as &dyn ExtractionStrategy),
        ReadingKind::Text,
        &mut documents,
    )
    .expect("the source is admitted");
    let published = report.Published_Into(&KnowledgeGraph::Empty());

    assert!(
        !Concepts_Of(&published).is_empty() && !Claims_Of(&published).is_empty(),
        "the corpus this test compares is empty on one side, so the comparison proves nothing"
    );

    let replayed =
        Replay_Records(&Records_At(&store)).expect("the log this repository wrote replays");

    assert_eq!(
        Concepts_Of(&replayed),
        Concepts_Of(&published),
        "the log the binary wrote holds different concepts than the pipeline publishes for the same \
         bytes and the same reading, so the composition root added a rule of its own"
    );
    assert_eq!(
        Claims_Of(&replayed),
        Claims_Of(&published),
        "the log the binary wrote holds different claims than the pipeline publishes for the same \
         bytes and the same reading"
    );
}

#[test]
fn Test_The_Citation_Should_Name_The_Bytes_The_Store_Holds()
{
    // Three ways of naming one document, asserted to be one name: what the report printed, what
    // `kwb-store` derives from the bytes, and what stands in the citation the binary recorded. A
    // composition root that cited a filename, a path or a counter would part from the address the
    // store files the bytes under, and following the citation would stop returning the source.
    let store = Store_For("citation");
    let world = Directory_For("citation");
    let source = Source_File(&world, "one.txt");

    let run = Admit_Into(
        &store,
        &source,
        &["--scope", SCOPE_NAME, "--says", CONCEPT_NAME, CLAIM_TEXT],
    );
    assert!(run.status.success(), "admit failed: {}", Stdout_Text(&run));

    let written = Document::Of(PASSAGE.as_bytes().to_vec()).Identity().Render();
    assert_eq!(
        Reported(&Stdout_Text(&run), "source"),
        written,
        "the report names the source something other than the address its bytes are stored at"
    );
    assert!(
        store.join(&written).is_file(),
        "nothing is filed under the address the report printed, so a reader following it finds \
         nothing"
    );

    let citation = Citation_In(&store);
    assert_eq!(
        Field(&citation, CITED_SOURCE),
        written,
        "the citation names something other than the bytes the claim was read out of: {citation:?}"
    );
    assert_eq!(
        Field(&citation, CITED_SCOPE),
        SCOPE_NAME,
        "the scope the run was given did not reach the assertion it is a scope of: {citation:?}"
    );
}

#[test]
fn Test_An_Absent_Scope_Should_Reach_The_Record_As_Absent()
{
    // `D-010` through the whole arrangement: an unstated scope is a real answer and stays
    // distinguishable from every stated one. The two runs below differ in nothing but that flag, so
    // the concept and the claim must come out byte-identical — the scope participates in the
    // assertion's identity and in nothing else — while the assertion records differ.
    let scoped_store = Store_For("scoped");
    let absent_store = Store_For("absent");
    let world = Directory_For("scope");
    let source = Source_File(&world, "one.txt");

    let scoped = Admit_Into(
        &scoped_store,
        &source,
        &["--scope", SCOPE_NAME, "--says", CONCEPT_NAME, CLAIM_TEXT],
    );
    let absent = Admit_Into(&absent_store, &source, &["--says", CONCEPT_NAME, CLAIM_TEXT]);
    assert!(scoped.status.success(), "admit failed: {}", Stdout_Text(&scoped));
    assert!(absent.status.success(), "admit failed: {}", Stdout_Text(&absent));

    assert_eq!(
        Field(&Citation_In(&absent_store), CITED_SCOPE),
        "",
        "a run that stated no scope recorded one anyway, which is the default `D-010` refuses"
    );

    let scoped_records = Records_At(&scoped_store);
    let absent_records = Records_At(&absent_store);
    assert_eq!(
        scoped_records.first().map(String::as_str),
        absent_records.first().map(String::as_str),
        "the concept record changed with the scope, so the scope reaches something it is not part \
         of: {scoped_records:?} against {absent_records:?}"
    );
    assert_eq!(
        scoped_records.get(1).map(String::as_str),
        absent_records.get(1).map(String::as_str),
        "the claim record changed with the scope, so a claim addresses its reader's scope"
    );
    assert_eq!(
        Field(&Citation_In(&scoped_store), CITED_CLAIM),
        Field(&Citation_In(&absent_store), CITED_CLAIM),
        "two scopes of one claim are two assertions of one claim, and this is not one claim"
    );
    assert_ne!(
        Citation_In(&scoped_store),
        Citation_In(&absent_store),
        "two scopes of one claim produced one assertion record, so the scope is not in the \
         assertion's identity"
    );
}

#[test]
fn Test_Every_Record_Should_Carry_An_Instant_The_Host_Clock_Reported()
{
    // `KWB-64`: a publication is stamped, and the stamp is a wall reading rather than an ordering.
    // The window is the run's own lifetime, taken from the clock the composition root chose — the
    // same `kwb-platform-xvpe` clock `Now()` in `keeping.rs` reads — so a record carrying a zero,
    // an epoch or a constant is outside it, and a record carrying a time from anywhere but this
    // process's clock is outside it too.
    let store = Store_For("instant");
    let world = Directory_For("instant");
    let source = Source_File(&world, "one.txt");

    let before = SystemClock.Now().Unix_Seconds();
    let run = Admit_Into(&store, &source, &["--says", CONCEPT_NAME, CLAIM_TEXT]);
    let after = SystemClock.Now().Unix_Seconds();
    assert!(run.status.success(), "admit failed: {}", Stdout_Text(&run));

    let records = Records_At(&store);
    assert!(!records.is_empty(), "the run published nothing to stamp");

    for record in &records
    {
        let published_at = PublicationTime::From_Unix_Seconds(Instant_Of(record));
        assert!(
            PublicationTime::From_Unix_Seconds(before) <= published_at
                && published_at <= PublicationTime::From_Unix_Seconds(after),
            "the instant on {record:?} is outside the window this run lived in, so it did not come \
             from the clock the composition root composes"
        );
    }
}

#[test]
fn Test_Durability_Should_Follow_The_Store_And_Nothing_Else()
{
    // `D-014`, reported rather than assumed: whether anything was kept is a fact about the run, and
    // the two words the report prints are that fact. Both halves are asserted because either alone
    // is satisfied by a report that always says the same thing.
    let world = Directory_For("durable");
    let source = Source_File(&world, "one.txt");

    let kept = Admit_Into(&Store_For("durable-store"), &source, &[
        "--says",
        CONCEPT_NAME,
        CLAIM_TEXT,
    ]);
    assert!(kept.status.success(), "admit failed: {}", Stdout_Text(&kept));
    let said = Stdout_Text(&kept);
    assert_eq!(
        Reported(&said, "documents"),
        "kept",
        "a run given a store did not report the bytes as kept: {said}"
    );
    assert_eq!(
        Reported(&said, "knowledge"),
        "kept",
        "a run given a store did not report the publications as kept: {said}"
    );

    // And the words are not merely the report's opinion: the run is given a working directory of
    // its own, and a run given no store leaves that directory as it found it.
    let unhoused = Directory_For("nowhere");
    let nowhere = Run_In(&unhoused, &["admit", source.display().to_string().as_str()]);
    assert!(
        nowhere.status.success(),
        "admit failed with no store: {}",
        Stdout_Text(&nowhere)
    );
    let said = Stdout_Text(&nowhere);
    assert_eq!(
        Reported(&said, "documents"),
        "in memory only",
        "a run given no store reported the bytes as kept: {said}"
    );
    assert_eq!(
        Reported(&said, "knowledge"),
        "in memory only",
        "a run given no store reported the publications as kept: {said}"
    );
    assert!(
        std::fs::read_dir(&unhoused)
            .expect("a readable working directory")
            .next()
            .is_none(),
        "a run given no store wrote something in its working directory, so durability does not \
         follow the store and nothing else"
    );
}

#[test]
fn Test_Admitting_One_Source_Twice_Should_Keep_One_Document()
{
    // The property that makes admitting a corpus again free, arriving through the composition
    // root's choice of store: the address is the content, so the same bytes are the same document
    // however many runs produce them. Both halves matter — the store does not grow, and the graph
    // the log replays into does not either.
    let store = Store_For("twice");
    let world = Directory_For("twice");
    let source = Source_File(&world, "one.txt");
    let flags = ["--says", CONCEPT_NAME, CLAIM_TEXT];

    let first = Admit_Into(&store, &source, &flags);
    assert!(first.status.success(), "admit failed: {}", Stdout_Text(&first));
    let after_one = Records_At(&store).len();

    let second = Admit_Into(&store, &source, &flags);
    assert!(second.status.success(), "admit failed: {}", Stdout_Text(&second));

    assert_eq!(
        Held_Addresses(&store),
        [Document::Of(PASSAGE.as_bytes().to_vec()).Identity().Render()],
        "a second run of one source left a second document behind, so the store is not addressing \
         bytes by their content"
    );
    assert!(
        Records_At(&store).len() > after_one,
        "the second run recorded nothing, so this test cannot tell a re-admission from a run that \
         did not happen"
    );

    let replayed = Replay_Records(&Records_At(&store)).expect("the log this repository wrote replays");
    assert_eq!(
        Concepts_Of(&replayed).len(),
        1,
        "admitting one source twice grew the graph, so a rerun over a corpus duplicates every \
         source it touches"
    );
    assert_eq!(Claims_Of(&replayed).len(), 1, "and the claim was duplicated with it");
}
