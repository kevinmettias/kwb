//! The word a listing shows for each world, and why it has to be two words.
//!
//! `D19-B` is the incident where a question was asked of the wrong world and answered
//! confidently, and `world.rs` is where that is made impossible rather than remembered: the
//! world is fixed when a tool is declared and is not a parameter. A reader finds out which
//! world they are reading from the `[current]` column, so `Name` is the last link in that
//! chain — and the link that fails silently, because two worlds rendering one word would leave
//! the listing looking correct.

use kwb_mcp::World;

#[test]
fn Test_Name_Should_Give_The_Word_A_Listing_Shows_For_The_Current_World()
{
    assert_eq!(World::Current.Name(), "current");
}

#[test]
fn Test_Name_Should_Give_The_Word_A_Listing_Shows_For_The_Historical_World()
{
    assert_eq!(World::Historical.Name(), "historical");
}

#[test]
fn Test_Name_Should_Tell_The_Two_Worlds_Apart()
{
    // The one property the listing depends on. If these ever collapsed to one word, every
    // `[current]` column would still render, still be spelled correctly, and say nothing —
    // which is `D19-B` reproduced one layer up.
    assert_ne!(World::Current.Name(), World::Historical.Name());
}
