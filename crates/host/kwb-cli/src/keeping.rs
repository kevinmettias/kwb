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

use kwb_domain::{KnowledgeGraph, Replay};
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
    return Replay(&records)
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
