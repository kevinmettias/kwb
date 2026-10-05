//! Admissions retain a closed concept's standing while keeping every new claim and citation.

use kwb_domain::{Concept, KnowledgeGraph, Publication, Replay_Records, Scope};
use kwb_ingest::{Admit_Source, ClaimText, ConceptName, Extraction, ExtractionLineage, ReaderName,
    ReadingKind, ReadingProtocol, SourceLocation, Stated};
use kwb_mcp::{Answer_Tool_Call, ToolName};
use kwb_store::{Document, DocumentStore};
use std::path::PathBuf;
use std::process::{Command, Output};

const KWB: &str = env!("CARGO_BIN_EXE_kwb");
const NEW_CLAIM: &str = "The later source still describes phlogiston.";
const NEW_SOURCE: &[u8] = b"A later historical source about the closed explanation.";
const REASON: &str = "The recorded evidence favours oxidation.";

struct Fixture
{
    root: PathBuf,
    store: String,
}

impl Fixture
{
    fn Of(name: &str) -> Self
    {
        let root = std::env::temp_dir().join(format!("kwb-closed-admission-{}-{name}", std::process::id()));
        std::fs::create_dir(&root).expect("an unused temporary directory");
        let fixture = Self { store: root.join("store").display().to_string(), root };
        fixture.Admit("oxidation", "Oxidation explains combustion.", b"The first source describes oxidation.");
        fixture.Admit("phlogiston", "Phlogiston was an earlier explanation.", b"The second source describes phlogiston.");
        return fixture;
    }

    fn Admit(&self, concept: &str, claim: &str, bytes: &[u8]) -> String
    {
        let source = self.root.join("source.txt");
        std::fs::write(&source, bytes).expect("a writable source");
        let output = Ran(&["admit", &source.display().to_string(), "--store", &self.store, "--says", concept, claim]);
        Succeeded(&output);
        return String::from_utf8(output.stdout).expect("a UTF-8 report");
    }

    fn Close(&self, verb: &str)
    {
        let mut args = vec![verb, "phlogiston"];
        if verb == "supersede"
        {
            args.extend_from_slice(&["--into", "oxidation"]);
        }
        args.extend_from_slice(&["--store", &self.store, "--because", REASON]);
        Succeeded(&Ran(&args));
    }

    fn Records(&self) -> Vec<String>
    {
        return std::fs::read_to_string(self.root.join("store/publications.log")).expect("the recorded log")
            .lines().map(str::to_owned).collect();
    }

    fn Graph(&self) -> KnowledgeGraph
    {
        return Replay_Records(&self.Records()).expect("the durable graph must replay");
    }

    fn Answer(&self, tool: &str, name: &str) -> Vec<String>
    {
        return Answer_Tool_Call(&self.Graph(), ToolName::Named(tool), name).expect("the actual agent query surface");
    }
}

impl Drop for Fixture
{
    fn drop(&mut self)
    {
        std::fs::remove_dir_all(&self.root).expect("a removable temporary fixture");
    }
}

fn Ran(arguments: &[&str]) -> Output
{
    return Command::new(KWB).args(arguments).output().expect("the built CLI should run");
}

fn Succeeded(output: &Output)
{
    assert!(output.status.success(), "command failed: {}", String::from_utf8_lossy(&output.stderr));
}

fn Count_Of(output: &str, field: &str) -> usize
{
    return output.lines().find_map(|line| return line.strip_prefix(field)).expect("the report must print this field")
        .trim().parse().expect("a numeric report field");
}

fn Assert_Closed_Admission(verb: &str)
{
    let fixture = Fixture::Of(verb);
    fixture.Close(verb);
    let before = fixture.Graph();
    let old_records = fixture.Records();
    let output = fixture.Admit("phlogiston", NEW_CLAIM, NEW_SOURCE);
    let after = fixture.Graph();
    let reader = Stated_Claims(&[NEW_CLAIM]);
    let report = Admit_Source(NEW_SOURCE.to_vec(), Some(&reader), ReadingKind::Text, &mut DocumentStore::Empty())
        .expect("the same source and reading should be admitted through the library");
    assert_eq!(report.Closed_Concepts_In(&before), 1);
    let publications = report.Publications_Into(&before);
    assert_eq!(publications.len(), 2);
    assert!(publications.iter().all(|publication| return !matches!(publication, Publication::Concept { .. })));
    let predicted = report.Published_Into(&before);
    assert_eq!(predicted.Every_Version().Concepts(), after.Every_Version().Concepts(), "direct graph publication must equal the recorded CLI replay");
    assert_eq!(predicted.Every_Version().Claims(), after.Every_Version().Claims());
    assert_eq!(predicted.Every_Version().Assertions(), after.Every_Version().Assertions());
    assert_eq!(Count_Of(&output, "closed"), 1, "the admission must report the one closed concept it named");
    let records = fixture.Records();
    assert_eq!(records.get(..old_records.len()).expect("the original log prefix"), old_records, "the prior log must stay unchanged");
    assert_eq!(records.len(), old_records.len().checked_add(2).expect("two more publication records"), "append exactly the new claim and its assertion");
    assert!(records.get(old_records.len()..).expect("the appended records").iter().all(|record| return !record.starts_with("concept\u{1f}")), "an admission re-published a closed concept");
    assert_eq!(before.Every_Version().Concepts(), after.Every_Version().Concepts(), "the concept standings and closure evidence must stay identical");
    assert_eq!(after.Every_Version().Claims().len(), 3);
    assert_eq!(after.Every_Version().Assertions().len(), 3);
    let quoted = fixture.Answer("held_neighbours", "phlogiston").join("\n");
    assert!(quoted.contains(NEW_CLAIM), "the new claim about a closed concept was lost: {quoted}");
    assert!(quoted.contains(&Document::Of(NEW_SOURCE.to_vec()).Identity().Render()), "the new claim must retain its exact source citation: {quoted}");
    assert!(quoted.contains(REASON), "the earlier reason was lost: {quoted}");
    assert!(!fixture.Answer("held_neighbours", "oxidation").join("\n").contains(NEW_CLAIM), "an admission moved a claim to the successor");
    if verb == "supersede"
    {
        assert_eq!(fixture.Answer("merge_losers", ""), vec![format!("phlogiston -> {} ({REASON})", Concept::Named("oxidation").Identity().Render())]);
    }
    let history = Ran(&["history", "--store", &fixture.store]);
    Succeeded(&history);
    let history = String::from_utf8(history.stdout).expect("a UTF-8 history report");
    for (field, count) in [("concepts", after.Current().Concepts().len()), ("claims", after.Current().Claims().len()), ("citations", after.Current().Assertions().len())]
    {
        assert_eq!(Count_Of(&history, field), count, "a fresh CLI replay must agree with the graph");
        assert_eq!(Count_Of(&output, field), count, "the admission report must agree with the replayed log");
    }
}

#[test]
fn Test_An_Admission_Should_Keep_A_Superseded_Concept_Closed_And_Cite_Its_New_Claim()
{
    Assert_Closed_Admission("supersede");
}

#[test]
fn Test_An_Admission_Should_Keep_A_Retired_Concept_Closed_And_Cite_Its_New_Claim()
{
    Assert_Closed_Admission("retire");
}

#[test]
fn Test_An_Admission_Naming_Only_Current_Concepts_Should_Report_Zero_Closed()
{
    let fixture = Fixture::Of("current");
    let output = fixture.Admit("phlogiston", NEW_CLAIM, NEW_SOURCE);
    assert_eq!(Count_Of(&output, "closed"), 0);
    assert_eq!(Count_Of(&output, "concepts"), 2);
    assert_eq!(Count_Of(&output, "claims"), 3);
    assert_eq!(Count_Of(&output, "citations"), 3);
}
fn Stated_Claims(claims: &[&str]) -> Stated
{
    let extractions = claims.iter().map(|claim| return Extraction::New(ConceptName::Named("phlogiston"), ClaimText::Stated(claim))).collect();
    return Stated::Of(extractions, SourceLocation::Named("as stated on the command line"),
        ExtractionLineage::Of(ReadingProtocol::Named("stated-by-a-person"), ReaderName::Named("the operator of kwb admit")),
        Scope::Unstated()).expect("a nonempty stated reading");
}

#[test]
fn Test_Two_Different_Claims_Should_Count_One_Closed_Concept_And_Keep_Both()
{
    let fixture = Fixture::Of("two-claims");
    fixture.Close("retire");
    let before = fixture.Graph();
    let reader = Stated_Claims(&[NEW_CLAIM, "Another observation about that explanation."]);
    let report = Admit_Source(NEW_SOURCE.to_vec(), Some(&reader), ReadingKind::Text, &mut DocumentStore::Empty())
        .expect("a readable source with two claims");
    assert_eq!(report.Closed_Concepts_In(&before), 1, "count concepts, not proposals or claims");
    assert_eq!(report.Publications_Into(&before).len(), 4, "both claims and their source assertions survive");
    let after = report.Published_Into(&before);
    assert_eq!(after.Every_Version().Concepts(), before.Every_Version().Concepts());
    assert_eq!(after.Every_Version().Claims().len(), before.Every_Version().Claims().len().checked_add(2).expect("two new claims"));
    assert_eq!(after.Every_Version().Assertions().len(), before.Every_Version().Assertions().len().checked_add(2).expect("two new assertions"));
}