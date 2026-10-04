//! Real reader requests are captured and replayed; no provider or network is used.

use kwb_domain::{Coverage, Scope};
use kwb_extract::{CallOutcome, CallRecord, ReadsText};
use kwb_ingest::{Admit_Source, ExtractionStrategy, ReadingKind};
use kwb_platform_xvpe::calling::{CachePolicy, CallPolicy, MicroDollars, PriceRates, RoleSettings, SpendBudget};
use kwb_platform_xvpe::inference::{AnswerValue, InferenceError, InferenceRequest, InferenceResponse, InferenceStrategy, ModelIdentifier, ReplayInference, ReplayRecording, RequestFingerprint, TokenUsage};
use kwb_platform_xvpe::remote_call::{DeterminismStrength, ReproducibilityScope, Strategy, TraceEquivalence};
use kwb_store::{Document, DocumentStore};
use std::cell::{Cell, RefCell};
use std::time::Duration;

const PASSAGE: &str = "Entropy does not decrease in an isolated system. ";

struct Capture<'a>(&'a RefCell<Vec<InferenceRequest>>);

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
        self.0.borrow_mut().push(request.clone());
        return Ok(Response(TokenUsage::New(0, 0, 0, 0)));
    }
}

fn Model() -> ModelIdentifier
{
    return ModelIdentifier::New("offline-call-accounting".to_owned());
}

fn Response(usage: TokenUsage) -> InferenceResponse
{
    return InferenceResponse::New(AnswerValue::Sequence(Vec::new()), String::new(), usage, Model());
}

fn Requests_Of(text: &str) -> Vec<InferenceRequest>
{
    let requests = RefCell::new(Vec::new());
    let reader = ReadsText::Over(Capture(&requests), Model(), Scope::Unstated());
    Read(&reader, text).expect("capturing questions should produce an empty but complete reading");
    return requests.into_inner();
}

fn Replay(requests: &[InferenceRequest], usage: TokenUsage) -> ReadsText<ReplayInference>
{
    let recordings = requests.iter().map(|request| {
        return ReplayRecording::New(RequestFingerprint::Of_Request(request).expect("the real question fingerprints"), Response(usage));
    }).collect();
    return ReadsText::Over(ReplayInference::From_Recordings(recordings), Model(), Scope::Unstated());
}

fn Read<Reader: InferenceStrategy>(reader: &ReadsText<Reader>, text: &str) -> Result<usize, kwb_ingest::ExtractionError>
{
    let source = Document::Of(text.as_bytes().to_vec()).Identity();
    return reader.Read(source, text.as_bytes(), ReadingKind::Text).map(|readings| return readings.len());
}

fn Settings() -> RoleSettings
{
    let rate = MicroDollars::From_Micros(1_000_000);
    return RoleSettings::New(PriceRates::New(rate, rate, rate, rate), CachePolicy::Of(0));
}

fn Policy(budget: SpendBudget) -> CallPolicy
{
    return CallPolicy::New(3, Duration::ZERO, budget);
}

#[test]
fn Test_Every_Passage_Should_Have_One_Roundtrippable_Record_With_Reported_Usage_And_Cost()
{
    let text = PASSAGE.repeat(130);
    let requests = Requests_Of(&text);
    assert_eq!(requests.len(), 1, "the existing adapter submits a text file as one indivisible page, even past its soft character target");
    let usage = TokenUsage::New(11, 3, 7, 2);
    let reader = Replay(&requests, usage).With_Calling(Settings(), Policy(SpendBudget::UNLIMITED));
    assert_eq!(Read(&reader, &text).expect("every captured question is recorded"), requests.len());
    let records = reader.Take_Call_Records();
    assert_eq!(records.len(), requests.len());
    for record in records
    {
        assert_eq!(record.Source(), Document::Of(text.as_bytes().to_vec()).Identity());
        assert_eq!(record.Model(), Model().As_Str());
        assert_eq!(record.Role(), "extractor");
        assert_eq!(record.Protocol(), kwb_extract::PROTOCOL);
        assert_eq!(record.Outcome(), CallOutcome::Answered);
        assert_eq!(record.Attempts(), 1);
        assert_eq!(record.Usage(), Some(usage));
        assert_eq!(record.Cost_Micros(), Some(23));
        let rendered = record.Render().expect("safe call metadata renders");
        assert!(!rendered.contains(PASSAGE), "the accounting log must not contain a prompt or passage");
        assert_eq!(CallRecord::Parse(&rendered).expect("the record round-trips"), record);
    }
    assert!(reader.Take_Call_Records().is_empty(), "taking records must not append them twice");
}

struct Scripted
{
    calls: Cell<u32>,
    succeed_on: u32,
}

impl Strategy for Scripted
{
    const STRENGTH: DeterminismStrength = <ReplayInference as Strategy>::STRENGTH;
    const SCOPE: ReproducibilityScope = <ReplayInference as Strategy>::SCOPE;
    const TRACE: TraceEquivalence = <ReplayInference as Strategy>::TRACE;
}

impl InferenceStrategy for Scripted
{
    fn Infer(&self, _request: &InferenceRequest) -> Result<InferenceResponse, InferenceError>
    {
        let call = self.calls.get().saturating_add(1);
        self.calls.set(call);
        if call < self.succeed_on
        {
            return Err(InferenceError::Transport { detail: "scripted offline transport failure".to_owned() });
        }
        return Ok(Response(TokenUsage::New(11, 3, 7, 2)));
    }
}

#[test]
fn Test_Exhausted_Attempts_Should_Remain_Unmet_With_Unknown_Tokens_And_Cost_Absent()
{
    let reader = ReadsText::Over(Scripted { calls: Cell::new(0), succeed_on: 4 }, Model(), Scope::Unstated())
        .With_Calling(Settings(), Policy(SpendBudget::UNLIMITED));
    let mut store = DocumentStore::Empty();
    let report = Admit_Source(PASSAGE.as_bytes().to_vec(), Some(&reader), ReadingKind::Text, &mut store).expect("source admission completes");
    assert!(matches!(report.Coverage(), Coverage::Unmet { .. }));
    assert!(report.Publications().is_empty());
    let records = reader.Take_Call_Records();
    assert_eq!(records.len(), 1);
    let record = records.first().expect("a record for the exhausted request");
    assert_eq!(record.Attempts(), 3);
    assert_eq!(record.Outcome(), CallOutcome::Abandoned);
    assert_eq!(record.Usage(), None, "unreported usage must never become zero");
    assert_eq!(record.Cost_Micros(), None, "unreported consumption must not produce a fabricated cost");
    assert_eq!(CallRecord::Parse(&record.Render().expect("the refusal renders")).expect("the refusal round-trips"), *record);
}

#[test]
fn Test_A_Retry_That_Eventually_Answers_Should_Count_All_Attempts()
{
    let reader = ReadsText::Over(Scripted { calls: Cell::new(0), succeed_on: 3 }, Model(), Scope::Unstated())
        .With_Calling(Settings(), Policy(SpendBudget::UNLIMITED));
    Read(&reader, PASSAGE).expect("the third attempt answers");
    let records = reader.Take_Call_Records();
    let record = records.first().expect("one completed request");
    assert_eq!(record.Attempts(), 3);
    assert_eq!(record.Outcome(), CallOutcome::Answered);
    assert_eq!(record.Usage(), Some(TokenUsage::New(11, 3, 7, 2)));
}

#[test]
fn Test_A_Spend_Ceiling_Should_Refuse_The_Whole_Source_And_Preserve_Charged_Usage()
{
    let reader = Replay(&Requests_Of(PASSAGE), TokenUsage::New(11, 3, 7, 2))
        .With_Calling(Settings(), Policy(SpendBudget::Of(MicroDollars::From_Micros(23))));
    assert!(Read(&reader, PASSAGE).is_err(), "reaching the spend ceiling must refuse the reading");
    let records = reader.Take_Call_Records();
    let record = records.first().expect("the charged call must remain visible");
    assert_eq!(record.Outcome(), CallOutcome::BudgetExhausted);
    assert_eq!(record.Attempts(), 1);
    assert_eq!(record.Cost_Micros(), Some(23));
    assert_eq!(record.Usage(), Some(TokenUsage::New(11, 3, 7, 2)));
}

#[test]
fn Test_An_Initially_Exhausted_Budget_Should_Make_No_Attempt_And_Invent_No_Consumption()
{
    let reader = Replay(&Requests_Of(PASSAGE), TokenUsage::New(11, 3, 7, 2))
        .With_Calling(Settings(), Policy(SpendBudget::Of(MicroDollars::ZERO)));
    assert!(Read(&reader, PASSAGE).is_err());
    let records = reader.Take_Call_Records();
    let record = records.first().expect("the budget stop is observable");
    assert_eq!(record.Outcome(), CallOutcome::BudgetExhausted);
    assert_eq!(record.Attempts(), 0);
    assert_eq!(record.Usage(), None);
    assert_eq!(record.Cost_Micros(), None);
}

#[test]
fn Test_A_Reader_With_No_Price_Card_Should_Not_Invent_A_Zero_Price()
{
    let reader = Replay(&Requests_Of(PASSAGE), TokenUsage::New(11, 3, 7, 2));
    Read(&reader, PASSAGE).expect("the recorded answer is readable");
    let records = reader.Take_Call_Records();
    assert_eq!(records.first().expect("one request").Cost_Micros(), None);
}

#[test]
fn Test_The_Call_Record_Codec_Should_Refuse_Malformed_Metadata()
{
    let reader = Replay(&Requests_Of(PASSAGE), TokenUsage::New(11, 3, 7, 2));
    Read(&reader, PASSAGE).expect("the recorded answer is readable");
    let record = reader.Take_Call_Records().pop().expect("one recorded request");
    let rendered = record.Render().expect("safe metadata renders");
    assert!(CallRecord::Parse(&format!("{rendered}\u{1f}extra")).is_err());
    let fields: Vec<&str> = rendered.split('\u{1f}').collect();
    for (index, bad) in [(0, "unknown-version"), (1, "bad\nrole"), (4, "invalid-address"), (5, "invented-outcome"), (6, "-1"), (7, "-"), (11, "not-a-cost"), (12, "-1")]
    {
        let mut malformed = fields.clone();
        *malformed.get_mut(index).expect("the rendered record has every field") = bad;
        assert!(CallRecord::Parse(&malformed.join("\u{1f}")).is_err(), "malformed field {index} survived");
    }
}
