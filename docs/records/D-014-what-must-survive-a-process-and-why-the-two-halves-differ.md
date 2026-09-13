---
id: D-014
type: decision
title: What must survive a process, and why the two halves are not the same question
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - storage
  - platform
relations:
  - target: D-012
    type: relates-to
  - target: D-008
    type: relates-to
---

# What must survive a process, and why the two halves are not the same question

## Decision

`D-012` left durability open and said it is a **backend rather than a schema**. That holds.
What this record adds is that there are **two** things to make durable, they are lost in
different ways, and a single answer covering both would be answering two questions with one.

- **The document store is a cache with a correctness role.** Its contents are perfectly
  reproducible from the sources, so losing it costs *work and verifiability*, never
  information — as long as the sources are still there. It goes to a **content-addressed file
  store**: one file per address, no index, no database.
- **The graph is not reproducible at all.** What it holds came from an interpretation of a
  passage, and re-reading will not reproduce it. Losing it loses *information*. It is made
  durable by an **append-only record of what was published**, replayed to reconstruct.

Both go through a port trait in `kwb-platform`. No library and no serialization format is
named, because nothing measured here forces one.

## The measurement

`kwb admit` (`KWB-26`) is the consumer `D-012` said this needed, and it loses everything on
exit. Run against one file on 2026-09-12:

| | |
|---|---|
| the same file admitted twice | `6b3433c3…1430` both times — **byte-identical address** |
| the same file, the claim reworded | same source address, **different claim** |

The first row is the document store's whole story: content addressing makes a re-admitted file
the same document, so re-running reconstructs it exactly. The second row is the graph's: the
claim is a function of *what somebody said the passage asserts*, not of the passage, and a
different reading gives a different claim with a different identity.

And the reading is not deterministic. The reference miner measured every recorded abandonment
as one schema failure on the first and only attempt — `InferenceError::Is_Retryable` is
`matches!(Transport | Throttled)`, so a nonconforming answer is terminal — which means a model
re-reading the same passage may not even return a well-formed answer, let alone the same one.

**So the asymmetry is not stylistic. One half is a derivation that can be repeated and the
other is a judgement that cannot.**

## The document store: content-addressed files

An address is the full 32-byte digest of the content, rendered as 64 lowercase hexadecimal
characters (`kwb-model`). That is already a filename, and a content-addressed store needs
nothing else:

- **No index.** The address *is* the lookup. An index would be a second thing to keep in step
  with the first, and `D-008`'s first requirement is what a second copy of a rule costs.
- **No overwrite path.** An address is only ever offered the content it names, so a write to an
  address that exists is either the same bytes or a preimage break. `kwb-store` already refuses
  the second and reports the first as `AlreadyPresent`.
- **Writes must be atomic.** Write to a temporary name and rename into place. A half-written
  file under a content address is the one way this store can lie: the address promises the
  bytes, and a truncated file breaks that promise silently, because nothing downstream re-hashes
  what it reads. **This is the requirement most likely to be got wrong**, and the reason it is
  called out rather than left to whoever implements it.

What the port must offer: create a directory, write bytes to a temporary path, rename, read,
test existence. Nothing else.

**What this does not buy.** Durability here is not a guarantee about the world. If the source
file is gone *and* the store is gone, the document is gone; content addressing makes the store
reconstructible, not the corpus. Whether sources are themselves archived is a different
question and this record does not answer it.

## The graph: an append-only record, replayed

A version is a value and a new version shares structure with the old one (`D-012`). Persisting
every version would be persisting the whole history as state; persisting the **transitions** is
smaller and is the same information, because the graph is a fold over what was published.

The prototype reached this and recorded it in its own `ROADMAP.md`, in the part listing what it
deliberately would *not* build:

> Full event sourcing for every entity — **an append-only derivation ledger is the right
> scope.** Event sourcing everywhere is "probably excessive".

So: an append-only record of publications, and the graph at any point is what replaying it up
to that point produces. That makes `D-008`'s temporal requirement fall out — the graph as of an
instant is a prefix of the record — rather than needing a second mechanism.

`xvpe-event-journal` is the shape, already extracted, and `D-012`'s adoption route applies.
**This record does not adopt it**, because adopting needs a second measurement nobody has made:
whether a record written for a *mining run* fits a *knowledge* publication. That is the sort of
assumption `D-004` exists to stop, and it is cheap to check later and expensive to unwind.

What the port must offer: append a record and read the records in order. Both are the same
filesystem surface the document store needs.

## What is deferred, and the condition for each

- **Adopting `xvpe-event-journal` rather than writing an append.** *Settled by* measuring its
  record shape against a publication, which nothing has done.
- **Compaction.** A replay is linear in publications, and nothing here has enough publications
  for that to matter. *Settled by* a corpus where replay time is measurable — which is the same
  kind of condition `D-012` used, and for the same reason: a performance decision taken before
  anything is slow is a guess with a schema.
- **Concurrent writers.** One process, one CLI. `kwb-platform` reserves a lock port and nothing
  claims it. *Settled by* a second writer existing.

## The warning that attaches to whoever implements this

Repeated from `D-008` because it is the one thing that cannot be caught afterwards. The
prototype's ranked retrieval re-derived its vector per query and measured **38 ms against
1.3 ms** at 7,914 chunks — and **both forms returned byte-identical rows**, so no correctness
test could ever have seen it.

A storage shape's cost is invisible to the tests that say it is correct. **A choice made here
cannot be validated by the tests passing.**

## Consequences

- `kwb-platform`'s traits are now owed something concrete: a filesystem port serving both
  halves, and a clock for the record's own timestamps. Its current state is no traits at all.
- `kwb-store` acquires a durable implementation behind its existing door, which is a new
  implementation behind an existing seam and not a change to the seam — the argument
  `kwb-platform` already makes about itself.
- `D-008`'s durability requirement is answered for the first time, and `D-012`'s only open half
  closes. The seven requirements `D-012` answered are untouched.

## Alternatives Considered

**One mechanism for both halves.** Rejected on the measurement: a store whose contents are
reproducible and a graph whose contents are not have different failure costs, and a single
answer would either over-engineer the first or under-protect the second.

**Persisting graph versions rather than transitions.** Rejected as persisting a fold's
intermediate results instead of its inputs — larger, and it loses the temporal read that falls
out of a prefix for free.

**A database for the graph.** Rejected by `D-012` already, on the prototype's own evidence:
91,527 of its 151,392 `src/` lines are generated migrations, and the requirements that
motivated the database are met more strongly by a value that cannot be queried wrongly.

**Deferring until there is a second writer or a large corpus.** Rejected: those conditions gate
*compaction* and *locking*, which this record defers for exactly that reason. They do not gate
whether anything survives at all, and `kwb admit` currently loses its work every time it runs.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that no record declares a relation to fails too. A
relation is declared in the frontmatter of the record that makes it; this is the other end, so
that a reader of this record can reach the ones that answer, amend or build on it. Before
`KWB-38`, 24 of 27 relations were reachable from one side only — which is how three records
came to assert things this repository had stopped doing.*

- `D-015`
