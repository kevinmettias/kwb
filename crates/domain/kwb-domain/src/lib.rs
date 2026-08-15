//! The epistemic reasoning surface.
//!
//! One crate for claims, concepts, argumentation, evidence, coverage and the derivation
//! ledger, mirroring the .NET prototype's single `Domain` project rather than
//! pre-splitting along lines nothing has yet grown into. The prototype's own shared
//! shape — every reasoning node carries its sources, its chunks, what it was derived
//! from, a confidence, and a review state — is read as the actual invariant worth
//! keeping, not the C# interfaces (`IEpistemicNode`, `IEdge`) that expressed it.
//!
//! The universal type kernel (`Domain/Universal` in the prototype: a discipline-neutral
//! type and relation hierarchy that domain-specific packs specialize) is this crate's
//! first real design question once work begins here — it is the piece the prototype's
//! own architecture treats as most load-bearing, and porting its *type tree* rather than
//! its *reasoning* would be porting the easy half.
//!
//! Nothing is implemented yet.

#![forbid(unsafe_code)]
