//! A concept: a thing the corpus talks about, addressed by what it is called.

use kwb_model::ContentIdentity;

/// The kind a concept's identity is derived under.
const CONCEPT: &str = "concept";

/// The one field a concept is addressed by.
const CANONICAL_NAME: &str = "canonical_name";

/// A concept, and the identity its canonical name gives it.
///
/// # Case is significant here and was not in the prototype
///
/// A divergence, recorded where it happens as `D-001` requires. The prototype arbitrated
/// name collisions with a partial unique index over `LOWER("CanonicalName")`, so `Polish
/// notation` and `polish notation` were one concept. `kwb-model`'s [`Normalize_Text`] deliberately
/// does not fold case — a reflow is not an edit and a changed capitalisation is — so here
/// they are two.
///
/// This is a divergence rather than an oversight, and it has a cost worth naming: two
/// concepts that differ only in case will not merge on identity, and something else will have
/// to decide whether they should. That is preferred to the alternative, which is folding
/// `BVH` into `bvh` and having no way back.
///
/// [`Normalize_Text`]: kwb_model::Normalize_Text
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Concept
{
    identity: ContentIdentity,
    canonical_name: String,
}

impl Concept
{
    /// Address a concept by its canonical name.
    ///
    /// The only way to obtain one, and it derives rather than accepts the identity.
    #[must_use]
    pub fn Named(canonical_name: &str) -> Self
    {
        use kwb_model::Derivation;
        use kwb_model::Normalize_Text;

        let identity = Derivation::Of(CONCEPT)
            .With_Text(CANONICAL_NAME, canonical_name)
            .Excluding(
                "source",
                "a concept named by two references is one concept with two citations",
            )
            .Seal()
            .Identity();

        return Self {
            identity,
            canonical_name: Normalize_Text(canonical_name),
        };
    }

    /// The address this concept has.
    #[must_use]
    pub const fn Identity(&self) -> ContentIdentity
    {
        return self.identity;
    }

    /// The name it is addressed by, normalized.
    ///
    /// **Normalized rather than as given**, so that this never disagrees with
    /// [`Identity`][Self::Identity]. Two spellings differing only in whitespace derive one
    /// identity — a reflow is not an edit — and before `KWB-32` they rendered as two different
    /// names, so which one a reader got back depended on which was published first. In a
    /// repository whose premise is that content decides everything, insertion order deciding
    /// anything is the defect.
    ///
    /// Case is **not** folded, here or anywhere: `BVH` and `bvh` are different concepts.
    #[must_use]
    pub fn Canonical_Name(&self) -> &str
    {
        return &self.canonical_name;
    }
}
