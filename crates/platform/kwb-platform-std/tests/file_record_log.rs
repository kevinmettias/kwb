//! Order is the meaning, so order is what this tests.

use kwb_platform::RecordLogStrategy;
use kwb_platform_std::FileRecordLog;

fn Scratch_Root(name: &str) -> std::path::PathBuf
{
    let root = std::env::temp_dir().join(format!("kwb-log-test-{name}"));
    if let Err(error) = std::fs::remove_dir_all(&root)
    {
        assert_eq!(
            error.kind(),
            std::io::ErrorKind::NotFound,
            "the scratch directory could not be cleared: {error}"
        );
    }
    return root.join("publications.log");
}

#[test]
fn Test_At_Should_Create_The_Log_File_And_The_Directory_Above_It()
{
    let path = Scratch_Root("creates-its-file");
    let above = path.parent().expect("the scratch path names a file inside a directory");
    assert!(!above.exists(), "the scratch directory survived the attempt to clear it");

    let log = FileRecordLog::At(&path).expect("At creates the log file and its directory");

    assert!(path.is_file(), "At returned a log over a file it never created");
    assert_eq!(log.Records().expect("At created the log file, so there is one to read"), Vec::<String>::new());
}

#[test]
fn Test_Records_Should_Read_Back_Oldest_First()
{
    let log = FileRecordLog::At(Scratch_Root("order")).expect("At creates the log file and its directory");

    log.Append("one").expect("the log file exists and is opened for appending");
    log.Append("two").expect("the log file exists and is opened for appending");
    log.Append("three").expect("the log file exists and is opened for appending");

    assert_eq!(log.Records().expect("the records were appended by the calls above"), ["one", "two", "three"]);
}

#[test]
fn Test_A_New_Log_Should_Be_Empty_Rather_Than_Absent()
{
    let log = FileRecordLog::At(Scratch_Root("empty")).expect("At creates the log file and its directory");

    assert_eq!(log.Records().expect("At created the log file, so there is one to read"), Vec::<String>::new());
}

#[test]
fn Test_Records_Should_Survive_The_Log_That_Wrote_Them()
{
    let path = Scratch_Root("survives");

    {
        let log = FileRecordLog::At(&path).expect("At creates the log file and its directory");
        log.Append("one").expect("the log file exists and is opened for appending");
        log.Append("two").expect("the log file exists and is opened for appending");
    }

    let reopened = FileRecordLog::At(&path).expect("the file exists from the log opened above");
    assert_eq!(
        reopened.Records().expect("the records were appended by the calls above"),
        ["one", "two"],
        "reopening a log truncated it, so the graph it records would replay short"
    );
}

#[test]
fn Test_Reopening_Should_Append_Rather_Than_Overwrite()
{
    let path = Scratch_Root("appends");
    FileRecordLog::At(&path).expect("At creates the log file and its directory").Append("one").expect("the log file exists and is opened for appending");

    FileRecordLog::At(&path).expect("the file exists from the log opened above").Append("two").expect("the log file exists and is opened for appending");

    assert_eq!(FileRecordLog::At(&path).expect("the file exists from the log opened above").Records().expect("the records were appended by the calls above"), ["one", "two"]);
}
