//! The properties a content store must have for an address to mean what it says.

use std::fs;
use std::path::PathBuf;

use kwb_platform::{ContentStoreStrategy, StorageError};
use kwb_platform_std::DirectoryContentStore;

/// A directory nothing else is using, named for the test that asked for it.
fn Scratch(name: &str) -> PathBuf
{
    let root = std::env::temp_dir().join(format!("kwb-store-test-{name}"));
    if let Err(error) = fs::remove_dir_all(&root)
    {
        assert_eq!(
            error.kind(),
            std::io::ErrorKind::NotFound,
            "the scratch directory could not be cleared: {error}"
        );
    }
    return root;
}

#[test]
fn Test_Stored_Bytes_Should_Read_Back_Unchanged()
{
    let root = Scratch("round-trip");
    let store = DirectoryContentStore::Under(&root).expect("Under creates the directory it is given");
    let content = b"fn main() {}\n\n  indented\t\x00\xFF".to_vec();

    store.Put("abcd", &content).expect("the root exists, so the store can stage the bytes");

    assert_eq!(store.Get("abcd").expect("the bytes were stored at this address above"), content);
    assert!(store.Holds("abcd").expect("Under created the root, so the medium can be asked"));
}

#[test]
fn Test_An_Absent_Address_Should_Be_Refused_Rather_Than_Empty()
{
    let root = Scratch("absent");
    let store = DirectoryContentStore::Under(&root).expect("Under creates the directory it is given");

    let refusal = store.Get("nothing").expect_err("must refuse");

    assert_eq!(refusal, StorageError::Absent { address: "nothing".to_owned() });
    assert!(!store.Holds("nothing").expect("Under created the root, so the medium can be asked"));
}

#[test]
fn Test_Storing_One_Address_Twice_Should_Be_Idempotent()
{
    let root = Scratch("idempotent");
    let store = DirectoryContentStore::Under(&root).expect("Under creates the directory it is given");

    store.Put("abcd", b"one").expect("the root exists, so the store can stage the bytes");
    store.Put("abcd", b"one").expect("a repeat write of one address is refused nothing");

    assert_eq!(store.Get("abcd").expect("the bytes were stored at this address above"), b"one".to_vec());
    assert_eq!(fs::read_dir(&root).expect("Under created the root, so it can be listed").count(), 1);
}

#[test]
fn Test_A_Completed_Write_Should_Leave_No_Staged_File_Behind()
{
    let root = Scratch("no-staging-left");
    let store = DirectoryContentStore::Under(&root).expect("Under creates the directory it is given");

    store.Put("abcd", b"one").expect("the root exists, so the store can stage the bytes");

    let names: Vec<String> = fs::read_dir(&root)
        .expect("Under created the root, so it can be listed")
        .map(|entry| return entry.expect("read_dir yields only entries it could read").file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, ["abcd"], "a staged file survived a successful write: {names:?}");
}

/// The atomicity design, demonstrated rather than asserted.
///
/// A race cannot be observed directly, so this observes the *mechanism*: the write is staged
/// under a different name and renamed into place. Making the rename fail — by putting a
/// directory where the file should go — leaves the staged file, which proves the bytes were
/// never written to the destination path directly.
///
/// It also checks the failure rule `D17` asks for: the staged file is **kept**, not cleaned up.
/// A leftover staged file is the only visible evidence that a write did not complete, and
/// deleting it would destroy the sign that anything went wrong.
#[test]
fn Test_A_Write_Should_Be_Staged_And_Renamed_Rather_Than_Written_In_Place()
{
    let root = Scratch("staged");
    let store = DirectoryContentStore::Under(&root).expect("Under creates the directory it is given");
    // A directory at the destination address: the rename cannot complete.
    fs::create_dir_all(root.join("abcd")).expect("creates the obstruction");

    let refusal = store.Put("abcd", b"one").expect_err("the rename cannot succeed");

    assert!(
        matches!(refusal, StorageError::Refused { .. }),
        "a write that could not complete must be refused, not reported: {refusal}"
    );
    assert!(
        root.join("abcd.staged").is_file(),
        "the bytes were written somewhere other than the staging path, so the write was not \
         staged and a reader could have seen a partial file under a complete address"
    );
    assert!(
        root.join("abcd").is_dir(),
        "the destination was written to directly despite the rename failing"
    );
}
