//! `kwb history`: the publication log replayed to a prefix, and what the graph held then.
//!
//! # Why a count and not a time
//!
//! `D-012` records temporal reconstruction as met and describes it as *the version published at
//! an instant*. A publication record carries a kind, a standing, a successor, a reason and the
//! entity, and **no time at all** — so an instant cannot be asked for in any form, and what a
//! prefix of the log actually answers is *as of the first N publications*. `D-012`'s amendment
//! says so; this command is the honest version of what the mechanism supports.
//!
//! # Why it is worth having as a count
//!
//! `D19-B`: a global query filter rewrote every query, so `merge-audit` resolved none of the
//! merge log's identifiers and printed *"nothing has been merged away"*. `kwb-mcp` answers
//! `merge_losers`, which says what was merged; this says what the graph looked like before it.
//! Those two together are the audit that incident could not perform.

use core::num::ParseIntError;
use std::process::ExitCode;

use kwb_domain::{KnowledgeGraph, Published_At, Replay_Records};
use kwb_platform_std::FileRecordLog;

use crate::arguments::{LeadingFlag, Nothing_Left, Store_Root_From_Arguments};
use crate::keeping::{Log_For, Records_Of};
use crate::refusals::Complained_Without_Usage;
use crate::{FAILURE_EXIT, USAGE_EXIT};

/// Why a `history` run could not answer what it was asked.
///
/// The count a `--through` or `--as-of` flag carries is the one thing this command reads
/// without wording for itself, and the parser knows something the command does not: whether the
/// text held a character that is not a digit, or a number too large for the type. Those are
/// different things for a person to fix, so [`ParseIntError`]'s own words are carried into the
/// complaint rather than replaced by a sentence that cannot tell them apart.
#[derive(Debug)]
enum AskedFailure
{
    /// A `--through` value that is not a count.
    NotACount
    {
        /// The value, as it was given.
        value: String,

        /// Why the parser refused it.
        cause: ParseIntError,
    },

    /// An `--as-of` value that is not a time in unix seconds.
    NotATime
    {
        /// The value, as it was given.
        value: String,

        /// Why the parser refused it.
        cause: ParseIntError,
    },

    /// A question this log cannot answer: a count past its end, or a log holding nothing
    /// timestamped to place an instant against.
    Unanswerable
    {
        /// The complaint, worded where the log's contents are known.
        said: String,
    },
}

impl core::fmt::Display for AskedFailure
{
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        return match self
        {
            Self::NotACount { value, cause } => write!(
                formatter,
                "--through takes a count, and {value:?} is not one: {cause}"
            ),
            Self::NotATime { value, cause } => write!(
                formatter,
                "--as-of takes a time in unix seconds, and {value:?} is not one: {cause}"
            ),
            Self::Unanswerable { said } => write!(formatter, "{said}"),
        };
    }
}

impl core::error::Error for AskedFailure
{
}

impl From<String> for AskedFailure
{
    fn from(said: String) -> Self
    {
        return Self::Unanswerable { said };
    }
}

/// `kwb history --store <dir> [--through <count>]`: the graph as of a publication count.
pub(crate) fn History_Command(arguments: &[&str]) -> ExitCode
{
    let LeadingFlag { value: store_root, rest } = Store_Root_From_Arguments(arguments);
    let log = match Log_For(store_root)
    {
        Ok(Some(log)) => log,
        Ok(None) => return No_Log_To_Replay(),
        Err(complaint) => return Complained_Without_Usage("kwb history", &complaint, FAILURE_EXIT),
    };

    return match Reported_History(&log, rest)
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(exit) => exit,
    };
}

/// The refusal for a `history` run that named no log to replay.
///
/// Without `--store` there is no log, and a graph replayed from no log is empty. Reporting zero
/// concepts would be true of the value and false about the corpus, which is the distinction
/// this repository spends most of its types on.
fn No_Log_To_Replay() -> ExitCode
{
    eprintln!("kwb history: --store names where the publication log is; there is no");
    eprintln!("history without one, because history is the log replayed.");
    return ExitCode::from(USAGE_EXIT);
}

/// The graph a prefix of the log folds into, reported.
///
/// # Errors
///
/// The exit code the run ends on: a log that cannot be read or replayed, a `--through` or
/// `--as-of` this log cannot answer, or a prefix the log does not hold.
fn Reported_History(log: &FileRecordLog, rest: &[&str]) -> Result<(), ExitCode>
{
    let records = Records_Of(log)
        .map_err(|complaint| return Complained_Without_Usage("kwb history", &complaint, FAILURE_EXIT))?;

    let through = Through_From_Arguments(rest, &records).map_err(|complaint| {
        return Complained_Without_Usage("kwb history", &complaint.to_string(), USAGE_EXIT);
    })?;

    let Some(prefix) = records.get(..through)
    else
    {
        return Err(Over_Asked(through, records.len()));
    };

    let graph = Replay_Records(prefix)
        .map_err(|cause| return Complained_Without_Usage("kwb history", &cause.to_string(), FAILURE_EXIT))?;

    Print_History(through, records.len(), &graph);

    return Ok(());
}

/// `--through <count>`, defaulting to the whole log.
///
/// # Errors
///
/// A count past the end of the log. Refused rather than clamped: a run that asked for more
/// history than exists and was quietly given everything would be told the corpus is older than
/// it is, and would have no way to tell that from a corpus that really is that old.
fn Through_From_Arguments(arguments: &[&str], records: &[String]) -> Result<usize, AskedFailure>
{
    let through = match arguments
    {
        [] => records.len(),
        [flag, value, rest @ ..] if *flag == "--through" => Through_Count(value, rest)?,
        [flag, value, rest @ ..] if *flag == "--as-of" => As_Of_Count(value, rest, records)?,
        _ => {
            return Err(AskedFailure::from(format!(
                "unexpected argument {:?}",
                arguments.first()
            )))
        }
    };

    if through > records.len()
    {
        return Err(AskedFailure::from(format!(
            "--through {through} was asked for and the log holds {} publications",
            records.len()
        )));
    }

    return Ok(through);
}

/// The count a `--through` flag carries.
///
/// # Errors
///
/// A value that is not a count, and anything the flag left behind.
fn Through_Count(value: &str, rest: &[&str]) -> Result<usize, AskedFailure>
{
    Nothing_Left(rest)?;

    return value.parse::<usize>().map_err(|cause| {
        return AskedFailure::NotACount {
            value: value.to_owned(),
            cause,
        };
    });
}

/// The count a `--as-of` flag asks for, by way of the time it named.
///
/// # Errors
///
/// A value that is not unix seconds, anything the flag left behind, and a log in which nothing
/// carries a time.
fn As_Of_Count(value: &str, rest: &[&str], records: &[String]) -> Result<usize, AskedFailure>
{
    Nothing_Left(rest)?;

    let asked = value.parse::<i64>().map_err(|cause| {
        return AskedFailure::NotATime {
            value: value.to_owned(),
            cause,
        };
    })?;

    return Through_Time(records, asked).map_err(AskedFailure::from);
}

/// How many publications had happened by a time.
///
/// # Why a log with no timestamps is refused rather than answered
///
/// A publication written before `KWB-64` carries no time, and `Published_At` reports that as
/// unknown rather than as an epoch. A log made entirely of those cannot place anything in time,
/// so every answer would be the same answer whatever was asked — which is a tool agreeing with
/// the question instead of answering it. `D-012`'s amendment is about exactly this distinction
/// between a value and the absence of one.
///
/// A log that carries *some* times answers for those, and the untimed prefix stays included:
/// those publications did happen before the first timed one, which is the only thing about them
/// that is known.
///
/// # Errors
///
/// A log in which nothing is timestamped.
fn Through_Time(records: &[String], asked: i64) -> Result<usize, String>
{
    let timed = records.iter().filter(|record| return Published_At(record).is_some()).count();
    if timed == 0
    {
        return Err(format!(
            "nothing in this log carries a time, so as-of {asked} cannot be answered. Every \
             publication here predates timestamps; --through takes a count, which this log can \
             answer"
        ));
    }

    // The log is append-only and written in order, so the publications that had happened by a
    // time are a prefix. Counting them rather than filtering keeps that true: `Replay_Records` refuses a
    // record naming something no earlier record published, and a filter could drop a concept
    // while keeping the claim about it.
    let through = records
        .iter()
        .take_while(|record| return Published_At(record).is_none_or(|at| return at <= asked))
        .count();

    return Ok(through);
}

/// The refusal for a run that asked for more history than the log holds.
///
/// Refused rather than clamped, which is `KWB-60`: a run given everything when it asked for
/// more would be told the corpus is older than it is, and could not tell that from a corpus
/// that really is that old.
fn Over_Asked(through: usize, held: usize) -> ExitCode
{
    eprintln!("kwb history: {through} publications were asked for and the log holds");
    eprintln!("{held}. Refusing rather than returning what there is.");
    return ExitCode::from(USAGE_EXIT);
}

/// What a replay found, in the library's own vocabulary.
fn Print_History(through: usize, held: usize, graph: &KnowledgeGraph)
{
    println!("through    {through} of {held}");
    println!("concepts   {}", graph.Current().Concepts().len());
    println!("claims     {}", graph.Current().Claims().len());
    println!("citations  {}", graph.Current().Assertions().len());
    println!("held       {}", graph.Every_Version().Concepts().len());
}

/// What the verb answers, in process, before and after it has a log.
///
/// `tests/history.rs` drives this command as a person runs it, which is the level at which the
/// counts it prints and the sentence it refuses by can be read. What that cannot state is the exit
/// code, and the exit code is the half a caller branches on: `tests/history.rs` checks that a
/// refusal failed, and this checks that it failed the way this verb says it does.
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn Test_History_Command_Should_Refuse_A_Run_With_No_Store_On_The_Command_Line()
    {
        // Without `--store` there is no log, and a graph replayed from no log is empty. Reporting
        // zero concepts would be true of the value and false about the corpus, so the run ends on
        // the usage exit instead: nothing failed, and what is wrong is that the command line did
        // not say where the history is kept.
        assert_eq!(History_Command(&[]), ExitCode::from(USAGE_EXIT));
    }

    #[test]
    fn Test_History_Command_Should_Refuse_A_Flag_That_Arrives_With_No_Store()
    {
        // The same refusal with more on the command line, and reached before the flag is read —
        // which is the part worth asserting. A verb that parsed `--through` first would answer the
        // question it was asked with the empty graph of a store nobody named.
        assert_eq!(History_Command(&["--through", "1"]), ExitCode::from(USAGE_EXIT));
    }

    #[test]
    fn Test_History_Command_Should_Answer_A_Log_Holding_A_Publication_Written_Before_There_Were_Times()
    {
        // The whole verb in process: the log is opened, read, replayed and reported. The fixture is
        // a real record of the shape this repository wrote before `KWB-64` — copied from a log, not
        // constructed to pass — so this is also the check that a log older than timestamps still
        // replays through the one command that reads it, which is the cost `D-014` could not pay.
        let store = std::env::temp_dir().join("kwb-cli-history-command");
        std::fs::create_dir_all(&store).expect("a writable temporary directory");
        std::fs::write(
            store.join("publications.log"),
            concat!(
                "concept\u{1F}asserted\u{1F}\u{1F}\u{1F}entropy\n",
                "claim\u{1F}asserted\u{1F}\u{1F}\u{1F}",
                "a05035869b31af055b5b16060404ea98130701b0b0e5a25be8a78c01427a652c",
                "\u{1F}It is non-decreasing.\n"
            ),
        )
        .expect("a writable temporary log");
        let named = store.display().to_string();

        assert_eq!(
            History_Command(&["--store", named.as_str()]),
            ExitCode::SUCCESS,
            "a log this repository wrote could not be replayed by the command that reads it"
        );
    }
}
