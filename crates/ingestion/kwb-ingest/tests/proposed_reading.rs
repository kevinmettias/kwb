//! What one reading of one source proposed, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/extraction.rs`
//!
//! Every reading in this workspace comes back as one of these and no test names any of its
//! functions. The unit this check reads is the test file's own stem, so
//! `src/extractor/proposed_reading.rs` is covered by a file called `proposed_reading.rs` and by
//! nothing else — and a change to what a reading carries would have arrived under the name of a
//! test about the seam that produced it.

use kwb_ingest::ClaimText;
use kwb_ingest::ConceptName;
use kwb_ingest::Extraction;
use kwb_ingest::ExtractionLineage;
use kwb_ingest::ProposedReading;
use kwb_ingest::ReaderName;
use kwb_ingest::ReadingProtocol;
use kwb_ingest::SourceLocation;
use kwb_model::ContentIdentity;
use kwb_store::Document;

/// The address of the document these readings are about, derived the way the store derives one.
fn The_Source() -> ContentIdentity
{
    return Document::Of(b"a source".to_vec()).Identity();
}

/// The address of a different document, so that *the source is this one* is a distinction rather
/// than a value.
fn Another_Source() -> ContentIdentity
{
    return Document::Of(b"another source".to_vec()).Identity();
}

fn A_Location() -> SourceLocation
{
    return SourceLocation::Named("chapter two");
}

fn A_Lineage() -> ExtractionLineage
{
    return ExtractionLineage::Of(
        ReadingProtocol::Named("read-the-text-v1"),
        ReaderName::Named("a text extractor"),
    );
}

/// What these readings propose: two statements, so that a count is asserted rather than a
/// presence.
fn Statements() -> Vec<Extraction>
{
    return vec![
        An_Extraction("entropy", "It is non-decreasing."),
        An_Extraction("entropy", "It is extensive."),
    ];
}

fn An_Extraction(concept: &str, claim: &str) -> Extraction
{
    return Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));
}

#[test]
fn Test_Of_Should_Keep_What_A_Reading_Says_Together()
{
    // The constructor's whole job, stated as one thing rather than four: a reading is what it was
    // handed, and none of the four parts is dropped or substituted for another. Nothing here is a
    // KWB entity and nothing here has an identity — an extractor that minted one would be a
    // second authority for what `D-002` owns — so a reading is only ever a carrier.
    let reading = ProposedReading::Of(The_Source(), A_Location(), Statements(), A_Lineage());

    assert_eq!(reading.Source(), The_Source());
    assert_eq!(reading.Location().Description(), "chapter two");
    assert_eq!(reading.Proposed().len(), Statements().len());
    assert_eq!(reading.Lineage().Protocol(), "read-the-text-v1");
    assert_eq!(reading.Lineage().Reader(), "a text extractor");
}

#[test]
fn Test_Source_Should_Be_The_Address_Of_The_Bytes_That_Were_Read()
{
    // An address and not a path. A renamed or moved file is the same source and an edited one is
    // not, which a filename cannot express — and admission refuses a reading about a document
    // other than the one it wrote, so this is the field that refusal compares.
    let here = ProposedReading::Of(The_Source(), A_Location(), Statements(), A_Lineage());
    let elsewhere = ProposedReading::Of(Another_Source(), A_Location(), Statements(), A_Lineage());

    assert_eq!(here.Source(), The_Source());
    assert_ne!(
        here.Source(),
        elsewhere.Source(),
        "two documents one address, so a reading about one would be cited as the other"
    );
}

#[test]
fn Test_Location_Should_Say_Where_In_That_Source_The_Passage_Was()
{
    // Deliberately opaque text rather than a page, an offset or a span: what locates a passage
    // differs by medium, and a structured location invented for one medium becomes the structure
    // every later medium has to pretend to fit. A reading carries **one** location, which is why
    // a reader that splits a source returns one reading per passage rather than merging them.
    let reading = ProposedReading::Of(
        The_Source(),
        SourceLocation::Named("p. 12, chapter two"),
        Statements(),
        A_Lineage(),
    );

    assert_eq!(reading.Location().Description(), "p. 12, chapter two");
    assert_ne!(
        reading.Location().Description(),
        A_Location().Description(),
        "the location is a constant rather than what the reading was given"
    );
}

#[test]
fn Test_Proposed_Should_Hand_Back_The_Statements_And_Nothing_Else()
{
    // Proposals, not claims: what comes back is text with no identity, in the order it arrived,
    // and it is the whole of what was proposed. A reading that returned a subset would be a
    // partial result shaped exactly like a complete one — the prototype's 1,367 `Barren` rows.
    let reading = ProposedReading::Of(The_Source(), A_Location(), Statements(), A_Lineage());
    let proposed: Vec<&str> = reading
        .Proposed()
        .iter()
        .map(|extraction| return extraction.concept_name.Text())
        .collect();

    assert_eq!(reading.Proposed().len(), Statements().len());
    assert_eq!(proposed.len(), Statements().len());
    assert_eq!(reading.Proposed(), Statements().as_slice());
}

#[test]
fn Test_Lineage_Should_Carry_The_Protocol_And_The_Reader()
{
    // Carried here and dropped at the door, which is a stated gap rather than an oversight: an
    // `Assertion` holds a source, a claim and a scope, so nothing about the *reading* survives
    // into the graph. It is carried at all so that a caller reporting which protocol produced a
    // corpus can ask, and so that a field nothing consumes is recorded as such rather than
    // reading like a field something does.
    // Named before it is passed, because one argument of four that is itself a call is four more
    // positions to hold at once; named, the reading below is the four things it is made of.
    let a_new_prompt = ExtractionLineage::Of(
        ReadingProtocol::Named("read-the-text-v2"),
        ReaderName::Named("a model"),
    );
    let under_a_new_prompt =
        ProposedReading::Of(The_Source(), A_Location(), Statements(), a_new_prompt);
    let reading = ProposedReading::Of(The_Source(), A_Location(), Statements(), A_Lineage());

    assert_eq!(reading.Lineage().Protocol(), "read-the-text-v1");
    assert_eq!(reading.Lineage().Reader(), "a text extractor");
    assert_ne!(
        reading.Lineage(),
        under_a_new_prompt.Lineage(),
        "two readings under one lineage, so a corpus produced under a new prompt reports the \
         apparatus that produced the old one"
    );
}
