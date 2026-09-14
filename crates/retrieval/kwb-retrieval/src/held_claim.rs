//! One claim of a held neighbourhood, and whether it is current **in the composed sense**.

use kwb_domain::Claim;
use kwb_domain::Versioned;

/// One claim of a held neighbourhood, and whether it is current **in the composed sense**.
///
/// # Why the flag is here and not read off the claim's own standing
///
/// A claim is current when its own standing is current **and its concept's is** — that is what
/// `CurrentKnowledge::Claims` composes, and it is the whole reason a merge closes the knowledge
/// under it rather than orphaning it.
///
/// Asking only `held.Standing().Is_Current()` gives the wrong answer for exactly the case this
/// type exists for: a claim under a superseded concept reports itself live. That was rendered
/// once, during `KWB-65`, and caught by reading the output rather than the types — the tool
/// built to end `D19-B`'s confusion reproducing it on its first run.
///
/// So the flag is computed by **asking the graph which claims are current**, not by restating
/// the rule. `Standing::Is_Current` stays the one liveness expression and this consumes it
/// through the composition that already exists.
#[derive(Debug)]
pub struct HeldClaim<'graph>
{
    /// The claim and what became of it.
    pub held: &'graph Versioned<Claim>,

    /// Whether it is current, concept and all.
    pub current: bool,
}
