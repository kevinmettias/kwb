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
            std::fs::create_dir_all(parent).map_err(|cause| {
                return StorageError::Refused {
                    doing: "creating the log directory",
                    cause: cause.to_string(),
                };
            })?;
        }

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|cause| {
                return StorageError::Refused {
                    doing: "opening the record log",
                    cause: cause.to_string(),
                };
            })?;

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
            .map_err(|cause| {
                return StorageError::Refused {
                    doing: "opening the record log to append",
                    cause: cause.to_string(),
                };
            })?;

        writeln!(file, "{record}").map_err(|cause| {
            return StorageError::Refused {
                doing: "appending a record",
                cause: cause.to_string(),
            };
        })?;

        return file.flush().map_err(|cause| {
            return StorageError::Refused {
                doing: "flushing an appended record",
                cause: cause.to_string(),
            };
        });
    }

    fn Records(&self) -> Result<Vec<String>, StorageError>
    {
        let file = File::open(&self.path).map_err(|cause| {
            return StorageError::Refused {
                doing: "opening the record log to read",
                cause: cause.to_string(),
            };
        })?;

        let mut records = Vec::new();
        for line in BufReader::new(file).lines()
        {
            let line = line.map_err(|cause| {
                return StorageError::Refused {
                    doing: "reading a record",
                    cause: cause.to_string(),
                };
            })?;
            if !line.is_empty()
            {
                records.push(line);
            }
        }

        return Ok(records);
    }
}
