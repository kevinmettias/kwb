//! What one run keeps: the store the bytes go to, the log the publications go to, and the graph
//! that log replays into.
//!
//! # Why these are one file and not three
//!
//! `D-014` decides both halves at once, and the reason is that either half alone is the same
//! mistake. A run that kept bytes without the log would have a document nothing points at; a run
//! that kept the log without replaying it would report counts that exclude what earlier runs
//! learned. Both are half of one arrangement, so they are chosen together and named together.
//!
//! # Why a store and not a serializer
//!
//! The names here are the library's — `DocumentStore`, `FileRecordLog` — and the only thing this
//! file adds is which implementation of each the binary picked. That is what a composition root
//! is for: `kwb-store` names `ContentStoreStrategy` and never an implementation of it, so
//! somebody has to choose one, and the choice belongs in one visible place rather than spread
//! through the code that writes.

use kwb_domain::{KnowledgeGraph, Replay_Records};
use kwb_ingest::AdmissionReport;
use kwb_platform::RecordLogStrategy;
use kwb_platform_std::{DirectoryContentStore, FileRecordLog};
use kwb_platform_xvpe::{PublicationClock, SystemClock};
use kwb_store::DocumentStore;

/// Everything one admission run carries: where its bytes go, and what the log already holds.
pub(crate) struct Kept
{
    /// Where admitted bytes are written.
    pub(crate) store: DocumentStore,

    /// The log beside the store, and the graph it folds into.
    pub(crate) recorded: Recorded,
}

/// The store a run writes through, and the log it records into.
///
/// # Errors
///
/// A root that cannot be used as a store, or a log that cannot be opened, read, or replayed.
pub(crate) fn Kept_At(root: Option<&str>) -> Result<Kept, String>
{
    let store = Store_For(root)?;
    let recorded = Recorded_At(root)?;

    return Ok(Kept { store, recorded });
}

/// The publication log a run records into, and the graph every earlier run left in it.
pub(crate) struct Recorded
{
    /// Where publications are recorded, when the run was told where.
    pub(crate) log: Option<FileRecordLog>,

    /// What earlier runs published, replayed.
    pub(crate) known: KnowledgeGraph,
}

/// The log at `root`, replayed.
///
/// # Errors
///
/// A log that cannot be opened, read, or replayed.
pub(crate) fn Recorded_At(root: Option<&str>) -> Result<Recorded, String>
{
    let log = Log_For(root)?;
    let known = Known_So_Far(log.as_ref())?;

    return Ok(Recorded { log, known });
}

/// The store a run writes through: durable when told where, in memory when not.
///
/// This function is the entire reason this crate exists. `kwb-store` names
/// `ContentStoreStrategy` and never an implementation of it, so somebody has to choose one, and
/// a composition root is where that choice is visible in one place rather than spread through
/// the code that writes.
///
/// # Errors
///
/// A root the medium would not accept as a store.
pub(crate) fn Store_For(root: Option<&str>) -> Result<DocumentStore, String>
{
    let Some(root) = root
    else
    {
        return Ok(DocumentStore::Empty());
    };

    let durable = DirectoryContentStore::Under(root)
        .map_err(|cause| return format!("cannot use {root} as a store: {cause}"))?;
    return Ok(DocumentStore::Backed_By(Box::new(durable)));
}

/// The publication log a run records into, when it was told where.
///
/// # Errors
///
/// A log the medium would not open.
pub(crate) fn Log_For(root: Option<&str>) -> Result<Option<FileRecordLog>, String>
{
    let Some(root) = root
    else
    {
        return Ok(None);
    };

    let log = FileRecordLog::At(std::path::Path::new(root).join("publications.log"))
        .map_err(|cause| return format!("cannot open the publication log: {cause}"))?;
    return Ok(Some(log));
}

/// The lines of a publication log.
///
/// # Errors
///
/// A log the medium would not hand back.
pub(crate) fn Records_Of(log: &FileRecordLog) -> Result<Vec<String>, String>
{
    return log
        .Records()
        .map_err(|cause| return format!("cannot read the publication log: {cause}"));
}

/// The graph a run starts from: what earlier runs published, replayed.
///
/// `D-014`: a graph is a fold over its publications, so replaying the log *is* loading the
/// graph. There is no second representation to keep in step with the log, which is the reason
/// transitions were recorded rather than versions.
///
/// # Errors
///
/// A log that cannot be read or replayed.
pub(crate) fn Known_So_Far(log: Option<&FileRecordLog>) -> Result<KnowledgeGraph, String>
{
    let Some(log) = log
    else
    {
        return Ok(KnowledgeGraph::Empty());
    };

    let records = Records_Of(log)?;
    return Replay_Records(&records)
        .map_err(|cause| return format!("the publication log cannot be replayed: {cause}"));
}

/// Record what this admission published, before reporting that it did.
///
/// Before, deliberately. `D19`: a report must not outrun the work, and a run that printed its
/// counts and then failed to record them would have told a reader about knowledge the next run
/// will not have.
///
/// # Errors
///
/// A publication the medium would not take.
pub(crate) fn Record_Into(
    log: Option<&FileRecordLog>,
    report: &AdmissionReport,
    at: Option<i64>,
) -> Result<(), String>
{
    let Some(log) = log
    else
    {
        return Ok(());
    };

    for publication in report.Publications()
    {
        log.Append(&publication.Record(at))
            .map_err(|cause| return format!("cannot record a publication: {cause}"))?;
    }

    return Ok(());
}

/// The wall time, from the clock this composition root chose.
///
/// # Why the clock is named here and nowhere else
///
/// `kwb-platform-xvpe` adopts the port and an implementation of it; this file picks which. That
/// is what a composition root is for, and it is why `kwb-domain` takes a number rather than a
/// clock — a library that knew where the time came from would be a library with an opinion
/// about the host.
pub(crate) fn Now() -> i64
{
    return SystemClock.Now().Unix_Seconds();
}

/// What each of these chooses, asserted where the choice is visible rather than where it is used.
///
/// This file is the composition root's smallest decision — which implementation of a port the
/// binary picked — and the decision is only observable from inside the process: `Is_Durable` is a
/// value, not something a run prints. `tests/history.rs` proves the same arrangement end to end by
/// finding the log again afterwards; these say which half was chosen for which root, including the
/// root that is not there.
///
/// The two that reach a medium use a directory named for the test that asked for it, removed first
/// so that what is read back belongs to this run and not to the last one.
#[cfg(test)]
mod tests
{
    use super::*;

    use kwb_domain::Scope;
    use kwb_ingest::{
        Admit_Source, ClaimText, ConceptName, Extraction, ExtractionLineage, ExtractionStrategy,
        ReaderName, ReadingKind, ReadingProtocol, SourceLocation, Stated,
    };

    use std::path::PathBuf;

    /// A directory nobody else is using, named for the test that asked for it.
    fn Temporary(test: &str) -> PathBuf
    {
        let root = std::env::temp_dir().join(format!("kwb-cli-keeping-{test}"));
        if root.exists()
        {
            std::fs::remove_dir_all(&root).expect("a removable temporary directory");
        }
        return root;
    }

    /// The report one admitted passage produces, from the pipeline rather than from a fixture.
    ///
    /// Built through `Admit_Source` because that is the only door an `AdmissionReport` comes out
    /// of — there is no public constructor — and because the publications `Record_Into` is about to
    /// count have to be the ones the pipeline really produces for there to be anything to assert.
    fn Admission(root: &str) -> AdmissionReport
    {
        let statements = vec![Extraction::New(
            ConceptName::Named("entropy"),
            ClaimText::Stated("It does not fall."),
        )];
        let said = Stated::Of(
            statements,
            SourceLocation::Named("as stated on the command line"),
            ExtractionLineage::Of(
                ReadingProtocol::Named("stated-by-a-person"),
                ReaderName::Named("the operator of kwb admit"),
            ),
            Scope::Unstated(),
        );
        let reader = said.as_ref().map(|said| return said as &dyn ExtractionStrategy);
        let mut store = Store_For(Some(root)).expect("a writable temporary store");

        return Admit_Source(b"a passage".to_vec(), reader, ReadingKind::Text, &mut store)
            .expect("a source of one passage");
    }

    #[test]
    fn Test_Store_For_Should_Answer_An_In_Memory_Store_When_It_Was_Told_Nowhere()
    {
        // `D-014` makes durability a consequence of `--store` and of nothing else. A run given no
        // root that got a durable store anyway would be a run whose bytes landed somewhere the
        // person never named, and the report would say `kept` about a place they cannot find.
        let store = Store_For(None).expect("an in-memory store needs no medium to be made");

        assert!(!store.Is_Durable(), "a run told nowhere kept its bytes on disk");
    }

    #[test]
    fn Test_Store_For_Should_Answer_A_Durable_Store_Under_The_Root_It_Was_Given()
    {
        // The function's whole reason for existing, and the one line of this file a reader looks
        // for: `kwb-store` names `ContentStoreStrategy` and never an implementation of it, so the
        // implementation is named here — and it has to be the durable one when a root was named,
        // or `--store` keeps nothing and the run reports that it did.
        let root = Temporary("store-for");
        let named = root.display().to_string();

        let store = Store_For(Some(named.as_str())).expect("a writable temporary root");

        assert!(store.Is_Durable(), "--store named a root and the run kept nothing on disk");
    }

    #[test]
    fn Test_Log_For_Should_Answer_No_Log_When_It_Was_Told_Nowhere()
    {
        // The other half of the same decision. No log is not a failure: `Record_Into` and
        // `Known_So_Far` both read it as nothing to do, which is what lets a run without `--store`
        // admit a file at all rather than refusing to start.
        assert_eq!(
            Log_For(None).map(|log| return log.is_some()),
            Ok(false),
            "a run told nowhere opened a log"
        );
    }

    #[test]
    fn Test_Log_For_Should_Open_The_Log_Beside_The_Root_It_Was_Given()
    {
        // Beside the root and not where the caller said, because `--store` names one place and
        // both halves live in it. That is what lets `history` find the log from the same flag the
        // admission wrote it under, which is a property no signature states.
        let root = Temporary("log-for");
        let named = root.display().to_string();

        let log = Log_For(Some(named.as_str())).expect("a writable temporary root");

        assert!(log.is_some(), "--store named a root and the run opened no log");
        assert!(
            root.join("publications.log").is_file(),
            "the log was opened somewhere other than the place the store also keeps its bytes"
        );
    }

    #[test]
    fn Test_Records_Of_Should_Hand_Back_The_Lines_In_The_Order_They_Were_Appended()
    {
        // The order is the answer and not a detail of it. A prefix of the log is what `history`
        // folds, `Replay_Records` refuses a record naming something no earlier record published,
        // and a reader that sorted or reversed the lines would make every replay fail on a log
        // that is perfectly good.
        let log = FileRecordLog::At(Temporary("records-of").join("publications.log"))
            .expect("a writable temporary log");
        log.Append("first").expect("a writable temporary log");
        log.Append("second").expect("a writable temporary log");

        assert_eq!(
            Records_Of(&log).expect("a readable temporary log"),
            ["first", "second"],
            "the lines came back in an order the log was not written in"
        );
    }

    #[test]
    fn Test_Known_So_Far_Should_Answer_An_Empty_Graph_When_There_Is_No_Log()
    {
        // `D-014`: a graph is a fold over its publications, so no log is the empty fold and not a
        // missing input. The first admission in a fresh directory has to be able to run, and it is
        // why `Kept_At(None)` is a working arrangement rather than a refusal.
        let known = Known_So_Far(None).expect("no log replays to the empty graph");

        assert!(
            known.Current().Concepts().is_empty(),
            "a graph was found with no log to fold it out of"
        );
    }

    #[test]
    fn Test_Known_So_Far_Should_Replay_The_Log_It_Was_Given()
    {
        // The fold itself, on a log this repository really wrote: the concept the record published
        // is the concept the graph has to hold. A reader that opened the log and answered an empty
        // graph would satisfy the test above and fail this one, which is why both exist.
        let root = Temporary("known-so-far");
        std::fs::create_dir_all(&root).expect("a writable temporary directory");
        std::fs::write(
            root.join("publications.log"),
            "concept\u{1F}asserted\u{1F}\u{1F}\u{1F}entropy\n",
        )
        .expect("a writable temporary log");
        let log = FileRecordLog::At(root.join("publications.log")).expect("a readable temporary log");

        let known = Known_So_Far(Some(&log)).expect("a log this repository wrote replays");

        assert_eq!(
            known.Current().Concepts().len(),
            1,
            "the replay produced a graph that does not hold the concept the log published"
        );
    }

    #[test]
    fn Test_Kept_At_Should_Choose_Both_Halves_From_The_One_Root()
    {
        // `D-014` decides the store and the log together, and the file says why: either half alone
        // is the same mistake. So both halves are asserted at once, because the defect this guards
        // is a version that read the root for one of them and not the other — which each of the
        // tests above would pass on its own.
        let root = Temporary("kept-at");
        let named = root.display().to_string();

        let kept = Kept_At(Some(named.as_str())).expect("a writable temporary root");

        assert!(kept.store.Is_Durable(), "the store half ignored the root it was given");
        assert!(kept.recorded.log.is_some(), "the log half ignored the root it was given");
        assert!(
            kept.recorded.known.Current().Concepts().is_empty(),
            "a fresh store replayed a graph out of nothing"
        );
    }

    #[test]
    fn Test_Kept_At_Should_Keep_Nothing_When_It_Was_Told_Nowhere()
    {
        // The other arrangement, asserted as an arrangement rather than as a failure: a run given
        // no root is documented, reported and expected. Both halves have to be empty, because a
        // run that kept its bytes on disk while reporting `in memory only` would be the report
        // outrunning the work — `D19`, one layer down.
        let kept = Kept_At(None).expect("nowhere is an arrangement, not a failure");

        assert!(!kept.store.Is_Durable(), "a run told nowhere kept its bytes on disk");
        assert!(kept.recorded.log.is_none(), "a run told nowhere opened a log");
    }

    #[test]
    fn Test_Recorded_At_Should_Answer_The_Log_And_The_Graph_It_Folds_Into()
    {
        // One value rather than two lookups, because the graph *is* the log replayed and a caller
        // that could hold one without the other could report counts excluding what earlier runs
        // learned. The empty case is the one that says so: no root, no log, no graph, and no
        // complaint — the pairing holds in the case where each half is nothing.
        let recorded = Recorded_At(None).expect("nowhere is an arrangement, not a failure");

        assert!(recorded.log.is_none(), "a run told nowhere opened a log");
        assert!(
            recorded.known.Current().Concepts().is_empty(),
            "a graph was reported with no log to fold it out of"
        );
    }

    #[test]
    fn Test_Record_Into_Should_Append_One_Record_Per_Publication()
    {
        // Before the report, deliberately — `D19`, a report must not outrun the work — so what is
        // asserted is that the count the run is about to print is already in the log. One record
        // per publication and not one per run, because it is the transitions that are recorded
        // (`D-014`) and a single record per run would lose the graph's middle.
        let root = Temporary("record-into");
        let named = root.display().to_string();
        let report = Admission(&named);
        let log = Log_For(Some(named.as_str()))
            .expect("a writable temporary root")
            .expect("a log under a root that was named");

        Record_Into(Some(&log), &report, Some(1_700_000_000)).expect("a writable temporary log");

        assert_eq!(
            Records_Of(&log).expect("a readable temporary log").len(),
            report.Publications().len(),
            "the log does not hold one record per publication the report is about to claim"
        );
    }

    #[test]
    fn Test_Record_Into_Should_Record_Nothing_When_There_Is_No_Log()
    {
        // The half that keeps a run without `--store` working: nothing to record into is not a
        // failure, and a version that refused here would make every in-memory run exit `1` on a
        // command line the help leads with.
        let root = Temporary("record-into-nowhere");
        let named = root.display().to_string();
        let report = Admission(&named);

        assert_eq!(Record_Into(None, &report, None), Ok(()));
    }

    #[test]
    fn Test_Now_Should_Answer_A_Wall_Time_And_Not_A_Count()
    {
        // The composition root picks the clock, and what a caller can check without holding a clock
        // of its own is that what comes back is a time at all. A counter starting at zero would put
        // every publication before the epoch, and `history --as-of` would answer every question
        // with the whole log — the failure `Through_Time` refuses to paper over one layer up.
        assert!(
            Now() > 1_577_836_800,
            "the clock answered a value that is not a wall time in this century, so publications \
             are being recorded against something that is not a date"
        );
    }
}
