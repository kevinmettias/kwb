//! Budget, retry and accounting composition around the reader's unchanged request.

use super::{Proposed_From, ReadsText, Request_For};
use crate::call_record::CallMeasures;
use crate::{CallOutcome, CallRecord};
use kwb_ingest::{Extraction, ExtractionError};
use kwb_model::ContentIdentity;
use kwb_platform_xvpe::calling::{CachePolicy, CallClock, CallObserver, CallPolicy, MicroDollars, ModelCaller, PriceRates, RoleSettings, RoleUsage, SpendBudget};
use kwb_platform_xvpe::inference::{InferenceStrategy, ModelRole, TokenUsage};
use kwb_platform_xvpe::reading::Passage;
use std::time::Duration;

pub(super) type Caller<'a, Reader> = ModelCaller<'a, 1, Reader, CallClock>;
const ROLE: ModelRole = ModelRole::Named(0, "extractor");

pub(super) const fn Free_Replay_Settings() -> RoleSettings
{
    return RoleSettings::New(PriceRates::New(MicroDollars::ZERO, MicroDollars::ZERO, MicroDollars::ZERO, MicroDollars::ZERO), CachePolicy::Of(0));
}

pub(super) const fn Single_Attempt() -> CallPolicy
{
    return CallPolicy::New(1, Duration::ZERO, SpendBudget::UNLIMITED);
}

impl<Reader> ReadsText<Reader>
{
    /// Configure calling for one complete source read. The request bytes stay unchanged.
    #[must_use]
    pub const fn With_Calling(mut self, settings: RoleSettings, policy: CallPolicy) -> Self
    {
        self.settings = settings;
        self.policy = policy;
        self.is_priced = true;
        return self;
    }

    /// Drain the observed records, including requests that failed or hit their ceiling.
    #[must_use]
    pub fn Take_Call_Records(&self) -> Vec<CallRecord>
    {
        return std::mem::take(&mut *self.calls.borrow_mut());
    }
}

impl<Reader: InferenceStrategy> ReadsText<Reader>
{
    /// Ask the unchanged question through the caller, recording refusal and cost.
    ///
    /// # Errors
    ///
    /// Refuses a spent budget, exhausted attempts or unusable answer as `ReaderFailed`.
    pub(super) fn Proposed_For(&self, source: ContentIdentity, passage: &Passage, caller: &mut Caller<'_, Reader>) -> Result<Vec<Extraction>, ExtractionError>
    {
        let request = Request_For(&self.model, passage.Text());
        let before = caller.Telemetry().Ledger().For_Role(ROLE);
        let started = caller.Telemetry().Inference_Time();
        let mut observer = AttemptObserver::default();
        let budget_stopped = caller.Is_Budget_Exhausted(&mut observer);
        let answer = if budget_stopped
        {
            None
        }
        else
        {
            caller.Ask(&request, ROLE, &mut observer)
        };
        let budget_stopped = budget_stopped || caller.Is_Budget_Exhausted(&mut observer);
        let result = Result_Of(answer.as_ref(), budget_stopped, &observer);
        let outcome = Outcome_Of(&result, budget_stopped, observer.malformed || answer.is_some());
        let after = caller.Telemetry().Ledger().For_Role(ROLE);
        let mut measures = Measures(before, after, observer.attempts, answer.is_some(), caller.Telemetry().Inference_Time().saturating_sub(started));
        if !self.is_priced
        {
            measures.cost_micros = None;
        }
        self.calls.borrow_mut().push(CallRecord::Made(source, self.model.As_Str(), outcome, measures));
        return result;
    }
}

fn Measures(before: RoleUsage, after: RoleUsage, failed_attempts: u64, answered: bool, wait: Duration) -> CallMeasures
{
    let usage = if after.Answered() > before.Answered()
    {
        Some(Token_Delta(before.Tokens(), after.Tokens()))
    }
    else
    {
        None
    };
    return CallMeasures { attempts: failed_attempts.saturating_add(u64::from(answered)), usage,
        cost_micros: usage.map(|_| return after.Cost().Micros().saturating_sub(before.Cost().Micros())), wait_nanos: wait.as_nanos() };
}

fn Token_Delta(before: TokenUsage, after: TokenUsage) -> TokenUsage
{
    return TokenUsage::New(after.Input().saturating_sub(before.Input()), after.Output().saturating_sub(before.Output()),
        after.Cache_Read().saturating_sub(before.Cache_Read()), after.Cache_Write().saturating_sub(before.Cache_Write()));
}

fn Failed(cause: &str) -> ExtractionError
{
    return ExtractionError::ReaderFailed { cause: cause.to_owned() };
}

fn Outcome_Of(result: &Result<Vec<Extraction>, ExtractionError>, budget: bool, malformed: bool) -> CallOutcome
{
    if budget
    {
        return CallOutcome::BudgetExhausted;
    }
    if result.is_ok()
    {
        return CallOutcome::Answered;
    }
    if malformed
    {
        return CallOutcome::Malformed;
    }
    return CallOutcome::Abandoned;
}

#[derive(Default)]
struct AttemptObserver
{
    attempts: u64,
    detail: Option<String>,
    malformed: bool,
}

impl CallObserver for AttemptObserver
{
    fn Attempt_Failed(&mut self, _role: ModelRole, attempt: u32, _retryability: kwb_platform_xvpe::calling::Retryability, detail: &str)
    {
        self.attempts = u64::from(attempt);
        self.detail = Some(detail.to_owned());
    }

    fn Answer_Refused(&mut self, _role: ModelRole, _detail: &str)
    {
        self.malformed = true;
    }
}

fn Result_Of(answer: Option<&kwb_platform_xvpe::inference::InferenceResponse>, budget_stopped: bool, observer: &AttemptObserver) -> Result<Vec<Extraction>, ExtractionError>
{
    if budget_stopped
    {
        return Err(Failed("the spend ceiling stopped the reading"));
    }
    return answer.map_or_else(
        || return Err(Failed(observer.detail.as_deref().unwrap_or("no answer was returned"))),
        |answer| return Proposed_From(answer.Answer()));
}

#[cfg(test)]
#[path = "calling_tests.rs"]
mod tests;