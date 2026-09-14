//! The seam an extractor is written against, and the four things it may not do.
//!
//! Everything the trait's signatures name is filed under its own name: [`ReadingKind`] is what
//! a reading needs, [`ProposedReading`] is what comes back, and [`ExtractionRefused`] is why
//! nothing did. This file is the contract itself.
//!
//! [`ReadingKind`]: crate::ReadingKind
//! [`ProposedReading`]: crate::ProposedReading
//! [`ExtractionRefused`]: crate::ExtractionRefused

use kwb_domain::Scope;
use kwb_model::ContentIdentity;

use crate::ExtractionRefused;
use crate::ProposedReading;
use crate::ReadingKind;

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
    /// Read a source and propose what it says, one reading per passage.
    ///
    /// `needed` is what kind of reading this source takes. An implementation that does not do
    /// that kind refuses with [`ExtractionRefused::CannotRead`] and proposes nothing.
    ///
    /// # Why many readings and not one
    ///
    /// `D-015` deferred this and named the condition: *a reader that splits*. `KWB-66` is that
    /// reader — it chunks a source and asks about each passage — so the question is live and
    /// settled here. A [`ProposedReading`] carries **one** location, so a reader that merged
    /// several passages into one reading would locate every proposal at whichever passage it
    /// picked. Returning one reading per passage keeps a location true instead of average.
    ///
    /// # Why a failed passage refuses the whole read
    ///
    /// All or nothing, deliberately. If a reader returned the passages it managed and dropped
    /// the rest, a source two thirds read would report the same shape as a source fully read —
    /// and admission would derive its coverage from what it was handed, which is the prototype's
    /// **1,367 `Barren` rows** exactly: a partial result recorded as a complete one.
    ///
    /// The cost is that one unreadable passage loses a whole document's work, which is a real
    /// cost and the right one while a source is a handful of passages. *Revisited when:* a
    /// corpus exists where a partial reading is worth keeping, which needs a way to say *these
    /// passages and not those* that does not read like a complete reading.
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
    ) -> Result<Vec<ProposedReading>, ExtractionRefused>;

    /// The scope assertions from this extractor are made at.
    ///
    /// On the extractor because how far a reading claims to reach is a property of the reading,
    /// and `D-010` puts scope on the assertion rather than the claim for exactly that reason.
    fn Scope(&self) -> Scope;
}
