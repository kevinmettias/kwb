//! A claim: something asserted about a concept, addressed by what it says.

use kwb_model::ContentIdentity;
use kwb_model::Derivation;
use kwb_model::Normalize;

use crate::Concept;

/// The kind a claim's identity is derived under.
const CLAIM: &str = "claim";

/// The concept the claim is about, participating by identity.
const CONCEPT: &str = "concept";

/// What the claim says.
const TEXT: &str = "text";

/// A claim about a concept, and the identity its content gives it.
///
/// # What is deliberately absent, and it is the important part
///
/// **A source.** Excluded from the derivation and the exclusion recorded as a value, so that
/// two references asserting one claim become one claim with two citations. This is the
/// mechanism the whole workspace rests on and this is the first domain type to rest on it.
///
/// **A scope.** `D-010` decided that a scope belongs to an *assertion* of a claim and never
/// to the claim, and the reason is this type's identity: a scope here would participate in
/// the derivation, and then a measurement and a mere preference saying the same thing would
/// become two rows that can never learn of each other. Whether they agree is the one question
/// worth asking of that pair.
///
/// So a claim has no standing of its own. It is a sentence about a concept. Who asserts it,
/// at what scope, on what evidence, and how strongly, are properties of an assertion — which
/// this type does not have yet, and which `D-010` says is where a grade attaches.
///
/// **A confidence.** For the same reason, and for a measured one: the prototype gave 40 of
/// its domain types `Confidence { get; set; } = 1.0`, a settable field defaulting to
/// certainty, which is `D20` in its purest form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Claim
{
    identity: ContentIdentity,
    concept: ContentIdentity,
    text: String,
}

impl Claim
{
    /// Assert something about a concept.
    ///
    /// The only way to obtain one, and it derives rather than accepts the identity. The
    /// concept participates by its identity rather than by its name, which is `D-006`'s
    /// dependency edge: a claim declares what its input *was* when it was derived, so a
    /// renamed concept gives its claims new identities and the staleness is computable
    /// rather than silent.
    #[must_use]
    pub fn About(concept: &Concept, text: &str) -> Self
    {
        let identity = Derivation::Of(CLAIM)
            .With_Identity(CONCEPT, &concept.Identity())
            .With_Text(TEXT, text)
            .Excluding(
                "source",
                "two references asserting one claim must become one claim with two citations",
            )
            .Excluding(
                "scope",
                "D-010: a scope belongs to an assertion of a claim, never to the claim",
            )
            .Seal()
            .Identity();

        return Self {
            identity,
            concept: concept.Identity(),
            text: Normalize(text),
        };
    }

    /// The address this claim has.
    #[must_use]
    pub const fn Identity(&self) -> ContentIdentity
    {
        return self.identity;
    }

    /// The concept this claim is about.
    #[must_use]
    pub const fn Concept(&self) -> ContentIdentity
    {
        return self.concept;
    }

    /// What the claim says, normalized.
    ///
    /// Normalized for the reason [`Concept::Canonical_Name`] is: an accessor that disagreed
    /// with the identity derived from the same input would make what a reader gets back a
    /// function of publication order rather than of content.
    ///
    /// [`Concept::Canonical_Name`]: crate::Concept::Canonical_Name
    #[must_use]
    pub fn Text(&self) -> &str
    {
        return &self.text;
    }
}
