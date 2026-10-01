//! The graph, and the record of what was published into it.
//!
//! [`KnowledgeGraph`] is where state lives and where the **one** liveness expression is
//! applied. [`Publication`] and its [`Replay_Records`] are how the graph becomes durable — `D-014`
//! decided it does so by replaying what was published — and [`ReplayError`] is the failure
//! that route can produce, which is why it is a type rather than a panic.
//!
//! [`KnowledgeGraph`]: knowledge_graph::KnowledgeGraph
//! [`Publication`]: publication::Publication
//! [`Replay_Records`]: publication::Replay_Records
//! [`ReplayError`]: replay_error::ReplayError

pub(crate) mod knowledge_graph;
pub(crate) mod publication;
pub(crate) mod replay_error;
