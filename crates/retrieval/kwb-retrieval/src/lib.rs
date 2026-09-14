//! Search and graph traversal over the domain model.
//!
//! The .NET prototype's `Mcp` host exposes nine read-only tools over exactly this
//! surface, with every reply bounded so an agent cannot exhaust its own context, and
//! excludes mutation from the surface entirely (the prototype's own `D10`). That
//! boundary — retrieval answers questions, admission is the only writer — is read as
//! the actual contract worth preserving, independent of the MCP protocol carrying it.
//!
//! # What is here, and the one third that is not
//!
//! [`CurrentQueries`] and [`HistoricalQueries`] are separate types, one per world, and
//! neither has a write path — `tests/read_only.rs` checks that against this crate's own
//! source rather than trusting it. Keyword matching and structural neighbourhoods are
//! implemented.
//!
//! **Semantic search is not, and is not stubbed.** It needs embeddings, nothing here produces
//! one, and `D-004` holds semantic reconciliation. A `Semantically_Like` that fell back to
//! keyword matching would be the prototype's `AdmitChunk` again: a path that looks wired,
//! answers plausibly, and is not doing the thing its name says.

#![forbid(unsafe_code)]

mod current_queries;
mod held_assertion;
mod held_claim;
mod held_neighbourhood;
mod historical_queries;
mod matching;
mod neighbourhood;

pub use current_queries::CurrentQueries;
pub use held_assertion::HeldAssertion;
pub use held_claim::HeldClaim;
pub use held_neighbourhood::HeldNeighbourhood;
pub use historical_queries::HistoricalQueries;
pub use neighbourhood::Neighbourhood;
