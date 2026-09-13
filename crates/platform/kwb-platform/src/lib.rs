//! Platform port traits, mirroring `nomos-platform` in `f:/repos/nomos`.
//!
//! The seam argument these traits rest on: **a second implementation is a new crate behind an
//! existing seam, never a change to the traits themselves.** That was never about whether XVPE
//! compiled, and it is the part worth keeping.
//!
//! What this file said until 2026-09-12 — that `kwb-platform-xvpe` is deliberately absent
//! because XVPE's foundations tier fails at 62 errors — was wrong twice over by the time
//! anybody read it. The premise was retired months earlier and `D-007` measured it dead; and
//! `kwb-platform-xvpe` **exists**, created by `KWB-24`, so this file was contradicting its own
//! workspace. It is recorded here rather than quietly deleted because it is the fourth copy of
//! one fact found in this repository, three of which had already been corrected — which is
//! what a restated fact costs, and why the reasons now live in records and the code points at
//! them.
//!
//! `D-007` is why adoption is by git reference and commit SHA rather than by `path`.
//! `D-012` is what was adopted and why it was adopted rather than written.
//!
//! # What is here
//!
//! [`ContentStoreStrategy`]: somewhere bytes can be kept under an address and read back
//! unchanged, atomically. `D-014` decided the document half of durability and this is the
//! surface it named — five operations, no listing, no deletion, no directory walk. A port wide
//! enough to be a filesystem would let a caller reach past `kwb-store`'s single write door,
//! which is the second route into storage `D-008` measured the cost of.
//!
//! [`RecordLogStrategy`]: an append-only sequence of records. `D-014`'s other half — a graph
//! is a fold over what was published, so recording the publications records the graph.
//!
//! The clock, lock and process ports the prototype's platform had are not here yet. They will
//! arrive when something needs them, one at a time, rather than as a set nothing consumes.

#![forbid(unsafe_code)]

mod content_store_port;
mod record_log_port;

pub use content_store_port::ContentStoreStrategy;
pub use content_store_port::StorageError;
pub use record_log_port::RecordLogStrategy;
