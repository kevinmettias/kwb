---
id: D-009
type: decision
title: What must be inspectable between stages, why the stage list is not the answer, and what this subject is actually waiting for
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - prototype
  - sequencing
  - ingestion
relations:
  - target: D-004
    type: relates-to
  - target: D-006
    type: relates-to
  - target: D-008
    type: relates-to
---

# What must be inspectable between stages, why the stage list is not the answer, and what this subject is actually waiting for

## Decision

Three boundary requirements are established, and they are derived from a measured incident
rather than from the corpus's stage list. They hold whatever stages this repository ends up
with, which is why they can be recorded now.

The stage list itself is **out of scope**, and this record names the condition that would
bring it into scope. That condition is not the one `D-004` currently names for this subject,
and the difference is recorded here because it changes when this work becomes possible.

Nothing here designs an intermediate representation, a format, or a pass.

## The incident the requirements are derived from

`D18`. `AliasDerivation.IsVariantOf` is correct. It has a test asserting in as many words
that `C++` and `C` are not variants, and that test passes. `C` was merged into `C++` on the
live corpus anyway — with `Nilpotent matrix` → `Newton's method`, `Taylor's theorem` →
`Total time`, and 141 others. **144 of 579 merges, 25%, fused ideas the rule explicitly
rejects.**

The rule was consulted every time and answered correctly every time. Its answers went into a
union-find, which computes a transitive closure, and *is a variant of* is not transitive:

```text
IsVariantOf("z m",         "zero mass")   = true    a legitimate acronym
IsVariantOf("z m",         "zero matrix") = true    and of that too
IsVariantOf("zero matrix", "zero mass")   = FALSE   it said no. Nobody asked it.
```

**1,878 green tests could not see it, because all of them asserted properties of the
predicate while the defect was in the structure that consumed it.**

That sentence is the whole derivation. Everything below follows from it and from nothing
else.

## The three requirements

### 1. A boundary carries what the stage *asserted*, not only what it *concluded*

In `D18` every assertion was correct and the conclusion was wrong. A boundary that had
carried only the conclusion — the equivalence classes — would have shown 579 merges and no
way to tell the 144 from the 435. A boundary carrying the assertions — the pairwise answers
the rule actually gave — makes the wrong merges *derivable*: an equivalence class containing
a pair the predicate rejects is a defect that can be computed, not one that has to be
noticed.

So the general form: **local, individually falsifiable assertions and the global structure
built from them are different artifacts, and a boundary that keeps only the second cannot
localise a fault in the step between them.** This is what "independently inspectable"
has to mean here, and it is stronger than "the data is dumpable".

### 2. A structural conclusion stays checkable against its assertions after the fact

The prototype built this and it is the part worth carrying: `kw merge-audit` re-asks the rule
about every merge already on record, so a destructive step stays falsifiable *after* it has
run. `D17` states the same requirement from the other side — an audit needs an independent
expectation, and `merge-audit` throws if it can account for fewer losers than the
`ConceptMerge` log records.

A boundary artifact that exists only during the run satisfies requirement 1 and not this one.
The 25% was found by re-asking, months later, on the live corpus.

### 3. A boundary artifact declares the identities of its inputs

Already decided as `D-006`, and named here because it is the third thing a boundary needs and
it would otherwise look absent. Without it, "which stage produced this" and "was this produced
from the current version of that" are unanswerable, and requirement 2's re-check cannot know
whether it is re-checking the same material.

`D-008`'s requirement 1 is the neighbouring hazard: the moment there are two representations
of a thing there are two ways to ask whether it is current.

## Why the stage list is out of scope, and what would bring it in

The prototype's `ROADMAP.md` carries this subject as item **I1** with a seven-stage list —
`Source IR → Extraction IR → Claim IR → Knowledge IR → Normalized IR → Ontology IR → Graph
IR` — and a closing predicate this record accepts as the right one: *a wrong merge is
localised to a named stage by inspecting IRs, not by reading SQL rows.*

`KWB-11` says a stage list copied from the corpus without derivation does not close it, and
the reason is visible the moment the list is compared to what this repository has committed
to. The bands table gives `kwb-ingest` **three** stages — link-concepts, normalize-concepts,
admit — "in the .NET prototype's own order until a reason to change it is found." Seven
against three. The seven-stage list is a design proposal from a corpus conversation; the
three-stage pipeline is what this workspace's own authority currently says. Neither is
evidence for the other, and **the prototype implements neither** — it has zero files
implementing an intermediate representation at all, which is why `D-003` applies: there is no
implementation here whose existence could be mistaken for validation.

Deciding the stages requires two inputs this repository does not have:

- **`KWB-3`** — the relation vocabulary and its algebra. `D18`'s lesson is that a stage
  boundary must separate assertions from the structure consuming them, and *which* structures
  are permitted to consume *which* assertions is exactly what the algebra says.
  `RelationAlgebra`'s comment on `Identity` is the point in one sentence: *"Identity is a
  genuine equivalence relation; that is precisely why it is the only kind safe to feed to a
  union-find."* A stage list drawn before that vocabulary exists would be drawn without the
  thing that determines where the boundaries need to be.
- **`KWB-5`** — the admission pipeline. A boundary is between two stages, and until the
  stages have algorithms, "what must be inspectable here" has no *here*.

**The condition is therefore: `KWB-3` and `KWB-5` closed.** Stated as the item requires,
rather than left as "later".

## What this subject is actually waiting for, which is not what `D-004` says

Recorded because it changes when this work becomes possible, and because the record that
holds it cannot currently be amended from here — `docs/records` is reserved wholesale by
other open items, so this is evidence for that amendment rather than the amendment.

`D-004` places "a canonical Knowledge IR" on its held list, and says the held list is
released when *"the reference miner's relevant experiments close"*, revising the record by
moving entries from the second list to the first.

Measured against that miner on 2026-09-12: **it produces no input to this subject and is not
capable of producing one.** Its experiments concern quote verification against extracted
text, page fidelity, claim grounding, model-reported bearing grades and abandonment — the
evidence model. Those are real inputs to other entries on the held list. None of them bears
on what representations must survive between this repository's ingestion stages, which is a
question about this repository's own pipeline and could not be answered by mining a library
however long it ran.

So this entry is not *pending*. It is **stranded**: waiting on an event that the record
promises and the named instrument will not produce. The practical cost is that it reads as
progressing while nothing is moving it, which is the same shape as the prototype's
`AdmitChunk` path being disarmed by a configuration default rather than closed — an
obligation that looks handled because nothing is complaining.

The real blockers are internal, they are named above, and they are reachable without the
miner. `D-004`'s own rule is unaffected and correct — observation proceeds, and a decision
that depends on an open input does not. What is wrong is only which input this entry depends
on.

## Consequences

- The three requirements hold for any stage list, so work that respects them is not blocked
  on the stage decision.
- `KWB-3` and `KWB-5` are now named as this subject's actual predecessors. Whoever closes
  `KWB-3` should know that a boundary design is waiting on it.
- `D-004`'s held list is owed a correction, and this record is the evidence for one entry of
  it. At least one other entry — cross-source corroboration — appears to be stranded for a
  related reason: the miner's finding identity includes the citation, so two sources
  asserting the same thing never meet within it. That is not established here and should be
  measured by the item that amends the record.
- No format, no pass, and no stage is proposed, which is what `D-004` holds this subject for.

## Alternatives Considered

**Adopting the seven-stage list and recording boundary requirements against it** was
rejected: `KWB-11` forbids it, and the comparison with the bands table's three shows why the
prohibition is right rather than procedural — the two lists disagree by a factor of two and
nothing in evidence adjudicates them.

**Deferring the whole item until `KWB-3` closes** was rejected because the three requirements
do not depend on the stage list. They are derived from an incident that has already happened,
and deferring them would leave the next pipeline to rediscover that asserting properties of a
predicate proves nothing about the structure consuming it.

**Recording the stranding finding as a separate observation** was rejected as a second
authority for something `D-004` governs. It is written here as evidence, explicitly not as an
amendment, and the amendment remains owed.
