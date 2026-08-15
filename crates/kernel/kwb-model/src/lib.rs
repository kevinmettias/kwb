//! Canonical identity for `KnowledgeWorkbench`.
//!
//! `D-001` (this repository's own) reads the .NET prototype's `DeterministicId` for the
//! mechanism, not the type: a claim's identity is derived from its concept, its
//! normalized text and its scope — deliberately excluding the source it was read from,
//! because that exclusion is the actual cross-source dedup mechanism ("two books
//! asserting the same claim become one claim with two citations" only holds if the
//! source is not part of what makes two claims the same claim).
//!
//! Nothing is implemented here yet. This crate exists so the identity discipline —
//! content-derived, never authored, never defaulted — is decided as this workspace's
//! band-1 kernel before anything above it can quietly invent its own.

#![forbid(unsafe_code)]
