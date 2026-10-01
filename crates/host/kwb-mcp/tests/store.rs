//! What the command line's store argument becomes, read as a value.
//!
//! # Why these are worth having at all, for two variants and a `matches!`
//!
//! `store.rs` argues the case for the type: a bare `bool` carried from `main` through two
//! functions with nothing at any of them saying which of the two a `true` was. That argument
//! only holds if the two states are in fact told apart and stay told apart, and the listing's
//! empty-corpus sentence is the one place that reads them. These pin the four combinations so a
//! later `Of` that answered `Named` for `None` — or an `Is_Absent` that became `!is_some` on
//! the wrong value — fails here rather than mis-reporting a corpus as absent.

use kwb_mcp::Store;

#[test]
fn Test_Of_Should_Read_A_Named_Store_From_The_Argument_That_Names_It()
{
    assert_eq!(Store::Of(Some("/a/store/directory")), Store::Named);
}

#[test]
fn Test_Of_Should_Read_A_Command_Line_That_Named_Nothing_As_No_Store()
{
    assert_eq!(Store::Of(None), Store::Absent);
}

#[test]
fn Test_Is_Absent_Should_Be_False_When_A_Store_Was_Named()
{
    assert!(!Store::Of(Some("/a/store/directory")).Is_Absent());
}

#[test]
fn Test_Is_Absent_Should_Be_True_When_The_Command_Line_Named_Nothing()
{
    assert!(Store::Of(None).Is_Absent());
}
