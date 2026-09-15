//! The flags a command line is read through, shared by the three verbs that read them.
//!
//! These are the parts of argument reading that do not belong to any one command: `admit` wants
//! a store, a scope and a list of `--says` groups; `history` and the two closings want a store
//! and a value per named flag. Reading a leading flag is the same act in all of them, and a copy
//! per verb is a copy that decides differently the day one of them is corrected.

/// A leading flag, read: the value it carried, and what it left behind.
///
/// The two are one answer rather than two. A value belongs to the flag that carried it and the
/// rest is what that flag did not claim, so a caller holding them as a pair has nothing but
/// position telling it which is which — and swapping them would take the remaining arguments for
/// the flag's value.
pub(crate) struct LeadingFlag<'arguments>
{
    /// The value the flag carried, when it led the arguments.
    pub(crate) value: Option<&'arguments str>,

    /// The arguments the flag did not claim.
    pub(crate) rest: &'arguments [&'arguments str],
}

/// `--store <dir>`, if it leads the remaining arguments.
///
/// Read before the extractions so that a misplaced `--store` is an unexpected argument rather
/// than a concept named `--store`, which is the kind of quiet misreading a hand-written command
/// line invites. For `admit` that ordering is the whole point; the other two verbs read the same
/// flag through this as well, so all three refuse a misplaced one the same way.
pub(crate) fn Store_Root_From_Arguments<'arguments>(
    arguments: &'arguments [&'arguments str],
) -> LeadingFlag<'arguments>
{
    return match arguments
    {
        [flag, root, rest @ ..] if *flag == "--store" => LeadingFlag {
            value: Some(root),
            rest,
        },
        _ => LeadingFlag {
            value: None,
            rest: arguments,
        },
    };
}

/// A named flag's value, if the flag leads the remaining arguments.
pub(crate) fn Flag_From_Arguments<'arguments>(
    arguments: &'arguments [&'arguments str],
    flag: &str,
) -> LeadingFlag<'arguments>
{
    return match arguments
    {
        [found, value, rest @ ..] if *found == flag => LeadingFlag {
            value: Some(value),
            rest,
        },
        _ => LeadingFlag {
            value: None,
            rest: arguments,
        },
    };
}

/// Refuse anything a flag did not claim.
///
/// # Errors
///
/// The first argument no flag took. Every flag in this binary takes one value, so a second one
/// after it is a command line whose author meant something the verb does not do — and both
/// verbs that end with a flag check that here rather than each wording its own refusal.
pub(crate) fn Nothing_Left(rest: &[&str]) -> Result<(), String>
{
    let Some(left) = rest.first()
    else
    {
        return Ok(());
    };

    return Err(format!("unexpected argument {left}"));
}

/// What each reader promises the verb that calls it.
///
/// Every function here takes the arguments that are left and hands back what it claimed, so the
/// contract is about *which* slice comes back — and that is the half a caller cannot check by
/// reading the call. A `--store` that does not lead, or that leads with nothing after it, is not a
/// store; a reader that took it anyway would turn a misplaced flag into a concept of that name.
///
/// These are asserted here rather than at the process boundary because there is nothing to drive:
/// the functions are pure, they take a slice and return a value, and the exit a verb ends on is a
/// separate question the verb's own tests ask.
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn Test_Store_Root_From_Arguments_Should_Take_The_Leading_Store_And_Hand_Back_Its_Value()
    {
        // The pair is one answer, and the two are one transposition apart at the call site: a
        // reader that swapped them would hand the verb the remaining arguments as its root. So
        // both are asserted, and the `rest` half is the one that catches it.
        let arguments = ["--store", "somewhere", "--scope", "local"];

        let LeadingFlag { value, rest } = Store_Root_From_Arguments(&arguments);

        assert_eq!(value, Some("somewhere"), "the root the flag carried was not read");
        assert_eq!(
            rest,
            ["--scope", "local"],
            "the reader claimed arguments the flag did not carry, so the verb would never see them"
        );
    }

    #[test]
    fn Test_Store_Root_From_Arguments_Should_Leave_A_Store_That_Leads_Nothing_Unclaimed()
    {
        // `--store` as the whole command line. It is not a store, and it must not become one by
        // default: a reader that answered `None` with the flag already consumed would have
        // deleted the argument, and the verb would run against no store and report that the run
        // kept nothing — rather than saying that `--store` needs a directory.
        let arguments = ["--store"];

        let LeadingFlag { value, rest } = Store_Root_From_Arguments(&arguments);

        assert_eq!(value, None, "a flag with nothing after it was read as a store");
        assert_eq!(
            rest,
            ["--store"],
            "the flag was consumed without a value, so the command line can no longer say what is \
             wrong with it"
        );
    }

    #[test]
    fn Test_Store_Root_From_Arguments_Should_Ignore_A_Store_That_Does_Not_Lead()
    {
        // Why `admit` reads this before the extractions, which is the ordering the module doc
        // gives as the whole point. A `--store` after a `--says` is a value of that group, and
        // reading the store first is what makes the misplacement an unexpected argument rather
        // than a concept named `--store`.
        let arguments = ["--says", "--store", "somewhere"];

        let LeadingFlag { value, rest } = Store_Root_From_Arguments(&arguments);

        assert_eq!(value, None, "a flag that did not lead was taken as the store");
        assert_eq!(rest, arguments, "a flag that did not lead was consumed anyway");
    }

    #[test]
    fn Test_Flag_From_Arguments_Should_Read_The_Flag_It_Was_Asked_For_And_No_Other()
    {
        // Two closings read `--into` and `--because` through this one function, so what it has to
        // decide is *which* flag it matched. A reader that took any leading `--word` would let a
        // `--because` answer `--into`, and a supersede would merge into a reason.
        let arguments = ["--into", "enthalpy", "--because", "D17"];

        let LeadingFlag { value, rest } = Flag_From_Arguments(&arguments, "--into");
        assert_eq!(value, Some("enthalpy"));
        assert_eq!(rest, ["--because", "D17"]);

        let LeadingFlag { value, rest } = Flag_From_Arguments(rest, "--into");
        assert_eq!(value, None, "a different flag was read as the one that was asked for");
        assert_eq!(rest, ["--because", "D17"], "the wrong flag was consumed");
    }

    #[test]
    fn Test_Nothing_Left_Should_Name_The_First_Argument_No_Flag_Claimed()
    {
        // What it refuses *by*, not where. Both verbs that end with a flag check here, and this
        // is the sentence a person reads, so an argument nobody claimed has to appear in it — or
        // they are told their command line is wrong without being told which word is wrong.
        //
        // The empty case is asserted first because a reader that refused everything would satisfy
        // the assertion below and make every well-formed command line fail.
        assert_eq!(Nothing_Left(&[]), Ok(()));

        let refusal = Nothing_Left(&["concept", "another"]).expect_err("two arguments no flag claimed");
        assert_eq!(
            refusal,
            "unexpected argument concept",
            "the refusal does not name the argument it refused"
        );
    }
}
