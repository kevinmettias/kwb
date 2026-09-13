//! The epistemic reasoning surface.
//!
//! One crate for claims, concepts, argumentation, evidence, coverage and the derivation
//! ledger, mirroring the .NET prototype's single `Domain` project rather than
//! pre-splitting along lines nothing has yet grown into. The prototype's own shared
//! shape — every reasoning node carries its sources, its chunks, what it was derived
//! from, a confidence, and a review state — is read as the actual invariant worth
//! keeping, not the C# interfaces (`IEpistemicNode`, `IEdge`) that expressed it.
//!
//! The universal type kernel — a discipline-neutral type and relation hierarchy that
//! domain packs specialize — is this crate's first real design question, and `D-011`
//! states what it has to guarantee before anything here declares a type. Read that
//! record first. It is requirements rather than a vocabulary, deliberately: `D-004`
//! holds the kernel because the prototype's evidence for it is thin, while listing the
//! relation algebra as operationally validated, so the two halves are not equally safe
//! to build on and `D-011` says which is which.
//!
//! Three things from it are worth knowing before reading any further into this crate.
//! The relation vocabulary has **twenty-one** kinds, not the nine discovery
//! discriminations a plan is tempted to name. They are **not** mutually exclusive, so
//! something has to decide which one wins when several fit. And a universal kind can
//! currently commit its specializations on three of the algebra's twelve properties,
//! which is why `D-011` asks for a constraint surface that reaches all of them — the
//! missing one that matters most is that transitivity can be forbidden and cannot be
//! required, so the property that makes `Identity` safe to feed to a union-find is the
//! one the vocabulary cannot assert.
//!
//! # What is here now
//!
//! `KWB-4`: [`Concept`], [`Claim`] and [`Coverage`]. `KWB-23`: [`Assertion`] and
//! [`Scope`], which are where a source and a domain attach — a claim has neither, and
//! `D-010` is why. `KWB-24`: [`Standing`] and [`KnowledgeGraph`], where state
//! lives and where the **one** liveness expression is applied.
//!
//! The two reads off a graph — [`CurrentKnowledge`] and [`EveryVersion`] — are different
//! types rather than one type with a flag, because `D19-B` is what a forgettable filter
//! costs. `KWB-25` connects `kwb-ingest` to it, so admission's output reaches something. The first two rest on `kwb-model`'s
//! identity and on its two exclusions; the third is the anti-data-loss primitive the
//! admission pipeline cannot be built safely without, which is why it comes before `KWB-5`
//! rather than with it.
//!
//! Nothing else is implemented yet, and the universal kernel above is deliberately not.

#![forbid(unsafe_code)]

mod assertion;
mod claim;
mod concept;
mod knowledge_graph;
mod publication;
mod coverage;
mod scope;
mod standing;

#[cfg(test)]
mod tests;

pub use assertion::Assertion;
pub use claim::Claim;
pub use concept::Concept;
pub use knowledge_graph::CurrentKnowledge;
pub use knowledge_graph::EveryVersion;
pub use knowledge_graph::KnowledgeGraph;
pub use knowledge_graph::Versioned;
pub use coverage::Coverage;
pub use publication::Published_At;
pub use publication::Publication;
pub use publication::Replay;
pub use publication::ReplayError;
pub use scope::Scope;
pub use standing::Standing;
