//! An append-only log in a file, one record per line.

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use kwb_platform::{RecordLogStrategy, StorageError};

/// An append-only record log backed by one file, one record per line.
///
/// A record contains no newline, because every text a `kwb-domain` type keeps is normalized and
/// normalization strips control characters. That is what makes one-record-per-line safe without
/// escaping, and it is a guarantee this type depends on rather than one it enforces — the
/// domain's own tests are where it is defended.
pub struct FileRecordLog
{
    path: PathBuf,
}

/// What the medium reported, as the refusal this crate's callers see.
///
/// Every operation in this file fails the one way — a `std::io::Error` becomes
/// [`StorageError::Refused`] carrying what was being attempted — and writing that mapping out at
/// each site made four short operations read as five-line ones. `doing` is the whole of what
/// differs between them.
///
/// The cause is borrowed rather than taken, because it is only read: the refusal this crate
/// reports is a `String`, so the error the medium produced is left where the caller's own
/// `map_err` closure found it rather than consumed to be described.
fn Refused_Operation(doing: &'static str, cause: &std::io::Error) -> StorageError
{
    return StorageError::Refused {
        doing,
        cause: cause.to_string(),
    };
}

impl FileRecordLog
{
    /// A log at this path, creating the file and its directory if they are not there.
    ///
    /// # Errors
    ///
    /// [`StorageError::Refused`] if the file could not be created.
    pub fn At(path: impl Into<PathBuf>) -> Result<Self, StorageError>
    {
        let path = path.into();
        if let Some(parent) = path.parent()
        {
            std::fs::create_dir_all(parent)
                .map_err(|cause| return Refused_Operation("creating the log directory", &cause))?;
        }

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|cause| return Refused_Operation("opening the record log", &cause))?;

        return Ok(Self { path });
    }
}

impl RecordLogStrategy for FileRecordLog
{
    /// Append one line, and flush it before returning.
    ///
    /// Flushed rather than left to the operating system's convenience, because a caller told a
    /// record was appended has been told the work is done -- `D19`'s rule that a report must not
    /// outrun the work, applied to the one place in this crate where it easily could.
    fn Append(&self, record: &str) -> Result<(), StorageError>
    {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|cause| return Refused_Operation("opening the record log to append", &cause))?;

        writeln!(file, "{record}")
            .map_err(|cause| return Refused_Operation("appending a record", &cause))?;

        return file
            .flush()
            .map_err(|cause| return Refused_Operation("flushing an appended record", &cause));
    }

    fn Records(&self) -> Result<Vec<String>, StorageError>
    {
        let file = File::open(&self.path)
            .map_err(|cause| return Refused_Operation("opening the record log to read", &cause))?;

        let mut records = Vec::new();
        for line in BufReader::new(file).lines()
        {
            let line = line.map_err(|cause| return Refused_Operation("reading a record", &cause))?;
            if !line.is_empty()
            {
                records.push(line);
            }
        }

        return Ok(records);
    }
}
