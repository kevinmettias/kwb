---
id: D-004
type: decision
title: Observation proceeds, and a decision whose answer depends on an open reconciliation does not
status: accepted
version: 2
authority: canonical-normative-record
tags:
  - prototype
  - sequencing
  - ecosystem
relations:
  - target: D-003
    type: relates-to
  - target: D-001
    type: relates-to
---

# Observation proceeds, and a decision whose answer depends on an open reconciliation does not

## Decision

Two reconciliations feed this repository's design: the prototype requirement inventory, closed
by `KWB-9`, and the reference miner in `f:/repos/xvpe`, which is still running experiments.
While any input remains open, the governing rule is not "nothing moves." It is:

> **Observation can proceed. A decision whose answer depends on the open input cannot.**

So this record names two lists, and neither is a restatement of the other.

> **Version 2, 2026-09-12.** Two lists were not enough, and several entries were on the wrong
> one. The amendment at the end of this record supersedes the sorting below without touching
> the rule above, which nothing measured contradicts. **Read the amendment before acting on
> either list.** The lists are left as written rather than edited in place, because what
> changed is worth being able to see.

### Safe to proceed on now

The prototype's evidence here is stable at *Exercised* or *Operationally validated* (`D-003`),
and the reference miner's remaining experiments cannot invalidate it:

```text
governing records and ledger tooling
identity mechanics
basic source and document representation
provenance primitives already genuinely established
store boundaries and the write door
deterministic derivation and version mechanisms
validated value types
the operationally validated relation algebra
translation of proven incidents into properties and tests
infrastructure already justified by this repository's own records
```

### Held for reconciliation

The reference miner is actively generating requirements for all of these, and the prototype's
own evidence for them is thin — in the sharpest case, `Domain/Evidence` is one file of 33 lines:

```text
the evidence ontology
a canonical Knowledge IR
knowledge scopes
canonical claim identity
semantic reconciliation
admission
coverage and exhaustion
the confidence / epistemic-strength model
promotion and authorization
cross-source corroboration
the relation kernel, wherever prototype evidence is weak
```

Held means: a record may capture an observation about one of these, and a ledger item may
*reserve* one. What is refused is architectural concretization — a type, a schema, or a closed
vocabulary asserted as the answer — before the inputs stabilise.

The two tracks run concurrently and synchronise only when work approaches that line. `KWB-9`
did not wait for the miner, and nothing else needs to either.

## Rationale

An "open input" is not an excuse to stop, and treating it as one is how a reconciliation pass
becomes a freeze. Most of the work this repository has to do — its records, its ledger tooling,
its identity mechanics, its store boundary — depends on prototype evidence the miner has no
bearing on whatever. Blocking those on an unrelated experiment costs real progress and buys
nothing.

The converse failure is the one this record actually exists to prevent, and it is specific: an
implementing model that starts concretizing architecture while the governing requirement is
still incomplete will produce something plausible, and plausible-and-early is harder to displace
than absent. The held list is exactly the set where that would happen, and the evidence model is
the clearest case — the miner is still discovering what the distinctions have to be
(mechanically verified, mechanically unverifiable, model observed, quote-verification
provenance, abandoned work as interpretation debt, modality fidelity), so designing KWB's
evidence model against 33 lines of prototype would replace one speculative model with another
and give the second one a repository to live in.

The rule draws the line where it is checkable. *Is this an observation, or a decision that the
open input could change?* is a question about the artifact being produced, answerable by looking
at it, rather than a question about how confident its author feels.

`AGENTS.md` already requires that a session not "introduce a second authority for something
already governed," and that is why the two lists live in one record rather than being repeated
in each item that touches them. An item names the list; it does not copy it.

## Consequences

`KWB-10` (storage semantics), `KWB-11` (the Knowledge IR) and `KWB-12` (knowledge scopes) each
reserve held ground. They are real items and may be claimed, but each one's `done_when` is a
*requirement* record — what the prototype proved is necessary, or what must survive between
stages — and not a design. An item on the held list that produces a schema has exceeded itself.

`KWB-8` is the one item whose gate is now mechanical rather than prose: its precondition was the
prototype inventory, `KWB-9` is that inventory, and the dependency edge exists. Whether this
repository depends on XVPE turns on what the Rust KWB will require and which of those
requirements XVPE is the right authority for under `D-135` — and the second half of that is
still partly on the held list, so closing `KWB-9` does not by itself close `KWB-8`.

This record is revised when the miner's relevant experiments close, by moving entries from the
second list to the first. It is not revised by an item that finds one entry inconvenient.

## Amendment: Two Lists Were Not Enough, 2026-09-12

Measured against the reference miner at xvpe `dev` `a7eee3c6e`, and against this repository's
own closed items. The rule this record exists for is unchanged. What was wrong is the sorting
and, in three cases, the dependency an entry was waiting on.

**A held entry can be in four states, not two.** The original sort had no way to say the
difference, and the difference decides whether waiting is a plan.

### Closed — the miner measured it, and these move to *safe to proceed*

**The evidence ontology, on its verification and provenance axis.** Three states are measured
and distinguished: a quote found in the passage's extracted text; a quote that cannot be
checked because the page's words are pixels; and a quote offered but absent from the passage,
which is a refusal rather than a verification state. The load-bearing lesson is sharper than
the trichotomy: **verifiability is a property of the evidence available, not of the delivery
mode, and the two must not share a predicate.** The miner keyed its check on *"what input must
the model be sent"*, which is true of figure-heavy pages as well as scans, and so exempted
**68 of 111** retained claims from a check they could have passed — carrying a value that said
the passage had no extracted text, which was false for all 68. Re-run under the corrected
predicate: **53 verify, 15 do not.** The conflation was withholding real grounding and hiding
real fabrications at the same time.

**Knowledge scopes, on the question of what a scope belongs to.** Closed by `D-010`, and it
needed no miner input at all — it is forced by `D-002`'s source exclusion, which this
repository decided long before. The promotion *threshold* stays open; see below.

### Pending — the miner is generating input, and the original rule applies unchanged

**The confidence and epistemic-strength model.** The decisive measurement so far: across two
corpora under one contract, the model's self-graded placement held flat at 0.33 against 0.34
while independently *resolved* placement fell from 0.50 to 0.07, with 78% of `named` bearings
resolving to nothing on the larger cell. A grade a source assigns its own assertion is not a
measurement of that assertion's standing. This is exactly the kind of result that would have
made an early answer wrong, which is the argument for the hold working as intended.

**Promotion and authorization**, which `D-010` shows depends on the above.

**Admission.** The miner's screening and abandonment data is input to it, and already carries
one correction: every recorded abandonment is attempt 1 of a configured 4, because
`InferenceError::Is_Retryable` is `matches!(Transport | Throttled)` and a nonconforming answer
is terminal. So a 27–36% abandonment rate is one contract failure per chunk on the only
attempt, not a capability ceiling. **Abandoned work is interpretation debt about the contract,
not evidence about the material.**

### Stranded — waiting on an instrument that cannot answer, with what would unstrand each

An entry here is **not** progressing, and the original wording promised it was.

**Cross-source corroboration.** The miner cannot produce this, by construction and by design.
`SourceClaimIdentity::Derive(source, window, claim)` absorbs **the source path and the page
window** along with the claim, so one claim asserted by two sources is two identities that can
never meet, and `FindingIdentity` is derived from that. This is the deliberate **inverse** of
this repository's own scheme, where `D-002` excludes the source precisely so that two books
asserting one claim become one claim with two citations. Neither is wrong for its own job —
the miner is addressing *where a thing was read*, and KWB is addressing *what a thing is*.
*Unstranded by:* the miner acquiring a second, source-excluding claim identity. That is a
change to the instrument, not a longer run of it.

**Coverage and exhaustion.** Both mining products write an append-only journal that **nothing
reads back**; no `coverage`, `resume` or `redundant` symbol exists in either. So *"nobody
looked"* is indistinguishable from *"looked and found nothing"* — which is the `Barren` versus
`Skipped` distinction this repository already owes `KWB-4`, occurring in the instrument that
was supposed to inform it. *Unstranded by:* a coverage ledger that is read, not another sweep.

**A canonical Knowledge IR.** `D-009` records the measurement and the reasoning. The miner's
experiments concern quote verification, fidelity, grounding, bearing grades and abandonment;
none bears on what representations must survive between *this repository's* ingestion stages,
and no length of run would change that. *Unstranded by:* `KWB-3`, now closed, and `KWB-5`.
Both internal, both reachable without the miner.

### Answered elsewhere — closed by this repository while this record still held it

**Canonical claim identity.** `KWB-1` shipped `ContentIdentity` and `Derivation`, with the
source excluded and the exclusion recorded as a value, and `KWB-15` added the opaque door.
That *is* the canonical claim identity mechanism. It was built while this list said the
subject was held, and nothing bad came of it because the work was the mechanism rather than a
vocabulary — but the record should not have been claiming to hold it.

The lesson is worth more than the correction: **a list of held subjects goes stale in both
directions**, and this one went stale silently because nothing re-reads it against the board.

### Unchanged, and still held

Semantic reconciliation, and the relation kernel wherever prototype evidence is weak — though
`D-011` has now stated what that kernel must guarantee, which is the requirement-record form
this hold has always permitted.

### What this amendment does not do

It does not weaken the rule. *Observation can proceed; a decision whose answer depends on the
open input cannot* survives every measurement above and is what made the stranded entries
findable at all. It designs nothing for any held subject.

And it does not claim the categories are now correct — only that they are now expressible. The
next revision should be triggered by something re-reading this list against the board, because
the failure this amendment corrects was not a wrong judgement. It was a list nobody checked.

## Alternatives Considered

**Blocking all design work until both reconciliations close** was rejected: it would stop
identity mechanics, the write door and this repository's own records — none of which the miner
touches — and it would make the inventory itself illegal to write, since that is design work by
any rule broad enough to catch the rest.

**Leaving the sequencing in `AGENTS.md`** was rejected because `AGENTS.md` is explicit that it
holds how to act safely and not what is true, and a list of held subjects is a claim about the
state of the design, which will change. `AGENTS.md` points at records; it does not carry them.

**Recording the hold inside `KWB-9`** was rejected because the hold outlives that item. `KWB-9`
closes when the inventory exists; the hold persists until the miner's experiments do.
