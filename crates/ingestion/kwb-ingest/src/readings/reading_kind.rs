//! What kind of reading a source needs.

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
///
/// It is filed apart from the extractor contract because the refusal for a kind an extractor
/// cannot do has to name it, and a vocabulary both sides of that seam quote is one neither
/// side owns.
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
