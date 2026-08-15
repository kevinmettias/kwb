//! The content-addressed document store.
//!
//! `D17`-`D21` in the .NET prototype's own `AGENTS.md` are the reason this crate exists
//! rather than an ORM: every one of those incidents was a write that reported success
//! while destroying evidence it had no basis to destroy — a merge that fused two
//! distinct concepts, a sweep that deleted a strong model's proofs because a weaker
//! model's silence read as disagreement, a scheduler that marked failed work
//! `Succeeded`. `kwb-model`'s coverage discipline (ran-and-found-nothing distinct from
//! never-ran) and this crate's own write door are where those become type-level
//! invariants rather than review conventions — once there is something here to enforce
//! them with.
//!
//! Nothing is implemented yet.

#![forbid(unsafe_code)]
