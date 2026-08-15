//! The admission pipeline.
//!
//! The .NET prototype's own stage order — `link-concepts` then `normalize-concepts`
//! then `admit`, the last of these itself planner, evaluator, mutation-planner and an
//! atomic-write coordinator in sequence — is kept as the default reading (`D-001`) until
//! a reason to change it is found, not because the order is proven optimal but because
//! `D18` in the prototype's history is a concrete, recorded failure of changing pipeline
//! assumptions (feeding a pairwise-correct predicate into a transitive-closure
//! algorithm) without re-deriving what the new stage actually guarantees.
//!
//! Nothing is implemented yet.

#![forbid(unsafe_code)]
