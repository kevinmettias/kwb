//! Why a reading did not happen, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/admission.rs`
//!
//! Every refusal this crate can report is turned into a `Coverage::Unmet` through `Prerequisite`,
//! and no test names that function. The unit this check reads is the test file's own stem, so
//! `src/extractor/extraction_error.rs` is covered by a file called `extraction_error.rs` and by
//! nothing else — and a change to the words a report compares would have arrived under the name
//! of a test about a report.

use kwb_ingest::ExtractionError;
use kwb_ingest::ReadingKind;

/// How many refusals the crate can produce, one per kind: those are the rows this table holds.
///
/// Named rather than written in the return type, because the number is a fact about
/// `ExtractionError` rather than about the test — and a kind added to it has to join this table
/// visibly rather than leave the distinctness check below asserting over a subset.
const REFUSALS_THIS_CRATE_CAN_PRODUCE: usize = 3;

/// One refusal of each kind this crate can produce.
///
/// A provider rather than three literals in the test, so that the three answers are one table a
/// fourth variant would visibly fail to join.
fn Every_Refusal() -> [ExtractionError; REFUSALS_THIS_CRATE_CAN_PRODUCE]
{
    return [
        ExtractionError::CannotRead {
            needed: ReadingKind::Visual,
        },
        ExtractionError::NotRead,
        ExtractionError::ReaderFailed {
            cause: "the answer did not parse".to_owned(),
        },
    ];
}

#[test]
fn Test_Prerequisite_Should_Name_What_Was_Needed_And_Absent()
{
    // A `&'static str`, because `Coverage::Unmet` takes one: a prerequisite is a decision written
    // in code rather than a message assembled at runtime, and that is what makes two reports of
    // the same unmet prerequisite comparable. This is the one place the three refusals become the
    // words a report compares.
    assert_eq!(
        ExtractionError::CannotRead {
            needed: ReadingKind::Visual
        }
        .Prerequisite(),
        "a reading this extractor cannot do"
    );
    assert_eq!(ExtractionError::NotRead.Prerequisite(), "a reader");
    assert_eq!(
        ExtractionError::ReaderFailed {
            cause: "the answer did not parse".to_owned()
        }
        .Prerequisite(),
        "an answer from the reader"
    );
}

#[test]
fn Test_Prerequisite_Should_Tell_The_Three_Refusals_Apart()
{
    // Three distinct answers, which is the assertion rather than the wording. A report that could
    // not tell *nobody read it* from *the reader failed* would send a person to fix the wrong
    // thing: the first means supply a `--says`, and the second means look at the reader. Both
    // halves are asserted, because any single answer satisfies an equality on its own.
    let mut prerequisites: Vec<&'static str> = Every_Refusal()
        .iter()
        .map(|refusal| return refusal.Prerequisite())
        .collect();

    assert_eq!(prerequisites.len(), Every_Refusal().len());
    prerequisites.sort_unstable();
    prerequisites.dedup();

    assert_eq!(
        prerequisites.len(),
        Every_Refusal().len(),
        "two refusals report one prerequisite, so a person reading a report cannot tell which of \
         them happened"
    );
}
