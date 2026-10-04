//! The calling seam is exercised with the published splitter's real multi-page passages.

use super::*;
use kwb_platform_xvpe::inference::{AnswerValue, InferenceResponse, ModelIdentifier, ReplayInference, ReplayRecording, RequestFingerprint};
use kwb_platform_xvpe::reading::{FidelityThresholds, PageNumber, PageProfile, PageText, PassageBudget, PassageSplitter, SectionBoundary};

const TEXT: &str = "Entropy does not decrease in an isolated system.";

fn Passages() -> Vec<Passage>
{
    let pages: Vec<PageText> = [TEXT, "Enthalpy is a thermodynamic potential."].into_iter().enumerate().map(|(index, text)| {
        return PageText::New(PageNumber::From_Zero_Based(u32::try_from(index).expect("two fixture pages")),
            PageProfile::New(u32::try_from(text.chars().count()).expect("short fixture text"), 0, 0), text.to_owned());
    }).collect();
    return SectionBoundary.Chunk_Pages(&pages, PassageBudget::New(4_000, 1).expect("valid budget"),
        FidelityThresholds::New(1, 2).expect("valid fidelity"));
}

fn Reader(passages: &[Passage], budget: SpendBudget) -> ReadsText<ReplayInference>
{
    let model = ModelIdentifier::New("multi-passage-replay".to_owned());
    let recordings = passages.iter().map(|passage| {
        let request = Request_For(&model, passage.Text());
        let response = InferenceResponse::New(AnswerValue::Sequence(Vec::new()), String::new(), TokenUsage::New(1, 0, 0, 0), model.clone());
        return ReplayRecording::New(RequestFingerprint::Of_Request(&request).expect("real request"), response);
    }).collect();
    let rate = MicroDollars::From_Micros(1_000_000);
    let settings = RoleSettings::New(PriceRates::New(rate, rate, rate, rate), CachePolicy::Of(0));
    return ReadsText::Over(ReplayInference::From_Recordings(recordings), model, kwb_domain::Scope::Unstated())
        .With_Calling(settings, CallPolicy::New(1, Duration::ZERO, budget));
}

#[test]
fn Test_Each_Real_Passage_Should_Record_Only_Its_Own_Reported_Usage()
{
    let passages = Passages();
    assert_eq!(passages.len(), 2, "the published splitter must yield both passages");
    let reader = Reader(&passages, SpendBudget::UNLIMITED);
    let source = kwb_store::Document::Of(TEXT.as_bytes().to_vec()).Identity();
    let mut caller = ModelCaller::New(&reader.reader, [reader.settings], reader.policy, CallClock);
    for passage in &passages
    {
        reader.Reading_Of(source, passage, &mut caller).expect("both recordings answer");
    }
    let records = reader.Take_Call_Records();
    assert_eq!(records.len(), passages.len());
    for record in records
    {
        assert_eq!(record.Outcome(), CallOutcome::Answered);
        assert_eq!(record.Attempts(), 1);
        assert_eq!(record.Usage(), Some(TokenUsage::New(1, 0, 0, 0)));
        assert_eq!(record.Cost_Micros(), Some(1), "cost is the request delta, not the running total");
        assert_eq!(CallRecord::Parse(&record.Render().expect("renders")).expect("parses"), record);
    }
}

#[test]
fn Test_A_Source_Budget_Should_Accumulate_Across_Real_Passages()
{
    let passages = Passages();
    assert_eq!(passages.len(), 2);
    let reader = Reader(&passages, SpendBudget::Of(MicroDollars::From_Micros(2)));
    let source = kwb_store::Document::Of(TEXT.as_bytes().to_vec()).Identity();
    let mut caller = ModelCaller::New(&reader.reader, [reader.settings], reader.policy, CallClock);
    let mut readings = passages.iter().map(|passage| return reader.Reading_Of(source, passage, &mut caller));
    assert!(readings.next().expect("first passage").is_ok());
    assert!(matches!(readings.next().expect("second passage"), Err(ExtractionError::ReaderFailed { .. })));
    let records = reader.Take_Call_Records();
    assert_eq!(records.len(), 2);
    assert_eq!(records.first().expect("first call").Outcome(), CallOutcome::Answered);
    assert_eq!(records.last().expect("second call").Outcome(), CallOutcome::BudgetExhausted);
    assert_eq!(records.last().expect("second call").Cost_Micros(), Some(1));
}