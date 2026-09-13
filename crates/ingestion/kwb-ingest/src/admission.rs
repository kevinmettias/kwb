//! Stage three: admit what survived, and report only what can be seen.

use core::num::NonZeroUsize;

use kwb_model::ContentIdentity;

use kwb_domain::Assertion;
use kwb_domain::Coverage;
use kwb_domain::KnowledgeGraph;
use kwb_domain::Publication;
use kwb_domain::Scope;
use kwb_domain::Standing;
use kwb_domain::Versioned;
use kwb_store::Document;
use kwb_store::DocumentStore;
use kwb_store::StoreError;
use kwb_store::Written;

use crate::ExtractionRefused;
use crate::ProposedReading;
use crate::ExtractionStrategy;
use crate::Link_Concepts;
use crate::Normalize_Concepts;
use crate::Normalized;
use crate::ReadingKind;

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
    refusal: Option<ExtractionRefused>,
    assertions: Vec<Assertion>,
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

    /// Who asserted what, and at what scope.
    ///
    /// One per admitted claim, sourced by the **content address of the document it came from**.
    /// That is what makes a citation resolve: the address is the digest, so following it
    /// returns the exact bytes the claim was read out of, or fails loudly because they are
    /// gone. `D-006`'s dependency edge, and the reason `D-014` keeps documents at all.
    ///
    /// This is where `D-010`'s decision becomes a thing the pipeline does rather than a thing
    /// a record says: a claim carries no source and no scope, and two sources asserting one
    /// claim are two assertions meeting at it.
    #[must_use]
    pub fn Assertions(&self) -> &[Assertion]
    {
        return &self.assertions;
    }

    /// What the store did with the source document, when there was one to write.
    #[must_use]
    pub const fn Source(&self) -> Option<&Written>
    {
        return self.source.as_ref();
    }

    /// Why the source was not read, when it was not.
    ///
    /// [`Coverage::Unmet`] says *a prerequisite was not satisfied* in a `&'static str`, which is
    /// a decision written in code and deliberately not a message assembled at runtime. That is
    /// right for the outcome and useless to the person who ran the command, who needs to know
    /// which reader gave up and what it said. This carries that, and only that — a report whose
    /// reading happened has [`None`] here, including a reading that proposed nothing, because
    /// that one is not a refusal.
    #[must_use]
    pub const fn Refusal(&self) -> Option<&ExtractionRefused>
    {
        return self.refusal.as_ref();
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
        // After the claims, because replay refuses an assertion naming a claim no earlier
        // record published -- the same ordering rule, one level further along.
        for assertion in &self.assertions
        {
            publications.push(Publication::Assertion {
                assertion: assertion.clone(),
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
        for assertion in &self.assertions
        {
            published = published.With_Assertion(Versioned::Asserted(assertion.clone()));
        }

        return published;
    }
}

/// Admit a source, read by a reader.
///
/// The source document goes through `kwb-store`'s one write door. The reader is asked what it
/// says, and whatever it proposes is linked, normalized, and returned.
///
/// # Why a reader rather than a prepared list of extractions
///
/// A list has no account of where it came from. Taking one meant that every assertion this
/// repository could produce was sourced by a content address and *nothing else* — no location
/// in the source, no protocol, no reader — while `README.md`'s first line said KWB ingests
/// references. Taking a reader makes the missing half impossible to leave out, because there
/// is no longer a way in that does not carry it.
///
/// It also puts the refusal where it can be acted on. A list cannot distinguish *the reader
/// failed* from *the reader found nothing*; both arrive as an empty slice. A reader returns
/// [`ExtractionRefused`] for the first, which becomes [`Coverage::Unmet`] below and never
/// [`Coverage::Barren`].
///
/// # What is still true
///
/// The write door is untouched: this function writes through `kwb-store` exactly as before,
/// and the reader never writes anything. Nothing the reader returns has an identity —
/// [`Link_Concepts`] and `kwb-model` derive every one of those from content, here, after the
/// reading. An extractor cannot bypass admission because it has nothing to bypass it with.
///
/// # Errors
///
/// [`StoreError`] if the source document is refused — a source of zero bytes is
/// [`StoreError::Vacuous`], because a document indistinguishable from a failed read is not
/// something to record having admitted.
///
/// **A reader that refuses is not an error here.** It is a report whose coverage is
/// [`Coverage::Unmet`], because the source was admitted and only the reading did not happen.
pub fn Admit(
    source: Vec<u8>,
    reader: Option<&dyn ExtractionStrategy>,
    needed: ReadingKind,
    store: &mut DocumentStore,
) -> Result<AdmissionReport, StoreError>
{
    let document = Document::Of(source);
    // Read before writing, because the address is derived from the content and does not depend
    // on the store having accepted it. Nothing is recorded either way until the write door
    // below runs -- the reading produces proposals, and proposals are not records.
    let reading = match reader
    {
        Some(reader) => reader
            .Read(document.Identity(), document.Content(), needed)
            .and_then(|reading| return Read_The_Right_Document(reading, document.Identity())),
        None => Err(ExtractionRefused::NotRead),
    };
    let source = store.Write(document)?;

    let reading = match reading
    {
        Ok(reading) => reading,
        Err(refusal) =>
        {
            return Ok(AdmissionReport {
                coverage: Coverage::Unmet {
                    prerequisite: refusal.Prerequisite(),
                },
                normalized: Normalize_Concepts(Link_Concepts(&[])),
                source: Some(source),
                refusal: Some(refusal),
                assertions: Vec::new(),
            });
        }
    };

    // A reader that was never asked has no scope to offer, and `Scope::Unstated` is what that
    // is. This spelled it `Scope::Named("")` until `KWB-50`, which is how a blank `--scope`
    // came to record the same thing as no `--scope` at all.
    let scope = reader.map_or_else(Scope::Unstated, ExtractionStrategy::Scope);
    let extractions = reading.Proposed();
    let normalized = Normalize_Concepts(Link_Concepts(extractions));

    // The citation. The source is the document's own address rather than a filename or a
    // title, so following it returns the bytes the claim was read out of -- and two documents
    // with the same content are one source, which is the same mechanism one crate down.
    let cited = source.Identity().Render();
    let assertions = normalized
        .Linked()
        .Claims()
        .iter()
        .map(|claim| return Assertion::By(&cited, claim, scope.clone()))
        .collect();

    return Ok(AdmissionReport {
        coverage: Coverage_Of_Reading(normalized.Linked().Claims().len()),
        normalized,
        source: Some(source),
        refusal: None,
        assertions,
    });
}

/// The reading, if it is about the document it was handed.
///
/// # Why this is checked rather than assumed
///
/// `Admit` cites the document *it* wrote, not the one the reading names, and it did so without
/// ever comparing them. A reader that returned a reading about a different document therefore
/// had its proposals attributed to this one, silently and with a citation that resolved
/// perfectly — to the wrong bytes.
///
/// That a citation resolves to the exact bytes a claim was read out of is the strongest thing
/// this repository claims, and `D-006` is the dependency edge that rests on it. An unchecked
/// field is not provenance; it is a field.
///
/// # Why the disagreement is a refusal and not a panic
///
/// It is a fault in the reader, and a fault in a reader is the case [`ExtractionRefused`]
/// already describes: it was asked and did not answer usably. Nothing was learned about *this*
/// source, which is what makes the outcome [`Coverage::Unmet`] rather than anything else — the
/// same landing every other unanswered reading gets, reached without a variant that would only
/// ever mean "the reader has a bug".
///
/// # Errors
///
/// [`ExtractionRefused::ReaderFailed`] when the reading names another document.
fn Read_The_Right_Document(
    reading: ProposedReading,
    handed: ContentIdentity,
) -> Result<ProposedReading, ExtractionRefused>
{
    if reading.Source() == handed
    {
        return Ok(reading);
    }

    return Err(ExtractionRefused::ReaderFailed {
        cause: format!(
            "the reading is about {}, and the document handed to it was {}. Refusing to cite \
             the second for what was read out of the first",
            reading.Source().Render(),
            handed.Render()
        ),
    });
}

/// The outcome of an admission whose reading **happened**, derived from what it found.
///
/// # The material examined is the source, and there is exactly one of it
///
/// This used to count *extractions* as the material, and map zero of them to
/// [`Coverage::Unmet`]. That was right while a caller handed in a prepared list, because an
/// empty list really did mean nothing had been offered. With a reader it is wrong, and wrong in
/// the direction that matters: a reader that read the source and proposed nothing **did examine
/// it**, and calling that a prerequisite unsatisfied throws away the one outcome `D17` needs —
/// evidence of absence.
///
/// So the units are now consistent. `examined` is material, the material is the source, and one
/// source was read. `found` is what survived linking and normalization.
///
/// A reading that did *not* happen never reaches here: [`Admit`] returns [`Coverage::Unmet`]
/// directly for a refusal, which is what keeps a reader's failure from ever being recorded as
/// the source having nothing in it. The prototype recorded 1,367 rows of exactly that confusion
/// and foreclosed two thirds of a book while reporting full coverage.
fn Coverage_Of_Reading(found: usize) -> Coverage
{
    // The source. A reading happened, so the material exists and was looked at, and `Coverage`
    // requires that to be non-zero precisely so this cannot be asserted without being true.
    let examined = NonZeroUsize::MIN;

    return Coverage::Of_Run(found, examined);
}
