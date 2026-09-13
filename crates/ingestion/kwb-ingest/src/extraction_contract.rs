//! What an extractor offers, and the four things it is not allowed to do.

use kwb_domain::Scope;
use kwb_model::ContentIdentity;

use crate::Extraction;

/// What kind of reading a source needs.
///
/// # This declares what a *reader does*, and deliberately does not measure what a *source is*
///
/// XVPE owns the second question. `xvpe-corpus-text::PageFidelity` distinguishes born-digital
/// from sparse from image-only, and the reference miner has spent a month measuring what those
/// distinctions have to be. KWB is an application over XVPE and does not restate what XVPE
/// owns, so nothing here grades a page.
///
/// The two questions are not the same one, and the miner is the evidence: a predicate that
/// conflated *what input a reader must be sent* with *what the text layer can settle* exempted
/// **68 of 111** claims from a check they could have passed. Merging them here would reproduce
/// that, so this type answers only the first -- and when a reading adapter arrives carrying
/// `PageFidelity`, it maps *into* this rather than replacing it.
///
/// Two values, because two is what the first extractor has to tell apart. `D-009`'s lesson
/// applies to vocabularies as much as to representations: a lattice invented before anything
/// needs it becomes the lattice everything afterwards has to pretend to fit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadingKind
{
    /// The text is already there to be read.
    Text,

    /// Somebody or something has to look at it.
    Visual,
}

impl ReadingKind
{
    /// The short stable name, for a report or a journal.
    #[must_use]
    pub const fn Name(&self) -> &'static str
    {
        return match *self
        {
            Self::Text => "text",
            Self::Visual => "visual",
        };
    }
}

/// Where in a source something was found.
///
/// Deliberately opaque text rather than a page, an offset or a span. What locates a passage
/// differs by medium, this repository reads one medium, and a structured location invented for
/// one medium becomes the structure every later medium has to pretend to fit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceLocation(String);

impl SourceLocation
{
    /// A location as the extractor describes it.
    #[must_use]
    pub fn Named(description: &str) -> Self
    {
        return Self(kwb_model::Normalize(description));
    }

    /// The description.
    #[must_use]
    pub fn Description(&self) -> &str
    {
        return &self.0;
    }
}

/// What produced an extraction, and under what protocol.
///
/// # Why lineage travels with the output and not with the claim
///
/// A claim's identity is derived from its content with the source excluded, so two books
/// asserting one thing are one claim (`D-002`). Lineage is a fact about *this reading*, not
/// about the proposition, and putting it in the claim would make one proposition read twice
/// into two claims — the contamination `D-010` prevents, arriving through the extractor.
///
/// It travels here so that *changing the model or the prompt does not silently redefine KWB
/// semantic identity*: a re-read under a new protocol produces a new extraction with new
/// lineage, and if it proposes the same proposition it lands on the same claim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtractionLineage
{
    protocol: String,
    reader: String,
}

impl ExtractionLineage
{
    /// The protocol that governed the reading, and who or what did it.
    ///
    /// `reader` is a person as readily as a model. The corpus's admission rule turns on whether
    /// a feature still makes sense with a perfect human annotator, and this type answers yes by
    /// construction — a person is a reader with a protocol, exactly like a model.
    #[must_use]
    pub fn Of(protocol: &str, reader: &str) -> Self
    {
        return Self {
            protocol: kwb_model::Normalize(protocol),
            reader: kwb_model::Normalize(reader),
        };
    }

    /// The protocol under which the source was read.
    #[must_use]
    pub fn Protocol(&self) -> &str
    {
        return &self.protocol;
    }

    /// Who or what read it.
    #[must_use]
    pub fn Reader(&self) -> &str
    {
        return &self.reader;
    }
}

/// What one reading of one source proposed.
///
/// # This is not an intermediate representation, and the name is chosen to keep it that way
///
/// `D-009` records that a canonical Knowledge IR is **stranded** — the corpus calls it the
/// biggest idea in the design and the prototype has zero files implementing one, and the
/// instrument that was supposed to inform it cannot. The risk this type is shaped against is
/// that a local representation which happened to ship first quietly becomes the canonical one
/// because nothing else existed.
///
/// So: source-local, named for the act rather than the architecture, and carrying only what one
/// reading of one source knows. If a multi-stage representation is ever decided, this is one
/// front end feeding it, not the thing it grew out of.
///
/// # Proposed, not asserted
///
/// Nothing here is a KWB entity and nothing here has an identity. The extractor returns text
/// and where it found it; `Admit` decides what becomes a concept, a claim and an assertion, and
/// `kwb-model` derives every address from content. An extractor that minted an identity would
/// be a second authority for the thing `D-002` owns — and the reference miner is the worked
/// example of why that matters, because its own claim identity absorbs the source path and page
/// window, which is the deliberate inverse of the rule that lets two books become one claim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProposedReading
{
    source: ContentIdentity,
    location: SourceLocation,
    proposed: Vec<Extraction>,
    lineage: ExtractionLineage,
}

impl ProposedReading
{
    /// What a reader proposes a passage of a source says.
    #[must_use]
    pub fn Of(
        source: ContentIdentity,
        location: SourceLocation,
        proposed: Vec<Extraction>,
        lineage: ExtractionLineage,
    ) -> Self
    {
        return Self {
            source,
            location,
            proposed,
            lineage,
        };
    }

    /// The document this was read out of, by its address.
    #[must_use]
    pub const fn Source(&self) -> ContentIdentity
    {
        return self.source;
    }

    /// Where in it.
    #[must_use]
    pub const fn Location(&self) -> &SourceLocation
    {
        return &self.location;
    }

    /// What the reading proposes the passage says. Proposals, not claims.
    #[must_use]
    pub fn Proposed(&self) -> &[Extraction]
    {
        return &self.proposed;
    }

    /// Under what protocol, and by whom.
    #[must_use]
    pub const fn Lineage(&self) -> &ExtractionLineage
    {
        return &self.lineage;
    }
}

/// Why a reading did not happen.
///
/// **Never a reading that found nothing.** That is a successful reading with an empty result,
/// and admission records it as `Barren` — evidence of absence. These are the cases where the
/// question was never answered, and admission records them as `Unmet`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExtractionRefused
{
    /// The source needs a kind of reading this extractor does not do.
    ///
    /// **The refusal is the point.** A text extractor handed a scanned page must say it cannot
    /// read it, because returning no proposals would be a reading that examined the page and
    /// found nothing in it — which is evidence of absence, and false.
    CannotRead
    {
        /// What kind of reading it would have needed.
        needed: ReadingKind,
    },

    /// There was no reader.
    ///
    /// Not a reader that failed — **no reader at all**, which is what `kwb admit` does when it
    /// is given a source and no `--says`. The source is admitted and archived; nothing is
    /// claimed about what it contains, because nobody looked.
    ///
    /// It is a variant here rather than a case the command line handles for itself so that
    /// there is one path to a report and one place that decides what an unread source means. A
    /// caller that assembled its own summary for this case would be the second authority, and
    /// the two would drift the way the five copies of this repository's retired compile premise
    /// did.
    NotRead,

    /// The reader was asked and did not answer usably.
    ///
    /// A model that returned malformed output, a transport that failed, a person who stopped.
    /// The distinction that matters is not the cause but the consequence: **nothing was
    /// learned about the source**, so nothing about the source may be recorded.
    ReaderFailed
    {
        /// What went wrong, for a human reading a report.
        cause: String,
    },
}

impl core::fmt::Display for ExtractionRefused
{
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        return match self
        {
            Self::CannotRead { needed } => write!(
                formatter,
                "this source needs {} reading and this extractor does not do it. Refusing \
                 rather than returning nothing, because a source that cannot be read has not \
                 been read and found empty",
                needed.Name()
            ),
            Self::NotRead => write!(
                formatter,
                "nothing read this source. It is admitted and kept, and nothing is asserted \
                 about what is in it, because nobody looked"
            ),
            Self::ReaderFailed { cause } => write!(
                formatter,
                "the reader did not answer usably: {cause}. Nothing was learned about the \
                 source, so nothing about the source is recorded"
            ),
        };
    }
}

impl ExtractionRefused
{
    /// What was needed and absent, as [`Coverage::Unmet`] requires it.
    ///
    /// A `&'static str`, because `Coverage` takes one: a prerequisite is a decision written in
    /// code, not a message assembled at runtime, and that is what makes two reports of the same
    /// unmet prerequisite comparable. The runtime detail — which reader, what it said — is on
    /// the refusal itself and reaches a person through [`Display`], not through this.
    ///
    /// [`Coverage::Unmet`]: kwb_domain::Coverage::Unmet
    /// [`Display`]: core::fmt::Display
    #[must_use]
    pub const fn Prerequisite(&self) -> &'static str
    {
        return match *self
        {
            Self::CannotRead { .. } => "a reading this extractor cannot do",
            Self::NotRead => "a reader",
            Self::ReaderFailed { .. } => "an answer from the reader",
        };
    }
}

impl core::error::Error for ExtractionRefused {}

/// Something that reads a source and proposes what it says.
///
/// # What an implementation may not do
///
/// It may not mint an identity, may not write anything, may not decide what is admitted, and
/// may not return an empty reading in place of a refusal. Everything it produces is a proposal
/// that `Admit` lowers through the identity, admission and provenance rules that already exist.
///
/// # The implementations this repository has and expects
///
/// [`Stated`] is a person supplying the claims — the perfect human annotator the corpus's
/// admission rule turns on, and what `kwb admit --says` has always been without saying so.
///
/// A model-backed implementation belongs **outside** the knowledge engine, behind this seam,
/// for the reason that rule gives: extraction as a step still makes sense with a human, so the
/// step is KWB's; extraction by a model does not, so the implementation is an AI capability KWB
/// happens to use.
///
/// [`Stated`]: crate::Stated
pub trait ExtractionStrategy
{
    /// Read a source and propose what it says.
    ///
    /// `needed` is what kind of reading this source takes. An implementation that does not do
    /// that kind refuses with [`ExtractionRefused::CannotRead`] and proposes nothing.
    ///
    /// # Why there is no `Reads(kind) -> bool` beside this
    ///
    /// It would be a second way to ask one question, and a caller that asked it could act on
    /// the answer and then call anyway. `D19-B` is what a second path to one decision costs
    /// here: a query filter that rewrote every query, including ones written by someone who had
    /// never heard of it, so `merge-audit` resolved none of the merge log's identifiers, printed
    /// *"nothing has been merged away"* and exited `0`. One door, and the refusal comes back
    /// through it.
    ///
    /// # Why the content is bytes
    ///
    /// Whether these bytes are text is the reader's judgement and part of what it is for.
    /// Decoding here would mean either refusing a source this crate has no basis to judge, or a
    /// lossy decode — and a lossy decode hands a reader mangled text it will read confidently,
    /// which is the one failure that produces a *wrong* claim rather than no claim.
    ///
    /// # Errors
    ///
    /// [`ExtractionRefused`] when the source cannot be read or the reader did not answer.
    /// Never returned for a source that was read and proposed nothing — that is an empty
    /// reading, and it is a success.
    fn Read(
        &self,
        source: ContentIdentity,
        content: &[u8],
        needed: ReadingKind,
    ) -> Result<ProposedReading, ExtractionRefused>;

    /// The scope assertions from this extractor are made at.
    ///
    /// On the extractor because how far a reading claims to reach is a property of the reading,
    /// and `D-010` puts scope on the assertion rather than the claim for exactly that reason.
    fn Scope(&self) -> Scope;
}
