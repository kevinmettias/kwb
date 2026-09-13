//! A content store over a directory, one file per address.

use std::fs;
use std::path::{Path, PathBuf};

use kwb_platform::{ContentStoreStrategy, StorageError};

/// A content-addressed store backed by a directory.
///
/// One file per address, named by the address. `D-014`: the address is already a filename, so
/// there is no index — an index would be a second thing to keep in step with the first.
pub struct DirectoryContentStore
{
    root: PathBuf,
}

impl DirectoryContentStore
{
    /// A store under this directory, creating it if it is not there.
    ///
    /// # Errors
    ///
    /// [`StorageError::Refused`] if the directory could not be created.
    pub fn Under(root: impl Into<PathBuf>) -> Result<Self, StorageError>
    {
        let root = root.into();
        fs::create_dir_all(&root).map_err(|cause| {
            return StorageError::Refused {
                doing: "creating the store directory",
                cause: cause.to_string(),
            };
        })?;
        return Ok(Self { root });
    }

    /// Where an address lands.
    fn Path_Of(&self, address: &str) -> PathBuf
    {
        return self.root.join(address);
    }
}

impl ContentStoreStrategy for DirectoryContentStore
{
    fn Holds(&self, address: &str) -> Result<bool, StorageError>
    {
        return Ok(self.Path_Of(address).is_file());
    }

    /// Write to a temporary name, then rename into place.
    ///
    /// **The rename is the whole design.** `D-014`: a reader must never see a partial value
    /// under a complete address, because the address is the digest and nothing downstream
    /// re-hashes what it reads — so a truncated file is a silent corruption rather than a loud
    /// failure. `fs::rename` is atomic within a filesystem on every platform this runs on, and
    /// the temporary is created in the same directory so that it always is one.
    ///
    /// A present address is left alone. The content is the address, so the bytes on disk are
    /// already the bytes being offered, and rewriting them would be a window in which a correct
    /// file is temporarily absent for no gain.
    fn Put(&self, address: &str, content: &[u8]) -> Result<(), StorageError>
    {
        let destination = self.Path_Of(address);
        if destination.is_file()
        {
            return Ok(());
        }

        let staged = self.Path_Of(&format!("{address}.staged"));
        Write_Then_Rename(&staged, &destination, content)?;
        return Ok(());
    }

    fn Get(&self, address: &str) -> Result<Vec<u8>, StorageError>
    {
        let path = self.Path_Of(address);
        if !path.is_file()
        {
            return Err(StorageError::Absent {
                address: address.to_owned(),
            });
        }

        return fs::read(&path).map_err(|cause| {
            return StorageError::Refused {
                doing: "reading a stored document",
                cause: cause.to_string(),
            };
        });
    }
}

/// Write the bytes to `staged`, then move them onto `destination` in one step.
///
/// A failed rename leaves the staged file behind rather than deleting it. That is deliberate:
/// a leftover `.staged` file is visible evidence that a write did not complete, and removing it
/// would destroy the only sign that anything went wrong — which is `D17`'s rule applied to this
/// function's own failure path.
fn Write_Then_Rename(staged: &Path, destination: &Path, content: &[u8]) -> Result<(), StorageError>
{
    fs::write(staged, content).map_err(|cause| {
        return StorageError::Refused {
            doing: "staging a document",
            cause: cause.to_string(),
        };
    })?;

    return fs::rename(staged, destination).map_err(|cause| {
        return StorageError::Refused {
            doing: "renaming a staged document into place",
            cause: cause.to_string(),
        };
    });
}
