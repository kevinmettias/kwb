//! Everything known about one concept: the concept, its claims, and who asserted them.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;

/// Everything known about one concept: the concept, its claims, and who asserted them.
///
/// A **neighbourhood** in the structural sense — concept to claim to assertion — and not in
/// the relation sense. There are no typed relations between concepts in this workspace yet,
/// and `D-011` is why: the universal relation vocabulary is held, is not mutually exclusive,
/// and cannot yet commit a kind to transitivity in both directions. Traversing edges that do
/// not exist would be inventing the thing that is held.
///
/// The current world's answer. The historical one is [`HeldNeighbourhood`], a different type
/// rather than this one with every field wrapped, because a caller that needs a closed claim
/// has had to name the world it is asking.
///
/// [`HeldNeighbourhood`]: crate::HeldNeighbourhood
#[derive(Clone, Debug)]
pub struct Neighbourhood<'graph>
{
    /// The concept at the centre.
    pub concept: &'graph Concept,

    /// The claims made about it.
    pub claims: Vec<&'graph Claim>,

    /// The assertions of those claims.
    pub assertions: Vec<&'graph Assertion>,
}
