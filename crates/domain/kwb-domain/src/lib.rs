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
//! Nothing is implemented yet.

#![forbid(unsafe_code)]
