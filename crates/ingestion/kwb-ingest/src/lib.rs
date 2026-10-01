//! The admission pipeline.
//!
//! The .NET prototype's own stage order — `link-concepts` then `normalize-concepts`
//! then `admit`, the last of these itself planner, evaluator, mutation-planner and an
//! atomic-write coordinator in sequence — is kept as the default reading (`D-001`) until
//! a reason to change it is found, not because the order is proven optimal but because
//! `D18` in the prototype's history is a concrete, recorded failure of changing pipeline
//! assumptions (feeding a pairwise-correct predicate into a transitive-closure
//! algorithm) without re-deriving what the new stage actually guarantees.
//!
//! # The three stages, and what each refuses
//!
//! `Link_Concepts` attaches a claim to the concept its own extraction named and infers
//! nothing, so it produces no relation whose algebra anything would have to check. It is
//! crate-private, and a stage nothing outside can call is a stage nothing outside can
//! bypass: the two stages below run only as [`Admit_Source`] runs them.
//!
//! `Normalize_Concepts` groups on content identity and **takes no predicate**, so a caller
//! cannot hand it a relation that is not transitive. That is the `D18` door, closed by not
//! existing rather than by being guarded.
//!
//! [`Admit_Source`] writes the source through `kwb-store`'s one write door, hands its claims back to
//! its caller in the same call, and derives its [`Coverage`] from what it examined and found.
//! It queues nothing, because `D19` is what happens when a producer outruns its consumer, and
//! `D-008` measured that the store claims and concepts belong in does not exist yet.
//!
//! # The stage before the three, declared and not yet built
//!
//! [`ExtractionStrategy`] is where a reading of a source arrives. It returns a
//! [`ProposedReading`] — proposals, no identity, no writes — which the three stages above then
//! lower. [`Stated`] implements it with a person, which is what `kwb admit --says` has always
//! been, and a model-backed reader belongs behind the same seam and outside this crate.
//!
//! [`Coverage`]: kwb_domain::Coverage

#![forbid(unsafe_code)]

mod admission_report;
mod concepts;
mod extractor;
mod readings;

#[cfg(test)]
mod tests;

pub use admission_report::Admit_Source;
pub use admission_report::AdmissionReport;
pub use concepts::concept_name::ConceptName;
pub use concepts::linked::Linked;
pub(crate) use concepts::linked::Link_Concepts;
pub(crate) use concepts::normalized::Normalize_Concepts;
pub use concepts::normalized::Normalized;
pub use extractor::extraction::Extraction;
pub use extractor::extraction_error::ExtractionError;
pub use extractor::extraction_lineage::ExtractionLineage;
pub use extractor::extraction_strategy::ExtractionStrategy;
pub use extractor::proposed_reading::ProposedReading;
pub use extractor::stated::Stated;
pub use readings::claim_text::ClaimText;
pub use readings::reader_name::ReaderName;
pub use readings::reading_kind::ReadingKind;
pub use readings::reading_protocol::ReadingProtocol;
pub use readings::source_location::SourceLocation;
