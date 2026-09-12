//! One source asserting one claim, at one scope. The thing a claim has none of.

use kwb_model::ContentIdentity;
use kwb_model::Derivation;

use crate::Claim;
use crate::Scope;

/// The kind an assertion's identity is derived under.
const ASSERTION: &str = "assertion";

/// The claim being asserted, participating by identity.
const CLAIM: &str = "claim";

/// Who asserts it.
const SOURCE: &str = "source";

/// How far they assert it reaches.
const SCOPE: &str = "scope";

/// One source asserting one claim at one scope.
///
/// # Why this type exists at all
///
/// `Claim` excludes the source from its identity so that two references asserting one claim
/// become **one claim with two citations**, and excludes the scope because `D-010` measured
/// that a scope in the identity would make a benchmark and a bare preference saying the same
/// thing two rows that can never learn of each other. Both exclusions are right, and together
/// they leave a claim with nowhere to record who said it or how far they meant it.
///
/// This is that place. **One claim, many assertions**, and the claim is where they meet —
/// which is the whole cross-source mechanism working rather than being described.
///
/// # A claim has no standing; an assertion is where standing would attach
///
/// `D-010`: *"a claim has no standing of its own. It is a sentence about a concept. Who
/// asserts it, at what scope, on what evidence, and how strongly, are properties of an
/// assertion."* So a query for what is known at a scope is a query over assertions, and
/// anything that gives a *claim* a scope is re-deriving the contamination `D-010` prevents.
///
/// # There is no confidence here, and its absence is tested
///
/// The grading half is on `D-004`'s held list as **pending** — the reference miner is
/// actively generating input to it, and what it has produced so far is the reason not to
/// guess: across two corpora under one contract a model's self-graded placement held flat at
/// 0.33 against 0.34 while independently *resolved* placement fell from 0.50 to 0.07. A grade
/// a source assigns its own assertion is not a measurement of that assertion's standing.
///
/// The prototype shipped the alternative: `Confidence { get; set; } = 1.0` in **forty**
/// domain types, a settable field defaulting to certainty, which is `D20` in its purest form.
/// Adding a grade here later changes this derivation and therefore every assertion identity
/// in the corpus, which is the cost that makes the decision worth having been recorded rather
/// than drifted into.
///
/// When a grade does arrive, the vocabulary for its evidence half already exists and should
/// not be rewritten: `xvpe-evidence` holds quote verification and claim grounding, extracted
/// on 2026-09-12 with this repository as the named second consumer. `D-007` governs how it
/// would be reached.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assertion
{
    identity: ContentIdentity,
    claim: ContentIdentity,
    source: String,
    scope: Scope,
}

impl Assertion
{
    /// Record that a source asserts a claim at a scope.
    ///
    /// The only way to obtain one. All three participate in the identity, so re-recording the
    /// same source asserting the same claim at the same scope is the same assertion, and any
    /// of the three differing makes a different one.
    #[must_use]
    pub fn By(source: &str, claim: &Claim, scope: Scope) -> Self
    {
        let identity = Derivation::Of(ASSERTION)
            .With_Identity(CLAIM, &claim.Identity())
            .With_Text(SOURCE, source)
            .With_Text(SCOPE, scope.Name())
            .Excluding(
                "strength",
                "D-004 holds the epistemic-strength model; a grade here would be assigned, not measured",
            )
            .Seal()
            .Identity();

        return Self {
            identity,
            claim: claim.Identity(),
            source: kwb_model::Normalize(source),
            scope,
        };
    }

    /// The address this assertion has.
    #[must_use]
    pub const fn Identity(&self) -> ContentIdentity
    {
        return self.identity;
    }

    /// The claim being asserted.
    ///
    /// This is where two sources' assertions meet, and the reason the claim's own identity
    /// excludes the source.
    #[must_use]
    pub const fn Claim(&self) -> ContentIdentity
    {
        return self.claim;
    }

    /// Who asserts it.
    #[must_use]
    pub fn Source(&self) -> &str
    {
        return &self.source;
    }

    /// How far they assert it reaches.
    #[must_use]
    pub const fn Scope(&self) -> &Scope
    {
        return &self.scope;
    }
}
