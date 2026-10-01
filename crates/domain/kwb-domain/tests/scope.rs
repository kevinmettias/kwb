//! How far an assertion is claimed to reach — asserted function by function.
//!
//! # Why this file is here rather than in `src/tests.rs`
//!
//! This crate's tests used to be one file, `src/tests.rs`, and its stem names no source file. So
//! every function below was exercised and addressed by nothing: the unit `check-test-coverage`
//! reads is the test file's own stem, and `src/epistemic/scope.rs` is covered by a file called
//! `scope.rs` and by nothing else. A failure in `Is_Unstated` arrived under the name of a test
//! about a claim.
//!
//! # The distinction these tests hold, and the incident that made it necessary
//!
//! `D-010`: an unstated scope stays distinguishable from every stated one, because a source that
//! did not say how far it meant has not said the narrowest thing. The way that failed was not a
//! default filling in a guess — it was a **constructor quietly producing the absent case from
//! present input**. `Scope::Named("   ")` *was* `Scope::Unstated()`, so `kwb admit --scope "   "`
//! and `kwb admit` with no `--scope` at all wrote byte-identical assertion records.

use kwb_domain::Scope;

/// How many shapes text that names nothing arrives in from a person or a shell.
const SHAPES_OF_TEXT_THAT_NAMES_NOTHING: usize = 4;

/// Text that names nothing, in the four shapes a person or a shell actually supplies.
fn Names_That_Say_Nothing() -> [&'static str; SHAPES_OF_TEXT_THAT_NAMES_NOTHING]
{
    return ["", "   ", "\t", "\n  \n"];
}

#[test]
fn Test_Named_Should_Refuse_Text_That_Names_Nothing()
{
    // A door rather than a guard at one command line: closing it here is what makes a blank input
    // reaching a record unreachable instead of merely unwritten.
    for names_nothing in Names_That_Say_Nothing()
    {
        assert_eq!(
            Scope::Named(names_nothing),
            None,
            "{names_nothing:?} named a scope, so blank input can reach a record again"
        );
    }

    assert!(
        Scope::Named("physical theory").is_some(),
        "a scope with a name was refused, so this test would pass on a constructor that refuses \
         everything"
    );
}

#[test]
fn Test_An_Unstated_Scope_Should_Be_Recognisable_Rather_Than_Guessed()
{
    // # What this test used to assert, and why it changed
    //
    // It passed `Scope::Named("   ")` and asserted the result was unstated. That was true, and it
    // was the defect: the mapping is defensible — whitespace does name nothing — but it happened
    // **silently**, so a person who typed a scope was told nothing and the record said they had
    // said nothing. `KWB-50` measured it.
    //
    // The rule is now that blank text names no scope and the caller says `Unstated` when that is
    // what it means. Left as a comment rather than replaced quietly, because a test that encoded
    // the old behaviour as intended is evidence about what was believed.
    let unstated = Scope::Unstated();

    assert!(unstated.Is_Unstated());
    assert_eq!(unstated.Name(), "", "the absent scope carries a name it should not have");
    assert_ne!(
        Scope::Named("physical theory").expect("a named scope"),
        unstated,
        "a stated scope and the absent one compare equal, so nothing downstream can tell them apart"
    );
}

#[test]
fn Test_Name_Should_Reflow_Whitespace_But_Not_Fold_Case()
{
    // Consistent with `Concept`, and wrong to decide differently here without a reason that applies
    // only here: a reflow is not an edit, and a changed capitalisation is.
    assert_eq!(
        Scope::Named("physical  theory\n").expect("a named scope").Name(),
        Scope::Named("physical theory").expect("a named scope").Name(),
        "a run of whitespace in a scope's name was not collapsed"
    );
    assert_ne!(
        Scope::Named("Physical theory").expect("a named scope").Name(),
        Scope::Named("physical theory").expect("a named scope").Name(),
        "case was folded, so two scopes a source stated separately are one"
    );
}

#[test]
fn Test_Is_Unstated_Should_Be_True_Of_The_Scope_That_Says_Nothing()
{
    assert!(Scope::Unstated().Is_Unstated());
    assert!(
        !Scope::Named("physical theory").expect("a named scope").Is_Unstated(),
        "a scope with a name reported itself as unstated, so the distinction is gone from the \
         query as well as from the constructor"
    );
}
