//! A field that was considered and deliberately left out of an identity.
//!
//! It is filed apart from [`Derivation`] because it is the half of a derivation that
//! contributes nothing to the digest: a reader asking what participated reads the
//! derivation, and a reader asking what was deliberately kept out — which is the question
//! this workspace's dedup mechanism turns on — finds the answer under this name.
//!
//! [`Derivation`]: crate::Derivation

/// A field that was considered and deliberately left out of an identity, with the reason.
///
/// Recording the exclusion is the point. `D-006` and the cross-source dedup mechanism both
/// rest on one exclusion — a claim's source — and an exclusion that exists only as an
/// omission is indistinguishable from an oversight. The prototype states its own source
/// exclusion in a doc comment; a doc comment is not available to a test.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Exclusion
{
    /// The field that was left out.
    pub field: &'static str,

    /// Why leaving it out is correct, in one phrase.
    pub because: &'static str,
}
