//! Order is the meaning, so order is what this tests.

use kwb_platform::RecordLogStrategy;
use kwb_platform_std::FileRecordLog;

fn Scratch(name: &str) -> std::path::PathBuf
{
    let root = std::env::temp_dir().join(format!("kwb-log-test-{name}"));
    let _ = std::fs::remove_dir_all(&root);
    return root.join("publications.log");
}

#[test]
fn Test_Records_Should_Read_Back_Oldest_First()
{
    let log = FileRecordLog::At(Scratch("order")).expect("creates");

    log.Append("one").expect("appends");
    log.Append("two").expect("appends");
    log.Append("three").expect("appends");

    assert_eq!(log.Records().expect("reads"), ["one", "two", "three"]);
}

#[test]
fn Test_A_New_Log_Should_Be_Empty_Rather_Than_Absent()
{
    let log = FileRecordLog::At(Scratch("empty")).expect("creates");

    assert_eq!(log.Records().expect("reads"), Vec::<String>::new());
}

#[test]
fn Test_Records_Should_Survive_The_Log_That_Wrote_Them()
{
    let path = Scratch("survives");

    {
        let log = FileRecordLog::At(&path).expect("creates");
        log.Append("one").expect("appends");
        log.Append("two").expect("appends");
    }

    let reopened = FileRecordLog::At(&path).expect("reopens");
    assert_eq!(
        reopened.Records().expect("reads"),
        ["one", "two"],
        "reopening a log truncated it, so the graph it records would replay short"
    );
}

#[test]
fn Test_Reopening_Should_Append_Rather_Than_Overwrite()
{
    let path = Scratch("appends");
    FileRecordLog::At(&path).expect("creates").Append("one").expect("appends");

    FileRecordLog::At(&path).expect("reopens").Append("two").expect("appends");

    assert_eq!(FileRecordLog::At(&path).expect("reopens").Records().expect("reads"), ["one", "two"]);
}
