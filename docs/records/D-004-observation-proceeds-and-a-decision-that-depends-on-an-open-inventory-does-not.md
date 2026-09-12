---
id: D-004
type: decision
title: Observation proceeds, and a decision whose answer depends on an open reconciliation does not
status: accepted
version: 1
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
