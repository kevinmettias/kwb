//! Stage three: admit what survived, and report only what can be seen.

use core::num::NonZeroUsize;

use kwb_domain::Coverage;
use kwb_store::Document;
use kwb_store::DocumentStore;
use kwb_store::StoreError;
use kwb_store::Written;

use crate::Extraction;
use crate::Link_Concepts;
use crate::Normalize_Concepts;
use crate::Normalized;

/// What one admission did, and what it could see while doing it.
///
/// # Why this is returned rather than journalled, and why the claims come back with it
///
/// `D19`. `kw admit` queued every passage of every book as a job of a kind nothing consumed;
/// the scheduler's `default:` arm deleted it and returned normally, so it was marked
/// `Succeeded`, and the run printed `chunks admitted 0` under the heading *Admitted*. The
/// rule it produced is that **a producer and its consumer belong in the same commit** — never
/// enqueue for a handler that does not exist.
///
/// This repository has nowhere to put a claim. `D-008` measured why: `kwb-store` is the
/// immutable artifact store, the mutable versioned graph that claims and concepts belong in
/// has no crate and no row in the bands table, and deciding it is blocked on work that has
/// not happened. So admission **hands its claims back to its caller, synchronously**, rather
/// than writing them somewhere that does not exist or queueing them for a consumer that was
/// never written. That is the one shape `D19` forbids, avoided by not having a queue.
///
/// # The coverage is derived, never asserted
///
/// [`Coverage`] here is computed from what the stage actually examined and actually produced,
/// so a report cannot claim admissions it does not hold. `D20`, and the reason
/// `AdmissionReport` has no constructor taking one.
#[derive(Clone, Debug)]
pub struct AdmissionReport
{
    coverage: Coverage,
    normalized: Normalized,
    source: Option<Written>,
}

impl AdmissionReport
{
    /// What became of the run: yielded, barren, skipped or unmet.
    #[must_use]
    pub const fn Coverage(&self) -> Coverage
    {
        return self.coverage;
    }

    /// The concepts and claims, handed back rather than stored.
    #[must_use]
    pub const fn Normalized(&self) -> &Normalized
    {
        return &self.normalized;
    }

    /// What the store did with the source document, when there was one to write.
    #[must_use]
    pub const fn Source(&self) -> Option<&Written>
    {
        return self.source.as_ref();
    }
}

/// Admit a source and the extractions taken from it.
///
/// The source document goes through `kwb-store`'s one write door. The extractions are linked,
/// normalized, and returned.
///
/// # Errors
///
/// [`StoreError`] if the source document is refused — a source of zero bytes is
/// [`StoreError::Vacuous`], because a document indistinguishable from a failed read is not
/// something to record having admitted.
pub fn Admit(
    source: Vec<u8>,
    extractions: &[Extraction],
    store: &mut DocumentStore,
) -> Result<AdmissionReport, StoreError>
{
    let source = store.Write(Document::Of(source))?;
    let normalized = Normalize_Concepts(Link_Concepts(extractions));

    return Ok(AdmissionReport {
        coverage: Coverage_Of(extractions.len(), normalized.Linked().Claims().len()),
        normalized,
        source: Some(source),
    });
}

/// The outcome of an admission, derived from what it examined and what it found.
///
/// Nothing examined is [`Coverage::Unmet`] rather than [`Coverage::Barren`], and the
/// distinction is the whole point of that type. A run handed no extractions did not look at a
/// source and find it empty — it was never given anything to look at, which is a prerequisite
/// unsatisfied. The prototype recorded 1,367 of exactly this confusion the other way round
/// and foreclosed two thirds of a book while reporting full coverage.
fn Coverage_Of(examined: usize, found: usize) -> Coverage
{
    return match NonZeroUsize::new(examined)
    {
        Some(examined) => Coverage::Of_Run(found, examined),
        None => Coverage::Unmet {
            prerequisite: "extraction produced nothing to admit",
        },
    };
}
