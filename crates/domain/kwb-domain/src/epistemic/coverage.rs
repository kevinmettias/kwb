//! Whether a rule ran, and what it found — kept as four facts that cannot be written in
//! place of one another.

use core::num::NonZeroUsize;

/// What became of a rule applied to some material.
///
/// # The incident this type exists for
///
/// The prototype had these four outcomes and still lost data with them, which is why this
/// type is shaped rather than merely enumerated. It wrote **1,367 `Barren` rows that meant
/// "the prerequisite had not run"**, permanently foreclosing two thirds of a book while
/// reporting full coverage. Nothing was broken. `Barren` and `Unmet` were both plain
/// members of one enum, so writing the wrong one was a typo that no type could catch and no
/// reader could later distinguish from the truth.
///
/// `D17` says the absence of an extraction is not evidence against an artefact. That rule is
/// unenforceable unless *ran and found nothing* is distinguishable from *never ran*, and
/// distinguishable is not the same as unequal.
///
/// # So each variant carries the fact that distinguishes it
///
/// - [`Barren`] requires the amount of material actually examined, and requires it to be
///   non-zero. A rule that looked at nothing has no evidence of absence to offer.
/// - [`Unmet`] requires naming the prerequisite that was not satisfied.
/// - [`Skipped`] requires the reason, as a `&'static str`, so it is a decision written in
///   code rather than a message assembled at runtime.
/// - [`Yielded`] requires a non-zero count, because a yield of nothing is a [`Barren`].
///
/// Recording an `Unmet` as a `Barren` therefore means inventing a number for material that
/// was never examined, and recording a `Barren` as an `Unmet` means naming a prerequisite
/// that was in fact satisfied. Neither is a slip.
///
/// # There is no default
///
/// `D20`: a value that reports the good case until somebody remembers to say otherwise is
/// not an unfinished feature, it is a false one. This type derives no [`Default`], so there
/// is no zero value for an unset field to land on, and [`Of_Run`] *computes* the outcome of
/// a run from what the run found rather than letting a caller assert it.
///
/// [`Barren`]: Self::Barren
/// [`Unmet`]: Self::Unmet
/// [`Skipped`]: Self::Skipped
/// [`Yielded`]: Self::Yielded
/// [`Of_Run`]: Self::Of_Run
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coverage
{
    /// The rule ran and produced this many findings.
    Yielded
    {
        /// How many. Non-zero, because a yield of nothing is a [`Coverage::Barren`].
        findings: NonZeroUsize,
    },

    /// The rule ran over material it can quantify, and found nothing.
    ///
    /// **This is the only outcome that is evidence of absence**, and it is the reason the
    /// other three exist.
    Barren
    {
        /// How much material was examined. Non-zero: a rule that examined nothing learned
        /// nothing, and the honest outcome there is [`Coverage::Unmet`], because having
        /// material to read is itself a prerequisite.
        examined: NonZeroUsize,
    },

    /// The rule was deliberately not run.
    ///
    /// # Nothing in this repository produces one, and that is recorded rather than left
    ///
    /// Every consumer here handles it — [`Name`], [`Findings`], [`Has_Run`] and
    /// [`Is_Evidence_Of_Absence`] each have an arm, and the tests exercise it. Nothing
    /// constructs one, because nothing in the pipeline declines to examine anything: `Admit_Source`
    /// takes one source and always reads it.
    ///
    /// **This is not the same as `kwb-store`'s `StoreError::Collision`**, which is also never
    /// produced. That one is unreachable by mathematics — reaching it needs a SHA-256 preimage
    /// break — so it can be written down as a branch that will not be taken. This one is
    /// perfectly producible and merely unbuilt, and a reader who found the two described alike
    /// would be told something false about both.
    ///
    /// Named in prose rather than linked, because `kwb-domain` does not depend on `kwb-store`
    /// and a link that cannot resolve is worse than a name that can be searched for.
    ///
    /// *What would produce it:* something that walks a **corpus** rather than a source, and so
    /// has occasion to decline one — a duplicate already admitted, a filter, a source the
    /// caller excluded. There is no such driver, and this variant is waiting on one.
    ///
    /// # Why it exists anyway
    ///
    /// The corpus's ontological admission rule at topic 61: a distinction earns its place by
    /// preventing an invalid coding. Without this variant a deliberate skip would have to be
    /// recorded as [`Barren`], which is *evidence of absence* and so the 1,367-row incident in
    /// another guise, or as [`Unmet`], which would claim a prerequisite failed when none did.
    /// Removing it would make the first skip anybody writes a miscoding.
    ///
    /// [`Name`]: Self::Name
    /// [`Findings`]: Self::Findings
    /// [`Has_Run`]: Self::Has_Run
    /// [`Is_Evidence_Of_Absence`]: Self::Is_Evidence_Of_Absence
    /// [`Barren`]: Self::Barren
    /// [`Unmet`]: Self::Unmet
    Skipped
    {
        /// Why, written in code. A decision, not a formatted string.
        because: &'static str,
    },

    /// The rule could not run, because something it needed had not happened.
    Unmet
    {
        /// What was needed and absent.
        prerequisite: &'static str,
    },
}

impl Coverage
{
    /// The outcome of a rule that **ran**, derived from what it found.
    ///
    /// This is the only way to obtain a [`Coverage::Yielded`] or a [`Coverage::Barren`], and
    /// it is a derivation rather than a choice — `D20`'s rule, that a computed property
    /// cannot fall out of step with the counts it describes and leaves no setter for a
    /// caller to forget.
    ///
    /// `examined` is non-zero by type, so a caller who has not examined anything cannot
    /// reach either outcome and must say which of the other two it is.
    #[must_use]
    pub const fn Of_Run(findings: usize, examined: NonZeroUsize) -> Self
    {
        return match NonZeroUsize::new(findings)
        {
            Some(findings) => Self::Yielded { findings },
            None => Self::Barren { examined },
        };
    }

    /// Whether the rule actually ran.
    #[must_use]
    pub const fn Has_Run(&self) -> bool
    {
        return matches!(*self, Self::Yielded { .. } | Self::Barren { .. });
    }

    /// Whether this outcome is evidence that there is nothing there.
    ///
    /// Only [`Coverage::Barren`] is, and this is the query `D17` needs: *the absence of an
    /// extraction is not evidence against an artefact*. A caller that sweeps, deprecates or
    /// deletes on the strength of having found nothing must ask this rather than ask whether
    /// a findings count is zero — three of the four outcomes have no findings and only one
    /// of them means there are none.
    #[must_use]
    pub const fn Is_Evidence_Of_Absence(&self) -> bool
    {
        return matches!(*self, Self::Barren { .. });
    }

    /// How many findings the rule produced, which is zero unless it ran and found some.
    #[must_use]
    pub const fn Findings(&self) -> usize
    {
        return match *self
        {
            Self::Yielded { findings } => findings.get(),
            Self::Barren { .. } | Self::Skipped { .. } | Self::Unmet { .. } => 0,
        };
    }

    /// The short stable name, for a report or a journal.
    #[must_use]
    pub const fn Name(&self) -> &'static str
    {
        return match *self
        {
            Self::Yielded { .. } => "yielded",
            Self::Barren { .. } => "barren",
            Self::Skipped { .. } => "skipped",
            Self::Unmet { .. } => "unmet",
        };
    }
}
