//! Stage zero: the extraction seam, and the four things it may not do.
//!
//! [`ExtractionStrategy`] may not mint an identity, may not write, may not decide what is
//! admitted, and may not return an empty reading in place of a refusal. The readers declared here
//! exist to be each of those failures, because a seam tested only against a well-behaved reader is
//! a seam whose refusals nothing has exercised — and a refusal that is not exercised is the shape
//! of the 1,367-row incident one stage earlier.
//!
//! [`ExtractionStrategy`]: crate::ExtractionStrategy

use super::*;

/// One reading a person states about the passage these tests share, under `lineage`, at `scope`.
///
/// One function rather than a copy per test, so that what differs between two readings is only
/// what the caller names. A second call site that also differed in the bytes read or in where the
/// passage was found could pass while the identity rule was broken for a reason nothing asserted.
fn Reading_Stated_By(
    lineage: ExtractionLineage,
    scope: Scope,
    statement: &str,
) -> ProposedReading
{
    let source = Document::Of(b"a source".to_vec()).Identity();
    let statements = vec![Offered("entropy", statement)];

    return Stated::Of(
        statements,
        SourceLocation::Named("chapter two"),
        lineage,
        scope,
    )
    .expect("the statements are non-empty, so Stated answers Some")
    .Read(source, b"the passage", ReadingKind::Text)
    .expect("a stated reading cannot fail")
    .remove(0);
}

/// The identity of the one claim a reading proposes, which is what a re-read must not change.
fn Claim_Of(reading: &ProposedReading) -> kwb_model::ContentIdentity
{
    let linked = Link_Concepts(reading.Proposed());

    return linked.Claims().first().expect("the reading proposed exactly one statement").Identity();
}

#[test]
fn Test_A_Reader_Who_Stated_Nothing_Should_Not_Become_A_Reader()
{
    // The invariant: model failure — or here, silence — must not become evidence of source
    // barrenness. A `Stated` holding no statements would `Read` successfully and propose
    // nothing, which admission reports as `Barren`: evidence there is nothing in the source.
    // Nobody examined anything, so the honest outcome is `Unmet`, and this is where that is
    // decided — by the reader being unconstructible rather than by every caller remembering.
    assert!(
        Stated::Of(
            Vec::new(),
            SourceLocation::Named("throughout"),
            A_Reading("a person"),
            Unstated(),
        )
        .is_none(),
        "silence was admitted as a reading, and an empty reading is evidence of absence"
    );

    assert!(
        Stated::Of(
            vec![Offered("entropy", "It is non-decreasing.")],
            SourceLocation::Named("throughout"),
            A_Reading("a person"),
            Unstated(),
        )
        .is_some(),
        "a reader who stated something must be a reader"
    );
}

#[test]
fn Test_Changing_The_Protocol_Should_Not_Redefine_What_A_Claim_Is()
{
    // The invariant this seam exists to protect: changing the model or the prompt must not
    // silently redefine KWB semantic identity. `D-002` excludes the source from a claim's
    // derivation so that two books asserting one thing are one claim; lineage is the same
    // question one level down, and it is answered by lineage travelling on the *reading*
    // rather than on the claim.
    //
    // The reference miner is the worked example of the other answer: its own claim identity
    // absorbs the source path and the page window, so the same sentence read twice is two
    // claims. A test that only checked the types would not see the difference.
    let statement = "It is non-decreasing in an isolated system.";
    let once = A_Reading_Under("read-once-v1", "a person");
    let again = A_Reading_Under("read-again-v2", "a model");
    let first = Reading_Stated_By(once, Unstated(), statement);
    let second = Reading_Stated_By(again, Unstated(), statement);

    assert_ne!(first.Lineage(), second.Lineage(), "the readings must differ, or this proves nothing");
    assert_eq!(
        Claim_Of(&first),
        Claim_Of(&second),
        "a re-read under a new protocol produced a different claim, so changing the prompt \
         silently redefined what this repository thinks a proposition is"
    );
}

#[test]
fn Test_A_Refusal_Should_Not_Be_Readable_As_A_Reading_That_Found_Nothing()
{
    // `Coverage`'s 1,367-row incident, one stage earlier. A refusal carries no proposals at
    // all — not an empty list of them — so there is no value a caller can take out of a failed
    // reading and hand to `Admit` that would make it look examined-and-empty. The failure is
    // in the return type, which is why it cannot be got wrong by a caller who forgets.
    let source = Document::Of(b"a source".to_vec()).Identity();
    let refusal = Silent
        .Read(source, b"the passage", ReadingKind::Text)
        .expect_err("a silent reader must refuse rather than propose nothing");

    // And it must say why in terms a report can print, because the consequence — nothing was
    // learned about the source — is the part a reader of that report has to act on, and it is
    // the part that distinguishes this from a source that really is empty.
    let reported = refusal.to_string();
    assert!(
        reported.contains("the answer did not parse"),
        "a refusal must report what went wrong: {reported}"
    );
    assert!(
        reported.contains("Nothing was learned about the source"),
        "a refusal must report the consequence, not only the cause: {reported}"
    );
}

#[test]
fn Test_A_Reading_Should_Carry_The_Address_Of_What_Was_Read()
{
    // Every admitted claim must identify its source occurrence and the protocol it was read
    // under. The source is carried as a content address rather than a path, so it names the
    // exact bytes admitted — a renamed or moved file is the same source, and an edited one is
    // not, which a filename cannot express.
    let source = Document::Of(b"a source".to_vec()).Identity();
    let reading = Stated::Of(
        vec![Offered("entropy", "It is non-decreasing.")],
        SourceLocation::Named("chapter two"),
        A_Reading("a person"),
        Scope::Named("physical theory").expect("a named scope"),
    )
    .expect("the statements are non-empty, so Stated answers Some")
    .Read(source, b"the passage", ReadingKind::Text)
    .expect("a stated reading cannot fail")
    .remove(0);

    assert_eq!(reading.Source(), source);
    assert_eq!(reading.Location().Description(), "chapter two");
    assert_eq!(reading.Lineage().Protocol(), "stated-by-a-person");
    assert_eq!(reading.Lineage().Reader(), "a person");
}

/// A reader that does text and nothing else, which is every extractor this repository expects
/// to gain first. It exists to demonstrate the refusal, because `Stated` cannot: a person has
/// no capability gap, so the asymmetry needs both sides present to be a test of anything.
struct TextOnly;

impl ExtractionStrategy for TextOnly
{
    fn Read(
        &self,
        source: kwb_model::ContentIdentity,
        content: &[u8],
        needed: ReadingKind,
    ) -> Result<Vec<ProposedReading>, ExtractionError>
    {
        if needed != ReadingKind::Text
        {
            return Err(ExtractionError::CannotRead { needed });
        }

        // Deliberately trivial: what a real reader proposes is not this test's subject. What is
        // its subject is that a reader which *can* read proposes, and one which cannot refuses.
        let proposed = core::str::from_utf8(content).map_or_else(
            |_| return Vec::new(),
            |text| return vec![Offered("a concept", text)],
        );

        return Ok(vec![ProposedReading::Of(
            source,
            SourceLocation::Named("throughout"),
            proposed,
            ExtractionLineage::Of("read-the-text-v1", "a text extractor"),
        )]);
    }

    fn Scope(&self) -> Scope
    {
        return Unstated();
    }
}

#[test]
fn Test_A_Text_Reader_Should_Refuse_A_Source_That_Needs_Looking_At()
{
    // The distinction the first extractor has to be able to draw. A scanned page handed to a
    // text reader is not a page with nothing in it, and the only way to keep those apart is for
    // the reader to say it cannot do this kind of reading.
    let mut store = DocumentStore::Empty();

    let report = Admit(b"a scan".to_vec(), Some(&TextOnly), ReadingKind::Visual, &mut store)
        .expect("the source is admitted even though it could not be read");

    Assert_Unmet_And_Not_Evidence_Of_Absence(&report, "a page nobody could read");
    assert_eq!(
        report.Refusal(),
        Some(&ExtractionError::CannotRead {
            needed: ReadingKind::Visual
        }),
        "the refusal must say which kind of reading was needed, or a report cannot say what \
         would have to change"
    );
}

#[test]
fn Test_The_Same_Reader_Should_Read_A_Source_That_Is_Already_Text()
{
    // The other half, and the reason the test above proves something: this reader refuses on
    // the kind of reading and not on everything, so the refusal is a distinction rather than a
    // reader that never works.
    let mut store = DocumentStore::Empty();

    let report = Admit(
        b"a passage".to_vec(),
        Some(&TextOnly),
        ReadingKind::Text,
        &mut store,
    )
    .expect("the source is non-empty, so admission runs");

    assert_eq!(report.Coverage().Name(), "yielded");
    assert_eq!(report.Assertions().len(), 1);
    assert!(report.Refusal().is_none(), "a reading that happened is not a refusal");
}

/// A reader that answers about a document it was not given.
///
/// Not a hypothetical. `Admit` cites the document *it* wrote and never compared it with the one
/// the reading names, so this reader's proposals were attributed to whatever source happened to
/// be passed in, with a citation that resolved perfectly to the wrong bytes.
struct Confused;

impl ExtractionStrategy for Confused
{
    fn Read(
        &self,
        _source: kwb_model::ContentIdentity,
        _content: &[u8],
        _needed: ReadingKind,
    ) -> Result<Vec<ProposedReading>, ExtractionError>
    {
        return Ok(vec![ProposedReading::Of(
            Document::Of(b"some other document".to_vec()).Identity(),
            SourceLocation::Named("throughout"),
            vec![Offered("entropy", "It is non-decreasing.")],
            A_Reading("a reader with the wrong book open"),
        )]);
    }

    fn Scope(&self) -> Scope
    {
        return Unstated();
    }
}

#[test]
fn Test_A_Reading_About_Another_Document_Should_Not_Be_Cited_As_This_One()
{
    // That a citation resolves to the exact bytes a claim was read out of is the strongest
    // thing this repository claims, and `D-006`'s dependency edge rests on it. An unchecked
    // field is not provenance; it is a field.
    let mut store = DocumentStore::Empty();

    let report = Admit(b"a source".to_vec(), Some(&Confused), ReadingKind::Text, &mut store)
        .expect("the source is admitted; only the reading is refused");

    Assert_Unmet_And_Not_Evidence_Of_Absence(&report, "a reader with the wrong book open");
    assert!(
        report.Assertions().is_empty(),
        "a reading about another document was cited as this one, so the citation resolves to \
         bytes the claim was not read out of"
    );
    assert!(
        report
            .Refusal()
            .is_some_and(|refusal| return matches!(refusal, ExtractionError::ReaderFailed { .. })),
        "a reader that answered about the wrong document was not reported as having failed"
    );
}

#[test]
fn Test_A_Reading_About_The_Right_Document_Should_Still_Be_Admitted()
{
    // The control. Without it the test above passes against a check that refuses every reading,
    // which would be a guard that reports the strongest possible provenance by admitting
    // nothing at all.
    let mut store = DocumentStore::Empty();

    let report = Admit(
        b"a source".to_vec(),
        Some(&Said(&[Offered("entropy", "It is non-decreasing.")], &Unstated())),
        ReadingKind::Text,
        &mut store,
    )
    .expect("the source is non-empty, so admission runs");

    assert_eq!(report.Assertions().len(), 1);
    assert!(report.Refusal().is_none());
}
