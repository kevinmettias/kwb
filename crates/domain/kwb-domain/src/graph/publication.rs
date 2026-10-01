//! What was published, written down so a graph can be rebuilt from it.
//!
//! [`Replay_Records`] is the reader over these records and [`ReplayError`] is what it refuses with,
//! which is filed in its own module because a caller reporting a log that would not replay
//! has no reason to carry the record format to reach it.
//!
//! [`Replay_Records`]: crate::Replay_Records
//! [`ReplayError`]: crate::ReplayError

use kwb_model::ContentIdentity;
use kwb_model::IdentityError;

use crate::Assertion;
use crate::Claim;
use crate::Concept;
use crate::KnowledgeGraph;
use crate::ReplayError;
use crate::Standing;
use crate::Versioned;

/// Between the fields of a record.
///
/// # Why this needs no escaping, and what that rests on
///
/// ASCII unit separator, which is a control character. Every text a domain type keeps is
/// normalized through `kwb-model`'s `Normalize_Text`, which **strips every C0 and C1 control
/// character**, so no field a record can carry contains this byte or a newline. The framing is
/// unambiguous because the domain guarantees it, not because an encoder escapes it.
///
/// That is the same guarantee `KWB-1` established for identity derivation, used a second time —
/// and it is the whole reason this format needs no serialization library. It also means the
/// guarantee is now load-bearing in two places: if a type ever kept text without normalizing
/// it, identities and records would both become forgeable. `tests/publication.rs` asserts the
/// forgery is impossible rather than assuming it.
pub(crate) const SEPARATOR: char = '\u{1F}';

/// A concept was published.
///
/// The kind names are `pub(crate)` because `record_time` needs them: how many fields a record of
/// a kind carried before `KWB-64` appended a time is a fact about that kind, and a copy of the
/// spelling there would be a second place deciding what a concept record is called.
pub(crate) const CONCEPT: &str = "concept";
/// A claim was published.
pub(crate) const CLAIM: &str = "claim";
/// An assertion was published.
pub(crate) const ASSERTION: &str = "assertion";

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

impl Publication
{
    /// Write this publication as one line, timestamped.
    ///
    /// # Why the time is a trailing field and why it is optional
    ///
    /// `Replay_Records` dispatches on how many fields a record has, so a field added anywhere changes
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
            Some(at) => Joined_Fields(&[&line, &at.to_string()]),
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
                let StandingWritten { name, successor, because } = Standing_Fields(standing);
                Joined_Fields(&[CONCEPT, name, &successor, &because, concept.Canonical_Name()])
            }
            Self::Claim { claim, standing } =>
            {
                let StandingWritten { name, successor, because } = Standing_Fields(standing);
                Joined_Fields(&[
                    CLAIM, name, &successor, &because, &claim.Concept().Render(), claim.Text(),
                ])
            }
            Self::Assertion { assertion, standing } =>
            {
                let StandingWritten { name, successor, because } = Standing_Fields(standing);
                Joined_Fields(&[
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
pub fn Replay_Records(records: &[String]) -> Result<KnowledgeGraph, ReplayError>
{
    use crate::versioning::record_time::Without_Time;

    let mut graph = KnowledgeGraph::Empty();

    for record in records
    {
        let fields: Vec<&str> = record.split(SEPARATOR).collect();
        let untimed = Without_Time(&fields);
        graph = Applied_Record(&graph, record, untimed.fields)?;
    }

    return Ok(graph);
}

/// One record, applied to the graph so far.
///
/// This decides which of the three shapes a record claims to be, and produces the refusal that
/// answers a record claiming none. Reading each shape is its own function because the three
/// differ in what they must already find published: a concept is named, where a claim and an
/// assertion are named by the address of something an earlier record put in the graph.
fn Applied_Record(
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
        _ => Err(Malformed_Record(record)),
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
        return Err(Malformed_Record(record));
    };

    let concept = Concept::Named(name);
    let standing = Standing_Of(standing, Successor_Of(successor), because)
        .ok_or_else(|| return Malformed_Record(record))?;

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
        return Err(Malformed_Record(record));
    };

    let address = ContentIdentity::Parse(concept)
        .map_err(|cause| return Unaddressed_Field(concept, cause))?;
    let held = Concept_At(graph, address).ok_or_else(|| return Unpublished_Address(concept))?;
    let claim = Claim::About(&held, text);
    let standing = Standing_Of(standing, Successor_Of(successor), because)
        .ok_or_else(|| return Malformed_Record(record))?;

    return Ok(graph.With_Claim(Versioned::Asserted(claim).Closed(standing)));
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

/// An assertion record, applied. `rest` is its fields after the kind.
///
/// As with a claim, the claim it is about must have been published already.
fn Applied_Assertion(
    graph: &KnowledgeGraph,
    record: &str,
    rest: &[&str],
) -> Result<KnowledgeGraph, ReplayError>
{
    use crate::Scope;

    let [standing, successor, because, claim, source, scope] = rest
    else
    {
        return Err(Malformed_Record(record));
    };

    // An empty trailing field is an unstated scope, and reading it that way is
    // deliberate rather than the swallow `Scope::Named` now refuses. A record is what
    // was already written: logs on disk carry the empty field for every assertion whose
    // source did not say how far it reached, and the record format is not what was
    // wrong. What was wrong was a *person's blank input* becoming that field, and that
    // is refused where the input arrives, not here where it is read back.
    let scope = Scope::Named(scope).unwrap_or_else(Scope::Unstated);
    let held = Claim_Named_By(graph, claim)?;
    let assertion = Assertion::By(source, &held, scope);
    let standing = Standing_Of(standing, Successor_Of(successor), because)
        .ok_or_else(|| return Malformed_Record(record))?;

    return Ok(graph.With_Assertion(Versioned::Asserted(assertion).Closed(standing)));
}

/// The claim an assertion record names by address, or the refusal that answers the field naming
/// it.
///
/// Two refusals, because a field can fail to name a claim in two ways that are not one. A field
/// that is not an address at all is [`Unaddressed_Field`], which carries the identity reader's own
/// complaint rather than replacing it -- that complaint is the only thing that says whether the
/// text was the wrong length or held a character outside the alphabet. An address that resolves to
/// nothing is [`Unpublished_Address`]: the record's shape read, so the log is out of order rather
/// than the assertion unreadable.
fn Claim_Named_By(graph: &KnowledgeGraph, field: &str) -> Result<Claim, ReplayError>
{
    let address = ContentIdentity::Parse(field)
        .map_err(|cause| return Unaddressed_Field(field, cause))?;

    return Claim_At(graph, address).ok_or_else(|| return Unpublished_Address(field));
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
///
/// Named rather than returned as a tuple because the three fields are joined and nothing in
/// their order says which is the name, which the address and which the reason.
struct StandingWritten
{
    /// The standing, named as a record spells it.
    name: &'static str,

    /// The successor's address, empty unless there is one.
    successor: String,

    /// Why it was closed, empty unless there is a reason.
    because: String,
}

/// A standing, as the fields a record writes.
fn Standing_Fields(standing: &Standing) -> StandingWritten
{
    return match standing
    {
        Standing::Asserted => StandingWritten
        {
            name: ASSERTED,
            successor: String::new(),
            because: String::new(),
        },
        Standing::Retired { because } => StandingWritten
        {
            name: RETIRED,
            successor: String::new(),
            because: because.clone(),
        },
        Standing::Superseded { by, because } => StandingWritten
        {
            name: SUPERSEDED,
            successor: by.Render(),
            because: because.clone(),
        },
    };
}

/// Fields, separated.
fn Joined_Fields(fields: &[&str]) -> String
{
    return fields.join(&SEPARATOR.to_string());
}

/// The refusal for a record this reader cannot read.
///
/// The record itself is carried rather than a reconstruction of it, so that a report of a log
/// that would not replay names the line a reader has to look at.
fn Malformed_Record(record: &str) -> ReplayError
{
    return ReplayError::Malformed {
        record: record.to_owned(),
    };
}

/// The refusal for a record naming something no earlier record published.
fn Unpublished_Address(missing: &str) -> ReplayError
{
    return ReplayError::OutOfOrder {
        missing: missing.to_owned(),
    };
}

/// The refusal for a record whose address field is not an address.
///
/// Not [`Malformed_Record`], because the record's shape read. What did not is the one field that had
/// to name something by address, and the identity reader's own complaint about it is carried
/// rather than replaced: it is the only thing that says whether the text was the wrong length
/// or held a character outside the alphabet.
///
/// [`Malformed_Record`]: ReplayError::Malformed
fn Unaddressed_Field(field: &str, cause: IdentityError) -> ReplayError
{
    return ReplayError::Unaddressed {
        field: field.to_owned(),
        cause,
    };
}

/// The successor field, read. `None` when the field is empty, which is how *no successor* is
/// written.
///
/// The emptiness becomes an absence here rather than inside [`Standing_Of`], so that *no
/// successor* has a representation of its own at the point the standing is decided. A successor
/// present where none belongs and one absent where one belongs are both records this writer
/// could not have produced, and the two are only distinguishable if absence is not also just
/// another string.
fn Successor_Of(field: &str) -> Option<&str>
{
    if field.is_empty()
    {
        return None;
    }

    return Some(field);
}

/// A standing, read from its two fields. `None` when they are not one.
///
/// A successor that is present where none belongs, or absent where one does, is refused rather
/// than ignored: both are records this writer could not have produced, so reading them would be
/// reading something else's file as if it were ours.
fn Standing_Of(name: &str, successor: Option<&str>, because: &str) -> Option<Standing>
{
    return match (name, successor, because.is_empty())
    {
        (ASSERTED, None, true) => Some(Standing::Asserted),
        (RETIRED, None, false) => Some(Standing::Retired {
            because: because.to_owned(),
        }),
        (SUPERSEDED, Some(address), false) => ContentIdentity::Parse(address).ok().map(|by| {
            return Standing::Superseded {
                by,
                because: because.to_owned(),
            };
        }),
        _ => None,
    };
}
