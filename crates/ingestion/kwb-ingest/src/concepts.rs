//! The two concept stages, and the name they group on.
//!
//! [`Link_Concepts`] attaches a claim to the concept its own extraction named and infers
//! nothing, so it produces no relation whose algebra anything would have to check.
//! [`Normalize_Concepts`] groups on content identity and **takes no predicate**, so a caller
//! cannot hand it a relation that is not transitive — the `D18` door, closed by not existing
//! rather than by being guarded. [`ConceptName`] is the name both stages agree on.
//!
//! [`Link_Concepts`]: linked::Link_Concepts
//! [`Normalize_Concepts`]: normalized::Normalize_Concepts
//! [`ConceptName`]: concept_name::ConceptName

pub(crate) mod concept_name;
pub(crate) mod linked;
pub(crate) mod normalized;
