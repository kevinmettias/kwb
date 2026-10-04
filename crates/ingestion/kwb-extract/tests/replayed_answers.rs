//! Adversarial constructed recordings exercise the real reader and admission seams offline.
//! Requests are captured from `ReadsText`, so no test duplicates its private prompt or schema.

use kwb_domain::{Coverage, Replay_Records, Scope};
use kwb_extract::ReadsText;
use kwb_ingest::{AdmissionReport, Admit_Source, ExtractionError, ExtractionStrategy, ReadingKind};
use kwb_platform_xvpe::inference::{AnswerValue, InferenceError, InferenceRequest, InferenceResponse, InferenceStrategy, ModelIdentifier, ReplayInference, ReplayRecording, RequestFingerprint, TokenUsage};
use kwb_platform_xvpe::remote_call::{DeterminismStrength, ReproducibilityScope, Strategy, TraceEquivalence};
use kwb_store::{Document, DocumentStore};
use std::cell::RefCell;

const PASSAGE: &str = "Entropy does not decrease in an isolated system.";

struct Capture<'a>(&'a RefCell<Option<InferenceRequest>>);

impl Strategy for Capture<'_>
{
    const STRENGTH: DeterminismStrength = <ReplayInference as Strategy>::STRENGTH;
    const SCOPE: ReproducibilityScope = <ReplayInference as Strategy>::SCOPE;
    const TRACE: TraceEquivalence = <ReplayInference as Strategy>::TRACE;
}

impl InferenceStrategy for Capture<'_>
{
    fn Infer(&self, request: &InferenceRequest) -> Result<InferenceResponse, InferenceError>
    {
        self.0.replace(Some(request.clone()));
        return Err(InferenceError::NotRecorded { detail: "capturing the actual request without answering".to_owned() });
    }
}

fn Model() -> ModelIdentifier
{
    return ModelIdentifier::New("release-recorded-reader".to_owned());
}

fn Request_Of(passage: &str, model: ModelIdentifier) -> InferenceRequest
{
    let captured = RefCell::new(None);
    let reader = ReadsText::Over(Capture(&captured), model, Scope::Unstated());
    let source = Document::Of(passage.as_bytes().to_vec()).Identity();
    assert!(reader.Read(source, passage.as_bytes(), ReadingKind::Text).is_err(), "capturing must never admit a reading");
    return captured.into_inner().expect("the real reader must have asked about the passage");
}

fn Reader_For(answer: AnswerValue, passage: &str, model: ModelIdentifier) -> ReadsText<ReplayInference>
{
    let request = Request_Of(passage, model.clone());
    let fingerprint = RequestFingerprint::Of_Request(&request).expect("the captured request should fingerprint");
    let response = InferenceResponse::New(answer, String::new(), TokenUsage::New(0, 0, 0, 0), model);
    let recording = ReplayRecording::New(fingerprint, response);
    return ReadsText::Over(ReplayInference::From_Recordings(vec![recording]), Model(), Scope::Unstated());
}

fn Proposition(concept: &str, claim: &str) -> AnswerValue
{
    return AnswerValue::Record(vec![("concept".to_owned(), AnswerValue::Text(concept.to_owned())), ("claim".to_owned(), AnswerValue::Text(claim.to_owned()))]);
}

fn Admitted(reader: &ReadsText<ReplayInference>) -> AdmissionReport
{
    let mut store = DocumentStore::Empty();
    let report = Admit_Source(PASSAGE.as_bytes().to_vec(), Some(reader), ReadingKind::Text, &mut store).expect("admitting source bytes should complete");
    let records: Vec<String> = report.Publications().iter().map(|publication| return publication.Record(None)).collect();
    Replay_Records(&records).expect("every generated publication must remain replayable");
    return report;
}

fn Assert_Unmet(report: &AdmissionReport)
{
    assert!(matches!(report.Coverage(), Coverage::Unmet { .. }), "malformed or unrecorded output became evidence: {:?}", report.Coverage());
    assert!(matches!(report.Refusal(), Some(ExtractionError::ReaderFailed { .. })));
    assert!(report.Publications().is_empty(), "an unusable reading published knowledge");
}

#[test]
fn Test_Replay_Should_Enforce_The_Declared_Proposition_Count_Without_Truncating()
{
    let overflow = AnswerValue::Sequence((0..33).map(|index| return Proposition("entropy", &format!("Proposition {index}"))).collect());
    let report = Admitted(&Reader_For(overflow, PASSAGE, Model()));
    Assert_Unmet(&report);
    let boundary = AnswerValue::Sequence((0..32).map(|index| return Proposition("entropy", &format!("Proposition {index}"))).collect());
    let report = Admitted(&Reader_For(boundary, PASSAGE, Model()));
    assert!(report.Refusal().is_none());
    assert_eq!(report.Normalized().Claims_Held(), 32, "the schema's inclusive boundary must be preserved");
}

#[test]
fn Test_Replayed_Wrong_Root_Or_Field_Types_Should_Be_Unmet()
{
    let malformed = [
        AnswerValue::Text("not a list".to_owned()),
        Proposition("entropy", "It does not decrease."),
        AnswerValue::Integer(7),
        AnswerValue::Null,
        AnswerValue::Sequence(vec![AnswerValue::Text("not a record".to_owned())]),
        AnswerValue::Sequence(vec![AnswerValue::Record(vec![("concept".to_owned(), AnswerValue::Text("entropy".to_owned()))])]),
        AnswerValue::Sequence(vec![AnswerValue::Record(vec![("concept".to_owned(), AnswerValue::Integer(7)), ("claim".to_owned(), AnswerValue::Text("A claim.".to_owned()))])]),
    ];
    for answer in malformed
    {
        Assert_Unmet(&Admitted(&Reader_For(answer, PASSAGE, Model())));
    }
}

#[test]
fn Test_Canonical_Empty_Replayed_Proposals_Should_Be_Refused_And_Counted()
{
    for text in ["", "\u{1f}", "\u{1}\n\u{7}"]
    {
        let answer = AnswerValue::Sequence(vec![Proposition(text, text)]);
        let report = Admitted(&Reader_For(answer, PASSAGE, Model()));
        assert_eq!(report.Normalized().Linked().Refused(), 1);
        assert!(report.Publications().is_empty());
    }
}

#[test]
fn Test_Otherwise_Complete_Replayed_Text_Should_Keep_Its_Current_Admission_Contract()
{
    // The schema declares no claim-length or uniqueness rule. Grounding and repeat
    // admission are separate release obligations; this regression does not invent them.
    let mut extra = vec![("concept".to_owned(), AnswerValue::Text("entropy".to_owned())), ("claim".to_owned(), AnswerValue::Text("A claim.".to_owned()))];
    extra.push(("extra".to_owned(), AnswerValue::Null));
    let answers = [
        vec![AnswerValue::Record(extra)],
        vec![Proposition("entropy", "A claim."), Proposition("entropy", "A claim.")],
        vec![Proposition("entropy", &"x".repeat(10_000))],
        vec![Proposition("a\u{1f}b", "first\u{1f}second")],
        vec![Proposition("a\nb", "first\nsecond")],
    ];
    for propositions in answers
    {
        let offered = propositions.len();
        let report = Admitted(&Reader_For(AnswerValue::Sequence(propositions), PASSAGE, Model()));
        assert!(report.Refusal().is_none());
        assert_eq!(report.Normalized().Linked().Refused(), 0);
        assert_eq!(report.Normalized().Claims_Held(), offered);
    }
}

#[test]
fn Test_A_Recording_For_Another_Passage_Or_Model_Should_Not_Answer_This_Source()
{
    let answer = AnswerValue::Sequence(vec![Proposition("entropy", "A claim.")]);
    let other_passage = Reader_For(answer.clone(), "Another source passage.", Model());
    let other_model = Reader_For(answer, PASSAGE, ModelIdentifier::New("another-reader".to_owned()));
    Assert_Unmet(&Admitted(&other_passage));
    Assert_Unmet(&Admitted(&other_model));
}
