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
pub(crate) fn Store_Root_From<'arguments>(
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
pub(crate) fn Flag_From<'arguments>(
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
