//! The flags a command line is read through, shared by the three verbs that read them.
//!
//! These are the parts of argument reading that do not belong to any one command: `admit` wants
//! a store, a scope and a list of `--says` groups; `history` and the two closings want a store
//! and a value per named flag. Reading a leading flag is the same act in all of them, and a copy
//! per verb is a copy that decides differently the day one of them is corrected.

/// `--store <dir>`, if it leads the remaining arguments.
///
/// Read before the extractions so that a misplaced `--store` is an unexpected argument rather
/// than a concept named `--store`, which is the kind of quiet misreading a hand-written command
/// line invites. For `admit` that ordering is the whole point; the other two verbs read the same
/// flag through this as well, so all three refuse a misplaced one the same way.
pub(crate) fn Store_Root_From<'arguments>(
    arguments: &'arguments [&'arguments str],
) -> (Option<&'arguments str>, &'arguments [&'arguments str])
{
    return match arguments
    {
        [flag, root, rest @ ..] if *flag == "--store" => (Some(root), rest),
        _ => (None, arguments),
    };
}

/// A named flag's value, if the flag leads the remaining arguments.
pub(crate) fn Flag_From<'arguments>(
    arguments: &'arguments [&'arguments str],
    flag: &str,
) -> (Option<&'arguments str>, &'arguments [&'arguments str])
{
    return match arguments
    {
        [found, value, rest @ ..] if *found == flag => (Some(value), rest),
        _ => (None, arguments),
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
