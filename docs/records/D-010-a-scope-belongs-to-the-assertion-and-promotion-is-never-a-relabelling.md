---
id: D-010
type: decision
title: A scope belongs to the assertion, promotion is never a relabelling, and the threshold is deferred
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - domain
  - provenance
  - sequencing
relations:
  - target: D-002
    type: relates-to
  - target: D-004
    type: relates-to
  - target: D-006
    type: relates-to
---

# A scope belongs to the assertion, promotion is never a relabelling, and the threshold is deferred

## Decision

Of the three things a scope could be — a property of a claim, of an assertion of a claim, or
of a context the claim is relativised to — it is **a property of the assertion**. That is not
a preference; it is forced by a mechanism this repository has already decided and cannot
give up.

**Promotion from a narrower scope to a broader one is never a relabelling.** A broader claim
is a new claim, conditioned, carrying the narrow assertions as its evidence, and the narrow
assertions are untouched by its existence.

**How much accumulated evidence suffices is deferred**, and the condition that would settle it
is named. Unlike the subject of `D-009`, this one is genuinely pending rather than stranded.

No scope enumeration is defined here. The corpus already named the levels and `KWB-12` is
explicit that naming them is the easy half.

## Why the scope cannot belong to the claim

`D-002` and `KWB-1` decided that a claim's identity is derived from its content with the
**source deliberately excluded**, so that two books asserting one claim become one claim with
two citations. `kwb-model` records that exclusion as a value rather than an absence precisely
because the whole workspace rests on it.

Put a scope in the claim and it participates in the identity. Then this, from the corpus —

```text
Under workload W, target H, constraints C, static dispatch was preferred.
```

— asserted by a benchmark and asserted by a user who simply prefers it, becomes **two
unrelated rows that can never learn of each other**. The one question worth asking of that
pair is whether the measurement and the preference agree, and an identity scheme that
separates them by scope makes that question unaskable. It is the cross-source dedup mechanism
failing in exactly the shape it was built to prevent, with scope playing the part of the
source.

**The third option fails differently and is worth naming.** Relativising the claim — folding
the scope into the content, so the claim *is* "within classical thermodynamics, entropy is
non-decreasing" — keeps identity honest but makes the scope **prose**. Nothing can compute
over it: two phrasings of one relativisation do not dedup, and the promotion rule below could
not be evaluated by anything but a reader. The corpus's own instruction is that *preserving
scope is essential*, and a scope preserved only as words is preserved the way the prototype
preserved its source exclusion — in a doc comment, where no test can reach it.

So: **one claim, many assertions, each assertion carrying its own scope and its own warrant.**
A claim has no scope. A query asks for claims having an assertion at or above a scope, which
is a question about assertions.

This is also what actually prevents the contamination `KWB-12` names. A user's preference and
a theorem are not kept apart by being different claims — they may be word for word the same
claim. They are kept apart because **the claim never inherits the standing of any assertion of
it.**

## Why promotion is never a relabelling

The corpus states the rule directly, at `XVPE/video 6.txt` §16:

> Suppose you choose `static dispatch` for one scheduler. That is a `ProjectDecision`, not
> `UniversalEngineeringKnowledge`. KWB may record the case as evidence: *Under workload W,
> target H, constraints C, static dispatch was preferred.* **Only after enough evidence could
> a broader conditional claim be formed.** This distinction is important for preventing KWB
> from becoming contaminated by arbitrary historical choices.

Two things in that passage are requirements rather than description, and both survive
derivation from what this repository has already decided.

**A broader claim is formed, not promoted.** It is a different claim with different content —
the conditions W, H and C are *in* the narrow one and generalised out of the broad one — so
under this repository's identity scheme it necessarily has a different identity. Promotion as
a state change on one row is not merely undesirable here; it is unrepresentable, because
changing the content changes the address. The mechanism `KWB-1` built happens to make the
wrong thing impossible, which is worth noticing rather than relying on.

**The narrow assertions survive as the broad claim's evidence.** `D-006` is how: a derived
artifact declares the identities its inputs had when it was derived, so a broad claim names
the narrow assertions it was formed from, and staleness is computed rather than stored. A
promotion that consumed its inputs would be a destructive edit with no evidence authorising
it, which is `D17`.

The general form of the same rule, from the corpus's evidence-escalation ladder — source,
compiler diagnostics, assembly, hardware counters, benchmark, system:

> **No single layer should be silently substituted for another.**

That is the requirement stated negatively, and stated negatively it is usable now. A claim
standing at a broad scope on the strength of an assertion made at a narrow one is a layer
silently substituted.

## What is deferred, and what would settle it

*"Only after enough evidence"* is the half this record does not answer. **How much, of what
kind, and judged how.**

It cannot be answered here because it is the same question as epistemic strength, and that is
on `D-004`'s held list. Anything written now would be a threshold invented to look decisive —
and a threshold is the worst possible thing to invent early, because once written it gets
satisfied.

**The condition: the evidence and epistemic-strength model closing.** And for this entry the
hold is working as `D-004` intends. The reference miner is generating real input to it, unlike
the case `D-009` records. Measured on 2026-09-12 across two corpora under one contract: the
model's self-graded placement held flat at 0.33 against 0.34 while independently *resolved*
placement fell from 0.50 to 0.07, with 78% of `named` bearings resolving to nothing on the
larger cell. A grade a source assigns its own assertion is not a measurement of that
assertion's standing.

That result bears directly on this record's deferred half, because promotion evidence will
arrive graded, and the grade will frequently be a self-report. A promotion rule built on
self-reported strength would promote on the strength of the thing being promoted saying it is
strong. So the deferral is not delay for its own sake; the input is being produced, and it is
already telling us something that would have made an early answer wrong.

## Consequences

- An assertion is a first-class thing here, not an edge decorated onto a claim. `KWB-3` and
  `KWB-4` inherit that: whatever the type kernel calls the relation between a source and a
  claim, it carries the scope and it is where a grade attaches.
- A claim carries no scope field, and a query for "what do we know at scope S" is a query over
  assertions. Anything that gives a claim a scope is re-deriving the contamination this record
  prevents.
- The scope levels themselves remain undefined here, deliberately. `XVPE/video 6.txt` §17
  names nine; the ontological admission rule at corpus topic 61 is the test each has to pass —
  a distinction earns its place by preventing an invalid merge, enabling a query, changing
  inference, or improving provenance — and applying that test to nine candidates is work, with
  an owner, that is not this item.
- `D-004`'s held list is owed a correction distinguishing this entry, which is pending and
  being fed, from `D-009`'s, which is stranded. Both records now carry the evidence for it.
  **Discharged.**

  **Condition met:** `KWB-20`

  It introduced exactly that distinction — closed, pending, stranded and answered elsewhere —
  and placed both entries.

## Alternatives Considered

**Scope as a property of the claim** is rejected above: it breaks cross-source dedup and makes
a measurement and a preference about the same thing mutually invisible.

**Scope as relativised content** is rejected because it reduces the scope to prose, which no
promotion rule can evaluate and no two phrasings of which will dedup.

**Deferring the whole question until the strength model closes** was rejected. The
assertion-versus-claim question is forced by `D-002` and needs no input from the miner, and
leaving it open would let the first type that needs a scope put one on a claim — the cheap,
obvious, wrong place — before anything existed to say otherwise.

**Writing a provisional promotion threshold to be tightened later** was rejected as the most
dangerous available option. A written threshold becomes the thing implementations satisfy, and
the measurement above suggests the obvious early answer — trust the grade attached to the
evidence — is the one that would have been wrong.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `D-011`
- `D-015`
