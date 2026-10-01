---
id: D-006
type: decision
title: A derivation declares what it was derived from, and staleness is computed rather than stored
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - derivation
  - epistemics
  - invalidation
relations:
  - target: D-003
    type: relates-to
  - target: D-004
    type: relates-to
---

# A derivation declares what it was derived from, and staleness is computed rather than stored

## Decision

A derived artifact records the **set of inputs whose content determined it**, each by the
content identity that input had at derivation time. Staleness is then a comparison, not a
field:

```text
an artifact is stale  ⟺  some input's current content identity
                          differs from the identity recorded on the edge
```

Six rules follow, and the rest of this record is their justification.

1. **A derivation edge relates a derived artifact to its inputs, not to the run that produced
   it.** Run, model, prompt and response belong on a provenance record, which answers *how did
   this come to exist*. The dependency edge answers *what would make this wrong*, and those are
   different questions with different shapes.
2. **Staleness is derived, never assigned.** There is no `IsStale` field, no `Invalidate()` that
   writes a flag, and therefore no state that can fall out of step with the thing it describes.
3. **A changed input and a retracted input are different outcomes and may not share a value.**
   Changed means the derivation is *stale* — it correctly described a state that has moved.
   Retracted means it is *unsupported* — the evidence it rested on is withdrawn.
4. **Invalidation marks. It never deletes.** The one exception is stated below and is narrow.
5. **A consumer learns an artifact is stale from the type it holds, not from remembering to
   check a field.**
6. **`derived-from` is a directed acyclic graph and invalidation is its transitive closure.** A
   cycle is a defect the write path refuses, not a condition traversal works around.

## Rationale

### The failure this exists to prevent

The prototype compiled a corpus and had no mechanism to notice the corpus changing underneath
it. Measured 2026-09-12 and recorded in `docs/corpus/what-kwb-is-for.md`: `DerivationDependency`
0 files, `Staleness` 0 files, `Invalidate` 0 files. Steps 7 and 8 of the design corpus's own
ten-step product loop — *derived connections are recomputed*, *stale syntheses are invalidated*
— have no implementation at all, which is what makes steps 1–6 a pipeline that runs once rather
than a loop.

`DerivationLedger` is the thing that looks like it should cover this, and the shape of its
record is why it cannot:

```text
DerivationRecord { RunId, ChunkId, NodeId, ModelVersion, PromptHash, ResponseHash,
                   DerivationRationale, Outcome, CreatedAt }
```

Every field answers *how this node came to exist*. None answers *which other knowledge this
node's correctness depends on*. A `ConceptSynthesis` computed from forty claims records the
prompt that produced it and not the forty claims, so retracting one of them leaves a synthesis
that still describes them, with nothing anywhere able to notice.

### Why identity does the work, and why that makes rule 2 free

`KWB-1` requires a content-derived identity whose construction requires the bytes it is derived
from. Given that, a dependency edge can carry the identity an input **had** when the derivation
ran, and staleness needs no bookkeeping at all: recompute the input's identity and compare. If
the content is unchanged the identity is unchanged, by construction. If the content changed the
identity changed, by construction.

This is why rule 2 costs nothing. A stored staleness flag would be a **default that reads
fresh** until something remembers to set it — precisely `D20` in the prototype's incident
register, where `PipelineOutcome` read `Clean` on a dead model because `Clean` was the enum's
zero value, and precisely the defect `CoverageOutcome` still carries today with `Yielded` at
zero. The prototype's own remediation of `D20` is stated as *derive a status from the evidence
rather than assigning it: a computed property cannot fall out of step with the counts it
describes, and it leaves no setter for a caller to forget.* That is this rule, applied one
subsystem over.

It also means `D-006` is **not implementable before `KWB-1`**, and should not be attempted
another way in the meantime. A derivation edge carrying a row id rather than a content identity
would need a separate change-detection mechanism, which is the bookkeeping this rule exists to
avoid.

### Changed versus retracted, and why one value will not do

The distinction is the same one `CoverageOutcome` draws between `Barren` and `Skipped`, for the
same reason: two situations that look identical in a count behave differently under repair.

- A **changed** input leaves a recomputable derivation. The old result was right about the old
  input; recomputing is expected to produce a comparable result.
- A **retracted** input leaves a derivation resting on withdrawn evidence. Recomputation may
  legitimately produce **nothing** — and a recomputation that produces nothing is a real answer
  about the remaining evidence, not a failure. Collapsing the two makes a genuine "there is no
  longer support for this" indistinguishable from "the job has not run yet", which is `D19`'s
  shape exactly.

A third outcome is forced by rule 6: an artifact may be stale **because something it depends on
is stale**, without any of its own direct inputs having changed. Propagated staleness is not
the same as direct staleness — the direct case names an input that moved, and the propagated
case names a path — and an explanation that cannot tell a user which it is fails the loop's own
step 9, *users inspect why a connection exists*.

### Mark, not delete — and the exception, with its test

`D17` is the record here and its rule is not "never delete derived data". It is:

> Delete-and-rebuild is safe only when the rebuild is deterministic and its input is complete.
> **The moment a model stands between the delete and the write, the rebuild can fail or return
> less — so write first, or do not delete.**

The prototype names its own licensed case: `ConceptGraphPipeline.ProjectPrerequisitesAsync`
makes no model call, reads every edge in the repository, and scopes its delete to derived rows
so hand-authored ones survive. That is the shape the exception covers, and it is narrow.

So the test a derivation must pass before invalidation is permitted to delete it:

```text
delete is permitted  ⟺  recomputation is deterministic
                    ∧   every input needed is present
                    ∧   the delete is scoped to derived rows
```

A synthesis, a classification, an extraction, an adjudication — anything with a model in its
path — fails the first conjunct and is therefore marked. This is not a conservative default
chosen for comfort; it is the conjunction, applied. `D17`'s own worked failure is a
`ReplaceAllAsync` that deleted unconditionally and wrote conditionally, so an
empty-but-valid model reply erased a concept's entire synthesis.

### The consumer must not have to remember

`D19-B` — the second incident the prototype numbered `D19` — is the argument. Concepts were
readable through two interfaces, and the failure in both directions was a caller *forgetting*:
a global filter rewrote queries written by people who had never heard of it, so `merge-audit`
resolved none of the merge log's ids, printed "nothing has been merged away", and exited `0`;
and with no filter, new code that forgot `Status != Deprecated` reported merge losers as live.
Its rule is **say which world you read**, and the mechanism is that needing the other world is
a dependency visible in a constructor.

Applied here: a read that may return stale artifacts and a read that may not are **different
types**, and neither is reachable by forgetting. A single type with an `is_stale` field a caller
may ignore reproduces the failure, and an invisible global filter reproduces the other half of
it. `KWB-6` already states this discipline for mutation and for temporality; this is its third
instance, which is evidence it is the right shape rather than a special case.

### The algebra, named rather than assumed

`D18` is the register's most expensive entry and its rule is: **name the algebra the algorithm
assumes, and prove the predicate has it.** A pairwise-correct `IsVariantOf` was fed to
union-find, which computes a transitive closure, and fused 144 of 579 merges because *is a
variant of* is not transitive.

So, stated rather than assumed: recursive invalidation computes a **transitive closure over
`derived-from`**, and that is licensed because `derived-from` is transitive **by construction**
rather than by observation — if A's content was determined by B, and B's by C, then a change to
C can change B and therefore A. The relation is not a similarity judgement; it is a record of
what was read.

Two obligations follow from saying so:

- **The graph must be acyclic, and the write path must enforce it.** A cycle makes the closure
  non-terminating and makes "what is stale" ill-defined. Refusing at write time is cheaper than
  detecting at traversal time, and the prototype already learned the general version of this —
  its `PrerequisiteCycleBreaker` exists because cycles reached the graph.
- **The test must exercise the closure, not only the edge.** `D18`'s second rule is *test the
  stage, not only the rule*: a test that asserts a single dependency edge behaves correctly
  cannot catch a defect in the structure that consumes it. `D21` adds the trap — a property test
  whose object is defined inside the test file constrains the specification and not the system.

### Reporting

`D19(a)`'s corollary applies directly, and it is the one that made that incident invisible:
**a report may only count what it can see.** An invalidation pass must report what it marked and
what was recomputed **separately**, because a pass that marks a thousand artifacts and
recomputes none has done something quite different from one that recomputed them all, and a
single number cannot tell them apart.

The same record forbids the obvious next step: *never enqueue for a handler that does not
exist.* Marking artifacts stale in the expectation that some future pass recomputes them is
exactly the `AdmitChunk` failure, which is **still unconsumed in the prototype today**, disarmed
by a configuration default rather than closed. So: marking is complete in itself. Whatever
triggers recomputation ships with its consumer or does not ship.

## Consequences

`KWB-1` is a hard prerequisite, and the dependency edge is its second consumer after
cross-source dedup — which is useful evidence for `KWB-1`'s own design, because an identity that
serves both is being asked to be stable across re-derivation, not merely unique.

The prototype's `DerivationLedger` is **not** the thing to carry over. It is a provenance
record, it is worth keeping as one, and it answers a different question. Two records, two
questions: *how did this come to exist* and *what would make this wrong*.

Every derived artifact in this repository becomes something that must declare its inputs. That
is a real constraint on every later item that produces one — synthesis, classification,
relation inference, coverage rollups — and it is deliberately stated before any of them exists,
because retrofitting a dependency declaration onto producers already written is the change this
record exists to avoid having to make.

## What This Record Does Not Cover

- **Versioning and snapshots** — loop step 10, *users compare current and historical
  interpretations*. It depends on this and is not this: a snapshot of a graph that cannot say
  which of its artifacts are stale is a snapshot of an unknown state. `GraphVersion` measures 0
  files in the prototype and remains unowned on this board.
- **When recomputation runs.** Scheduling, queueing and prioritisation are excluded on `D19`'s
  authority, above.
- **The coverage model and the evidence ontology.** Both are on `D-004`'s held list while the
  reference miner's experiments run. This record deliberately states what a dependency *relates*
  and what invalidation *guarantees* without defining what evidence *is*; where the two meet — a
  recomputation is arguably a coverage event — the held side wins and this record says nothing.
- **Storage.** `KWB-10` holds the question of what storage semantics are required. Recursive
  invalidation is the strongest case yet for the recursive-query capability that item is
  weighing, and that is an input to `KWB-10`, not a decision here.

## Alternatives Considered

**A stored staleness flag with an invalidation pass that sets it** is the obvious
implementation and is rejected by `D20`: a field that reports freshness until someone remembers
to say otherwise is not an unfinished feature, it is a false one. It also requires the
invalidation pass to be correct and to have run, where the computed form requires neither.

**Timestamp comparison — the derived artifact is stale if any input was modified more recently**
was rejected because it makes staleness a property of clocks rather than of content. A
re-ingestion that produces byte-identical content would mark every dependent stale, and the
prototype has the matching lesson one level down: `DeterministicId` normalises whitespace
precisely so that a reflowed line, a changed line ending or differently-indented model output is
not treated as an edit.

**Recording only the input *set* without per-input identities** was rejected because it can
answer *is this stale* but not *which input moved*, which is what loop step 9 needs and what
makes a repair targeted rather than a full recompute.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `D-009`
- `D-010`
