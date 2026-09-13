//! Stage three: admit what survived, and report only what can be seen.

use core::num::NonZeroUsize;

use kwb_domain::Coverage;
use kwb_domain::KnowledgeGraph;
use kwb_domain::Publication;
use kwb_domain::Standing;
use kwb_domain::Versioned;
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
/// So admission **hands its claims back to its caller, synchronously**, and never queues them.
/// That is the one shape `D19` forbids, avoided by not having a queue.
///
/// When this was written there was also nowhere to put a claim — `D-008` had measured that the
/// mutable versioned graph claims belong in had no crate. `D-012` decided it and `KWB-24` built
/// it, so that is no longer true and this comment does not pretend otherwise. What survives the
/// change is the reason: publishing is [`Published_Into`], a separate call the caller makes,
/// because *what was admitted* and *where it goes* are different decisions. Handing the result
/// back is what keeps the second one the caller's.
///
/// [`Published_Into`]: Self::Published_Into
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

    /// What this admission published, as publications.
    ///
    /// The same things [`Published_Into`] adds to a graph, in the same order, expressed as the
    /// transitions `D-014` records rather than as the graph they produce. A caller recording
    /// them does not have to take a graph apart to find out what changed — which it could not
    /// do correctly anyway, since a graph is a fold and a fold does not remember its inputs.
    ///
    /// Concepts precede claims, because a claim is about a concept and replay refuses a claim
    /// naming one no earlier record published. That ordering is a property of this method and
    /// not an accident of iteration; the replay tests are what would catch it changing.
    ///
    /// [`Published_Into`]: Self::Published_Into
    #[must_use]
    pub fn Publications(&self) -> Vec<Publication>
    {
        let mut publications = Vec::new();

        for concept in self.normalized.Concepts()
        {
            publications.push(Publication::Concept {
                concept: concept.clone(),
                standing: Standing::Asserted,
            });
        }
        for claim in self.normalized.Linked().Claims()
        {
            publications.push(Publication::Claim {
                claim: claim.clone(),
                standing: Standing::Asserted,
            });
        }

        return publications;
    }

    /// Publish what was admitted into a graph, returning the new graph.
    ///
    /// # Why this is a separate call rather than something `Admit` does
    ///
    /// `Admit` writes the source document and decides what is admissible. Publishing decides
    /// *where the result goes*, and those are different decisions with different failure
    /// modes — the first can refuse a source, the second cannot refuse anything, because by
    /// then the work is done and dropping it would be the loss `D19` is about.
    ///
    /// Keeping them apart also means a caller can admit, inspect the report, and decide. A
    /// pipeline that published unconditionally would have no place to put that decision.
    ///
    /// # Why it returns a graph instead of mutating one
    ///
    /// The caller still holds the graph it passed in, unchanged and queryable. That is what
    /// makes the previous version a thing that was *kept* rather than a thing that was
    /// overwritten, and it is the property `D-012` adopted the persistent map for.
    #[must_use]
    pub fn Published_Into(&self, graph: &KnowledgeGraph) -> KnowledgeGraph
    {
        let mut published = graph.clone();

        for concept in self.normalized.Concepts()
        {
            published = published.With_Concept(Versioned::Asserted(concept.clone()));
        }
        for claim in self.normalized.Linked().Claims()
        {
            published = published.With_Claim(Versioned::Asserted(claim.clone()));
        }

        return published;
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
