//! The extractor side: what an extractor offers, and the seam it offers it through.
//!
//! [`Extraction`] is one concept-and-claim pair as an extractor produced it, before any of it is
//! believed. [`ExtractionStrategy`] is the seam it arrives through, and [`Stated`] is the one
//! implementation this repository has — a person, which is what `kwb admit --says` has always
//! been. [`ProposedReading`] is what comes back: proposals, no identity and no writes, which the
//! three stages above then lower. [`ExtractionError`] is why nothing did, and
//! [`ExtractionLineage`] is what is recorded of where it came from.
//!
//! A model-backed implementation belongs behind the same seam and outside this crate; the reason
//! is stated once, on the trait, rather than repeated here.
//!
//! [`Extraction`]: extraction::Extraction
//! [`ExtractionStrategy`]: extraction_strategy::ExtractionStrategy
//! [`Stated`]: stated::Stated
//! [`ProposedReading`]: proposed_reading::ProposedReading
//! [`ExtractionError`]: extraction_error::ExtractionError
//! [`ExtractionLineage`]: extraction_lineage::ExtractionLineage

pub(crate) mod extraction;
pub(crate) mod extraction_error;
pub(crate) mod extraction_lineage;
pub(crate) mod extraction_strategy;
pub(crate) mod proposed_reading;
pub(crate) mod stated;
