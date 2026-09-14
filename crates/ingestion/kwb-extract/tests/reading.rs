//! A model reading a source, exercised offline against recorded answers.
//!
//! # Why every test here replays
//!
//! No provider is in this workspace and none is going to be: `xvpe-ai-inference` carries none,
//! `ReadsText` takes a strategy rather than choosing one, and `ReplayInference` answers from
//! recordings. So the suite needs no credential, no network and no spend, and it is exactly
//! reproducible — which is what lets a refusal be *asserted* rather than hoped for.
//!
//! # What these recordings are, said plainly
//!
//! They are **constructed, not captured.** Nothing in this repository can capture one yet,
//! because capturing needs the live provider that is deliberately outside it. `KWB-57` is the
//! item where a fixture built from a reconstruction passed while the real instance slipped
//! through, so the hazard is named rather than hidden — and mitigated where it can be: every
//! recording is keyed by `Request_For`, the same function the reader asks through, so a fixture
//! cannot answer a question the reader does not ask.

use kwb_domain::KnowledgeGraph;
use kwb_domain::Scope;
use kwb_extract::ReadsText;
use kwb_extract::Request_For;
use kwb_ingest::Admit;
use kwb_ingest::ExtractionRefused;
use kwb_ingest::ExtractionStrategy;
use kwb_ingest::ProposedReading;
use kwb_ingest::ReadingKind;
use kwb_model::ContentIdentity;
use kwb_platform_xvpe::inference::AnswerValue;
use kwb_platform_xvpe::inference::InferenceResponse;
use kwb_platform_xvpe::inference::ModelIdentifier;
use kwb_platform_xvpe::inference::ReplayInference;
use kwb_platform_xvpe::inference::ReplayRecording;
use kwb_store::Document;
use kwb_store::DocumentStore;

/// The model these recordings are attributed to.
fn Model() -> ModelIdentifier
{
    return ModelIdentifier::New("a-recorded-reader".to_owned());
}

/// One proposition, as an answer conforming to `Propositions_Schema`.
fn Proposition(concept: &str, claim: &str) -> AnswerValue
{
    return AnswerValue::Record(vec![
        ("concept".to_owned(), AnswerValue::Text(concept.to_owned())),
        ("claim".to_owned(), AnswerValue::Text(claim.to_owned())),
    ]);
}

/// The passage most of these tests read: one sentence, so reading it yields one proposition.
const PASSAGE: &str = "Entropy does not decrease in an isolated system.";

/// What a claim two documents assert must carry: one citation per source, so a claim two
/// documents corroborate holds two assertions and not one.
const CITATIONS_FOR_TWO_SOURCES: usize = 2;

/// A recording answering the question this reader asks about `passage`.
fn Recorded(passage: &str, answer: AnswerValue) -> ReplayRecording
{
    let request = Request_For(&Model(), passage);
    let usage = kwb_platform_xvpe::inference::TokenUsage::New(0, 0, 0, 0);
    let response = InferenceResponse::New(answer, String::new(), usage, Model());

    return ReplayRecording::New(
        kwb_platform_xvpe::inference::RequestFingerprint::Of_Request(&request),
        response,
    );
}

/// A reader answering from these recordings.
fn Reader(recordings: Vec<ReplayRecording>) -> ReadsText<ReplayInference>
{
    return ReadsText::Over(
        ReplayInference::From_Recordings(recordings),
        Model(),
        Scope::Named("physical theory").expect("a named scope"),
    );
}

/// A reader answering `passage` with the propositions given, in the order given.
fn Reader_Answering(passage: &str, propositions: &[(&str, &str)]) -> ReadsText<ReplayInference>
{
    let answers = propositions
        .iter()
        .map(|(concept, claim)| return Proposition(concept, claim))
        .collect();

    return Reader(vec![Recorded(passage, AnswerValue::Sequence(answers))]);
}

/// A reader answering `passage` with one proposition NOT wrapped in the sequence the schema
/// requires, which is the malformed answer two tests below need to refuse.
fn Reader_Answering_Malformed(passage: &str, concept: &str, claim: &str) -> ReadsText<ReplayInference>
{
    return Reader(vec![Recorded(passage, Proposition(concept, claim))]);
}

/// The reading a text reader must produce from one recorded passage: exactly one, about the
/// document it was handed, under the protocol this crate declares.
///
/// Every test that reads a passage ends here, so the four things that make a reading usable are
/// asserted once rather than re-derived at each call site.
fn Assert_One_Reading_About(readings: &[ProposedReading], source: ContentIdentity, concept: &str)
{
    assert_eq!(readings.len(), 1, "one passage, one reading");
    let reading = readings.first().expect("one");
    assert_eq!(reading.Proposed().len(), 1);
    assert_eq!(reading.Proposed().first().expect("one").concept_name, concept);
    assert_eq!(reading.Source(), source, "a reading must be about the document it was handed");
    assert_eq!(
        reading.Lineage().Protocol(),
        kwb_extract::PROTOCOL,
        "the protocol travels with the reading, so a re-read under a new one is distinguishable"
    );
}

/// One graph holding what two documents say, each read by a reader that answers it with the same
/// proposition.
///
/// This is the corroboration setup: the same proposition reached from two sources is the whole
/// reason identity ignores the source, so testing it needs both documents read and both published.
fn Corroborated(callen: &str, kittel: &str) -> KnowledgeGraph
{
    let proposition = Proposition("entropy", "It does not decrease in an isolated system.");
    let reader = Reader(vec![
        Recorded(callen, AnswerValue::Sequence(vec![proposition.clone()])),
        Recorded(kittel, AnswerValue::Sequence(vec![proposition])),
    ]);
    let mut store = DocumentStore::Empty();
    let graph = KnowledgeGraph::Empty();

    let first = Admit(callen.as_bytes().to_vec(), Some(&reader), ReadingKind::Text, &mut store)
        .expect("admits");
    let after_first = first.Published_Into(&graph);
    let second = Admit(kittel.as_bytes().to_vec(), Some(&reader), ReadingKind::Text, &mut store)
        .expect("admits");

    return second.Published_Into(&after_first);
}

#[test]
fn Test_A_Source_Should_Be_Read_Without_Anybody_Typing_What_It_Says()
{
    // The defining first operation. Until `KWB-66` the only reader was a person typing `--says`,
    // and the README's Trying it section said so in as many words.
    let source = Document::Of(PASSAGE.as_bytes().to_vec()).Identity();
    let reader = Reader_Answering(PASSAGE, &[("entropy", "It does not decrease in an isolated system.")]);

    let readings = reader
        .Read(source, PASSAGE.as_bytes(), ReadingKind::Text)
        .expect("a recorded passage is read");

    Assert_One_Reading_About(&readings, source, "entropy");
}

#[test]
fn Test_An_Answer_That_Does_Not_Conform_Should_Refuse_And_Propose_Nothing()
{
    // The user-stated invariant: malformed model output never becomes a graph mutation. It is
    // enforced a layer below KWB -- `InferenceStrategy`'s contract is that a schema is a
    // constraint and not a suggestion -- and this fixes that KWB maps it to a refusal rather
    // than to an empty reading, which would be evidence the passage asserts nothing.
    let source = Document::Of(PASSAGE.as_bytes().to_vec()).Identity();

    // A record where the schema requires a sequence. Recorded, so the refusal comes from the
    // real mechanism rather than from a check written here.
    let reader = Reader_Answering_Malformed(PASSAGE, "entropy", "It does not decrease.");

    let refusal = reader
        .Read(source, PASSAGE.as_bytes(), ReadingKind::Text)
        .expect_err("an answer that does not conform is refused");

    assert!(
        matches!(refusal, ExtractionRefused::ReaderFailed { .. }),
        "a malformed answer must be a reader that did not answer usably, not a source with \
         nothing in it: {refusal:?}"
    );
}

#[test]
fn Test_A_Passage_Nobody_Recorded_Should_Refuse_Rather_Than_Answer_Emptily()
{
    // `ReplayInference` reports `NotRecorded`, which is a reader that was asked and did not
    // answer. Returning no proposals instead would say the passage asserts nothing -- the
    // distinction `Coverage` exists for.
    let passage = "Enthalpy is a thermodynamic potential.";
    let source = Document::Of(passage.as_bytes().to_vec()).Identity();
    let reader = Reader(Vec::new());

    let refusal = reader
        .Read(source, passage.as_bytes(), ReadingKind::Text)
        .expect_err("an unrecorded passage is refused");

    assert!(matches!(refusal, ExtractionRefused::ReaderFailed { .. }), "{refusal:?}");
}

#[test]
fn Test_A_Source_Needing_A_Look_Should_Be_Refused_By_A_Text_Reader()
{
    let passage = "a scan";
    let source = Document::Of(passage.as_bytes().to_vec()).Identity();
    let reader = Reader(Vec::new());

    let refusal = reader
        .Read(source, passage.as_bytes(), ReadingKind::Visual)
        .expect_err("a text reader refuses a source needing a look");

    assert!(
        matches!(
            refusal,
            ExtractionRefused::CannotRead {
                needed: ReadingKind::Visual
            }
        ),
        "{refusal:?}"
    );
}

#[test]
fn Test_A_Refused_Reading_Should_Reach_Admission_As_Unmet_And_Never_As_Barren()
{
    // End to end, because the seam's whole purpose is what happens downstream of a refusal.
    let passage = "Entropy does not decrease in an isolated system.";
    let mut store = DocumentStore::Empty();
    let reader = Reader(Vec::new());

    let report = Admit(
        passage.as_bytes().to_vec(),
        Some(&reader),
        ReadingKind::Text,
        &mut store,
    )
    .expect("the source is admitted even though the reading did not happen");

    assert_eq!(report.Coverage().Name(), "unmet");
    assert!(
        !report.Coverage().Is_Evidence_Of_Absence(),
        "a model that failed was recorded as evidence the source is empty"
    );
    assert!(report.Assertions().is_empty());
}

#[test]
fn Test_Two_Sources_Read_By_A_Model_Should_Meet_At_One_Claim()
{
    // The property every identity decision in this workspace was made to support, reached for
    // the first time **without a person typing the claim**. Two different documents, read
    // separately, proposing the same proposition: one claim, two citations.
    let corpus = Corroborated(
        "Callen says entropy does not decrease in an isolated system.",
        "Kittel says entropy does not decrease in an isolated system.",
    );

    assert_eq!(
        corpus.Current().Claims().len(),
        1,
        "two documents proposing one proposition became more than one claim, so the source is \
         contaminating identity -- the miner's failure, reached through the extractor"
    );
    assert_eq!(
        corpus.Current().Assertions().len(),
        CITATIONS_FOR_TWO_SOURCES,
        "one claim must carry a citation per source, or corroboration cannot be counted"
    );
    assert_eq!(corpus.Current().Concepts().len(), 1);
}

#[test]
fn Test_A_Malformed_Answer_Should_Reach_Admission_As_Unmet_And_Never_As_Barren()
{
    // The invariant end to end, on the path that actually broke. Before `Proposed_From` refused,
    // a non-conforming answer produced no proposals, no proposals is an empty reading, an empty
    // reading is a success, and a successful reading that found nothing is `Barren` -- evidence
    // of absence. A model answering nonsense would have been recorded as a source asserting
    // nothing, which is the 1,367-row incident with a model in place of a prerequisite.
    let mut store = DocumentStore::Empty();
    let reader = Reader_Answering_Malformed(PASSAGE, "entropy", "It does not decrease.");

    let report = Admit(PASSAGE.as_bytes().to_vec(), Some(&reader), ReadingKind::Text, &mut store)
        .expect("the source is admitted even though the answer was not usable");

    assert_eq!(report.Coverage().Name(), "unmet");
    assert!(
        !report.Coverage().Is_Evidence_Of_Absence(),
        "a malformed answer was recorded as evidence the source is empty"
    );
    assert!(report.Assertions().is_empty(), "nothing malformed reached the graph");
}

#[test]
fn Test_A_Passage_That_Asserts_Nothing_Should_Be_Barren_And_Not_A_Refusal()
{
    // The control, and the distinction the whole `Coverage` type exists for. A reader that read
    // the passage and honestly found no propositions is *not* a refusal: that is the one outcome
    // which is evidence of absence, and collapsing it into the test above would make the guard
    // pass by calling every reading a failure.
    let passage = "A page of front matter.";
    let mut store = DocumentStore::Empty();
    let reader = Reader(vec![Recorded(passage, AnswerValue::Sequence(Vec::new()))]);

    let report = Admit(
        passage.as_bytes().to_vec(),
        Some(&reader),
        ReadingKind::Text,
        &mut store,
    )
    .expect("admits");

    assert_eq!(report.Coverage().Name(), "barren");
    assert!(
        report.Coverage().Is_Evidence_Of_Absence(),
        "a passage read and found empty is the one outcome that is evidence of absence"
    );
    assert!(report.Refusal().is_none(), "an empty reading is a success, not a refusal");
}
