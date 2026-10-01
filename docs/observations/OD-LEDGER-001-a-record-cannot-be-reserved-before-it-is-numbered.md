---
id: OD-LEDGER-001
type: observation
title: A record cannot be reserved before it is numbered, so decision items reserve the directory and the board serialises
status: open
version: 2
authority: observation
tags:
  - ledger
  - process
relations:
  - target: D-005
    type: relates-to
---

# A record cannot be reserved before it is numbered, so decision items reserve the directory and the board serialises

Measured on 2026-09-12, across one session that closed `KWB-2`, `KWB-8`, `KWB-10`, `KWB-11`,
`KWB-15`, `KWB-16` and `KWB-18`. Written as an observation because it is a finding about how
work is coordinated here, not a decision about what this repository is.

`D-005` decided this repository adopts `nomos-ledger`'s **schema** and takes **no dependency**
on its code. The board is nonetheless operated by running that tool's binary, so its
behaviour is this repository's operating reality, and its limits are this repository's limits.
Nothing below asserts anything about Nomos, which is a sibling this repository does not
govern. Where a remedy belongs to the tool, what is recorded is what the mechanism would have
to offer, not how to build it.

---

## The root cause

**An item that will write a decision cannot name the decision it will write, because the
record has no identifier until it is authored.**

The tool reserves at file granularity and says so. Asked to reserve a record file directly it
answers:

```
docs/records/d-004 is already published as docs/records/D-004-....md. If this item is a
new decision, a record identifier is allocated once — choose the next free one. If it
amends that record, say so with `--amends docs/records/D-004-....md`, which reserves the
file it edits
```

So file granularity exists, and it exists for **records that already exist**. `--amends`
reserves a published file. There is no counterpart for a record that will be published by the
item being authored. An item whose `done_when` is *"a record here states…"* has nothing to
name, and the only reservation left to it is the containing directory.

That is why five of this board's items reserve `docs/records` wholesale. It is not
carelessness in the authoring; it is the only expression available.

**What the mechanism would have to offer:** a way to allocate the next free record identifier
*at item-authoring time* and reserve it, so that an item which will write `D-010` reserves
`D-010` and nothing else. Allocation and reservation are the same act for an amendment
already — the tool's own message says an identifier is allocated once — and the gap is that
this act is available only in the direction that looks backwards.

## The symptom: the records directory is a global lock

`add` refuses a new item whose territory overlaps an **open** item. Not a claimed one — an
open one. Measured:

```
$ nomos work add --item KWB-17 --amends D-004 ...
docs/records/d-004 is already reserved by KWB-3, which is open. Choose another
identifier, or retire that item if it is not work
```

`KWB-3` was `ready`, held by nobody. The item being added was a correction to `D-004` that
**`D-004` itself mandates** — it says in as many words that it is revised when its held list's
inputs close. The board could not accept the item its own record calls for.

The reservations at the time:

| Item | State | Territory |
|---|---|---|
| `KWB-3` | ready | `crates/domain/kwb-domain`, `docs/records` |
| `KWB-8` | ready | `docs/records`, `AGENTS.md` |
| `KWB-10` | ready | `docs/records`, `docs/corpus` |
| `KWB-11` | ready | `docs/records`, `docs/corpus` |
| `KWB-12` | ready | `docs/records`, `docs/corpus` |

Claiming any one of them moved the other four to `held`, which is the lease working exactly as
designed and is not the problem. The problem is the layer above: **while any decision item is
unclosed, no new decision item can be proposed at all.** A board whose purpose is to hold work
that has been noticed cannot hold work noticed while other work is open.

The refusal's suggestion — *"retire that item if it is not work"* — inverts it. `KWB-3` is
work, and it is one of the most substantial items on the board.

**Aggravating factor: territory cannot be narrowed after authoring.** There is no `amend`, and
`decline` exists for an item that turned out not to be work and would discard the authored
prose. So an over-broad reservation, once written, is permanent for the life of the item.

**What the mechanism would have to offer:** either allocation-time reservation as above, which
removes the reason to reserve the directory, or a correction path for an item's territory that
is not `decline`.

---

## Three smaller findings, measured the same day

### `work show` does not show three fields a session needs, and this section used to say something stronger and wrong

**Corrected 2026-09-12, after 23 further items.** This section previously said the tool
*"advertises prose and prints none"*. That is false. Asked about `KWB-6` it returns all three
abandon reasons, each several hundred words, in full.

What it shows is exactly what its help enumerates — the claim, every claim given up on with the
reason given, and the verification. The phrase *"one item in full"* is what oversells it; the
implementation does what the list says. **The first version of this section blamed the
implementation for the summary line and overstated the defect, in a document whose whole subject
is a tool advertising more than it does.** Left visible rather than quietly restated, for the
reason this repository leaves every superseded claim visible.

The narrower statement is also the more useful one. Three fields are not shown, and each is
something `AGENTS.md`'s own loop asks a session to use:

| Not shown | Which step needs it |
|---|---|
| `why` | 4 — *read only the authorities your item needs* |
| `territory` | 5 — *implement inside your territory, and nothing else* |
| `done_when`, and the predicate | 6 — *run the item's predicate yourself* |

So the loop asks for three things the tool that is supposed to be the interface to the board
cannot supply, and a session must read `work/ledger.json` directly to get them — a second way to
read the board, which is what a board exists to prevent.

### Abandon reasons accumulate, and every one of them is shown

Recorded because it is a strength, and because it was not known when this document was written.

`KWB-6` has been claimed and given up three times, each with a statement of what had been built
and what was left. `show` prints all three, oldest first, so a reader sees not only where the
item stands but how its remaining scope narrowed. Nothing replaced anything.

That bears on this document's own subject. The root cause above is about work the board cannot
express; this is the board expressing something well — a partial result handed forward in the
item's own record, rather than in a commit message somebody has to know to go looking for.


### `--amends` refuses the identifier a record declares for itself

```
$ nomos work add --amends D-004 ...
d-004 is declared as an amendment and no record here carries it.
```

`docs/records/D-004-....md` exists and its frontmatter reads `id: D-004`. Only the path
spelling resolves. The identifier is how every record in this repository cites every other —
`D-130`, `ARC-ECOSYSTEM-001`, `D-006` — so the single place the canonical spelling is rejected
is the place it is most natural to type. The lowercasing visible in the error suggests the
lookup normalises the input and then matches it against something that is not normalised the
same way.

### The board reserves ground and does not defend it

Nothing prevents a session writing into territory it does not hold. Two slips in this session,
both the same shape — write the file, then author the item that reserves it:

- `README.md` was edited while holding `KWB-8`, whose territory was `docs/records` and
  `AGENTS.md`. `KWB-18` was authored afterwards to cover ground already touched.
- `D-009` was written before `KWB-11` was claimed at all.

Both are recorded in their commits rather than tidied away. The point is not that a session
was careless twice; it is that **`AGENTS.md` states the territory rule as though it were
enforced**, and a reader has no way to tell from the tool that it is advisory. A lock that
cannot be violated and a convention that can are different things to build a process on, and
this one is the second while reading as the first.

This is the honest limit of the model rather than a defect to fix: the ledger coordinates
sessions that cannot see each other, and it does so through a file. It cannot arbitrate a
filesystem. Worth writing down so that the guarantee is not over-read.

---

## What works, recorded so the findings are not read as a verdict

`finish` runs the gate's lint step, derived from `.github/workflows/gate.yml` rather than
restated in the ledger, then the item's own predicate, and records the item done only if both
exit zero. An item whose predicate passes while the gate is red does not finish. That closes
the gap this repository's `KWB-13` was written for and it is the single most valuable property
the board has: *done* is not a thing a session can assert.

**Measured again since.** It has now refused a finish twice more — `KWB-5` and `KWB-34`, both on
clippy errors the session had not run yet, both reported as *"the work may be right and still
cannot land"*, with the item's own predicate deliberately not run. Three refusals across 43
items is the property working rather than a rough edge.


The lease works. Claiming one item moved four overlapping items to `held` immediately, and
`takeover` exists for a lapsed claim and records what it displaced.

The schema is right. Every finding above is about the interface to the board, not about what
the board stores.

## What is owed

- The remedy for the root cause belongs to the tool, and `D-005` is why this repository does
  not reach for it: it adopted the schema and deliberately took no dependency on the code.
  This observation is the evidence that would reopen that decision if the cost of the gap
  grows, and it is deliberately not reopening it now — one session's friction is not a reason
  to take on 14,008 lines.
- In the meantime, items authored here should reserve the narrowest path they can, and an item
  that will write a record should expect to reserve `docs/records` and to block the board's
  other decision work while it is open. Knowing that is the mitigation.
- `D-004`'s held list is owed a correction that this session could not author, for exactly the
  reason above. `D-009` carries the evidence for one entry of it.
