//! A reader that is not a person.
//!
//! # What this crate is, and what it is deliberately not
//!
//! `kwb-ingest` declares [`ExtractionStrategy`] — what a reader offers and the four things it
//! may not do — and `Stated` implements it with a person typing `--says`. This implements it
//! with a model, and that is the *whole* of the difference: the same seam, a different reader.
//!
//! The corpus's ontological admission rule is why the split falls here. *If the model were
//! replaced by a perfect human annotator, would this feature still make sense?* Extraction as a
//! step: yes, so the step is KWB's and lives in `kwb-ingest`. Extraction by a model: no, so the
//! implementation is an AI capability KWB happens to use, and it lives here, behind the seam,
//! in a crate the rest of the workspace does not depend on.
//!
//! # What it may not do, enforced by what it can reach
//!
//! It mints no identity — nothing here can, because `ContentIdentity` is derived in `kwb-model`
//! from content and this crate never constructs one. It writes nothing — it has no dependency on
//! `kwb-store`, so the one write door is not reachable from here even by mistake. It decides no
//! admission — it returns [`ProposedReading`]s and `Admit` decides what becomes a concept, a
//! claim and an assertion.
//!
//! **No provider is here either.** `xvpe-ai-inference` carries none, and this crate takes an
//! [`InferenceStrategy`] rather than choosing one, so what answers a question is the composition
//! root's decision. Every test in this workspace runs on a replay strategy over committed
//! recordings, and no credential or network dependency enters.
//!
//! [`ExtractionStrategy`]: kwb_ingest::ExtractionStrategy
//! [`ProposedReading`]: kwb_ingest::ProposedReading
//! [`InferenceStrategy`]: kwb_platform_xvpe::inference::InferenceStrategy

#![forbid(unsafe_code)]

mod reads_text;

pub use reads_text::PROTOCOL;
pub use reads_text::Request_For;
pub use reads_text::Propositions_Schema;
pub use reads_text::ReadsText;
