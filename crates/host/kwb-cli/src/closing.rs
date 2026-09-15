//! `kwb retire` and `kwb supersede`: closing a concept, with the evidence `D17` requires.
//!
//! # Why one file for two verbs
//!
//! The verbs differ in exactly one thing — whether there is a successor — and that difference is
//! a field on [`Closing`]. Everything else is shared: both read the same flags, both refuse the
//! same three ways, both apply a transition to the graph that was already there, and both record
//! it before reporting it. Written twice, the second copy would be the one that forgot to record.

use std::process::ExitCode;

use kwb_domain::{Concept, KnowledgeGraph, Publication, Standing, Versioned};
use kwb_platform::RecordLogStrategy;
use kwb_platform_std::FileRecordLog;

use crate::arguments::{Flag_From, LeadingFlag, Nothing_Left, Store_Root_From};
use crate::keeping::Recorded_At;
use crate::refusals::{Complained, Wrong_Command_Line};
use crate::FAILURE_EXIT;

/// `kwb retire <concept> --store <dir> --because <reason>`
/// `kwb supersede <concept> --into <concept> --store <dir> --because <reason>`
///
/// # Why a reason is required rather than optional
///
/// `D17`: destruction requires evidence, and the question it demands be answerable before a
/// delete, deprecate, supersede or overwrite is *what belief authorises this, and what would
/// falsify it*. A closure with no recorded reason cannot answer either half, and the prototype
/// found **144 of 579 merges** wrong months later only because it had a log to re-read.
pub(crate) fn Close_Command(arguments: &[&str], merging: Option<()>) -> ExitCode
{
    let Some((name, rest)) = arguments.split_first()
    else
    {
        return Wrong_Command_Line("expected a concept");
    };

    let closing = match Closing_From(name, rest, merging)
    {
        Ok(closing) => closing,
        Err(usage) => return usage,
    };

    return match Closed(&closing)
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(complaint) => Complained("kwb", &complaint, FAILURE_EXIT),
    };
}

/// A closure as its command line described it: what is closed, what it is closed into, why, and
/// where that is recorded.
struct Closing<'arguments>
{
    /// The concept being closed.
    name: &'arguments str,

    /// The successor `--into` named, when the verb merges.
    successor: Option<&'arguments str>,

    /// The reason `--because` gave, which `D17` requires rather than accepts.
    because: &'arguments str,

    /// Where `--store` said the publication log is.
    store_root: &'arguments str,

    /// `Some(())` for `supersede`, `None` for `retire`.
    merging: Option<()>,
}

/// The closure a command line asks for.
///
/// # Errors
///
/// The usage for one that asks for too little, which is what [`Required_Of`] answers.
fn Closing_From<'arguments>(
    name: &'arguments str,
    arguments: &'arguments [&'arguments str],
    merging: Option<()>,
) -> Result<Closing<'arguments>, ExitCode>
{
    let LeadingFlag { value: successor, rest } = Flag_From(arguments, "--into");
    let LeadingFlag { value: store_root, rest } = Store_Root_From(rest);
    let LeadingFlag { value: because, rest } = Flag_From(rest, "--because");

    let (because, store_root) = Required_Of(because, store_root, rest)?;

    return Ok(Closing {
        name,
        successor,
        because,
        store_root,
        merging,
    });
}

/// The reason and the place to record it, both of which a closure cannot do without.
///
/// Anything the flags left behind is refused here too, so that the three ways a closing command
/// line can be wrong are answered in one place rather than three that have drifted apart.
///
/// # Errors
///
/// The usage for a command line carrying an argument no flag claimed, no reason, or nowhere to
/// record the closure.
fn Required_Of<'arguments>(
    because: Option<&'arguments str>,
    store_root: Option<&'arguments str>,
    rest: &[&str],
) -> Result<(&'arguments str, &'arguments str), ExitCode>
{
    if let Err(complaint) = Nothing_Left(rest)
    {
        return Err(Wrong_Command_Line(&complaint));
    }

    let Some(because) = because.filter(|reason| return !reason.trim().is_empty())
    else
    {
        return Err(Wrong_Command_Line(
            "--because is required. D17: destruction requires evidence",
        ));
    };

    let Some(store_root) = store_root
    else
    {
        return Err(Wrong_Command_Line(
            "--store is required; closing a concept nothing keeps changes nothing",
        ));
    };

    return Ok((because, store_root));
}

/// A closure, applied to what was known and recorded before it is reported.
///
/// # Errors
///
/// The first complaint of the three steps that can make one, in their order: a log that cannot
/// be opened or replayed, a closure `D17` refuses, and a record the medium would not take.
fn Closed(closing: &Closing) -> Result<(), String>
{
    let recorded = Recorded_At(Some(closing.store_root))?;
    let applied = Applied_To(closing, &recorded.known)?;

    // Recorded before anything is printed. `D19`: a report must not outrun the work, and a
    // closure announced but not recorded is one the next run will not know about.
    Record_Closure(recorded.log.as_ref(), &applied.publication)?;

    Print_Closure(closing.name, &applied.after);

    return Ok(());
}

/// A closure, and the graph it produces.
struct Applied
{
    /// The closure as the transition `D-014` records, rather than as the graph it produces.
    publication: Publication,

    /// The graph after it.
    after: KnowledgeGraph,
}

/// A closure applied to what was known.
///
/// Applied to what was known, so the count reported is the corpus after this closure and not
/// this closure in isolation. The previous graph is untouched, which is what makes the state
/// before a merge a thing that was kept.
///
/// # Errors
///
/// The complaint `D17` produces when it refuses this closure.
fn Applied_To(closing: &Closing, known: &KnowledgeGraph) -> Result<Applied, String>
{
    let concept = Concept::Named(closing.name);
    let standing = Closing_Standing(closing, known, &concept)?;

    let after = known.With_Concept(Versioned::Asserted(concept.clone()).Closed(standing.clone()));
    let publication = Publication::Concept { concept, standing };

    return Ok(Applied { publication, after });
}

/// Record the closure, when there is a log to record it in.
///
/// # Errors
///
/// A log the medium refused to append to. `D19`: a report must not outrun the work, and a
/// closure announced but not recorded is one the next run will not know about.
fn Record_Closure(log: Option<&FileRecordLog>, publication: &Publication) -> Result<(), String>
{
    let Some(log) = log
    else
    {
        return Ok(());
    };

    return log
        .Append(&publication.Record(None))
        .map_err(|cause| return format!("cannot record the closure: {cause}"));
}

/// What a closure left behind, in the library's own vocabulary.
fn Print_Closure(name: &str, after: &KnowledgeGraph)
{
    println!("closed     {name}");
    println!("current    {}", after.Current().Concepts().len());
    println!("held       {}", after.Every_Version().Concepts().len());
}

/// The standing a closure produces, refusing the merges `D17` would not authorise.
///
/// # Errors
///
/// A `supersede` with no `--into`, and whatever [`Merged_Into`] refuses about the successor it
/// was given.
fn Closing_Standing(
    closing: &Closing,
    known: &KnowledgeGraph,
    concept: &Concept,
) -> Result<Standing, String>
{
    if closing.merging.is_none()
    {
        return Ok(Standing::Retired {
            because: closing.because.to_owned(),
        });
    }

    let Some(successor) = closing.successor
    else
    {
        return Err("supersede needs --into <concept>".to_owned());
    };

    return Merged_Into(successor, closing, known, concept);
}

/// The standing a supersede produces, once its successor is known to be a real one.
///
/// # Errors
///
/// A concept that would supersede itself, and a successor nothing published.
fn Merged_Into(
    successor: &str,
    closing: &Closing,
    known: &KnowledgeGraph,
    concept: &Concept,
) -> Result<Standing, String>
{
    let into = Concept::Named(successor);
    if into.Identity() == concept.Identity()
    {
        return Err("a concept cannot supersede itself".to_owned());
    }

    if !Held_By(known, &into)
    {
        return Err(format!("nothing published a concept named {successor} to merge into"));
    }

    return Ok(Standing::Superseded {
        by: into.Identity(),
        because: closing.because.to_owned(),
    });
}

/// Whether the graph holds a concept with this identity.
///
/// The successor has to be one the graph holds. A merge into something nobody published is a
/// merge whose successor cannot be resolved, which is how `merge-audit` came to resolve none of
/// the merge log and report that nothing had been merged away.
fn Held_By(known: &KnowledgeGraph, into: &Concept) -> bool
{
    return known
        .Every_Version()
        .Concepts()
        .iter()
        .any(|candidate| return candidate.Value().Identity() == into.Identity());
}
