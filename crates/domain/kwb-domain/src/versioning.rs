//! When a thing was true, and what it took to overwrite it.
//!
//! [`Versioned`] is the anti-data-loss primitive `KWB-25` says the admission pipeline cannot
//! be built safely without; [`Published_At`] is the record time that names when a value was
//! written. [`CurrentKnowledge`] and [`EveryVersion`] are the two reads over any of them, and
//! they are different types rather than one type with a flag because `D19-B` is what a
//! forgettable filter costs.
//!
//! [`Versioned`]: versioned::Versioned
//! [`Published_At`]: record_time::Published_At
//! [`CurrentKnowledge`]: current_knowledge::CurrentKnowledge
//! [`EveryVersion`]: every_version::EveryVersion

pub(crate) mod current_knowledge;
pub(crate) mod every_version;
pub(crate) mod record_time;
pub(crate) mod versioned;
