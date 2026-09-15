//! The epistemic nodes: the things a domain says, and the scope and standing they carry.
//!
//! Every type here is a node the graph holds and a reading can propose. They are one group
//! because they are what `KWB-4` and `KWB-23` put in the crate together, and because none of
//! them is meaningful without the others: a [`Claim`] attaches to a [`Concept`] through an
//! [`Assertion`], and what makes either worth asking about is its [`Coverage`] and its
//! [`Standing`].
//!
//! [`Claim`]: claim::Claim
//! [`Concept`]: concept::Concept
//! [`Assertion`]: assertion::Assertion
//! [`Coverage`]: coverage::Coverage
//! [`Standing`]: standing::Standing

pub(crate) mod assertion;
pub(crate) mod claim;
pub(crate) mod concept;
pub(crate) mod coverage;
pub(crate) mod scope;
pub(crate) mod standing;
