//! What a `ToolName` carries, and what it deliberately refuses to decide.
//!
//! # What these can and cannot prove
//!
//! `tool_name.rs` gives one reason for the type: `Answer_Tool_Call` took `tool` and `argument`
//! as two adjacent `&str`s, so a caller could hand them over in the wrong order and the compiler
//! would accept it. No test of this type can prove that swap impossible — the compiler is what
//! proves it now, by making the two positions different types. What a test can do is pin the
//! two things the type is responsible for on its own: that it hands back exactly the name it
//! was built from, and that it validates nothing, because an unknown name is a wrong *call*
//! rather than a value that should not exist.

use kwb_mcp::ToolName;

#[test]
fn Test_Named_Should_Carry_The_Name_It_Was_Given()
{
    assert_eq!(ToolName::Named("search").Text(), "search");
}

#[test]
fn Test_Named_Should_Refuse_Nothing_So_An_Unknown_Name_Is_Not_A_Wrong_Value()
{
    // The refusal belongs to the dispatcher, which answers `None` rather than this type
    // declining to exist — `tool_name.rs` says so, and this is the test that would notice if
    // somebody moved the check down here. A `Named` that started rejecting unknown names would
    // turn a wrong call into a wrong value, and `Answer_Tool_Call`'s `None` arm would become
    // unreachable.
    let unknown = ToolName::Named("delete_everything");

    assert_eq!(unknown.Text(), "delete_everything");
}

#[test]
fn Test_Text_Should_Give_Back_The_Name_The_Caller_Gave_It()
{
    // Borrowing rather than owning is the design, so the lifetime is the claim: the name this
    // hands back is the caller's own text, not a copy that could drift from it.
    let owned = String::from("get_concept");

    assert_eq!(ToolName::Named(&owned).Text(), "get_concept");
    assert_eq!(ToolName::Named("get_concept").Text(), owned);
}
