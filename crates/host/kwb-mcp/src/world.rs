//! Which set of versions a tool reads.
//!
//! It is filed apart from [`Tool`] because it is the half of a tool declaration that carries
//! the `D19-B` guarantee: the world is fixed when a tool is declared rather than passed when
//! it is called, and a type that is part of a declaration this load-bearing is one a reader
//! should be able to find by its own name.
//!
//! [`Tool`]: crate::Tool

/// Which set of versions a tool reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World
{
    /// What is current. Merge losers and retired concepts are not here.
    Current,

    /// Every version, including what is closed.
    Historical,
}

impl World
{
    /// The short name shown in a listing.
    #[must_use]
    pub const fn Name(self) -> &'static str
    {
        return match self
        {
            Self::Current => "current",
            Self::Historical => "historical",
        };
    }
}
