//! A reading locates the actual page span; it does not invent character offsets.

use kwb_domain::Scope;
use kwb_extract::ReadsText;
use kwb_ingest::{ExtractionStrategy, ReadingKind};
use kwb_platform_xvpe::inference::{AnswerValue, InferenceError, InferenceRequest, InferenceResponse, InferenceStrategy, ModelIdentifier, ReplayInference, TokenUsage};
use kwb_platform_xvpe::remote_call::{DeterminismStrength, ReproducibilityScope, Strategy, TraceEquivalence};
use kwb_store::Document;

struct EmptyAnswer;

impl Strategy for EmptyAnswer
{
    const STRENGTH: DeterminismStrength = <ReplayInference as Strategy>::STRENGTH;
    const SCOPE: ReproducibilityScope = <ReplayInference as Strategy>::SCOPE;
    const TRACE: TraceEquivalence = <ReplayInference as Strategy>::TRACE;
}

impl InferenceStrategy for EmptyAnswer
{
    fn Infer(&self, _request: &InferenceRequest) -> Result<InferenceResponse, InferenceError>
    {
        return Ok(InferenceResponse::New(AnswerValue::Sequence(Vec::new()), String::new(), TokenUsage::New(0, 0, 0, 0), Model()));
    }
}

fn Model() -> ModelIdentifier
{
    return ModelIdentifier::New("offline-location-fixture".to_owned());
}

#[test]
fn Test_A_Reading_Of_One_Text_Page_Should_Name_The_One_Based_Page_Span()
{
    let text = b"Entropy does not decrease in an isolated system.";
    let source = Document::Of(text.to_vec()).Identity();
    let reader = ReadsText::Over(EmptyAnswer, Model(), Scope::Unstated());
    let readings = reader.Read(source, text, ReadingKind::Text).expect("the scripted answer should produce a reading");
    assert_eq!(readings.len(), 1, "the short text must produce one passage, not an empty measurement");
    let reading = readings.first().expect("one actual passage reading");
    assert_eq!(reading.Source(), source);
    assert_eq!(reading.Location().Description(), "pages 1 to 1 of the text");
}
