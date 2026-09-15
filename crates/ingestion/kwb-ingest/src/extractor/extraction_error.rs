//! Why a reading did not happen.

use crate::ReadingKind;

/// Why a reading did not happen.
///
/// **Never a reading that found nothing.** That is a successful reading with an empty result,
/// and admission records it as `Barren` — evidence of absence. These are the cases where the
/// question was never answered, and admission records them as `Unmet`.
///
/// It is filed apart from the extractor contract because admission is the reader of it rather
/// than the extractor: a refusal is what a report is built from when nothing was learned, and
/// nothing there needs the trait that produced it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExtractionError
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

impl core::fmt::Display for ExtractionError
{
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        return match self
        {
            Self::CannotRead { needed } => write!(
                formatter,
                concat!(
                    "this source needs {} reading and this extractor does not do it. Refusing ",
                    "rather than returning nothing, because a source that cannot be read has ",
                    "not been read and found empty"
                ),
                needed.Name()
            ),
            Self::NotRead => write!(
                formatter,
                concat!(
                    "nothing read this source. It is admitted and kept, and nothing is ",
                    "asserted about what is in it, because nobody looked"
                )
            ),
            Self::ReaderFailed { cause } => write!(
                formatter,
                concat!(
                    "the reader did not answer usably: {cause}. Nothing was learned about the ",
                    "source, so nothing about the source is recorded"
                ),
                cause = cause
            ),
        };
    }
}

impl ExtractionError
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

impl core::error::Error for ExtractionError
{
}
