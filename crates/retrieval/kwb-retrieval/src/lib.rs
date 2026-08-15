//! Search and graph traversal over the domain model.
//!
//! The .NET prototype's `Mcp` host exposes nine read-only tools over exactly this
//! surface, with every reply bounded so an agent cannot exhaust its own context, and
//! excludes mutation from the surface entirely (the prototype's own `D10`). That
//! boundary — retrieval answers questions, admission is the only writer — is read as
//! the actual contract worth preserving, independent of the MCP protocol carrying it.
//!
//! Nothing is implemented yet.

#![forbid(unsafe_code)]
