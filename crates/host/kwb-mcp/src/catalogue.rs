//! The tool surface as something a client can ask about, rather than only something a reader
//! can be told.
//!
//! # Why this is its own file
//!
//! [`Catalogue`] is the only type in this crate that exists for a caller this program has never
//! met, and it is the only one that can be **wrong about the surface** — a descriptor whose name
//! or sentence drifted from [`TOOLS`] would advertise a tool nobody dispatches, or describe one
//! wrongly, and nothing else in the crate would notice. It is filed by its own name so a reader
//! asking *what does an agent see* has one file to open.
//!
//! # Why it projects `TOOLS` rather than declaring anything
//!
//! `tests/tool_surface.rs` is the guard that a declared tool dispatches. This is the same fact
//! stated a third time if a second table is allowed to exist, and a fact stated three times
//! drifts in two of them. So a descriptor's name and its sentence are read *out of* [`TOOLS`] on
//! every call to [`ToolCatalogStrategy::Tools`], and a call is answered by
//! [`Answer_Tool_Call`] — the same function the command line is answered by. The two surfaces
//! cannot disagree about what a tool says or what it answers, because they are not two
//! implementations of one promise; they are one implementation with two doors.
//!
//! [`TOOLS`]: crate::TOOLS
//! [`Answer_Tool_Call`]: crate::Answer_Tool_Call

use kwb_domain::KnowledgeGraph;
use kwb_platform_xvpe::remote_call::DeterminismStrength;
use kwb_platform_xvpe::remote_call::ReproducibilityScope;
use kwb_platform_xvpe::remote_call::ServerIdentity;
use kwb_platform_xvpe::remote_call::Strategy;
use kwb_platform_xvpe::remote_call::ToolAnswer;
use kwb_platform_xvpe::remote_call::ToolCatalogStrategy;
use kwb_platform_xvpe::remote_call::ToolDescriptor;
use kwb_platform_xvpe::remote_call::TraceEquivalence;

use crate::Answer_Tool_Call;
use crate::Rendered_Answer;
use crate::ToolName;
use crate::TOOLS;

/// This server's own name, as a client shows it to a person and keys its configuration on.
const SERVER_NAME: &str = "kwb-mcp";

/// The one field a tool's arguments may carry.
///
/// The name the command line already uses for the same thing — `kwb-mcp <store> <tool>
/// [<argument>]` — so a reader comparing the two surfaces is comparing one word, not two.
const ARGUMENT: &str = "argument";

/// The argument schema every tool publishes, since every tool takes the same one thing.
///
/// One constant rather than five literals, and not because they would be identical by accident:
/// [`Answer_Tool_Call`] reaches every tool through one `&str`, so a per-tool schema would be a
/// per-tool claim the dispatcher cannot honour. When a tool grows a second argument, the
/// dispatcher grows with it and this constant splits — in that order.
///
/// `argument` is not required, and that is deliberate rather than lax. `merge_losers` takes
/// nothing at all, and a schema demanding a value it ignores would make every client send one.
/// An absent field reads as the empty string, which is exactly what the command line does when
/// no argument follows the tool's name.
///
/// Assembled from two literal fragments so no line of this file carries a run of spaces inside
/// a string — the shape `KWB-52` scans for, and one a wrapped JSON schema would produce by
/// indentation alone.
const ARGUMENTS_SCHEMA: &str = concat!(
    r#"{"type":"object","properties":{"argument":{"type":"string","#,
    r#""description":"what the tool is asked, in the words the tool answers in"}}}"#,
);

/// The read-only tool surface, as a catalogue a remote client can list and call.
///
/// Holds the corpus rather than a way to reach one: a corpus is a fold over a publication log,
/// which [`Corpus_At`] performs once, and a catalogue that re-read the store per call would
/// answer two calls of one session out of two different worlds.
///
/// [`Corpus_At`]: crate::Corpus_At
pub struct Catalogue<'graph>
{
    graph: &'graph KnowledgeGraph,
}

impl<'graph> Catalogue<'graph>
{
    /// The surface over `graph`.
    #[must_use]
    pub const fn Over(graph: &'graph KnowledgeGraph) -> Self
    {
        return Self { graph };
    }
}

/// `None`, for the reason Nomos's own catalogue gives: a call reaches a corpus that was folded
/// from a file, so what it answers depends on what was on disk when that fold happened. Two
/// runs over one store answer identically — that is the corpus being a value, not the tool
/// being reproducible — and this is the declaration a caller reads, so it says what is true of
/// the tool rather than what is true of the store.
impl Strategy for Catalogue<'_>
{
    const STRENGTH: DeterminismStrength = DeterminismStrength::None;
    const SCOPE: ReproducibilityScope = ReproducibilityScope::SingleRun;
    const TRACE: TraceEquivalence = TraceEquivalence::NotApplicable;
}

impl ToolCatalogStrategy for Catalogue<'_>
{
    fn Identity(&self) -> ServerIdentity
    {
        // The version is this package's own rather than a literal: one typed by hand stops
        // being true at the next release and nothing notices.
        return ServerIdentity::Of(SERVER_NAME, env!("CARGO_PKG_VERSION"));
    }

    fn Tools(&self) -> Vec<ToolDescriptor>
    {
        return TOOLS
            .iter()
            .map(|tool| {
                return ToolDescriptor::Of(tool.name, tool.summary, ARGUMENTS_SCHEMA);
            })
            .collect();
    }

    /// A call, answered by the function the command line is answered by.
    ///
    /// # Why the argument is read here rather than by the caller
    ///
    /// The seam hands this a JSON document as text, because the layer that routes a call does
    /// not interpret its payload and must not appear to. What a tool takes *is* this crate's
    /// business, so the reading happens here — one optional string field — and a document that
    /// is not one is [`ToolAnswer::Failed`] rather than a panic: this is reached by clients that
    /// did not compile against it, and one bad call must not end a session.
    fn Call_With_Json_Arguments(&self, name: &str, arguments: &str) -> ToolAnswer
    {
        let argument = match Argument_Of(arguments)
        {
            Ok(argument) => argument,
            Err(complaint) => return ToolAnswer::Failed(complaint),
        };

        // Unreachable for a name the protocol layer already found in `Tools`, and answered
        // rather than asserted anyway: the refusal is the honest thing to say if that ever
        // stops being true, and a panic here would end a session over a client's typo.
        let Some(lines) = Answer_Tool_Call(self.graph, ToolName::Named(name), &argument)
        else
        {
            return ToolAnswer::Failed(format!("no tool named {name}"));
        };

        return ToolAnswer::Produced(Rendered_Answer(&lines));
    }
}

/// The one argument a call named, read out of the document it arrived in.
///
/// An absent field is the empty string, which is the same thing the command line passes when a
/// tool is named with nothing after it. A field that is present and is not a string is refused
/// rather than coerced: `{"argument": 5}` is a caller that has misread the schema, and answering
/// it with the search `5` would hide that.
fn Argument_Of(arguments: &str) -> Result<String, String>
{
    let Ok(document) = serde_json::from_str::<serde_json::Value>(arguments)
    else
    {
        return Err(format!("the arguments are not a JSON document: {arguments}"));
    };

    return match document.get(ARGUMENT)
    {
        None | Some(serde_json::Value::Null) => Ok(String::new()),
        Some(serde_json::Value::String(argument)) => Ok(argument.clone()),
        Some(other) => Err(format!(
            "the {ARGUMENT} must be a string, and this call sent {other}"
        )),
    };
}
