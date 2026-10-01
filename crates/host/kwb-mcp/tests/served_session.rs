//! The served session: what an agent actually reaches, and whether it is the same thing a
//! person at a terminal reaches.
//!
//! # Why this suite runs over in-memory lines rather than a socket
//!
//! The framing is `BufRead::lines` over anything that reads and writes bytes, so a session a
//! test drives through a `&[u8]` and a session a client drives through this process's own pipes
//! are the same code on the same path. A test that opened a socket would be testing the socket.
//!
//! # What is asserted, and why each is a distinct claim
//!
//! - **The listing is the surface.** Not "a listing came back": the names *and the sentences* are
//!   compared against [`TOOLS`] in order, which is the whole of "projected from `TOOLS` rather
//!   than restated". A descriptor table that drifted from the dispatched one would pass a test
//!   that only counted tools.
//! - **A call answers what the command line prints.** The same store, the same tool and the same
//!   argument are driven both ways, and the binary's own standard output is the expectation —
//!   not a second call to the library, which would compare the implementation with itself.
//! - **A call the catalogue cannot read fails the call, not the session.** The seam's own
//!   contract is that a bad call must not end a session, and that is a property of this crate's
//!   answer rather than of the backend that delivered it.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use kwb_domain::{Assertion, Claim, Concept, KnowledgeGraph, Publication, Scope, Standing};
use kwb_mcp::{Catalogue, Corpus_At, TOOLS};
use kwb_platform::RecordLogStrategy;
use kwb_platform_std::FileRecordLog;
use kwb_platform_xvpe::remote_call::Serve_Tools;
use kwb_platform_xvpe::remote_call::ToolCatalogStrategy;
use serde_json::Value;

/// The binary this test was built alongside.
const KWB_MCP: &str = env!("CARGO_BIN_EXE_kwb-mcp");

/// The word the command line is given to ask for a session rather than an answer.
///
/// Spelled here rather than read from the binary's own constant, for the same reason the
/// protocol revision above is: what is being pinned is the word a *client* types, and a test that
/// imported the value would go on passing if the word changed.
const SERVE: &str = "--serve";

/// The claim the served corpus holds.
const CLAIM: &str = "It is non-decreasing in an isolated system.";

/// The words a query has to carry to reach [`CLAIM`], and the argument the call below sends.
const QUERY: &str = "isolated system";

/// The protocol revision this server declares to a client.
///
/// A literal rather than a constant read out of the backend: what is being pinned is what a
/// *client* sees, and a test that read the value from the thing it is checking would go on
/// passing if the value changed. The revision this workspace adopts is part of `KWB-112`'s
/// subject, and moving it is a decision rather than maintenance.
const DECLARED_REVISION: &str = "2024-11-05";

/// Text a call could carry as its arguments that is not a document at all.
///
/// No line can deliver this: the backend renders every payload as a document before the
/// catalogue sees it. It is here because the seam promises a call is *never* a panic, and a
/// promise about what cannot arrive through this repository's own front door is still a promise
/// to whoever implements or drives the trait next.
const NOT_A_DOCUMENT: &str = "not a document at all";

/// A document this catalogue can read and will not accept.
///
/// This one a client can send, and it is the interesting case: the call is well formed, the
/// protocol layer is satisfied, and the argument is of a type no tool can be asked with.
const IMPOSSIBLE_ARGUMENT: &str = r#"{"argument":5}"#;

/// A store directory nothing else is using, named for the test that asked for it.
fn Scratch_Store(name: &str) -> PathBuf
{
    let root = std::env::temp_dir().join(format!("kwb-mcp-served-{name}"));
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
fn Scratch_Name(root: &Path) -> &str
{
    return root
        .to_str()
        .expect("a path under the temp directory is text on every platform this builds for");
}

/// Publish one concept, one claim and one citation, in the order a replay reads them.
///
/// The same three records `kwb admit` writes, so the session below is served a corpus a real
/// run could have produced rather than one assembled in memory.
fn Publish_Corpus(root: &Path)
{
    let log = FileRecordLog::At(root.join("publications.log"))
        .expect("At creates the log file and its directory");
    let concept = Concept::Named("entropy");
    let claim = Claim::About(&concept, CLAIM);
    let assertion = Assertion::By(
        "Callen 1985",
        &claim,
        Scope::Named("physical theory").expect("a named scope"),
    );

    let published = [
        Publication::Concept {
            concept,
            standing: Standing::Asserted,
        },
        Publication::Claim {
            claim,
            standing: Standing::Asserted,
        },
        Publication::Assertion {
            assertion,
            standing: Standing::Asserted,
        },
    ];

    for publication in published
    {
        log.Append(&publication.Record(None))
            .expect("the log file exists and is opened for appending");
    }
}

/// The corpus the session is served and the command line is pointed at.
///
/// Named by the test that asks, because the suite runs its tests at once and a store two of them
/// shared would have one deleting the file the other is reading — which is how this helper was
/// written the first time, and it failed on Windows with a permission error rather than on a
/// stale count.
fn Published_Corpus(name: &str) -> (PathBuf, KnowledgeGraph)
{
    let root = Scratch_Store(name);
    Publish_Corpus(&root);

    let graph = Corpus_At(Some(Scratch_Name(&root))).expect("the published log replays");
    return (root, graph);
}

/// Every answer a session returned for `requests`, in order, each as its own document.
///
/// The count is asserted here rather than at each call site: fewer answers than requests is the
/// framing's own failure, and every test below reads a position rather than searching.
fn Answers(graph: &KnowledgeGraph, requests: &[&str]) -> Vec<Value>
{
    let catalogue = Catalogue::Over(graph);
    let session = format!("{}\n", requests.join("\n"));

    let mut answered = Vec::new();
    Serve_Tools(&catalogue, session.as_bytes(), &mut answered)
        .expect("an in-memory session cannot fail to read or write");

    let text = String::from_utf8(answered).expect("a JSON-RPC answer is UTF-8");
    let documents: Vec<Value> = text
        .lines()
        .map(|line| return serde_json::from_str(line).expect("every answer is one JSON document"))
        .collect();

    assert_eq!(
        documents.len(),
        requests.len(),
        "the session answered {} of {} requests, so the framing dropped one",
        documents.len(),
        requests.len()
    );
    return documents;
}

/// The `result` of an answer, which every request in this suite is answered with.
fn Result_Of(answer: &Value) -> &Value
{
    return answer
        .get("result")
        .expect("the request is answered with a result rather than refused");
}

/// The text of a tool call's answer, out of the one content block it carries.
fn Call_Text(answer: &Value) -> &str
{
    return Result_Of(answer)
        .pointer("/content/0/text")
        .and_then(Value::as_str)
        .expect("a tool call answers with one text block");
}

/// Every answer the *binary* returned for `requests`, in order, each as its own document.
///
/// The in-process session above and this differ in exactly one thing — the process — and that
/// is the thing a client actually talks to. Without this the `--serve` branch in `main` is a
/// line nothing exercises: deleting it would leave every other test in this file green, because
/// they all build the catalogue themselves.
fn Answers_From_The_Binary(store: &str, requests: &[&str]) -> Vec<Value>
{
    let session = format!("{}\n", requests.join("\n"));
    let mut child = Command::new(KWB_MCP)
        .arg(store)
        .arg(SERVE)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("the binary this test was built alongside should start");

    let mut input = child.stdin.take().expect("a session's input pipe is opened");
    input
        .write_all(session.as_bytes())
        .expect("the session's input is writable");
    // Closed by dropping it, which is how a session ends: the server reads until the client
    // stops writing, and a pipe left open here would wait for a line that never comes.
    drop(input);

    let served = child.wait_with_output().expect("a session ends when its input does");
    assert!(
        served.status.success(),
        "kwb-mcp {store} {SERVE} exited {:?}: {}",
        served.status.code(),
        String::from_utf8_lossy(&served.stderr)
    );

    let text = String::from_utf8(served.stdout).expect("a JSON-RPC answer is UTF-8");
    return text
        .lines()
        .map(|line| {
            // Named rather than `expect`ed, because the way this fails is worth reading: a
            // `--serve` that printed the surface listing instead of starting a session is a
            // wiring defect, and the line that came out says which it was.
            return serde_json::from_str(line).unwrap_or_else(|cause| {
                panic!("a served answer is not one JSON document ({cause}): {line}")
            });
        })
        .collect();
}

/// What the command line prints for one tool over one store, asserted to have run.
///
/// The trailing newline `println!` adds is dropped: the session's answer is the renderer's
/// output and this is the same output as a terminal shows it.
fn Printed(store: &str, tool: &str, argument: Option<&str>) -> String
{
    let mut command = Command::new(KWB_MCP);
    command.arg(store).arg(tool);
    if let Some(argument) = argument
    {
        command.arg(argument);
    }

    let printed = command.output().expect("the binary this test was built alongside should run");
    assert!(
        printed.status.success(),
        "kwb-mcp {store} {tool} exited {:?}: {}",
        printed.status.code(),
        String::from_utf8_lossy(&printed.stderr)
    );

    let stdout = String::from_utf8(printed.stdout).expect("the command line prints text");
    return stdout.trim_end_matches('\n').to_owned();
}

/// A `tools/call` request for `tool`, sending `arguments` exactly as written.
fn Call_Request(tool: &str, arguments: &str) -> String
{
    return format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/call\",\"params\":{{\"name\":\"{tool}\",\"arguments\":{arguments}}}}}"
    );
}

#[test]
fn Test_A_Handshake_Should_Name_The_Server_And_Declare_Only_Tools()
{
    let documents = Answers(
        &KnowledgeGraph::Empty(),
        &[r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#],
    );
    let handshake = Result_Of(documents.first().expect("the handshake is answered"));

    assert_eq!(
        handshake.get("protocolVersion").and_then(Value::as_str),
        Some(DECLARED_REVISION),
        "the revision this server declares is not the one a client would be told to expect"
    );
    assert_eq!(
        handshake.pointer("/serverInfo/name").and_then(Value::as_str),
        Some("kwb-mcp"),
        "a client keys its configuration on the server's own name: {handshake}"
    );
    assert_eq!(
        handshake.pointer("/capabilities/tools").and_then(Value::as_object),
        Some(&serde_json::Map::new()),
        "the one capability this server has is not declared, or it declares more than one"
    );
}

#[test]
fn Test_A_Listing_Should_Name_And_Describe_Exactly_The_Tools_The_Surface_Declares()
{
    let documents = Answers(
        &KnowledgeGraph::Empty(),
        &[r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}"#],
    );
    let listed = Result_Of(documents.first().expect("the listing is answered"))
        .get("tools")
        .and_then(Value::as_array)
        .expect("a listing carries a list of tools");

    let names: Vec<&str> = listed
        .iter()
        .filter_map(|tool| return tool.get("name").and_then(Value::as_str))
        .collect();
    let declared: Vec<&str> = TOOLS.iter().map(|tool| return tool.name).collect();

    assert_eq!(
        names, declared,
        "the listing is not the surface: a client choosing between these tools is choosing \
         between tools this crate does not dispatch"
    );

    // The sentences too, and not as a courtesy: a descriptor whose text was written a second
    // time is the drift this projection exists to make impossible, and a name-only comparison
    // would not notice it.
    let sentences: Vec<&str> = listed
        .iter()
        .filter_map(|tool| return tool.get("description").and_then(Value::as_str))
        .collect();
    let declared_sentences: Vec<&str> = TOOLS.iter().map(|tool| return tool.summary).collect();

    assert_eq!(
        sentences, declared_sentences,
        "a tool is described to a client in words this crate does not use for it"
    );

    assert!(
        listed
            .iter()
            .all(|tool| return tool.get("inputSchema").is_some_and(Value::is_object)),
        "a tool whose argument schema is not a schema object looks to a client like a tool \
         taking no arguments: {listed:#?}"
    );
}

#[test]
fn Test_A_Call_Should_Answer_What_The_Command_Line_Prints_For_The_Same_Tool_And_Store()
{
    let (root, graph) = Published_Corpus("command-line");
    let store = Scratch_Name(&root);

    // Three shapes, because the two doors have to agree about each: a tool that names an
    // argument, one that takes none at all — where the command line passes the empty string and
    // the session must arrive at the same one — and one whose answer is *several* lines, which
    // is the case that makes the folding of lines onto one string load-bearing. Without it a
    // rendering that joined on anything at all would satisfy this test against `search`.
    let cases = [
        ("search", Some(QUERY), format!("{{\"argument\":{}}}", Value::from(QUERY))),
        ("neighbours", Some("entropy"), r#"{"argument":"entropy"}"#.to_owned()),
        ("merge_losers", None, "{}".to_owned()),
    ];

    for (tool, argument, arguments) in cases
    {
        let documents = Answers(&graph, &[Call_Request(tool, &arguments).as_str()]);
        let answered = Call_Text(documents.first().expect("the call is answered"));

        assert_eq!(
            answered,
            Printed(store, tool, argument),
            "{tool} answered a session with something other than what the command line prints \
             for the same store, so the two surfaces disagree about what a tool says"
        );
    }
}

#[test]
fn Test_The_Search_A_Call_Reaches_Should_Be_The_Corpus_It_Was_Pointed_At()
{
    // The control for the comparison above. Two surfaces agreeing on `(nothing)` would satisfy
    // that test while neither reached the corpus, and this one fails if the argument never
    // arrives or the claims never replay.
    let (_root, graph) = Published_Corpus("reaches-corpus");
    let documents = Answers(
        &graph,
        &[Call_Request("search", &format!("{{\"argument\":{}}}", Value::from(QUERY))).as_str()],
    );

    assert_eq!(
        Call_Text(documents.first().expect("the call is answered")),
        CLAIM,
        "the query the call carried did not reach the claim the store published"
    );
}

#[test]
fn Test_An_Answer_Of_Several_Lines_Should_Be_Served_As_Several_Lines()
{
    // The comparison above cannot see this one, and the reason is worth stating rather than
    // leaving as a gap: `Rendered_Answer` is the *one* renderer both doors use, so a fold that
    // ran the lines together would change what the command line prints and what a session
    // answers in the same direction and the two would still be equal.
    //
    // A neighbourhood is the case that shows it. It is a listing — a concept, its claims, their
    // citations — that a person reads as lines, and the mutation this test was written against
    // is that fold joining on a space instead of a newline, which left every other test in this
    // file green while it made every listing one line.
    let (root, graph) = Published_Corpus("several-lines");
    let documents = Answers(
        &graph,
        &[Call_Request("neighbours", r#"{"argument":"entropy"}"#).as_str()],
    );
    let served = Call_Text(documents.first().expect("the call is answered"));
    let printed = Printed(Scratch_Name(&root), "neighbours", Some("entropy"));

    assert_eq!(
        served.lines().count(),
        printed.lines().count(),
        "a session and the command line report one neighbourhood in a different number of lines"
    );
    assert!(
        served.contains('\n'),
        "a neighbourhood is a listing and this answer is one line: {served}"
    );
}

#[test]
fn Test_The_Binary_Served_A_Session_Should_Answer_What_This_Process_Serves()
{
    // Every other test here builds the catalogue itself, so none of them touches the wiring from
    // a command line to `Serve_Tools`. Deleting the `--serve` branch in `main` would leave this
    // file green while the flag stopped meaning anything, which is the gap this test closes: the
    // requests go to a real child process and the answers are compared against the in-process
    // session for the same store, so `argv` is on the path being asserted.
    let (root, graph) = Published_Corpus("binary");
    let store = Scratch_Name(&root);

    let search = Call_Request(
        "search",
        &format!("{{\"argument\":{}}}", Value::from(QUERY)),
    );
    let requests = [
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
        search.as_str(),
    ];

    assert_eq!(
        Answers_From_The_Binary(store, &requests),
        Answers(&graph, &requests),
        "a client served by the binary is answered something other than what this process serves \
         for the same store, so the flag reaches a different surface than the library does"
    );
}

#[test]
fn Test_Arguments_A_Catalogue_Cannot_Read_Should_Fail_The_Call_Rather_Than_End_The_Session()
{
    // The seam promises totality — a call that cannot be made is refused by the protocol layer
    // and a tool that fails answers with a failure — and this is the half of that promise this
    // crate owns. Reached directly rather than over a line, because the backend renders every
    // payload as a document before it gets here, so no line can carry the text these do.
    let graph = KnowledgeGraph::Empty();
    let catalogue = Catalogue::Over(&graph);

    for unreadable in [NOT_A_DOCUMENT, IMPOSSIBLE_ARGUMENT]
    {
        let answer = catalogue.Call_With_Json_Arguments("search", unreadable);
        assert!(
            answer.Is_A_Failure(),
            "{unreadable} was answered rather than refused: {answer:?}"
        );
        assert!(
            answer.Text().contains("argument"),
            "the refusal does not say which part of the call could not be read: {}",
            answer.Text()
        );
    }

    // And a session survives one, answered as a failed tool and not as a broken connection —
    // which is the half of the promise a client experiences, and the reason a preceding
    // argument's shape is the tool's business rather than the protocol's.
    let documents = Answers(
        &graph,
        &[
            Call_Request("search", IMPOSSIBLE_ARGUMENT).as_str(),
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
        ],
    );

    assert_eq!(
        Result_Of(documents.first().expect("the call is answered"))
            .get("isError")
            .and_then(Value::as_bool),
        Some(true),
        "a call the catalogue could not read was reported as a tool that ran"
    );
    assert!(
        documents.get(1).is_some_and(|answer| return answer.get("result").is_some()),
        "the session stopped answering after a call it could not read"
    );
}
