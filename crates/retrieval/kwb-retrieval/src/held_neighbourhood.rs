//! Everything ever attached to one concept, each with the standing it holds.

use kwb_domain::Concept;
use kwb_domain::Versioned;

use crate::HeldAssertion;
use crate::HeldClaim;

/// Everything ever attached to one concept, each with the standing it holds.
///
/// # Why this is a second type and not [`Neighbourhood`] with a flag
///
/// `D-012` decided that the two worlds are distinct *values* rather than one type a caller could
/// ask the wrong question of, and `KWB-6` asked for the same of the read surface. The difference
/// is not decoration: everything here is a [`Versioned`], because in this world a claim can be
/// retired and a reader who could not tell a live claim from a closed one would have exactly the
/// confusion `D19-B` is about — `merge-audit` asked the historical question of the current graph
/// and was told nothing had been merged away.
///
/// So the standing travels with every part. A caller cannot render one of these without having
/// been handed what it would need to say so.
///
/// [`Neighbourhood`]: crate::Neighbourhood
#[derive(Debug)]
pub struct HeldNeighbourhood<'graph>
{
    /// The concept at the centre, and what became of it.
    pub concept: &'graph Versioned<Concept>,

    /// Every claim made about it, live or closed, each saying which it is.
    pub claims: Vec<HeldClaim<'graph>>,

    /// Every assertion of those claims, each saying whether it is current.
    pub assertions: Vec<HeldAssertion<'graph>>,
}
