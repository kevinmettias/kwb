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

/// The one assertion only this file can make, because [`Sealed::From`] is `pub(crate)`.
///
/// The three questions a finished derivation answers are asked from outside, in
/// `tests/sealed.rs`, which is where a `pub` function's test belongs. The constructor cannot
/// go there: a file under `tests/` compiles as a separate package and sees only the `pub`
/// surface, so the one place entitled to assemble this record is reachable from nowhere but
/// here.
#[cfg(test)]
mod tests
{
    use super::*;

    use crate::Derivation;

    #[test]
    fn Test_From_Should_Take_The_Parts_It_Is_Given_And_Add_Nothing_Of_Its_Own()
    {
        // What a record of a derivation must not do is invent any part of it: an identity that
        // nothing was derived from, a field that never participated, or a reason the caller did
        // not write. The parts are handed over here rather than derived, because that is the
        // only way to tell the constructor from the derivation that normally calls it.
        let identity = Derivation::Of("claim").With_Text("text", "a passage").Seal().Identity();
        let excluded = [Exclusion {
            field: "source",
            because: "two books asserting one claim must become one claim with two citations",
        }];

        let sealed = Sealed::From(identity, vec!["text"], excluded.to_vec());

        assert_eq!(sealed.Identity(), identity, "the constructor minted an identity of its own");
        assert_eq!(sealed.Included(), ["text"], "a field nobody wrote was reported as participating");
        assert_eq!(sealed.Excluded(), excluded, "the exclusion was not carried through unchanged");
    }
}
