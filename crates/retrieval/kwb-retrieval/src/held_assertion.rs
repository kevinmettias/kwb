//! One assertion of a held neighbourhood, and whether it is current in the composed sense.

use kwb_domain::Assertion;
use kwb_domain::Versioned;

/// One assertion of a held neighbourhood, and whether it is current in the composed sense.
///
/// The same composition as [`HeldClaim`], one level further: *an assertion of a claim that is
/// not current is not a current assertion*, which is how the graph puts it. A citation of a
/// claim whose concept was merged away is not evidence for anything live, and a listing that
/// said otherwise would be the citation resolving perfectly to something retired.
///
/// [`HeldClaim`]: crate::HeldClaim
#[derive(Debug)]
pub struct HeldAssertion<'graph>
{
    /// The assertion and what became of it.
    pub held: &'graph Versioned<Assertion>,

    /// Whether it is current, claim and concept and all.
    pub current: bool,
}
