//! One tool an agent may call, and the world it reads.
//!
//! It is filed apart from the surface that lists the tools for the reason [`World`] gives for
//! being filed apart from [`Tool`]: the declaration carries the `D19-B` guarantee, and a reader
//! looking for what a tool *is* should be able to find it by its own name rather than by
//! reading through the crate root.
//!
//! [`Tool`]: crate::Tool
//! [`World`]: crate::World

use crate::World;

/// One tool an agent may call, and the world it reads.
///
/// # Why the world is part of the tool rather than a parameter
///
/// `D19-B`. A single `search` tool with a `include_retired` flag is the shape that let
/// `merge-audit` ask the current world a question about merge losers, resolve none, print
/// *"nothing has been merged away"* and exit `0`. Here the world a tool reads is fixed when
/// the tool is declared, so an agent choosing a tool has chosen a world, and there is no
/// argument it can omit to land in the wrong one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tool
{
    /// The name an agent calls.
    pub name: &'static str,

    /// Which world it answers from.
    pub world: World,

    /// What it does, in one line, for a tool listing.
    pub summary: &'static str,
}
