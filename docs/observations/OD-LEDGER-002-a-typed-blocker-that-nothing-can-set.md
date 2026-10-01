---
id: OD-LEDGER-002
type: observation
title: The ledger declares six typed reasons an item cannot be worked on, and no path in the tool can set any of them
status: open
version: 1
authority: observation
tags:
  - ledger
  - process
---

# The ledger declares six typed reasons an item cannot be worked on, and no path in the tool can set any of them

Measured on 2026-09-13, while deciding what to do about `KWB-6` — an item three separate
sessions had read, found finished, and put back. Written as an observation because it is a
finding about how work is coordinated here, not a decision about what this repository is.

`D-005` decided this repository adopts `nomos-ledger`'s **schema** and takes **no dependency**
on its code. The board is nonetheless operated by running that tool's binary, so its behaviour
is this repository's operating reality, and its limits are this repository's limits. Nothing
below asserts anything about Nomos, its priorities, or what it should build. Where a remedy
belongs to the tool, what is recorded is what the mechanism would have to offer, not how to
build it — `OD-LEDGER-001`'s framing, kept because the situation is the same one.

---

## The root cause

**The ledger has a typed vocabulary for why an item cannot be worked on, and no way to say it.**

`nomos-ledger` declares `Blocker`, an enum of six variants, in
`crates/substrate/nomos-ledger/src/exclusion/blocker.rs`:

| Variant | What it says |
|---|---|
| `Dependency` | waiting on other items |
| `Decision` | waiting on a decision somebody has to make |
| `TerritoryMismatch` | the territory does not match what the work touches |
| `ExternalResource` | waiting on something outside this repository |
| `NeedsSplit` | too large to claim as one unit |
| `Other` | something else, stated |

This is not an incidental field. Its own doc comment derives the six from `WORK-LEDGER-005`, a
normative accepted corpus requirement, and explains at length why a seventh cause the
requirement names is **declined rather than missing**, citing `OD-LEDGER-017`. It is argued
work. The reason it gives for the enum existing is worth quoting, because it is the reason this
observation exists too:

> A typed reason rather than free prose, so a query can answer "what is blocked on a decision?"
> without matching strings. The prototype stored this as a sentence, and the result was that
> nobody could tell how much of the backlog was waiting on a person versus waiting on a
> dependency.

Every item on this board serialises `blocked`. `nomos work list` takes `--state blocked`. The
audit response carries a `blocked` collection. And:

- Across the **82 items** on this board, **zero** carry a blocker.
- Searched across the whole of `f:/repos/nomos`, `Blocker::` appears **14 times**: once in its
  own doc comment, twelve times in `nomos-ledger`'s unit tests, and once in an integration test
  that assigns `item.blocked` directly.
- The `work` subcommands are `list`, `show`, `add`, `finish`, `claim`, `renew`, `takeover`,
  `abandon`, `decline`, `validate` and `audit`. None of them takes a blocker.

So the field is reachable only by hand-editing `work/ledger.json`, which `AGENTS.md` forbids for
exactly the reason that makes the board trustworthy — *done* is not a thing a session can
assert, and neither should *blocked* be.

`depends_on` is the expression that does work, and it works: ten items here use it, and an item
whose dependency is unfinished is reported `waiting` rather than `ready`. But it orders items
**against each other**. There is no way to say *this waits on something outside the repository*,
which is precisely what `ExternalResource` is named for.

## The symptom: an item three sessions each paid to re-discover

`KWB-6` was authored as *kwb-retrieval and kwb-mcp have no query surface*. That was true when it
was written. It stopped being true once keyword and neighbourhood queries were built, both
worlds became separate types with no write path, and `kwb-mcp` began serving a real corpus by
replaying a publication log. What remained was semantic search, which needs embeddings that
nothing in this repository produces, and which `D-004` holds.

It was abandoned three times:

| Abandoned at | What the reason said |
|---|---|
| unix 1789258287 | two of three query kinds built; semantic needs embeddings and `D-004` holds it |
| unix 1789259352 | everything built except semantic search; blocked on held ground rather than effort |
| unix 1789262095 | everything built except semantic search; do not stub it |

Three sessions read a `ready` capability item, re-derived one finding, wrote it down carefully,
and returned the item to the board in the state that would send the next session after it
again. The abandon reasons were good. They were also invisible: **a board listing shows a title
and nothing else**, and `show` is a second command a session runs only once it has decided the
item is worth reading.

That is the concrete cost, and it is the whole of the cost — no work was done wrong, no claim
went stale, nothing shipped broken. What was spent was three sessions' attention on a question
the board had the vocabulary to answer once.

## What the mechanism would have to offer

A way to **set a blocker through the tool**, and to have it show where a session looks. The
schema already says what; the gap is that nothing says it. Concretely, the two halves:

- **A verb.** Something in the shape of the existing ones — an item, a holder, a typed cause and
  its detail — so that a session which discovers an item is blocked can record *why*, in the
  vocabulary the schema already declares, instead of writing prose into an `abandon` reason
  that the next listing will not show.
- **A listing that does not offer it.** An item with a blocker set should not be reported
  `ready`. `waiting`, `held`, `snagged` and `stranded` already exist for the cases the tool can
  derive; `blocked` exists as a listing filter and can currently only ever return nothing.

Both are about the interface to the board, not about what the board stores. The schema is
right, which is the same conclusion `OD-LEDGER-001` reached about a different gap.

## What this repository does in the meantime

**An item blocked on something outside this repository says so in its title**, because the title
is the only field a board listing shows. `KWB-82` is the first to do it, and reads:

```
BLOCKED ON EMBEDDINGS: semantic search is the only third of the query surface not built, and
nothing here produces a vector
```

It is a workaround and is named as one. A title is prose in the field the schema gave to a
summary, which is the arrangement the `Blocker` doc comment says the prototype had and the enum
exists to replace. This repository is doing the thing that enum was written to stop, because the
alternative is doing nothing.

The same shape applies to the other item this repository is waiting on from outside: the
provider adapter for `kwb-extract` is deliberately not in this workspace, and gates both the
volume work and `D-015`'s reading record. It is not on the board at all, which is the other
available workaround and a worse one — work that is waiting is at least visible.

## A second gap, found while writing this

This observation declares **no relation** to any record, and that is deliberate rather than an
omission.

`tests/contract` checks that a declared relation is reachable from both ends, and that no
record's *Referenced By* names something that declares no relation back. Both read
`docs/records/` only. So an observation's declared relation is checked in **neither** direction,
and adding `OD-LEDGER-002` to a record's *Referenced By* would fail
`Test_No_Record_Should_Claim_A_Reference_Nobody_Declared` — not because the reference is wrong,
but because the guard cannot see the file that declares it. A guard failing for the wrong reason
is worse than one passing for the wrong reason, so this is recorded rather than worked around.

`OD-LEDGER-001` does declare `relates-to D-005`, and `D-005` does not list it back. Nothing
fails, because nothing looks. The observations directory is routed by `AGENTS.md` and is
unguarded.

## What is owed

- **The remedy belongs to the tool**, and `D-005` is why this repository does not reach for it.
  This observation is evidence that would reopen that decision if the cost grows; one item read
  three times is not that, and is deliberately not reopening it now.
- **Two obligations, stated separately** so neither can hide the other — `KWB-80`'s finding, and
  this is the first record written after it: (1) a verb that sets a typed blocker, and (2) a
  listing that stops reporting a blocked item as `ready`. Either is useful without the other.
- **Observations are unguarded.** Whether the relation checks should read `docs/observations/`
  as well as `docs/records/` is a question this observation raises and does not answer, because
  answering it is a change to `tests/contract` and not a finding about the board.
