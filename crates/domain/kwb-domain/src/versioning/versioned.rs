//! Anything the graph holds, together with where it stands.
//!
//! It is filed apart from [`KnowledgeGraph`] because it is the entry rather than the map: the
//! graph's own file is about publishing into a versioned structure, and this is about what a
//! single held thing is — a value, and the standing that says whether it is current.
//!
//! [`KnowledgeGraph`]: crate::KnowledgeGraph

use crate::Standing;

/// Anything the graph holds, together with where it stands.
///
/// One type for concepts, claims and assertions, because standing means the same thing for
/// all three, and a second copy of that idea would be a second place for it to drift.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Versioned<Value>
{
    value: Value,
    standing: Standing,
}

impl<Value> Versioned<Value>
{
    /// As asserted.
    #[must_use]
    pub const fn Asserted(value: Value) -> Self
    {
        return Self {
            value,
            standing: Standing::Asserted,
        };
    }

    /// What is held.
    #[must_use]
    pub const fn Value(&self) -> &Value
    {
        return &self.value;
    }

    /// Where it stands.
    #[must_use]
    pub const fn Standing(&self) -> &Standing
    {
        return &self.standing;
    }
}

impl<Value: Clone> Versioned<Value>
{
    /// The same thing, closed.
    #[must_use]
    pub fn Closed(&self, standing: Standing) -> Self
    {
        return Self {
            value: self.value.clone(),
            standing,
        };
    }
}
