//! What a refusal says, which is the whole of what this crate decides for itself.
//!
//! # Why the messages are asserted here and not in `kwb-platform-std`
//!
//! That crate's tests walk both ports against a filesystem and they already assert *which* refusal
//! comes back — `StorageError::Absent { address }` and `StorageError::Refused { .. }` both appear
//! there. What they do not assert, because it is not an adapter's to decide, is what a refusal
//! **says**. The message is this crate's, and before this file nothing in the workspace read one.
//!
//! That matters more here than it would in a crate with a body of logic, because a refusal is the
//! only thing this crate produces at run time. The two variants exist so a caller can tell *nothing
//! is there* from *the medium broke* without reading prose — the trait's own documentation says an
//! empty read could only ever mean a fault, since `kwb-store` refuses to write a document of no
//! bytes — and a message that blurred the two would be the one place the distinction leaked.

use std::error::Error;

use kwb_platform::StorageError;

#[test]
fn Test_An_Absent_Address_Should_Be_Named_In_Its_Refusal()
{
    // The address is the half a caller can act on: it is what the reader asked for, and it is what
    // somebody looking at a log has to resolve. A refusal saying only that something was absent
    // sends the reader back to the code to find out what.
    let refusal = StorageError::Absent {
        address: "b7c1f0".to_owned(),
    };

    assert!(
        refusal.to_string().contains("b7c1f0"),
        "the refusal does not name the address that was asked for: {refusal}"
    );
}

#[test]
fn Test_A_Refused_Medium_Should_Carry_What_Was_Attempted_And_What_It_Said()
{
    // Both halves, and for the same reason: `doing` is which operation failed and `cause` is the
    // medium's own words. Dropping either leaves a caller with a failure it cannot report and a
    // reader with nothing to search for.
    let refusal = StorageError::Refused {
        doing: "Put",
        cause: "the temporary file could not be renamed into place".to_owned(),
    };

    let said = refusal.to_string();
    assert!(
        said.contains("Put"),
        "the refusal does not say what was being attempted: {said}"
    );
    assert!(
        said.contains("renamed into place"),
        "the refusal does not carry what the medium reported, so the one part of the cause this \
         crate cannot know is thrown away: {said}"
    );
}

#[test]
fn Test_The_Two_Refusals_Should_Be_Told_Apart_By_Their_Kind()
{
    // Written with the same text on both sides so that the *message* is as nearly one message as
    // two variants can produce, and the two are still different values. That is the property a
    // caller matching on the kind depends on: a port that answered one where the other was true
    // would tell a reader that nothing had ever been stored at an address the medium in fact
    // could not be asked about, and the distinction `Absent` exists to draw would be gone.
    let absent = StorageError::Absent {
        address: "a passage".to_owned(),
    };
    let refused = StorageError::Refused {
        doing: "a passage",
        cause: "a passage".to_owned(),
    };

    assert_ne!(
        absent, refused,
        "the two refusals compare equal, so a caller cannot branch on which one it has"
    );
}

#[test]
fn Test_A_Refusal_Should_Travel_Out_Of_A_Port()
{
    // Every method of both ports returns `Result<_, StorageError>` and every caller propagates with
    // `?`, so the value has to be usable as a boxed error. Asserted rather than assumed because it
    // is a property of a trait impl with no body — `impl core::error::Error for StorageError {}` —
    // and an empty impl is exactly the kind that can be deleted without any test noticing.
    let refused: Box<dyn Error> = Box::new(StorageError::Refused {
        doing: "Get",
        cause: "the file could not be opened".to_owned(),
    });

    assert!(
        refused.to_string().contains("the file could not be opened"),
        "a boxed refusal lost its message on the way out: {refused}"
    );
    assert!(
        refused.source().is_none(),
        "the refusal claims a cause it does not carry, so a caller printing the chain would print \
         an empty link"
    );
}
