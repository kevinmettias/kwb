---
id: D-013
type: decision
title: The work ledger is shared coordination, D-005's deferral condition has been met, and this repository's position is that it belongs to XVPE
status: accepted
version: 2
authority: canonical-normative-record
tags:
  - ecosystem
  - ledger
  - platform
relations:
  - target: D-005
    type: relates-to
  - target: D-012
    type: relates-to
---

# The work ledger is shared coordination, and this repository's position is that it belongs to XVPE

## Decision

`D-005` deferred owning any ledger code, and it named the condition that would decide the
question rather than leaving it open-ended. **That condition has been met.** This record states
this repository's position on what follows, and deliberately stops short of deciding it.

The position: **a work ledger is a shared coordination mechanism, it is neither knowledge nor
software-engineering analysis, and under `D-135` it is XVPE's.**

This record decides nothing about Nomos and moves no code. `AGENTS.md` routes the question of
what each product owns to `ARC-ECOSYSTEM-001` in `f:/repos/nomos`, and this repository does not
restate what it does not own. What it can do is put a measured position in front of that
decision, which is what this is.

## Why the question is open again

`D-005`'s reasoning was specific, and worth quoting rather than paraphrasing because the part
that has changed is a fact and not a judgement:

> `D-135` is the wrong test for `KWB-7` — it governs where *new* domain-neutral code is
> authored. `nomos-ledger` is 14,008 existing lines, so `D-122` / `ARC-ECOSYSTEM-001`'s
> **two-product proof** governs.

At the time there was one product. **There are now two.** This repository operates its board by
running that binary from this root, every day, and has closed twenty-one items through it. The
two-product proof is not an argument anybody has to make; it is a thing that happened.

`D-005` was right when it was written and is right about everything except its premise, which is
the good kind of superseded.

## What the current arrangement costs, measured

`docs/observations/OD-LEDGER-001`, 2026-09-12. The findings are there and are not restated here;
what matters for *this* decision is their shape.

The root cause is that **an item cannot reserve a record it has not numbered yet**, so every
decision item reserves the whole `docs/records` directory, and `add` refuses any new item
overlapping an open one — meaning **while any decision item is unclosed, no new decision item
can be proposed at all.** In one session that blocked a correction to `D-004` that `D-004`
itself mandates, for several hours, behind an item nobody was holding.

The point is not that this is a serious defect. It is that **it is nobody's to fix.**

- This repository cannot fix it, and `D-005` is why, correctly: the code is not ours.
- Nomos has no reason to prioritise a constraint it does not feel. Its own items are authored
  by people who know its record numbering, and the friction lands here.

A shared mechanism with no owner who can change it accumulates exactly this: small, real,
unfixable-by-anyone costs. That is an ownership problem wearing a tooling problem's clothes, and
no amount of care in authoring items here will touch it.

## Why it is XVPE's rather than either product's

A work ledger answers *what work exists, who holds it, what territory it reserves, and what must
pass before it is done*. Nothing in that sentence is about knowledge, and nothing in it is about
software-engineering analysis. Two products already need it and a third would.

**`xvpe-event-journal` is the worked precedent, and it points the same way.** It was extracted
from the reference miner on 2026-09-10 because *"the part that does not know what is being
journalled"* was sitting in a tool above the engine where nothing else could reach it. The event
names stayed with the run that has them; the open-append-flush mechanism moved. A ledger splits
along the identical seam — the schema, the lease, the territory arithmetic and the gate
derivation are the mechanism; which items exist and what they mean is each product's own.

`D-012` decided the second store the same way and for the same reason, so this is not a new
principle being introduced for one case. It is the third time in two days that the answer to
*"where does this belong"* has been *"XVPE owns the mechanism, the product owns the catalogue"*.

## What this repository would consume, and what it would keep

**Consume:** the schema, the state machine, the lease and take-over, territory reservation and
overlap arithmetic, and the gate derivation — `finish` running the workflow's own lint step
before the item's predicate and recording done only if both exit zero. That last one is the
single most valuable property this board has and the one this repository would least want to
re-implement.

**Keep:** `work/ledger.json` and everything in it. The items are this repository's own and are
not portable; `KWB-5` means nothing in another workspace.

**Adoption is by git reference and commit SHA into a quarantining crate, never by `path`** —
`D-007`, unchanged. With the qualification that a ledger is today operated as a **binary run from
a repository root**, not linked as a library, so "adoption" may not mean a dependency edge at
all. That distinction is worth settling wherever this lands, because the two have very different
costs and `D-007` was written about the second.

## Consequences

- `D-005` is not repealed here. Its decision — adopt the schema, take no dependency — remains
  this repository's operating reality until something upstream changes, and nothing in this
  record permits taking a dependency today.
- `ARC-ECOSYSTEM-001` is owed this. The evidence is `OD-LEDGER-001` plus the fact of a second
  product, and both are written down rather than argued.
- If the answer upstream is that the ledger stays with Nomos, then `OD-LEDGER-001`'s findings
  should travel with it, because they are the cheapest available list of what a second consumer
  needed and could not ask for.
- **The same question is owed for the evidence vocabulary.** `D-004` v2 moved the evidence
  ontology's verification and provenance axis to *closed*, and the vocabulary that closed it —
  quote verification, claim grounding, the fidelity stratification — lives in
  `crates/apps/tools/xvpe-reference-miner`, above everything, reachable by nothing. That is the
  same shape as the journal before it moved, with this repository as the named second consumer.

## Alternatives Considered

**Reopening `D-005` and taking on ledger code here.** Rejected. One session's friction is not a
reason to adopt 14,008 lines, and it would make this repository the owner of a mechanism two
other products want — which is the failure `D-135` exists to prevent, in the opposite direction.

**Writing a small ledger of this repository's own.** Rejected as worse than either alternative:
it would give the ecosystem three schemas, and the property most worth having — that *done* is
not a thing a session can assert — is the part that took real work to get right.

**Filing the findings as a Nomos issue and leaving it there.** Rejected as insufficient rather
than wrong. The findings are symptoms; the decision they point at is about ownership, and an
issue against the current owner cannot raise that.

**Saying nothing until asked.** Rejected: `D-005` named a condition, the condition has been met,
and a record whose trigger fires silently is `D-004`'s own lesson from two days ago — a list
nobody re-reads goes stale in both directions.

## Amendment: The Evidence Vocabulary Moved, 2026-09-12

The closing note says the evidence vocabulary *"lives in
`crates/apps/tools/xvpe-reference-miner`, above everything, reachable by nothing"*, and offers
it as the next question of this record's shape.

It was extracted the same day, to `xvpe-evidence` in XVPE's `foundations/contracts` — `no_std`,
no dependencies, with `QuoteVerification` and `ClaimGrounding` moved and the bearing grade and
the combined confidence deliberately left behind, because a bearing is a fact about a claim and
a *codebase* and a knowledge base has none. This repository is named there as the measured
second consumer.

So the question this record raised has been answered in the direction it pointed, which does not
change what it decided about the ledger.
