//! What a passage is said to assert, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/admission.rs`
//!
//! Every admission in this workspace goes in through `ClaimText`, and none of those tests names
//! it. The unit this check reads is the test file's own stem, so `src/readings/claim_text.rs` is
//! covered by a file called `claim_text.rs` and by nothing else — and a change to `Stated` or
//! `Text` would have surfaced under the name of a test about a citation.

use kwb_ingest::ClaimText;

/// Text whose case is the only thing that would move if it were folded.
///
/// The one normalization that looks obviously helpful and is not, and `kwb-model`'s own rule says
/// why: folding case would make `Polish notation` and `polish notation` one assertion.
fn Same_Text_Two_Ways() -> [&'static str; 2]
{
    return ["BVH", "bvh"];
}

#[test]
fn Test_Stated_Should_Carry_The_Assertion_Exactly_As_It_Was_Given()
{
    // The partner of `ConceptName`, and unnormalized for the same reason: this is where a model's
    // output arrives, and nothing about it is trusted yet. What is asserted is that the stage
    // which decides what one assertion is — `kwb-domain`'s `Claim` — is the stage that decides
    // it, rather than this type deciding for it one stage earlier.
    let said = "  Entropy is non-decreasing.  ";

    assert_eq!(
        ClaimText::Stated(said).Text(),
        said,
        "an assertion was reflowed on the way in, so the stage that derives a claim's identity \
         never saw what was asserted"
    );
}

#[test]
fn Test_Text_Should_Hand_Back_What_Was_Carried()
{
    // What was put in comes out, and two assertions differing in case are two assertions. A type
    // that folded case would report the pair below as one claim, and a claim is the thing two
    // books asserting one thing are supposed to meet at — a folding here would make them meet at
    // the wrong one.
    let shouted = Same_Text_Two_Ways().first().copied().expect("the pair");
    let quiet = Same_Text_Two_Ways().last().copied().expect("the pair");

    assert_eq!(ClaimText::Stated(shouted).Text(), shouted);
    assert_ne!(
        ClaimText::Stated(shouted).Text(),
        ClaimText::Stated(quiet).Text(),
        "case was folded here, which would make `Polish notation` and `polish notation` one \
         assertion"
    );
}
