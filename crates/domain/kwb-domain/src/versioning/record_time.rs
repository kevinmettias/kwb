//! The trailing time a record may carry, and the arity that decides whether it does.
//!
//! `KWB-64` appended a time to every record. Appended rather than inserted, because replay
//! dispatches on how many fields a record has: a field added anywhere changes the arity of every
//! record, so a log written before it would stop replaying. Both shapes therefore have to keep
//! reading, and telling them apart is this module's whole job.
//!
//! Filed apart from [`publication`] because the time is a later axis on the same format rather
//! than part of what a record means. Nothing here reads a field's content: everything here
//! answers *is this one of the shapes that carries a time, and if so which*, and the fields it
//! hands back are still unread text.
//!
//! [`publication`]: crate::publication

use crate::graph::publication::ASSERTION;
use crate::graph::publication::CLAIM;
use crate::graph::publication::CONCEPT;
use crate::graph::publication::SEPARATOR;

/// How many fields a concept record carried before `KWB-64` added a time.
///
/// Named rather than written at the reader, because the count is a property of the record
/// format and not of the one function that happens to check it. A concept is named, a claim adds
/// the claim's text, and an assertion adds that claim's source and scope.
const CONCEPT_FIELDS: usize = 5;
/// How many fields a claim record carried before `KWB-64` added a time.
const CLAIM_FIELDS: usize = 6;
/// How many fields an assertion record carried before `KWB-64` added a time.
const ASSERTION_FIELDS: usize = 7;

/// A record's fields, and the time it carries when it says.
///
/// Named rather than returned as a pair because nothing in `(&[…], Option<i64>)` says which half
/// is the record and which is when it was published, and a reader has to go back to the function
/// to find out.
pub(crate) struct UntimedRecord<'fields>
{
    /// The record's fields, without its trailing time.
    pub(crate) fields: &'fields [&'fields str],

    /// When it was published, when the record says. `None` for a log written before `KWB-64`,
    /// which is *unknown* and not a zero.
    pub(crate) at: Option<i64>,
}

/// The record's fields without its time, and the time if it carried one.
pub(crate) fn Without_Time<'fields>(fields: &'fields [&'fields str]) -> UntimedRecord<'fields>
{
    let Some((without, time)) = Timestamped_Fields(fields)
    else
    {
        return UntimedRecord { fields, at: None };
    };

    return UntimedRecord {
        fields: without,
        at: Some(time),
    };
}

/// The record split at its time, when it carries one.
///
/// `None` covers every shape that is not a timestamped record of a known kind — an untimed
/// record, a record this reader does not know, and a record at the timestamped arity whose last
/// field is not a number. The last of those is deliberately the same answer as the other two: it
/// is not a timestamped record with a broken time, it is a record this reader does not know, and
/// `Applied_Record` refuses it as malformed rather than silently dropping a field.
fn Timestamped_Fields<'fields>(
    fields: &'fields [&'fields str],
) -> Option<(&'fields [&'fields str], i64)>
{
    let kind = fields.first()?;
    let expected = Untimed_Arity(kind)?;
    if fields.len() != expected.saturating_add(1)
    {
        return None;
    }

    let time = fields.last().and_then(|last| return last.parse::<i64>().ok())?;
    let without = fields.get(..expected)?;

    return Some((without, time));
}

/// How many fields a record of this kind carries before `KWB-64` added a time.
///
/// Named per kind rather than inferred, because the alternative is asking whether the last
/// field *looks like* a number — and an assertion's last field is a scope, which a source is
/// free to call `1985`. A format that can be misread by content is one that will be.
fn Untimed_Arity(kind: &str) -> Option<usize>
{
    // The constants, not their spellings. A copy of `"concept"` here would be a second place
    // that decides what a concept record is called, and it would go on agreeing until the day
    // somebody renamed one of them -- at which point every record of that kind would read as a
    // kind this reader does not know, and replay would refuse a log it wrote itself.
    return match kind
    {
        _ if kind == CONCEPT => Some(CONCEPT_FIELDS),
        _ if kind == CLAIM => Some(CLAIM_FIELDS),
        _ if kind == ASSERTION => Some(ASSERTION_FIELDS),
        _ => None,
    };
}

/// When a record was published, if it says.
///
/// [`None`] for a publication written before `KWB-64`, which is *unknown* and not a zero. A
/// caller asking what the graph looked like at a time must decide what to do about records that
/// cannot answer, rather than being handed an epoch that sorts before everything.
#[must_use]
pub fn Published_At(record: &str) -> Option<i64>
{
    let fields: Vec<&str> = record.split(SEPARATOR).collect();

    return Without_Time(&fields).at;
}

#[cfg(test)]
mod tests
{
    //! [`Without_Time`], which no caller outside this crate can reach.
    //!
    //! A file under `tests/` compiles as its own package and can see the public surface only, and
    //! the unit a test belongs to is the stem of the file it sits in. This file's stem is
    //! `record_time`, so the split is tested here against fields directly, which is the only way to
    //! ask about the two halves separately. `Published_At` is asserted from outside, in
    //! `tests/record_time.rs`, where a record can only be reached as one line of text.
    //!
    //! [`Without_Time`]: Without_Time

    use super::*;

    /// A concept record as it was written before `KWB-64` appended a time: the kind, the standing,
    /// the two fields the standing always occupies, and the name.
    const UNTIMED: [&str; 5] = [CONCEPT, "asserted", "", "", "entropy"];

    /// The same record with a time appended, which is the whole of what `KWB-64` changed.
    const STAMPED: [&str; 6] = [CONCEPT, "asserted", "", "", "entropy", "1730000000"];

    #[test]
    fn Test_Without_Time_Should_Split_A_Record_At_Its_Trailing_Time()
    {
        // Both shapes have to keep reading, because replay dispatches on how many fields a record
        // has: a log written before the time existed would otherwise stop replaying, and `D-014`
        // made the graph durable *by* replay.
        let untimed = Without_Time(&UNTIMED);
        assert_eq!(
            untimed.fields,
            UNTIMED.as_slice(),
            "a record written before there was a time lost a field it did have"
        );
        assert_eq!(
            untimed.at, None,
            "a record that does not say when it was published was given a time anyway, and there \
             is no epoch that can be told apart from a real one"
        );

        let stamped = Without_Time(&STAMPED);
        assert_eq!(
            stamped.fields,
            UNTIMED.as_slice(),
            "the time was left on the record, so every field the reader hands back after it is one \
             position out"
        );
        assert_eq!(
            stamped.at,
            Some(1_730_000_000),
            "the time is on the record and was not read back off it"
        );
    }
}
