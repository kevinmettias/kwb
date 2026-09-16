//! The contract between this crate's types and `kwb_model`'s derivation, asserted from outside
//! both.
//!
//! # Why this file is named for a property rather than for a type
//!
//! Every type in this crate derives its address through [`Derivation`], and the two crates agree
//! only by **spelling**: a kind string, a set of field names, which of the three doors a value goes
//! through, and what normalization does to it. Each side's own tests can see its own half of that
//! and no more. `kwb-model` does not know what a concept is, and this crate's tests read the same
//! private constants the code they exercise reads — so the two could drift apart with every test on
//! both sides still passing, and what would break is every address this repository has ever handed
//! out.
//!
//! The derivations below are therefore written out **by hand**, with the kind and the field names
//! spelled as literals in this file. That is the whole assertion: a kind renamed on either side, a
//! field that stopped being where a name goes, or a value moved from the text door to the opaque
//! one all change the digest, and nothing else in either crate would notice.
//!
//! # Why a stem that names no source file is the right name for it
//!
//! The unit `check-test-coverage` reads is the test file's own stem, so a file called
//! `one_identity.rs` addresses no source file and covers nothing. That is correct rather than a
//! gap: the property here is about the seam between two crates and belongs to neither side's
//! files. Filing it under `concept.rs` would say the contract is the concept's, when a claim and an
//! assertion derive through the same builder and the same field name for the concept they are
//! about.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::Scope;
use kwb_model::ContentIdentity;
use kwb_model::Derivation;
use kwb_model::Normalize_Text;

/// The kind a concept's address is derived under.
///
/// Written here rather than imported, because a test that took the constant from the code under
/// test could not notice the code changing it — and this is the only place a change to either
/// side's spelling is visible at all.
const CONCEPT_KIND: &str = "concept";

/// The one field a concept participates by.
const CONCEPT_NAME_FIELD: &str = "canonical_name";

/// The kind a claim's address is derived under.
const CLAIM_KIND: &str = "claim";

/// The field a claim's concept participates under — the same name a concept's own kind uses, and
/// deliberately not a second spelling of it.
const CLAIM_CONCEPT_FIELD: &str = "concept";

/// The field a claim's text participates under.
const CLAIM_TEXT_FIELD: &str = "text";

/// The kind an assertion's address is derived under.
const ASSERTION_KIND: &str = "assertion";

/// The field an assertion's claim participates under.
const ASSERTION_CLAIM_FIELD: &str = "claim";

/// The field an assertion's source participates under.
const ASSERTION_SOURCE_FIELD: &str = "source";

/// The field an assertion's scope participates under.
const ASSERTION_SCOPE_FIELD: &str = "scope";

/// The one name the fixtures below are about.
const NAME: &str = "entropy";

/// What the fixtures say about it.
const TEXT: &str = "It is non-decreasing in an isolated system.";

/// Who says it.
const SOURCE: &str = "Callen 1985";

/// How far they meant it.
const SCOPE_NAME: &str = "physical theory";

/// The same scope, written the way a model that re-indented its own output would write it.
const SCOPE_NAME_REFLOWED: &str = "physical   theory";

/// How many spellings of one name differ only in the whitespace around it, which is every spelling
/// a reflow of that name can take.
const SPELLINGS_DIFFERING_ONLY_IN_WHITESPACE: usize = 4;

// ---- the scheme, written out where a change to either side is visible ----

/// # What the exclusions are not asserted here
///
/// Every one of the three derivations below leaves out a field the domain type has no way to
/// record — a concept's source, a claim's source and scope, an assertion's strength — and an
/// exclusion contributes nothing to the digest by definition, so it cannot be asserted by
/// comparing addresses. What *is* asserted is the complement, and it is the same guarantee: the
/// address equals the digest of a derivation carrying **exactly** the fields listed here and
/// nothing else. A crate that had folded a source into a concept's address would answer a
/// different digest, and so would one that had stopped folding in the name.
#[test]
fn Test_A_Concept_Should_Address_Itself_By_Its_Name_Under_The_Concept_Kind()
{
    let derived = Derivation::Of(CONCEPT_KIND).With_Text(CONCEPT_NAME_FIELD, NAME).Seal();

    assert_eq!(
        Concept::Named(NAME).Identity(),
        derived.Identity(),
        "a concept's address is no longer the digest of its canonical name under the kind \
         `concept`, so every concept this workspace has published has changed address"
    );
}

#[test]
fn Test_A_Claim_Should_Address_Itself_By_Its_Concept_And_Its_Text_Under_The_Claim_Kind()
{
    let concept = Concept::Named(NAME);

    let derived = Derivation::Of(CLAIM_KIND)
        .With_Identity(CLAIM_CONCEPT_FIELD, &concept.Identity())
        .With_Text(CLAIM_TEXT_FIELD, TEXT)
        .Seal();

    assert_eq!(
        Claim::About(&concept, TEXT).Identity(),
        derived.Identity(),
        "a claim's address is no longer the digest of its concept's identity and its text under \
         the kind `claim`, so the cross-source dedup the whole workspace rests on has changed \
         meaning"
    );
}

#[test]
fn Test_An_Assertion_Should_Address_Itself_By_Its_Claim_Source_And_Scope_Under_The_Assertion_Kind()
{
    let concept = Concept::Named(NAME);
    let claim = Claim::About(&concept, TEXT);
    let scope = Scope::Named(SCOPE_NAME).expect("a named scope");

    let derived = Derivation::Of(ASSERTION_KIND)
        .With_Identity(ASSERTION_CLAIM_FIELD, &claim.Identity())
        .With_Text(ASSERTION_SOURCE_FIELD, SOURCE)
        .With_Text(ASSERTION_SCOPE_FIELD, SCOPE_NAME)
        .Seal();

    assert_eq!(
        Assertion::By(SOURCE, &claim, scope).Identity(),
        derived.Identity(),
        "an assertion's address is no longer the digest of its claim, its source and its scope \
         under the kind `assertion`, so who asserted what, at what scope, has changed meaning"
    );
}

// ---- the dependency edge, which is the one door that is not the text door ----

#[test]
fn Test_A_Claim_Should_Address_Its_Concept_By_Identity_Rather_Than_By_Name()
{
    // `With_Identity` rather than `With_Text` is `D-006`'s dependency edge, and it is a choice
    // made *at this seam*: a claim declares the identity its input had when it was derived, so a
    // concept that was re-spelled gives its claims new addresses and the staleness is computable
    // rather than silent.
    //
    // Read off the crate's own answers rather than off a derivation, because the failure it guards
    // is a claim that took a concept's *name*: two concepts differing in a spelling the text door
    // folds would then be one concept's worth of claims, and `Claim::Concept` would have nothing
    // to hand back but the concept's name.
    let entropy = Concept::Named(NAME);
    let enthalpy = Concept::Named("enthalpy");
    let claim = Claim::About(&entropy, TEXT);

    assert_eq!(
        claim.Concept(),
        entropy.Identity(),
        "a claim does not hand back the address of the concept it is about"
    );
    assert_ne!(
        claim.Identity(),
        Claim::About(&enthalpy, TEXT).Identity(),
        "two claims saying the same thing about two different concepts share one address, so the \
         concept stopped participating and `D-006`'s edge is gone"
    );
}

// ---- normalization is one rule, applied by both sides ----

/// Spellings of one name that differ only in the whitespace around it.
///
/// A table rather than a list inside the test, because the property is about all of them at once
/// and a spelling added here must be seen by the assertion below it.
fn One_Name_Reflowed() -> [&'static str; SPELLINGS_DIFFERING_ONLY_IN_WHITESPACE]
{
    return [NAME, " entropy", "entropy ", "  entropy  "];
}

/// Every spelling in a reflow table answers one address.
///
/// Named rather than left where it is used, because "all of these are one concept" is the property
/// the table exists to be asked about, and the message it fails with has to name the spelling that
/// disagreed.
fn Assert_Every_Reflow_Answers_The_Address(spellings: &[&'static str], address: &ContentIdentity)
{
    for spelling in spellings
    {
        assert_eq!(
            Concept::Named(spelling).Identity(),
            *address,
            "{spelling:?} named a different concept, so a reflow is an edit to one of the two \
             crates and not to the other"
        );
    }
}

#[test]
fn Test_A_Reflow_Should_Not_Move_A_Concept_Between_The_Two_Crates()
{
    // Both sides normalize, and the assertion that matters is that they normalize *the same way*.
    // The expected address is built from the text the crate actually keeps, so a domain type that
    // stored an unnormalized spelling while the model digested a normalized one would answer two
    // addresses for one name — and which of the two a reader got back would depend on which was
    // published first.
    let reflowed = Concept::Named("  entropy  ");

    let derived = Derivation::Of(CONCEPT_KIND)
        .With_Text(CONCEPT_NAME_FIELD, reflowed.Canonical_Name())
        .Seal();

    assert_eq!(
        reflowed.Canonical_Name(),
        Normalize_Text("  entropy  "),
        "the name this crate keeps is not the text the model would have digested, so the address \
         and what a reader is handed back come from two different strings"
    );

    Assert_Every_Reflow_Answers_The_Address(&One_Name_Reflowed(), &derived.Identity());
}

#[test]
fn Test_A_Reflow_Should_Not_Move_An_Assertion_Between_The_Two_Crates()
{
    // The same property for the two fields an assertion carries as text, and it is worth asking
    // separately: a concept's name is normalized by the domain type before it reaches the builder,
    // while an assertion's source is handed to the builder raw and normalized by it. Two routes to
    // one rule, and this is where they would come apart.
    let concept = Concept::Named(NAME);
    let claim = Claim::About(&concept, TEXT);
    let reflowed_source = "Callen   1985";

    assert_eq!(
        Assertion::By(
            reflowed_source,
            &claim,
            Scope::Named(SCOPE_NAME_REFLOWED).expect("a named scope"),
        )
        .Identity(),
        Assertion::By(
            SOURCE,
            &claim,
            Scope::Named(SCOPE_NAME).expect("a named scope"),
        )
        .Identity(),
        "a source spelled with a wider gap, or a scope that was reflowed, made a different \
         assertion — so the two crates disagree about what a reflow is and one claim becomes two"
    );
}

// ---- what a consumer actually holds ----

#[test]
fn Test_An_Address_This_Crate_Hands_Out_Should_Round_Trip_Through_The_Model()
{
    // `AGENTS.md` routes a change to the identity scheme through Nomos's `D-137`, which admits a
    // KWB-minted identifier on the understanding that its value is whatever string identity
    // `kwb-model` derives. A consumer stores that string, so it has to be a string the model can
    // read back — otherwise a claim this crate published is one no later process can identify.
    let identity = Concept::Named(NAME).Identity();

    assert_eq!(
        ContentIdentity::Parse(&identity.Render()).expect("an address this crate handed out"),
        identity,
        "an address this crate published does not survive being rendered and read back, so a \
         stored citation cannot be resolved against the graph it came from"
    );
    assert_ne!(
        identity.Render(),
        Concept::Named("enthalpy").Identity().Render(),
        "two concepts render one address, so one of them is unreachable behind the other's"
    );
}
