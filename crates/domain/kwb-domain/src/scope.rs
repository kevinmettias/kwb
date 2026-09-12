//! How far an assertion is claimed to reach.

use kwb_model::Normalize;

/// The domain over which a source asserts a claim.
///
/// # Why this is a value and not an enumeration
///
/// `XVPE/video 6.txt` §17 names nine levels — universal mathematics down through hardware
/// evidence to user preferences — and `D-010` is explicit that naming the levels is the easy
/// half and that this repository defines no enumeration yet. The ontological admission rule
/// at corpus topic 61 is the test each candidate level has to pass: a distinction earns its
/// place by preventing an invalid merge, enabling a query, changing inference, or improving
/// provenance. Applying that test to nine candidates is work with an owner, and it is not
/// this type.
///
/// So a scope is an opaque, normalized name. It participates in an assertion's identity, so
/// two scopes that differ are two assertions, and nothing here claims to know which of them
/// is broader. **Ordering scopes is what promotion needs and `D-010` deferred**, and a type
/// that offered a comparison would be answering that question by accident.
///
/// # Why it is normalized but not folded
///
/// Through `kwb-model`'s [`Normalize`], for the reason every other text field goes through
/// it: a reflow is not an edit. Case is deliberately preserved, so `Thermodynamics` and
/// `thermodynamics` are two scopes — consistent with `Concept`, and wrong to decide
/// differently here without a reason that applies only here.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scope(String);

impl Scope
{
    /// A scope named by text.
    #[must_use]
    pub fn Named(name: &str) -> Self
    {
        return Self(Normalize(name));
    }

    /// The scope's name, normalized.
    #[must_use]
    pub fn Name(&self) -> &str
    {
        return &self.0;
    }

    /// Whether the scope names nothing.
    ///
    /// An assertion with no scope is a real thing — a source can assert something without
    /// saying how far it reaches — and it is distinguishable from an assertion scoped to the
    /// empty string only because there is no way to construct the latter.
    #[must_use]
    pub fn Is_Unstated(&self) -> bool
    {
        return self.0.is_empty();
    }
}
