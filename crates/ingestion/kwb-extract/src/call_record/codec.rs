//! The versioned call-log line; it carries no prompt or answer bytes.

use super::{CallMeasures, CallOutcome, CallRecord, CallRecordError};
use kwb_model::ContentIdentity;
use kwb_platform_xvpe::inference::TokenUsage;

const SEPARATOR: char = '\u{1f}';
const VERSION: &str = "kwb-call-v1";

impl CallRecord
{
    /// Render one complete log line, without its line terminator.
    ///
    /// # Errors
    ///
    /// Refuses text containing control characters instead of forging record fields.
    pub fn Render(&self) -> Result<String, CallRecordError>
    {
        Valid_Text(&self.role)?;
        Valid_Text(&self.model)?;
        Valid_Text(&self.protocol)?;
        let usage = self.measures.usage;
        let fields = [VERSION.to_owned(), self.role.clone(), self.model.clone(), self.protocol.clone(), self.source.Render(),
            self.outcome.Name().to_owned(), self.measures.attempts.to_string(),
            Number(usage.map(TokenUsage::Input)), Number(usage.map(TokenUsage::Output)),
            Number(usage.map(TokenUsage::Cache_Read)), Number(usage.map(TokenUsage::Cache_Write)),
            Number(self.measures.cost_micros), self.measures.wait_nanos.to_string()];
        return Ok(fields.join(&SEPARATOR.to_string()));
    }

    /// Read a line rendered by this type, preserving unknown counts as absent.
    ///
    /// # Errors
    ///
    /// Refuses an unknown version, shape, outcome, identity or numeric field.
    pub fn Parse(record: &str) -> Result<Self, CallRecordError>
    {
        let fields: Vec<&str> = record.split(SEPARATOR).collect();
        let [version, role, model, protocol, source, outcome, attempts, input, output, read, write, cost, wait] = fields.as_slice()
        else
        {
            return Err(Invalid("wrong field count"));
        };
        if *version != VERSION
        {
            return Err(Invalid("unsupported version"));
        }
        Valid_Text(role)?;
        Valid_Text(model)?;
        Valid_Text(protocol)?;
        let measures = CallMeasures { attempts: Unsigned(attempts)?, usage: Usage([input, output, read, write])?,
            cost_micros: Optional_Number(cost)?, wait_nanos: wait.parse().map_err(|_| return Invalid("invalid wait"))? };
        return Ok(Self { role: (*role).to_owned(), model: (*model).to_owned(), protocol: (*protocol).to_owned(),
            source: ContentIdentity::Parse(source).map_err(|_| return Invalid("invalid source identity"))?,
            outcome: Outcome(outcome)?, measures });
    }
}

fn Invalid(detail: &'static str) -> CallRecordError
{
    return CallRecordError { detail };
}

fn Valid_Text(text: &str) -> Result<(), CallRecordError>
{
    if text.is_empty() || text.chars().any(char::is_control)
    {
        return Err(Invalid("empty or control-bearing text"));
    }
    return Ok(());
}

fn Number(number: Option<u64>) -> String
{
    return number.map_or_else(|| return "-".to_owned(), |number| return number.to_string());
}

fn Optional_Number(text: &str) -> Result<Option<u64>, CallRecordError>
{
    if text == "-"
    {
        return Ok(None);
    }
    return Unsigned(text).map(Some);
}

fn Unsigned(text: &str) -> Result<u64, CallRecordError>
{
    return text.parse().map_err(|_| return Invalid("invalid unsigned count"));
}

fn Usage(fields: [&str; 4]) -> Result<Option<TokenUsage>, CallRecordError>
{
    let [input, output, read, write] = fields.map(Optional_Number);
    return match (input?, output?, read?, write?)
    {
        (None, None, None, None) => Ok(None),
        (Some(input), Some(output), Some(read), Some(write)) => Ok(Some(TokenUsage::New(input, output, read, write))),
        _ => Err(Invalid("partial token counts cannot be represented by the adopted usage type")),
    };
}

fn Outcome(text: &str) -> Result<CallOutcome, CallRecordError>
{
    return match text
    {
        "answered" => Ok(CallOutcome::Answered),
        "malformed" => Ok(CallOutcome::Malformed),
        "abandoned" => Ok(CallOutcome::Abandoned),
        "budget-exhausted" => Ok(CallOutcome::BudgetExhausted),
        _ => Err(Invalid("unknown outcome")),
    };
}
