//! Completeness is judged over canonical text before concepts or claims are admitted.

use kwb_domain::Scope;
use kwb_ingest::{Admit_Source, ClaimText, ConceptName, Extraction, ExtractionLineage, ReaderName, ReadingKind, ReadingProtocol, SourceLocation, Stated};
use kwb_store::DocumentStore;

const EMPTY_AFTER_NORMALIZING: [&str; 6] = ["\u{1}", "\u{1f}", "\u{0}\u{7}", "\u{1} \u{7}", "\t\u{1}\n\u{7}\r ", "\u{85}\u{7}\u{2028}"];

fn Extraction_Of(concept: &str, claim: &str) -> Extraction
{
    return Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));
}

#[test]
fn Test_Is_Complete_Should_Refuse_Either_Half_That_Normalizes_To_Nothing()
{
    for empty in EMPTY_AFTER_NORMALIZING
    {
        assert!(!Extraction_Of(empty, "A real claim.").Is_Complete(), "empty concept accepted: {empty:?}");
        assert!(!Extraction_Of("entropy", empty).Is_Complete(), "empty claim accepted: {empty:?}");
    }
    for text in ["a\u{1}", "\u{1}a"]
    {
        assert!(Extraction_Of(text, text).Is_Complete(), "nonempty canonical text refused");
    }
}

#[test]
fn Test_Admission_Should_Count_Every_Incomplete_Extraction_And_Publish_None_Of_Them()
{
    let extractions: Vec<Extraction> = EMPTY_AFTER_NORMALIZING.iter().flat_map(|empty| return [Extraction_Of(empty, "A real claim."), Extraction_Of("entropy", empty)]).collect();
    let offered = extractions.len();
    let lineage = ExtractionLineage::Of(ReadingProtocol::Named("control-only-fixture"), ReaderName::Named("a test"));
    let reader = Stated::Of(extractions, SourceLocation::Named("throughout"), lineage, Scope::Unstated()).expect("a nonempty list of proposals is a reading");
    let mut store = DocumentStore::Empty();
    let report = Admit_Source(b"A source with real bytes.".to_vec(), Some(&reader), ReadingKind::Text, &mut store).expect("source admission should complete");
    assert_eq!(report.Normalized().Linked().Refused(), offered, "incomplete extractions were silently lost or admitted");
    assert!(report.Normalized().Concepts().is_empty());
    assert_eq!(report.Normalized().Claims_Held(), 0);
    assert!(report.Assertions().is_empty());
    assert!(report.Publications().is_empty(), "an incomplete extraction became canonical knowledge");
}
