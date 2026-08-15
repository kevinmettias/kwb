//! Band 0 — the only authoritative statement of `KnowledgeWorkbench` protocol semantics.
//!
//! Every vocabulary a peer must share in order to speak to KWB lives here, and nothing
//! else does — no HTTP route, MCP tool name, CLI spelling or client SDK method is
//! authoritative for these meanings; they are all projections of what this crate says.
//! This mirrors `nomos-contracts` in `f:/repos/nomos` exactly, on purpose: the same
//! admission test, the same reason for it, so a contributor moving between the two
//! repositories is not also relearning what a band-0 crate is for.
//!
//! # What earns a place here
//!
//! A type is admitted when it **crosses a subsystem, process or plugin boundary and the
//! parties on both sides need one stable shared representation of it**. Everything else
//! stays in the crate that owns it, and crosses a boundary as a projection.
//!
//! # Why nothing is admitted yet
//!
//! Nomos's own `D-137` admits `KnowledgeReferenceId` — an opaque, KWB-minted identifier
//! for a claim, rationale or decision — into `nomos-contracts`, so that a Nomos finding
//! or governed rule projection can cite one without Nomos needing to understand KWB's
//! internal graph. That record deliberately does not mint a matching KWB-side type: the
//! value Nomos carries is exactly the string form of whatever identity `kwb-model`
//! eventually derives for a claim or concept, and no additional wrapper earns its place
//! here until `kwb-model` exists and that string's shape is a decision rather than a
//! guess. This crate stays empty until that decision has something concrete to name.
//!
//! # Why this crate names almost nothing, once it names anything
//!
//! Types here are reimplemented by systems that will never compile this crate — Nomos,
//! a TypeScript client, a platform in another Rust workspace entirely. A dependency
//! added here makes the protocol KWB-shaped and forces those peers to vendor a Rust
//! crate in order to agree with us. `serde` is the single exception, because the
//! artifact a peer actually reads is the JSON Schema generated from these declarations.

#![forbid(unsafe_code)]
