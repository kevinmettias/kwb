//! The concept a passage is about, asserted in the file named for it.
//!
//! # Why this file sits beside `src/tests/extraction.rs`
//!
//! The seam's own tests go through `ConceptName` on their way in — every `An_Extraction` builds
//! one — and none of them names it. The unit this check reads is the test file's own stem, so
//! `src/concepts/concept_name.rs` is covered by a file called `concept_name.rs` and by nothing
//! else, and a change to `Named` or `Text` would have surfaced under the name of a test about the
//! seam.

use kwb_ingest::ConceptName;

/// One name, and the same name with whitespace around it.
///
/// A provider rather than a literal in the test, because the pair is the whole fixture: what is
/// being asserted is what happened to the difference between them.
fn Spaced_And_Tight() -> [&'static str; 2]
{
    return ["  Entropy  ", "Entropy"];
}

#[test]
fn Test_Named_Should_Carry_The_Name_Exactly_As_The_Extractor_Gave_It()
{
    // The boundary where a model's output arrives, and the one place this type could do harm by
    // being helpful. A name normalized here would have its identity derived one stage before
    // linking derives it, and the stage that decides what is one concept would be handed a name
    // whose shape it did not decide — which is exactly the difference `ReadingProtocol` says has
    // to stay visible, and `ConceptName` says has to stay visible *here*.
    // Taken by pattern rather than by position, so that nothing here has to promise the pair is
    // present: the fixture's return type fixes its length, and a binding of a fixed-length array
    // has no case to fail in.
    let [spaced, _] = Spaced_And_Tight();

    assert_eq!(
        ConceptName::Named(spaced).Text(),
        spaced,
        "a concept name was reflowed on the way in, so linking no longer sees what was said"
    );
}

#[test]
fn Test_Text_Should_Hand_Back_What_Was_Carried()
{
    // What was put in is what comes out, and two names that differ are two names. The second
    // half is the one that matters: if the type collapsed whitespace, the two halves of this
    // fixture would be one value and the identity of every concept in the graph would be derived
    // from text nobody said.
    let [spaced, tight] = Spaced_And_Tight();

    assert_eq!(ConceptName::Named(tight).Text(), tight);
    assert_ne!(
        ConceptName::Named(spaced),
        ConceptName::Named(tight),
        "the trailing and leading blanks were normalized away, which is the identity decision \
         this type exists to leave to linking"
    );
}
