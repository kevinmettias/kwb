//! The one crate in this workspace permitted to name XVPE.
//!
//! # Why a quarantine crate rather than a dependency wherever it is wanted
//!
//! `D-007` decided this repository takes no `path` dependency on XVPE, and that the reason is
//! coupling rather than breakage — the crates KWB wants compile clean and pull no third-party
//! package between them, but a `path` edge binds this repository's reproducibility to another
//! repository's **working tree**. Adoption is therefore by git reference and commit SHA, and
//! it happens in one place so that *what* is adopted and *at which commit* is answerable by
//! reading one manifest.
//!
//! The boundary is the point. Nothing else here names `xvpe-`, so a reader asking "what does
//! this repository depend on, and at what version" has exactly one file to read, and a change
//! to that answer is a change to this crate.
//!
//! # What is adopted, and why it is not written here
//!
//! `D-012` decided the mutable versioned graph is adopted rather than built. A persistent map
//! returns a **new version** that shares most of its nodes with the previous one; the old
//! version is unchanged and remains independently queryable. That is what turns `D-008`'s
//! *which-world read* from a discipline into a fact — the current graph and last Tuesday's
//! graph are two values a caller holds, and there is no filter to bypass and none to forget.
//!
//! Writing a worse one here to avoid three third-party crates would be the trade the wrong way
//! round.
//!
//! # What this crate deliberately does not do
//!
//! **It adds no behaviour.** It re-exports, and the re-export is named for what this
//! repository uses it for rather than for what XVPE calls it, so that a later change of
//! mechanism is a change here and not everywhere. It holds no knowledge vocabulary: what a
//! version *contains* is `kwb-domain`'s, which is the split `D-012` records as XVPE owning the
//! mechanism and the product owning the catalogue.

#![forbid(unsafe_code)]

/// The persistent map versioned state is built on.
///
/// Named for its role here rather than for its implementation: callers in this workspace want
/// *a map whose previous versions stay valid*, and the fact that it is a hash array mapped
/// trie is `xvpe-collections-persistent`'s business.
pub use xvpe_collections_persistent::HamtMap as VersionedMap;

/// The clock a published record is timestamped by.
///
/// Named for its role here rather than for its implementation, like the map above. What a
/// caller in this workspace wants is *the wall time a publication happened*; that XVPE spells
/// it `WallClockStrategy` and can also hand out a monotonic clock is `xvpe-clock`'s business.
///
/// # Why this is adopted and not declared
///
/// `D-014` said `kwb-platform` is owed *a clock for the record's own timestamps*. It is not
/// owed a clock *port*: a clock has no knowledge-domain semantics — its definition never
/// mentions a claim, a concept or a source — so `D-135` puts it in XVPE, and XVPE has one.
/// Declaring a second here would be the rival authority `KWB-49`, `KWB-55` and `KWB-61` each
/// removed from somewhere else.
pub use xvpe_clock::WallClockStrategy as PublicationClock;

/// The time a clock reports.
pub use xvpe_clock::Timestamp as PublicationTime;

/// The implementation a host composes in, for a process that has an operating system.
///
/// Behind `xvpe-clock`'s `std` feature, which is why this crate takes that crate with default
/// features on: adopting the port without it would leave `kwb-cli` with nothing to pass, which
/// is a port with no implementation and no caller.
pub use xvpe_clock::HostedWallClock as SystemClock;
