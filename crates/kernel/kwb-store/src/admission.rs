//! What the store did with a document it accepted.
//!
//! This is the one fact a caller counting admissions needs and the receipt's other fields
//! cannot supply: whether the bytes landed now or were already held. It is filed apart
//! from [`Written`] because the two answer different questions — this one is the store's
//! verdict on the write, and the receipt is the evidence that the write happened — and a
//! reader looking for either finds it under the name it carries.
//!
//! [`Written`]: crate::Written

/// What the store did with a document it accepted.
///
/// Both variants are successes and they are **not the same fact**. A caller counting
/// documents admitted must be able to tell bytes that landed from bytes that were already
/// held, because a run that admitted nothing new and a run that admitted a corpus are
/// otherwise one number.
///
/// # There is no third variant and no default
///
/// `D20` in the prototype's register was established by `PipelineOutcome`, whose four
/// values distinguished `Clean` from `Degraded` and whose `Clean` was the enum's zero
/// value — so `AdmissionReport.Outcome`, a settable field nothing set, read `Clean` on a
/// dead model. A field that reports success until someone remembers to say otherwise is
/// not an unfinished feature, it is a false one.
///
/// The rule `D20` produced was to derive a status from the evidence rather than assign it,
/// and failing that to order the enum so the zero value is the unknown or worst case. This
/// type takes the first option and closes the second: it derives no [`Default`], so
/// there is no zero value to fall into, and it is never assigned — [`DocumentStore::Write`]
/// computes it from what the store held, which cannot fall out of step with what the store
/// holds.
///
/// [`DocumentStore::Write`]: crate::DocumentStore::Write
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admission
{
    /// The store did not hold these bytes and now does.
    Stored,

    /// The store already held these exact bytes, so the write changed nothing.
    ///
    /// This is idempotence, not a refusal, and it is what content addressing buys: the
    /// same document offered twice is one document. It is reported rather than silently
    /// folded into success because a stage that cannot say how much it deferred is the
    /// reporting half of `D19`.
    AlreadyPresent,
}
