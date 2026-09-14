//! Canonical identity for `KnowledgeWorkbench`.
//!
//! An artifact here is addressed by **what it is**, never by a key somebody authored and
//! never by a value the runtime minted. [`ContentIdentity`] is that address, and
//! [`Derivation`] is the only way to mint one.
//!
//! # The mechanism, and why it is the dedup mechanism
//!
//! Two books asserting the same claim must become one claim carrying two citations. That
//! only holds if the source a claim was read from is **not** part of what makes two claims
//! the same claim — so the source is excluded from the derivation, deliberately, and
//! [`Derivation::Excluding`] records the exclusion as a value rather than leaving it as an
//! absence. An exclusion that exists only as an omission cannot be told apart from an
//! oversight, and this is the one the whole workspace rests on.
//!
//! The same mechanism gives a citation the two properties it actually needs, which are one
//! property seen from two sides: a claim re-extracted unchanged keeps its identity, so a
//! citation written into a document **survives the re-run**; and a claim whose content
//! changed gets a new identity, so the old citation **fails loudly** instead of quietly
//! resolving to text that no longer says what was cited.
//!
//! # What is read from the prototype, and what diverges
//!
//! `D-001` reads the .NET prototype for its mechanism, never its types. Carried whole,
//! because each was paid for:
//!
//! - **Whitespace normalized, case deliberately not.** A reflow is not an edit; a changed
//!   capitalisation is.
//! - **A delimiter between every part**, because a hash of a concatenation is only as
//!   unambiguous as its delimiters.
//! - **An absent field still occupies its place**, so `(a, -, b)` cannot collide with
//!   `(a, b, -)`.
//!
//! Two deliberate divergences, both recorded where they happen:
//!
//! - **The digest is not truncated.** The prototype took the first 16 bytes of SHA-256 and
//!   stamped RFC 9562 version-8 fields onto them, because its identity had to be a `Guid`
//!   for EF Core to map it. Nothing here imposes that, so nothing here pays for it: the
//!   identity is all 32 bytes. A truncation that buys no interoperability is only a
//!   narrower collision margin.
//! - **The separator's safety is enforced rather than inherited.** The prototype's argument
//!   for `0x1F` depends on .NET treating it as whitespace; Rust does not. The guarantee is
//!   restored explicitly by stripping control characters in [`Normalize`], which is what
//!   reading for the mechanism rather than the type is supposed to catch.
//!
//! # Construction requires the content
//!
//! There is no public constructor taking an arbitrary string. [`Derivation::Seal`] mints an
//! identity and can only be reached by handing over the content; [`ContentIdentity::Parse`]
//! recognizes one minted earlier, which is the read side of a recorded dependency edge
//! (`D-006`) and of a citation a peer carries (`D-002`). The prototype's own
//! `DeterministicId.From(params string?[])` was the bare static this forbids, and the
//! measured consequence was 87 types defaulting to a fresh random identity against 9 files
//! that called the deriving function at all.

#![forbid(unsafe_code)]

mod content_identity;
mod derivation;
mod exclusion;
mod identity_error;
mod sealed;

#[cfg(test)]
mod tests;

pub use content_identity::ContentIdentity;
pub use content_identity::IDENTITY_BYTES;
pub use content_identity::IDENTITY_CHARACTERS;
pub use derivation::Derivation;
pub use derivation::Normalize;
pub use exclusion::Exclusion;
pub use identity_error::IdentityError;
pub use sealed::Sealed;
