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
/// Adopted from `xvpe-host-clock` after the host clocks moved there on 2026-09-29.
/// `D-007`'s 2026-09-30 amendment records the default-feature settings and their cost.
pub use xvpe_host_clock::HostedWallClock as SystemClock;

/// Cutting a source into passages small enough to ask about.
///
/// Named for the role: what this repository wants is *a passage a reader can be asked about*,
/// and that XVPE calls it a chunk and measures a page fidelity onto it is `xvpe-corpus-text`'s
/// business. `KWB-47` said this is where `PageFidelity` would arrive, and `KWB-66` is where it
/// maps into `ReadingKind` rather than replacing it.
pub mod reading
{
    pub use xvpe_corpus_text::Chunk as Passage;
    pub use xvpe_corpus_text::ChunkBudget as PassageBudget;
    pub use xvpe_corpus_text::ChunkingStrategy as PassageSplitter;
    pub use xvpe_corpus_text::FidelityThresholds;
    pub use xvpe_corpus_text::PageFidelity;
    pub use xvpe_corpus_text::PageNumber;
    pub use xvpe_corpus_text::PageProfile;
    pub use xvpe_corpus_text::PageText;
    pub use xvpe_corpus_text::SectionBoundary;
}

/// Answering a caller that did not compile against this program.
///
/// # What is here, and the one thing that is not
///
/// A catalogue — what this server offers, each entry carrying a sentence and an argument schema
/// — and the line-framed JSON-RPC 2.0 session that serves it. What is *not* here is the
/// catalogue itself. Which tools exist and what each one means is `kwb-mcp`'s, because a tool
/// list is knowledge this product has and a wire format is not; `KWB-112` is where that split
/// was made, and it is `D-012`'s split stated one band up.
///
/// # Why the re-export names are XVPE's and `reading`'s are not
///
/// [`reading`] renames what it takes, so a change of mechanism is a change in one file. These
/// are not renamed, deliberately: `ToolCatalogStrategy`, `ToolDescriptor` and the three
/// determinism constants are the *names a caller of this seam implements against*, and a
/// product implementing a trait this workspace spelled differently would be a second
/// vocabulary for one contract — which is the rival authority `D-135` exists to avoid, not the
/// quarantine.
///
/// # What is deliberately left behind
///
/// The socket transport, the method surface, the refusals and the protocol revision are all
/// reachable here and none is re-exported: `kwb-mcp` serves over stdio and nothing in this
/// workspace opens a listener, so an unexported door is one no dependent can come to depend
/// on. It is added the day something needs it, which is what the quarantine is for.
pub mod remote_call
{
    pub use xvpe_primitives::DeterminismStrength;
    pub use xvpe_primitives::ReproducibilityScope;
    pub use xvpe_primitives::Strategy;
    pub use xvpe_primitives::TraceEquivalence;
    pub use xvpe_remote_call::ServerIdentity;
    pub use xvpe_remote_call::ToolAnswer;
    pub use xvpe_remote_call::ToolCatalogStrategy;
    pub use xvpe_remote_call::ToolDescriptor;
    pub use xvpe_remote_call_backend_json::Serve_Tools;
}

/// Asking a question and getting an answer, with the answer's shape constrained.
///
/// # Why the whole request vocabulary comes through rather than a narrower door
///
/// A narrower one would mean this crate deciding what a request may contain, which is adding
/// behaviour — the one thing its charter forbids. The quarantine is not that KWB touches few
/// XVPE types; it is that **one crate names XVPE**, so a change of mechanism is a change here.
///
/// What the quarantine does exclude is a provider. Neither adopted crate carries one, and no
/// credential or network dependency enters this workspace, which is why every test downstream
/// runs on [`ReplayInference`] against committed recordings.
///
/// [`ReplayInference`]: inference::ReplayInference
pub mod inference
{
    pub use xvpe_ai_inference::AnswerValue;
    pub use xvpe_ai_inference::ContentBlock;
    pub use xvpe_ai_inference::InferenceError;
    pub use xvpe_ai_inference::InferenceRequest;
    pub use xvpe_ai_inference::InferenceResponse;
    pub use xvpe_ai_inference::InferenceStrategy;
    pub use xvpe_ai_inference::ModelIdentifier;
    pub use xvpe_ai_inference::ModelRole;
    pub use xvpe_ai_inference::RequestFingerprint;
    pub use xvpe_ai_inference::RequestTerms;
    pub use xvpe_ai_inference::ResponseSchema;
    pub use xvpe_ai_inference::SchemaField;
    pub use xvpe_ai_inference::SchemaNode;
    pub use xvpe_ai_inference::TokenUsage;
    pub use xvpe_ai_inference::recording_codec;
    pub use xvpe_ai_inference::ReplayInference;
    pub use xvpe_ai_inference::ReplayRecording;
}
