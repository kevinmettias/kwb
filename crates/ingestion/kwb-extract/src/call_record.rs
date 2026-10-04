//! One model request, with only its observed accounting and source lineage.

use kwb_model::ContentIdentity;
use kwb_platform_xvpe::inference::TokenUsage;

mod codec;

/// The result computed for one model request. No variant is a default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallOutcome
{
    /// A conforming answer supplied proposals.
    Answered,
    /// An answer was received but could not become proposals.
    Malformed,
    /// Every permitted attempt failed to supply an answer.
    Abandoned,
    /// The spend ceiling stopped the reading.
    BudgetExhausted,
}

impl CallOutcome
{
    /// The stable word recorded in a call log.
    #[must_use]
    pub const fn Name(self) -> &'static str
    {
        return match self
        {
            Self::Answered => "answered",
            Self::Malformed => "malformed",
            Self::Abandoned => "abandoned",
            Self::BudgetExhausted => "budget-exhausted",
        };
    }
}

/// Observed accounting for a request; unreported usage and cost remain absent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallRecord
{
    source: ContentIdentity,
    model: String,
    protocol: String,
    role: String,
    outcome: CallOutcome,
    measures: CallMeasures,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CallMeasures
{
    pub attempts: u64,
    pub usage: Option<TokenUsage>,
    pub cost_micros: Option<u64>,
    pub wait_nanos: u128,
}

impl CallRecord
{
    pub(crate) fn Made(source: ContentIdentity, model: &str, outcome: CallOutcome, measures: CallMeasures) -> Self
    {
        return Self { source, model: model.to_owned(), protocol: crate::PROTOCOL.to_owned(), role: "extractor".to_owned(), outcome, measures };
    }

    /// The source document whose passage was asked about.
    #[must_use]
    pub const fn Source(&self) -> ContentIdentity
    {
        return self.source;
    }

    /// The requested model, without its prompt or answer.
    #[must_use]
    pub fn Model(&self) -> &str
    {
        return &self.model;
    }

    /// The reading protocol under which the question was asked.
    #[must_use]
    pub fn Protocol(&self) -> &str
    {
        return &self.protocol;
    }

    /// The request's role.
    #[must_use]
    pub fn Role(&self) -> &str
    {
        return &self.role;
    }

    /// The computed request outcome.
    #[must_use]
    pub const fn Outcome(&self) -> CallOutcome
    {
        return self.outcome;
    }

    /// The number of inference attempts, including failed attempts.
    #[must_use]
    pub const fn Attempts(&self) -> u64
    {
        return self.measures.attempts;
    }

    /// Reported token counts, or absent when no answer reported consumption.
    #[must_use]
    pub const fn Usage(&self) -> Option<TokenUsage>
    {
        return self.measures.usage;
    }

    /// Accounted integer cost, or absent when consumption was not reported.
    #[must_use]
    pub const fn Cost_Micros(&self) -> Option<u64>
    {
        return self.measures.cost_micros;
    }

    /// Observed inference wait, in nanoseconds.
    #[must_use]
    pub const fn Wait_Nanos(&self) -> u128
    {
        return self.measures.wait_nanos;
    }
}

/// A call record could not be rendered or parsed without inventing information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallRecordError
{
    detail: &'static str,
}

impl core::fmt::Display for CallRecordError
{
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        return write!(formatter, "invalid call record: {}", self.detail);
    }
}

impl std::error::Error for CallRecordError {}
