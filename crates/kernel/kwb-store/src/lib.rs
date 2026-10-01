//! The content-addressed document store.
//!
//! `D17`-`D21` in the .NET prototype's own `AGENTS.md` are the reason this crate exists
//! rather than an ORM: every one of those incidents was a write that reported success
//! while destroying evidence it had no basis to destroy — a merge that fused two
//! distinct concepts, a sweep that deleted a strong model's proofs because a weaker
//! model's silence read as disagreement, a scheduler that marked failed work
//! `Succeeded`.
//!
//! # The shape of the answer
//!
//! One door, and a receipt that cannot be forged.
//!
//! [`DocumentStore::Write`] is the only method in this crate that takes `&mut self`, and
//! `tests/one_door.rs` asserts that by reading this crate's own source, so the claim is
//! checked rather than merely written down. Everything the store has to be true about a
//! document is therefore decided in one function, and the failure the prototype actually
//! suffered — bytes reaching storage by a route the invariants never saw — has nowhere to
//! happen.
//!
//! [`Written`] is what that door returns, and it is built from a [`Document`], which
//! cannot exist without the bytes it addresses. A success value for work nobody did is not
//! a thing this crate's types can express, which is `D19` read as a type rather than as a
//! rule to remember. [`Admission`] is derived from what the store held rather than
//! assigned by whoever wrote the call, which is `D20` read the same way.
//!
//! `kwb-model`'s coverage discipline — ran-and-found-nothing distinct from never-ran — is
//! the other half of that reading, and it is `KWB-4`'s to build.
//!
//! # What is deliberately not here yet
//!
//! **Durability, transactions, temporal reads, ranked retrieval, any database at all.**
//! `KWB-10` is the open item that asks which of those the prototype proved are required,
//! and answering it here first — by growing a file path, or a connection string, or a
//! trait shaped like the one Postgres would want — is the architectural concretization
//! `D-004` holds until its inputs stabilise.
//!
//! **A document kind.** That is the first row of the universal epistemic type kernel, and
//! `KWB-3` owns it.
//!
//! **A delete path.** Not deferred — absent by construction. An address derived from
//! content is only ever offered the content it names, so there is no overwrite to perform
//! and nothing for `D17`'s "destruction requires evidence" to guard. When this repository
//! does acquire a reason to remove something, that reason arrives with the evidence rule
//! attached and is an item of its own.

#![forbid(unsafe_code)]

mod admission;
mod document;
mod document_store;
mod store_error;
mod written;

pub use admission::Admission;
pub use document::Document;
pub use document_store::DocumentStore;
pub use store_error::StoreError;
pub use written::Written;
