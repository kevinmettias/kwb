//! A finished derivation: the identity, and a record of how it was reached.
//!
//! It is filed apart from [`Derivation`] because the two are the two ends of one act: the
//! derivation is the builder, and this is what it produces — the identity plus the evidence
//! of what went into it, which is what a test asserts against rather than the digest alone.
//!
//! [`Derivation`]: crate::Derivation

use crate::Exclusion;
use crate::ContentIdentity;

/// A finished derivation: the identity, and a record of how it was reached.
#[derive(Clone, Debug)]
pub struct Sealed
{
    identity: ContentIdentity,
    included: Vec<&'static str>,
    excluded: Vec<Exclusion>,
}

impl Sealed
{
    /// Assemble the record of a finished derivation.
    ///
    /// Crate-private, and it is the only way to obtain one: [`Derivation::Seal`] is the only
    /// place that has seen the content, so it is the only place entitled to say what
    /// participated. A public constructor here would be a way to claim a layout nothing was
    /// derived from.
    ///
    /// [`Derivation::Seal`]: crate::Derivation::Seal
    pub(crate) fn From(
        identity: ContentIdentity,
        included: Vec<&'static str>,
        excluded: Vec<Exclusion>,
    ) -> Self
    {
        return Self {
            identity,
            included,
            excluded,
        };
    }

    /// The identity.
    #[must_use]
    pub const fn Identity(&self) -> ContentIdentity
    {
        return self.identity;
    }

    /// The fields that participated, in the order they were written.
    #[must_use]
    pub fn Included(&self) -> &[&'static str]
    {
        return &self.included;
    }

    /// The fields deliberately left out, with their reasons.
    #[must_use]
    pub fn Excluded(&self) -> &[Exclusion]
    {
        return &self.excluded;
    }
}
