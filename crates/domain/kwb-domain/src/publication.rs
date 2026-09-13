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
    /// Write this publication as one line.
    #[must_use]
    pub fn Record(&self) -> String
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
        graph = Applied(&graph, record, &fields)?;
    }

    return Ok(graph);
}

/// One record, applied to the graph so far.
fn Applied(
    graph: &KnowledgeGraph,
    record: &str,
    fields: &[&str],
) -> Result<KnowledgeGraph, ReplayError>
{
    let malformed = || {
        return ReplayError::Malformed {
            record: record.to_owned(),
        };
    };

    return match fields
    {
        [kind, standing, successor, because, name] if *kind == CONCEPT =>
        {
            let concept = Concept::Named(name);
            let standing = Standing_Of(standing, successor, because).ok_or_else(malformed)?;
            Ok(graph.With_Concept(Versioned::Asserted(concept).Closed(standing)))
        }
        [kind, standing, successor, because, concept, text] if *kind == CLAIM =>
        {
            let address = ContentIdentity::Parse(concept).map_err(|_| return malformed())?;
            let held = Concept_At(graph, address).ok_or_else(|| {
                return ReplayError::OutOfOrder {
                    missing: (*concept).to_owned(),
                };
            })?;
            let claim = Claim::About(&held, text);
            let standing = Standing_Of(standing, successor, because).ok_or_else(malformed)?;
            Ok(graph.With_Claim(Versioned::Asserted(claim).Closed(standing)))
        }
        [kind, standing, successor, because, claim, source, scope] if *kind == ASSERTION =>
        {
            let address = ContentIdentity::Parse(claim).map_err(|_| return malformed())?;
            let held = Claim_At(graph, address).ok_or_else(|| {
                return ReplayError::OutOfOrder {
                    missing: (*claim).to_owned(),
                };
            })?;
            let assertion = Assertion::By(source, &held, Scope::Named(scope));
            let standing = Standing_Of(standing, successor, because).ok_or_else(malformed)?;
            Ok(graph.With_Assertion(Versioned::Asserted(assertion).Closed(standing)))
        }
        _ => Err(malformed()),
    };
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
