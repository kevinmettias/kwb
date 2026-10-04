//! Admission coverage counts returned passage readings, never imagined source work.

use kwb_domain::{Coverage, Scope};
use kwb_ingest::{Admit_Source, AdmissionReport, ClaimText, ConceptName, Extraction, ExtractionError, ExtractionLineage, ExtractionStrategy, ProposedReading, ReaderName, ReadingKind, ReadingProtocol, SourceLocation, Stated};
use kwb_model::ContentIdentity;
use kwb_store::DocumentStore;

const SOURCE: &[u8] = b"A source containing several passages.";

struct Scripted(Vec<Vec<Extraction>>);

fn Lineage() -> ExtractionLineage
{
    return ExtractionLineage::Of(ReadingProtocol::Named("offline-coverage-fixture"), ReaderName::Named("fixture reader"));
}

impl ExtractionStrategy for Scripted
{
    fn Read(&self, source: ContentIdentity, _content: &[u8], _needed: ReadingKind) -> Result<Vec<ProposedReading>, ExtractionError>
    {
        return Ok(self.0.iter().enumerate().map(|(index, proposals)| {
            return ProposedReading::Of(source, SourceLocation::Named(&format!("passage {index}")), proposals.clone(), Lineage());
        }).collect());
    }

    fn Scope(&self) -> Scope
    {
        return Scope::Unstated();
    }
}

fn Admit(reader: &dyn ExtractionStrategy) -> AdmissionReport
{
    return Admit_Source(SOURCE.to_vec(), Some(reader), ReadingKind::Text, &mut DocumentStore::Empty()).expect("the source can be admitted");
}

fn Offered(concept: &str, claim: &str) -> Extraction
{
    return Extraction::New(ConceptName::Named(concept), ClaimText::Stated(claim));
}

#[test]
fn Test_No_Readings_Should_Be_Unmet_Without_Claiming_Any_Material_Was_Examined()
{
    let report = Admit(&Scripted(Vec::new()));
    assert_eq!(report.Coverage(), Coverage::Unmet { prerequisite: "the reader returned no reading" });
    assert!(!report.Coverage().Is_Evidence_Of_Absence());
    assert!(report.Publications().is_empty());
    assert!(report.Source().is_some(), "the admitted source must remain retained");
}

#[test]
fn Test_Three_Readings_With_No_Admissible_Claims_Should_Report_Three_Examined_Passages()
{
    let reader = Scripted(vec![Vec::new(), vec![Offered("", "refused"), Offered("", "also refused")], Vec::new()]);
    let report = Admit(&reader);
    let Coverage::Barren { examined } = report.Coverage()
    else
    {
        panic!("expected evidence of absence, got {:?}", report.Coverage());
    };
    assert_eq!(examined.get(), 3, "neither the source count nor the two offered propositions is the examined passage count");
    assert!(report.Coverage().Is_Evidence_Of_Absence());
    assert!(report.Publications().is_empty());
}

#[test]
fn Test_A_Yield_Should_Count_Admitted_Claims_Separately_From_Reading_Count()
{
    let reader = Scripted(vec![vec![Offered("entropy", "A state function."), Offered("enthalpy", "A thermodynamic potential.")],
        vec![Offered("temperature", "An intensive property.")]]);
    let report = Admit(&reader);
    let Coverage::Yielded { findings } = report.Coverage()
    else
    {
        panic!("expected yield, got {:?}", report.Coverage());
    };
    assert_eq!(findings.get(), 3, "two readings can yield three distinct claims");
    assert_eq!(report.Assertions().len(), 3);
    assert!(!report.Coverage().Is_Evidence_Of_Absence());
}

#[test]
fn Test_A_Persons_Blank_Statement_Should_Remain_One_Examined_Reading_With_No_Claim()
{
    let reader = Stated::Of(vec![Offered("entropy", " ")], SourceLocation::Named("unspecified"), Lineage(), Scope::Unstated())
        .expect("one statement is offered even though admission will refuse its blank claim");
    let report = Admit(&reader);
    let Coverage::Barren { examined } = report.Coverage()
    else
    {
        panic!("expected barren, got {:?}", report.Coverage());
    };
    assert_eq!(examined.get(), 1);
    assert!(report.Assertions().is_empty());
    assert_eq!(report.Normalized().Linked().Refused(), 1);
}
