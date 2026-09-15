//! Whether the command line named a store to serve.
//!
//! Filed apart from the tools for the reason [`World`] is: it is a fact about the command line
//! rather than about what the surface can answer, and the listing's sentence about an empty
//! corpus turns on it.
//!
//! [`World`]: crate::World

/// Whether the command line named a store to serve.
///
/// The listing says *pass a store directory to serve a real corpus* only when none was named, and
/// that is a fact about the command line rather than about what is left of it: a run given a
/// store and no tool has named no tool and is still not the empty listing. A bare `bool` would
/// carry that from `main` through two functions with nothing at any of them saying which of the
/// two a `true` was.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Store
{
    /// A store directory was named, and the corpus is what its log folds into.
    Named,

    /// No store directory was named. The listing is over an empty graph, and says so.
    Absent,
}

impl Store
{
    /// Which store a command line named, read from the argument that names it.
    ///
    /// The first argument is the store directory, so a command line that named no store named
    /// nothing at all.
    #[must_use]
    pub const fn Of(root: Option<&str>) -> Self
    {
        if root.is_some()
        {
            return Self::Named;
        }

        return Self::Absent;
    }

    /// Whether no store was named.
    #[must_use]
    pub const fn Is_Absent(self) -> bool
    {
        return matches!(self, Self::Absent);
    }
}
