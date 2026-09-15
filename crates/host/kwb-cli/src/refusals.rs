//! How the binary answers a command it could not carry out.
//!
//! Three shapes, told apart by what is being answered rather than by exit code, because two of
//! them exit `2` for different reasons. A command line this binary could not *read* is shown the
//! syntax; a well-formed command whose *run* failed is not, because a person who typed it right
//! has already been shown how to type it and the usage would bury the one line that says what
//! went wrong; and a command whose *arguments* were mistyped is shown the syntax again, since
//! the syntax is what is wrong.
//!
//! They live apart from the verbs that call them because every verb calls at least two, and the
//! copies that accumulated while each verb owned its own are what `KWB-73` found disagreeing.

use std::process::ExitCode;

use crate::usage::Print_Usage;
use crate::USAGE_EXIT;

/// A command line this binary could not read, as the exit that answers it.
///
/// Two lines and an exit code, repeated at every refusal in this crate, and `main` reaches it
/// from two directions — nothing typed at all, and a word that is not a verb — so writing it
/// out at each site left a reader comparing the copies to be sure they agreed.
pub(crate) fn Wrong_Command_Line(complaint: &str) -> ExitCode
{
    eprintln!("kwb: {complaint}");
    Print_Usage();
    return ExitCode::from(USAGE_EXIT);
}

/// A run that failed, under the name of the command that ran.
///
/// The verb is a `&'static str` because it is a name this binary was compiled with and never
/// something assembled at runtime: every caller passes a literal. Saying so is what keeps the
/// verb and the complaint from being swapped, since a complaint read at runtime cannot be given
/// a `'static` lifetime and the exchange would not build.
///
/// The same shape as [`Wrong_Command_Line`] without the usage: a person who typed a well-formed
/// command has already been shown how to type it, and printing the usage under a store error
/// would bury the one line that says what actually went wrong.
pub(crate) fn Complained_Without_Usage(verb: &'static str, complaint: &str, code: u8) -> ExitCode
{
    eprintln!("{verb}: {complaint}");
    return ExitCode::from(code);
}

/// A command line whose **arguments** were mistyped, under the name of the command that read them.
///
/// The verb is a `&'static str` for the reason [`Complained_Without_Usage`] gives.
///
/// [`Complained_Without_Usage`] answers a run that was well-formed and could not finish, and it is right that
/// it withholds the usage. This is the other case: what was wrong is the syntax itself, so the
/// syntax is what the person is shown. The two are told apart by which of them is being
/// answered — an argument the command does not take, or a run that could not proceed — and not
/// by the exit code, because a bad `--through` count and an unreadable log both exit `2` and
/// only the first is a question about how to type the command.
pub(crate) fn Complained_With_Usage(verb: &'static str, complaint: &str) -> ExitCode
{
    eprintln!("{verb}: {complaint}");
    Print_Usage();
    return ExitCode::from(USAGE_EXIT);
}

/// What each refusal answers with, which is the half of it a caller can read.
///
/// These three write two lines and end the process, so the words they print are checked where a
/// person reads them — at the process boundary, in `tests/history.rs`. What is left inside the
/// process is the exit code, and that is not the lesser half: it is what a script branches on and
/// what tells a failed run apart from a mistyped command line. The three functions differ in
/// nothing else, which is why the difference has to be a test rather than a note.
#[cfg(test)]
mod tests
{
    use super::*;

    use crate::FAILURE_EXIT;

    #[test]
    fn Test_Wrong_Command_Line_Should_Answer_With_The_Usage_Exit_Whatever_The_Complaint()
    {
        // The complaints vary and the code must not. `main` reaches this from two directions —
        // nothing typed at all, and a word that is not a verb — so a code that came from the
        // complaint would make those two disagree with each other and with every other refusal
        // that shows the syntax.
        assert_eq!(Wrong_Command_Line("expected a verb"), ExitCode::from(USAGE_EXIT));
        assert_eq!(Wrong_Command_Line("expected a concept"), ExitCode::from(USAGE_EXIT));
    }

    #[test]
    fn Test_Complained_Without_Usage_Should_Carry_The_Code_It_Was_Handed()
    {
        // Why the code is a parameter at all, and the reason this is not `Wrong_Command_Line`
        // under a longer name. A failed run and a mistyped argument both arrive here, they exit
        // `1` and `2`, and a version that answered `USAGE_EXIT` regardless would report a run
        // that could not finish as a person's typing mistake — the one thing the two lines of
        // prose above cannot say, because prose is not what a script reads.
        assert_eq!(
            Complained_Without_Usage("kwb history", "cannot read the publication log", FAILURE_EXIT),
            ExitCode::from(FAILURE_EXIT),
            "a run that failed was reported as a command line that was mistyped"
        );
        assert_eq!(
            Complained_Without_Usage("kwb history", "no such store", USAGE_EXIT),
            ExitCode::from(USAGE_EXIT),
            "the code the caller chose was not the code that came back"
        );
    }

    #[test]
    fn Test_Complained_With_Usage_Should_Answer_With_The_Usage_Exit_For_A_Well_Formed_Run()
    {
        // The case the module doc gives for this existing at all: nothing failed, and what is
        // wrong is the syntax, so the syntax is shown and the code says so. It takes the same two
        // arguments as its sibling above and differs in one of them, which is exactly why the
        // pair cannot be told apart by reading the call.
        assert_eq!(
            Complained_With_Usage("kwb history", "--through takes a count"),
            ExitCode::from(USAGE_EXIT),
            "an argument that was mistyped was not answered with the usage exit"
        );
    }
}
