//! How far an assertion is claimed to reach.

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
/// Through `kwb-model`'s [`Normalize_Text`](kwb_model::Normalize_Text), for the reason every other
/// text field goes through it: a reflow is not an edit. Case is deliberately preserved, so
/// `Thermodynamics` and `thermodynamics` are two scopes — consistent with `Concept`, and wrong to
/// decide differently here without a reason that applies only here.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scope(String);

impl Scope
{
    /// A scope named by text, or [`None`] if the text names nothing.
    ///
    /// # Why this refuses rather than returning the unstated scope
    ///
    /// It used to return `Self(Normalize_Text(name))`, so `Named("")` and `Named("   ")` *were* the
    /// unstated scope. Measured on 2026-09-12 against a rebuilt binary: `kwb admit --scope "   "`
    /// and `kwb admit` with no `--scope` at all wrote byte-identical assertion records. A person
    /// who typed a scope and had it swallowed was told nothing, and the record said they had
    /// said nothing.
    ///
    /// That is `D-010` inverted. The decision is that an unstated scope stays distinguishable
    /// from every stated one, because a source that did not say how far it meant has not said
    /// the narrowest thing — and the way it failed was not a default filling in a guess, which
    /// is what `D-010` guards, but a **constructor quietly producing the absent case from
    /// present input**.
    ///
    /// `kwb-store` already refuses this shape one crate down: a document of zero bytes is
    /// `StoreError::Vacuous`, because something indistinguishable from a failed read is not
    /// worth recording as admitted. Named in prose rather than linked, because `kwb-domain`
    /// does not depend on `kwb-store`.
    ///
    /// So the two cases now have two constructors, and neither can produce the other.
    #[must_use]
    pub fn Named(name: &str) -> Option<Self>
    {
        use kwb_model::Normalize_Text;

        let normalized = Normalize_Text(name);
        if normalized.is_empty()
        {
            return None;
        }

        return Some(Self(normalized));
    }

    /// The scope of a source that did not say how far it reached.
    ///
    /// A real answer and not a missing one, which is why it is a constructor with a name rather
    /// than an empty string every caller has to recognise. Three callers spelled it
    /// `Scope::Named("")` before this existed, and each of them had to know that the empty
    /// string meant *unstated* rather than *a scope whose name is blank*.
    ///
    /// The type it returns is the same type a named scope returns, deliberately. An unstated
    /// scope participates in an assertion's identity exactly like a stated one — two sources
    /// that both declined to say how far they reached are agreeing about something, and
    /// splitting the type would make that unsayable.
    #[must_use]
    pub fn Unstated() -> Self
    {
        return Self(String::new());
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
