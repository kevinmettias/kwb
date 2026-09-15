//! The trailing time a record may carry, read off whole records rather than off fields.
//!
//! # Why this file is here rather than in `src/tests.rs`
//!
//! This crate's tests used to be one file, `src/tests.rs`, and its stem names no source file. So
//! every function below was exercised and addressed by nothing: the unit `check-test-coverage`
//! reads is the test file's own stem, and `src/versioning/record_time.rs` is covered by a file
//! called `record_time.rs` and by nothing else. A failure in `Published_At` arrived under the name
//! of a test about a merge.
//!
//! # What only a caller can ask
//!
//! `Without_Time` is the split itself and is `pub(crate)`; its test sits beside it in
//! `src/versioning/record_time.rs`, where the fields and the time can be asked about separately.
//! From outside there is one number, and the question it answers is whether a record says when it
//! was published — **unknown** and not a zero, because a caller temporally reading the graph has to
//! decide what to do about records that cannot answer rather than being handed an epoch that sorts
//! before everything.
//!
//! The records here are written by `Publication::Record`, which is the producer. A test that typed
//! a record out by hand would go on passing after the format changed, and the format is the half
//! that is not allowed to change quietly: `D-014` made the graph durable *by* replay, so a change
//! that orphans existing logs loses the thing it was built for.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::Publication;
use kwb_domain::Published_At;
use kwb_domain::Scope;
use kwb_domain::Standing;

/// The time the fixtures below say they were published at, in Unix seconds.
const PUBLISHED_AT: i64 = 1_730_000_000;

/// One publication of each kind, which is what tells apart a reader that reads a trailing field
/// from one that reads it only for the kinds whose records have one.
fn Every_Kind() -> [Publication; 3]
{
    let concept = Concept::Named("entropy");
    let claim = Claim::About(&concept, "It is non-decreasing in an isolated system.");
    let assertion = Assertion::By(
        "Callen 1985",
        &claim,
        Scope::Named("physical theory").expect("a named scope"),
    );

    return [
        Publication::Concept {
            concept,
            standing: Standing::Asserted,
        },
        Publication::Claim {
            claim,
            standing: Standing::Asserted,
        },
        Publication::Assertion {
            assertion,
            standing: Standing::Asserted,
        },
    ];
}

/// An assertion whose scope is the text of a year.
///
/// An assertion's last field before a time is a scope, and a source is free to call a scope `1985`.
/// A reader that decided a record carried a time by asking whether its last field *looks like a
/// number* would read this scope as one.
fn An_Assertion_Scoped_To_A_Year() -> Publication
{
    let concept = Concept::Named("entropy");
    let claim = Claim::About(&concept, "It is non-decreasing in an isolated system.");

    return Publication::Assertion {
        assertion: Assertion::By(
            "Callen 1985",
            &claim,
            Scope::Named("1985").expect("a named scope"),
        ),
        standing: Standing::Asserted,
    };
}

#[test]
fn Test_Published_At_Should_Answer_The_Time_A_Record_Carries()
{
    for publication in Every_Kind()
    {
        assert_eq!(
            Published_At(&publication.Record(Some(PUBLISHED_AT))),
            Some(PUBLISHED_AT),
            "a record that says when it was published does not answer with that time, so a caller \
             asking what the graph looked like at a moment cannot find the records that describe it"
        );
    }
}

#[test]
fn Test_Published_At_Should_Answer_Nothing_For_A_Record_Written_Before_There_Was_A_Time()
{
    // Both shapes have to keep reading, because replay dispatches on how many fields a record has.
    // A log written before `KWB-64` genuinely has no time -- not a zero, not an epoch, not the
    // moment it happened to be read back.
    for publication in Every_Kind()
    {
        assert_eq!(
            Published_At(&publication.Record(None)),
            None,
            "a record that does not say when it was published was given a time anyway, and there \
             is no epoch that can be told apart from a real one"
        );
    }
}

#[test]
fn Test_Published_At_Should_Not_Mistake_A_Scope_For_A_Time()
{
    // The reason the arity is a fact about the kind rather than a guess about the content. A scope
    // called `1985` read as a time would drop the scope off the record and hand a caller a
    // publication date of 1985 seconds after the epoch.
    let publication = An_Assertion_Scoped_To_A_Year();

    assert_eq!(
        Published_At(&publication.Record(None)),
        None,
        "a scope that reads like a number was taken for the record's time, so the record lost its \
         scope and gained a publication date nothing wrote"
    );
    assert_eq!(
        Published_At(&publication.Record(Some(PUBLISHED_AT))),
        Some(PUBLISHED_AT),
        "the same record with a time appended is not read, so the fix refused the record it was \
         written to accept"
    );
}

#[test]
fn Test_Published_At_Should_Answer_Nothing_For_A_Record_It_Cannot_Read()
{
    // A record of a kind this reader does not know, and no record at all. Both are *cannot answer*
    // rather than *published at zero*, which is the same distinction the rest of this file turns
    // on.
    assert_eq!(Published_At("nonsense"), None, "a record of an unknown kind was given a time");
    assert_eq!(Published_At(""), None, "an empty line was given a time");
}
