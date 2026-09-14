//! What was published, written down so a graph can be rebuilt from it.

use kwb_model::ContentIdentity;

use crate::Assertion;
use crate::Claim;
use crate::Concept;
use crate::KnowledgeGraph;
use crate::Scope;
use crate::Standing;
use crate::Versioned;

/// Between the fields of a record.
///
/// # Why this needs no escaping, and what that rests on
///
/// ASCII unit separator, which is a control character. Every text a domain type keeps is
/// normalized through `kwb-model`'s `Normalize`, which **strips every C0 and C1 control
/// character**, so no field a record can carry contains this byte or a newline. The framing is
/// unambiguous because the domain guarantees it, not because an encoder escapes it.
///
/// That is the same guarantee `KWB-1` established for identity derivation, used a second time —
/// and it is the whole reason this format needs no serialization library. It also means the
/// guarantee is now load-bearing in two places: if a type ever kept text without normalizing
/// it, identities and records would both become forgeable. `tests/publication.rs` asserts the
/// forgery is impossible rather than assuming it.
const SEPARATOR: char = '\u{1F}';

/// A concept was published.
const CONCEPT: &str = "concept";
/// A claim was published.
const CLAIM: &str = "claim";
/// An assertion was published.
const ASSERTION: &str = "assertion";

/// How many fields a concept record carried before `KWB-64` added a time.
///
/// Named rather than written at the reader, because the count is a property of the record
/// format and not of the one function that happens to check it. A concept is named, a claim adds
/// the claim's text, and an assertion adds that claim's source and scope.
const CONCEPT_FIELDS: usize = 5;
/// How many fields a claim record carried before `KWB-64` added a time.
const CLAIM_FIELDS: usize = 6;
/// How many fields an assertion record carried before `KWB-64` added a time.
const ASSERTION_FIELDS: usize = 7;

/// The standing a record carries.
const ASSERTED: &str = "asserted";
/// Closed with no successor.
const RETIRED: &str = "retired";
/// Closed against a successor.
const SUPERSEDED: &str = "superseded";

/// One thing published into a graph.
///
/// A graph is a fold over these, so recording them records the graph — `D-014`, which chose
/// transitions over versions because they are the same information and smaller, and because a
/// temporal read then falls out as a prefix rather than needing a second mechanism.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Publication
{
    /// A concept, at some standing.
    Concept
    {
        /// The concept.
        concept: Concept,
        /// Where it stands.
        standing: Standing,
    },

    /// A claim, at some standing.
    Claim
    {
        /// The claim.
        claim: Claim,
        /// Where it stands.
        standing: Standing,
    },

    /// An assertion, at some standing.
    Assertion
    {
        /// The assertion.
        assertion: Assertion,
        /// Where it stands.
        standing: Standing,
    },
}

/// Why a record could not be read back.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReplayError
{
    /// The record's shape is not one this reader knows.
    Malformed
    {
        /// The record, as read.
        record: String,
    },

    /// A claim or assertion named something the records had not published yet.
    ///
    /// Records are a sequence and their order is part of their meaning: a claim is about a
    /// concept, so the concept's record comes first. Out of order is refused rather than
    /// guessed at, because guessing would mean constructing a claim about a concept nobody
    /// recorded.
    OutOfOrder
    {
        /// What was named.
        missing: String,
    },
}

impl core::fmt::Display for ReplayError
{
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        return match self
        {
            Self::Malformed { record } => write!(formatter, "not a publication record: {record:?}"),
            Self::OutOfOrder { missing } => write!(
                formatter,
                "a record names {missing}, which no earlier record published. Refusing rather \
                 than guessing: a claim about a concept nobody recorded is not a claim"
            ),
        };
    }
}

impl core::error::Error for ReplayError {}

impl Publication
{
    /// Write this publication as one line, timestamped.
    ///
    /// # Why the time is a trailing field and why it is optional
    ///
    /// `Replay` dispatches on how many fields a record has, so a field added anywhere changes
    /// the arity of every record and a log written before this item would stop replaying. That
    /// is not an acceptable cost for gaining a column: `D-014` made the graph durable *by*
    /// replay, so a format change that orphans existing logs loses the thing it was built for.
    ///
    /// Appended, therefore, with a matching arm for each kind, so both shapes read. And
    /// [`Option`], because a publication from before this item genuinely has no time — not a
    /// zero, not an epoch, not the moment it happened to be read back. `Coverage` is this
    /// repository's long argument about the difference between *unknown* and *a value*.
    ///
    /// Unix seconds rather than a `Timestamp`, so that `kwb-domain` records a number and the
    /// composition root is the only place that knows where a clock comes from.
    #[must_use]
    pub fn Record(&self, at: Option<i64>) -> String
    {
        let line = self.Untimed();

        return match at
        {
            Some(at) => Joined(&[&line, &at.to_string()]),
            None => line,
        };
    }

    /// The record without its time, which is exactly the record this repository wrote before
    /// `KWB-64` and is still what replay reads when a log carries no timestamps.
    fn Untimed(&self) -> String
    {
        return match self
        {
            Self::Concept { concept, standing } =>
            {
                let (name, successor, because) = Standing_Fields(standing);
                Joined(&[CONCEPT, name, &successor, &because, concept.Canonical_Name()])
            }
            Self::Claim { claim, standing } =>
            {
                let (name, successor, because) = Standing_Fields(standing);
                Joined(&[
                    CLAIM,
                    name,
                    &successor,
                    &because,
                    &claim.Concept().Render(),
                    claim.Text(),
                ])
            }
            Self::Assertion {
                assertion,
                standing,
            } =>
            {
                let (name, successor, because) = Standing_Fields(standing);
                Joined(&[
                    ASSERTION,
                    name,
                    &successor,
                    &because,
                    &assertion.Claim().Render(),
                    assertion.Source(),
                    assertion.Scope().Name(),
                ])
            }
        };
    }
}

/// Rebuild a graph from the records that produced it.
///
/// # Errors
///
/// [`ReplayError`] if a record is not one this reader knows, or names something no earlier
/// record published.
pub fn Replay(records: &[String]) -> Result<KnowledgeGraph, ReplayError>
{
    let mut graph = KnowledgeGraph::Empty();

    for record in records
    {
        let fields: Vec<&str> = record.split(SEPARATOR).collect();
        let (fields, _) = Without_Time(&fields);
        graph = Applied(&graph, record, fields)?;
    }

    return Ok(graph);
}

/// How many fields a record of this kind carries before `KWB-64` added a time.
///
/// Named per kind rather than inferred, because the alternative is asking whether the last
/// field *looks like* a number — and an assertion's last field is a scope, which a source is
/// free to call `1985`. A format that can be misread by content is one that will be.
fn Untimed_Arity(kind: &str) -> Option<usize>
{
    // The constants, not their spellings. A copy of `"concept"` here would be a second place
    // that decides what a concept record is called, and it would go on agreeing until the day
    // somebody renamed one of them -- at which point every record of that kind would read as a
    // kind this reader does not know, and replay would refuse a log it wrote itself.
    return match kind
    {
        _ if kind == CONCEPT => Some(CONCEPT_FIELDS),
        _ if kind == CLAIM => Some(CLAIM_FIELDS),
        _ if kind == ASSERTION => Some(ASSERTION_FIELDS),
        _ => None,
    };
}

/// The record's fields without its time, and the time if it carried one.
fn Without_Time<'fields>(fields: &'fields [&'fields str]) -> (&'fields [&'fields str], Option<i64>)
{
    let Some((without, time)) = Timestamped(fields)
    else
    {
        return (fields, None);
    };

    return (without, Some(time));
}

/// The record split at its time, when it carries one.
///
/// `None` covers every shape that is not a timestamped record of a known kind — an untimed
/// record, a record this reader does not know, and a record at the timestamped arity whose last
/// field is not a number. The last of those is deliberately the same answer as the other two: it
/// is not a timestamped record with a broken time, it is a record this reader does not know, and
/// `Applied` refuses it as malformed rather than silently dropping a field.
fn Timestamped<'fields>(
    fields: &'fields [&'fields str],
) -> Option<(&'fields [&'fields str], i64)>
{
    let kind = fields.first()?;
    let expected = Untimed_Arity(kind)?;
    if fields.len() != expected.saturating_add(1)
    {
        return None;
    }

    let time = fields.last().and_then(|last| return last.parse::<i64>().ok())?;
    let without = fields.get(..expected)?;

    return Some((without, time));
}

/// When a record was published, if it says.
///
/// [`None`] for a publication written before `KWB-64`, which is *unknown* and not a zero. A
/// caller asking what the graph looked like at a time must decide what to do about records that
/// cannot answer, rather than being handed an epoch that sorts before everything.
#[must_use]
pub fn Published_At(record: &str) -> Option<i64>
{
    let fields: Vec<&str> = record.split(SEPARATOR).collect();
    let (_, at) = Without_Time(&fields);

    return at;
}

/// One record, applied to the graph so far.
///
/// This decides which of the three shapes a record claims to be, and produces the refusal that
/// answers a record claiming none. Reading each shape is its own function because the three
/// differ in what they must already find published: a concept is named, where a claim and an
/// assertion are named by the address of something an earlier record put in the graph.
fn Applied(
    graph: &KnowledgeGraph,
    record: &str,
    fields: &[&str],
) -> Result<KnowledgeGraph, ReplayError>
{
    return match fields
    {
        [kind, rest @ ..] if *kind == CONCEPT => Applied_Concept(graph, record, rest),
        [kind, rest @ ..] if *kind == CLAIM => Applied_Claim(graph, record, rest),
        [kind, rest @ ..] if *kind == ASSERTION => Applied_Assertion(graph, record, rest),
        _ => Err(Malformed(record)),
    };
}

/// The refusal for a record this reader cannot read.
///
/// The record itself is carried rather than a reconstruction of it, so that a report of a log
/// that would not replay names the line a reader has to look at.
fn Malformed(record: &str) -> ReplayError
{
    return ReplayError::Malformed {
        record: record.to_owned(),
    };
}

/// The refusal for a record naming something no earlier record published.
fn Unpublished(missing: &str) -> ReplayError
{
    return ReplayError::OutOfOrder {
        missing: missing.to_owned(),
    };
}

/// A concept record, applied. `rest` is its fields after the kind.
fn Applied_Concept(
    graph: &KnowledgeGraph,
    record: &str,
    rest: &[&str],
) -> Result<KnowledgeGraph, ReplayError>
{
    let [standing, successor, because, name] = rest
    else
    {
        return Err(Malformed(record));
    };

    let concept = Concept::Named(name);
    let standing = Standing_Of(standing, successor, because)
        .ok_or_else(|| return Malformed(record))?;

    return Ok(graph.With_Concept(Versioned::Asserted(concept).Closed(standing)));
}

/// A claim record, applied. `rest` is its fields after the kind.
///
/// The concept it is about must have been published already: a claim record names its concept by
/// address, and an address that resolves to nothing means the log is out of order rather than
/// that the claim is unreadable.
fn Applied_Claim(
    graph: &KnowledgeGraph,
    record: &str,
    rest: &[&str],
) -> Result<KnowledgeGraph, ReplayError>
{
    let [standing, successor, because, concept, text] = rest
    else
    {
        return Err(Malformed(record));
    };

    let address = ContentIdentity::Parse(concept).map_err(|_| return Malformed(record))?;
    let held = Concept_At(graph, address).ok_or_else(|| return Unpublished(concept))?;
    let claim = Claim::About(&held, text);
    let standing = Standing_Of(standing, successor, because)
        .ok_or_else(|| return Malformed(record))?;

    return Ok(graph.With_Claim(Versioned::Asserted(claim).Closed(standing)));
}

/// An assertion record, applied. `rest` is its fields after the kind.
///
/// As with a claim, the claim it is about must have been published already.
fn Applied_Assertion(
    graph: &KnowledgeGraph,
    record: &str,
    rest: &[&str],
) -> Result<KnowledgeGraph, ReplayError>
{
    let [standing, successor, because, claim, source, scope] = rest
    else
    {
        return Err(Malformed(record));
    };

    let address = ContentIdentity::Parse(claim).map_err(|_| return Malformed(record))?;
    let held = Claim_At(graph, address).ok_or_else(|| return Unpublished(claim))?;
    // An empty trailing field is an unstated scope, and reading it that way is
    // deliberate rather than the swallow `Scope::Named` now refuses. A record is what
    // was already written: logs on disk carry the empty field for every assertion whose
    // source did not say how far it reached, and the record format is not what was
    // wrong. What was wrong was a *person's blank input* becoming that field, and that
    // is refused where the input arrives, not here where it is read back.
    let scope = Scope::Named(scope).unwrap_or_else(Scope::Unstated);
    let assertion = Assertion::By(source, &held, scope);
    let standing = Standing_Of(standing, successor, because)
        .ok_or_else(|| return Malformed(record))?;

    return Ok(graph.With_Assertion(Versioned::Asserted(assertion).Closed(standing)));
}

/// The concept at an address, whatever its standing.
///
/// Read from every version rather than the current one: a claim about a concept that was
/// retired is still a claim that was published, and replay reconstructs what happened rather
/// than what is currently true.
fn Concept_At(graph: &KnowledgeGraph, address: ContentIdentity) -> Option<Concept>
{
    return graph
        .Every_Version()
        .Concepts()
        .into_iter()
        .find(|held| return held.Value().Identity() == address)
        .map(|held| return held.Value().clone());
}

/// The claim at an address, whatever its standing.
fn Claim_At(graph: &KnowledgeGraph, address: ContentIdentity) -> Option<Claim>
{
    return graph
        .Every_Version()
        .Claims()
        .into_iter()
        .find(|held| return held.Value().Identity() == address)
        .map(|held| return held.Value().clone());
}

/// A standing, written as **two** fields: its name, and a successor that is empty unless there
/// is one.
///
/// Always two, so that every record of a given kind has the same number of fields whatever its
/// standing. A variable-width standing would have made the field count carry meaning, and a
/// reader would have had to know the standing before it could finish splitting — which is the
/// kind of format where a value eventually decides how it is parsed.
fn Standing_Fields(standing: &Standing) -> (&'static str, String, String)
{
    return match standing
    {
        Standing::Asserted => (ASSERTED, String::new(), String::new()),
        Standing::Retired { because } => (RETIRED, String::new(), because.clone()),
        Standing::Superseded { by, because } => (SUPERSEDED, by.Render(), because.clone()),
    };
}

/// A standing, read from its two fields. `None` when they are not one.
///
/// A successor that is present where none belongs, or absent where one does, is refused rather
/// than ignored: both are records this writer could not have produced, so reading them would be
/// reading something else's file as if it were ours.
fn Standing_Of(name: &str, successor: &str, because: &str) -> Option<Standing>
{
    return match (name, successor.is_empty(), because.is_empty())
    {
        (ASSERTED, true, true) => Some(Standing::Asserted),
        (RETIRED, true, false) => Some(Standing::Retired {
            because: because.to_owned(),
        }),
        (SUPERSEDED, false, false) => ContentIdentity::Parse(successor).ok().map(|by| {
            return Standing::Superseded {
                by,
                because: because.to_owned(),
            };
        }),
        _ => None,
    };
}

/// Fields, separated.
fn Joined(fields: &[&str]) -> String
{
    return fields.join(&SEPARATOR.to_string());
}
