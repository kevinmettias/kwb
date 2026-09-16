//! The boundary between this crate and the pipeline that consumes it: `kwb-ingest`, `kwb-domain`,
//! `kwb-store`, and the inference strategy underneath the reader.
//!
//! # What can be asserted from out here, and what cannot
//!
//! `ReadsText` is an [`ExtractionStrategy`], and what it hands back or refuses is what decides
//! what a graph becomes. The half a consumer can check from outside is the refusal half, and this
//! file checks it: the reader is handed to admission as the trait object the composition root
//! really hands it, a reading that does not happen is asserted to leave the graph *exactly as it
//! was* and to reach admission as `unmet` rather than as evidence of absence, and the source it
//! could not read is asserted to be in the store anyway.
//!
//! **The other half -- a reading that succeeded -- cannot be asserted from here, and the reason is
//! not an oversight.** A replay is keyed by the fingerprint of the request the reader asks, and
//! that fingerprint absorbs the instructions and the schema. Both are private to this crate:
//! `Request_For` and `Propositions_Schema` are `pub(crate)` deliberately, so that a fixture cannot
//! record against a question the reader never asks -- `KWB-57` is the item where a fixture built
//! from a reconstruction passed while the real instance slipped through. `src/lib.rs` states the
//! consequence and accepts it: *a test under `tests/` is a separate crate, which reaches only
//! `pub` names, so the request the reader asks and the schema it asks under would have to be
//! exported for the suite's sake, and an export made for a test outlives the test.* A successful
//! reading is therefore asserted in `src/tests.rs`, which is where that argument puts it.
//!
//! Naming the gap here rather than leaving it to be discovered is the point: a suite that quietly
//! covered only the refusals would look like a suite that covered the seam.
//!
//! # Why `admission_seam` and not a source file's stem
//!
//! The unit `check-test-coverage` reads is the test file's own stem, so this file addresses no
//! source file and covers nothing -- correctly, since its subject is the boundary between this
//! crate and another rather than either side's files. It follows `kwb-mcp`'s `tests/retrieval_seam.rs`,
//! which is the same shape for the same reason.

use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::KnowledgeGraph;
use kwb_domain::Scope;
use kwb_domain::Versioned;
use kwb_extract::ReadsText;
use kwb_ingest::Admit_Source;
use kwb_ingest::ExtractionError;
use kwb_ingest::ExtractionStrategy;
use kwb_ingest::ReadingKind;
use kwb_model::ContentIdentity;
use kwb_platform_xvpe::inference::ModelIdentifier;
use kwb_platform_xvpe::inference::ReplayInference;
use kwb_store::Document;
use kwb_store::DocumentStore;

/// The passage every admission below is handed. Readable text, so the only thing that can refuse
/// it is the model the recordings do not answer.
const PASSAGE: &str = "Entropy does not decrease in an isolated system.";

/// The scope the reader is built at, which `D-010` puts on the assertion rather than on the claim.
const SCOPE_NAME: &str = "physical theory";

/// What a reader this crate did not build is stopped by, so that the graph below starts with
/// something in it: an empty graph cannot show that an admission left one alone.
const ALREADY_KNOWN_NAME: &str = "enthalpy";
const ALREADY_KNOWN_CLAIM: &str = "It is a thermodynamic potential.";

/// The model these recordings are attributed to. Nothing is recorded for it, so every request it
/// is asked is one no fixture answers.
fn Model() -> ModelIdentifier
{
    return ModelIdentifier::New("a-replay-with-no-recordings".to_owned());
}

/// The reader this crate offers, over a replay strategy that holds nothing.
///
/// Built the way the composition root builds one: the strategy is the caller's choice and this
/// crate takes it rather than choosing one, which is the whole of what "no provider is here"
/// means in `src/lib.rs`.
fn Reader() -> ReadsText<ReplayInference>
{
    return ReadsText::Over(
        ReplayInference::From_Recordings(Vec::new()),
        Model(),
        Scope::Named(SCOPE_NAME).expect("a named scope"),
    );
}

/// A graph holding one concept and one claim, which no admission below is allowed to change.
fn Already_Known() -> KnowledgeGraph
{
    let concept = Concept::Named(ALREADY_KNOWN_NAME);

    return KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(concept.clone()))
        .With_Claim(Versioned::Asserted(Claim::About(&concept, ALREADY_KNOWN_CLAIM)));
}

/// The addresses a graph's current world holds, as two named lists a test can compare.
///
/// Two fields rather than a pair, because both lists are `Vec<ContentIdentity>`: a pair of those is
/// two positions a caller has to remember the order of, and the compiler would accept a test that
/// swapped the concepts for the claims.
#[derive(Debug, PartialEq)]
struct HeldWorld
{
    /// The addresses of the concepts the graph currently holds.
    concepts: Vec<ContentIdentity>,

    /// The addresses of the claims the graph currently holds.
    claims: Vec<ContentIdentity>,
}

/// The addresses a graph's current world holds, as two lists that a test can compare.
///
/// Addressed rather than counted, because a count is satisfied by a different concept of the same
/// size -- and because the address is what the whole workspace uses to say *this is the same
/// thing*, so a test that a graph was unchanged should say it in that vocabulary.
fn Held(graph: &KnowledgeGraph) -> HeldWorld
{
    let concepts = graph
        .Current()
        .Concepts()
        .into_iter()
        .map(|concept| return concept.Identity())
        .collect();
    let claims = graph
        .Current()
        .Claims()
        .into_iter()
        .map(|claim| return claim.Identity())
        .collect();

    return HeldWorld { concepts, claims };
}

#[test]
fn Test_A_Reading_That_Did_Not_Happen_Should_Leave_The_Graph_Exactly_As_It_Was()
{
    // The user-stated invariant, asserted where a consumer can see it rather than where the report
    // can. `src/tests.rs` asserts that the *report* carries nothing; this asserts that the graph a
    // caller published into is unchanged, which is the claim that actually matters and the one a
    // report-level assertion cannot make -- a refusal that reached the graph by some other path
    // would still report nothing.
    //
    // The reader is handed over as the trait object the composition root hands it, so this also
    // pins that a consumer can use this crate's reader without knowing what it is.
    let before = Already_Known();
    let reader = Reader();
    let mut store = DocumentStore::Empty();

    let report = Admit_Source(
        PASSAGE.as_bytes().to_vec(),
        Some(&reader as &dyn ExtractionStrategy),
        ReadingKind::Text,
        &mut store,
    )
    .expect("the source is admitted even though the reading did not happen");

    assert_eq!(
        report.Coverage().Name(),
        "unmet",
        "a reader that was asked and did not answer was recorded as something else"
    );
    assert!(
        !report.Coverage().Is_Evidence_Of_Absence(),
        "a reader that failed was recorded as evidence the source is empty, which is the 1,367-row \
         incident with a model in place of a prerequisite"
    );

    let after = report.Published_Into(&before);
    assert_eq!(
        Held(&after),
        Held(&before),
        "a reading that did not happen changed the graph, so the invariant is being kept by the \
         report and not by the pipeline"
    );
}

#[test]
fn Test_A_Source_Needing_A_Look_Should_Be_Refused_Without_Asking_A_Model()
{
    // Two refusals are possible here and they leave different evidence, which is how this test can
    // tell that the reader declined rather than asked. Nothing is recorded for the model above, so
    // a reader that *consulted* it would come back `ReaderFailed`; coming back `CannotRead` says
    // the capability question was answered before any request was sent. That distinction is the
    // point of the variant, and only the outside view can see it: from `src/tests.rs` the reader's
    // own code is in scope, and a test there reads the branch rather than the behaviour.
    let before = Already_Known();
    let reader = Reader();
    let mut store = DocumentStore::Empty();

    let report = Admit_Source(
        PASSAGE.as_bytes().to_vec(),
        Some(&reader),
        ReadingKind::Visual,
        &mut store,
    )
    .expect("the source is admitted even though it needed a look");

    assert!(
        matches!(
            report.Refusal(),
            Some(ExtractionError::CannotRead {
                needed: ReadingKind::Visual
            })
        ),
        "a text reader did not refuse a source needing a look, or refused it as a failed reader: \
         {:?}",
        report.Refusal()
    );
    assert_eq!(
        report.Coverage().Name(),
        "unmet",
        "a source this reader cannot read was recorded as a source with nothing in it"
    );
    assert_eq!(
        Held(&report.Published_Into(&before)),
        Held(&before),
        "a source that was never read reached the graph"
    );
}

#[test]
fn Test_A_Source_Should_Be_Kept_When_Its_Reading_Did_Not_Happen()
{
    // The lifecycle the two crates impose on each other, and the half of it that is easy to get
    // backwards: the source is written **before** the reader is consulted, so a refusal costs the
    // reading and not the source. A later run over a working reader finds the bytes where this one
    // left them, which is what makes admitting a whole corpus possible when a model is down.
    let reader = Reader();
    let mut store = DocumentStore::Empty();

    let report = Admit_Source(
        PASSAGE.as_bytes().to_vec(),
        Some(&reader),
        ReadingKind::Text,
        &mut store,
    )
    .expect("the source is admitted even though the reading did not happen");

    let receipt = report.Source().expect("a source that was written has a receipt");
    assert_eq!(
        receipt.Identity(),
        Document::Of(PASSAGE.as_bytes().to_vec()).Identity(),
        "the address the report names is not the address the same bytes are stored at, so a \
         caller could not find the source it just admitted"
    );
    assert!(
        store.Has_Document(receipt.Identity()),
        "a source whose reading failed was not kept, so a rerun with a working reader would have \
         nothing to read"
    );
}

#[test]
fn Test_Reading_One_Source_Twice_Should_Keep_One_Document()
{
    // Re-admission is the question a consumer actually asks: may a corpus be admitted again after
    // the reader changes? Yes, and the second run must not grow the store -- the document is
    // addressed by its content, so the same bytes are the same document however many times they
    // arrive. Asserted through this crate's reader because that is the reader a rerun would use.
    let reader = Reader();
    let mut store = DocumentStore::Empty();

    for _ in 0..2
    {
        Admit_Source(
            PASSAGE.as_bytes().to_vec(),
            Some(&reader),
            ReadingKind::Text,
            &mut store,
        )
        .expect("the source is admitted even though the reading did not happen");
    }

    assert_eq!(
        store.Length(),
        1,
        "reading one source twice kept two documents, so a rerun duplicates every source it \
         touches"
    );
}
