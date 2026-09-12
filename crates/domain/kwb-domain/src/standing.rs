//! Where a concept stands, and the one expression that says whether it is current.

use kwb_model::ContentIdentity;

/// What has become of a concept.
///
/// # The six CHECK constraints this replaces
///
/// The prototype carried the same fact in **two columns** — `Status = Deprecated` said the
/// concept was no longer asserted, `ValidUntil` said this version was closed at a known
/// instant — and then needed six SQL `CHECK` constraints to stop a row saying both things
/// inconsistently:
///
/// ```sql
/// CHECK ("ValidUntil" IS NULL OR "Status" = 'Deprecated')
/// CHECK ("SupersededBy" IS NULL OR "ValidUntil" IS NOT NULL)
/// ```
///
/// Its own `ConceptLiveness` names why they were needed: *"this predicate cannot be asked a
/// question that has two answers."*
///
/// One enum, and there is no contradictory state to refuse. A concept cannot be closed
/// without being closed *somehow*, and cannot name a successor without being closed, because
/// neither is expressible. `D-008`'s fifth requirement, met in the type rather than in the one
/// store that has the constraints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standing
{
    /// The concept is asserted. This is the only current state.
    Asserted,

    /// Closed with no successor — retired on its own.
    Retired,

    /// Closed against a successor: a merge loser.
    ///
    /// **Kept, never removed.** `D17` is why, and `D19-B` is what reading them wrongly costs:
    /// `merge-audit` resolved the merge log's identifiers against a view that excluded losers,
    /// resolved none, printed *"nothing has been merged away"* and exited `0`. Every merge on
    /// record could have been wrong and the gate would have passed.
    Superseded
    {
        /// The concept this one was merged into.
        by: ContentIdentity,
    },
}

impl Standing
{
    /// **The liveness expression.** There is exactly one, and this is it.
    ///
    /// # Why this being the only copy is the requirement
    ///
    /// The prototype had the rule in four places — a Postgres global query filter, an
    /// in-memory repository, a JSON repository, and a partial unique index — and the providers
    /// came to disagree about which concepts exist.
    ///
    /// The proof is in its own migration history. `IX_Concepts_CanonicalName_Unique_Active`
    /// was created filtering on `Status <> 'Deprecated'` and corrected three weeks later to
    /// also filter on `ValidUntil IS NULL`. **For three weeks the index enforced half the rule
    /// while the code documented the whole of it, and nothing failed** — because a half-rule is
    /// a weaker constraint, and weaker constraints raise no errors.
    ///
    /// A rule restated in a second language drifts, and the drift is silent. So
    /// `tests/one_liveness.rs` scans this crate's own source and fails if a second definition
    /// appears, which is the only form of this requirement a test can hold.
    #[must_use]
    pub const fn Is_Current(&self) -> bool
    {
        return matches!(*self, Self::Asserted);
    }

    /// The concept this one was merged into, when it was.
    ///
    /// Distinct from [`Is_Current`] on purpose: *not current* and *merged into something* are
    /// different facts, and a retired concept has the first without the second.
    ///
    /// [`Is_Current`]: Self::Is_Current
    #[must_use]
    pub const fn Superseded_By(&self) -> Option<ContentIdentity>
    {
        return match *self
        {
            Self::Superseded { by } => Some(by),
            Self::Asserted | Self::Retired => None,
        };
    }
}
