---
id: D-012
type: decision
title: The second store is not a database, and most of it is not this repository's to build
status: accepted
version: 5
authority: canonical-normative-record
tags:
  - storage
  - ecosystem
  - platform
relations:
  - target: D-008
    type: amends
  - target: D-007
    type: relates-to
  - target: D-005
    type: relates-to
---

# The second store is not a database, and most of it is not this repository's to build

## Decision

`D-008` found that the bands table names one store where the prototype's evidence describes
two, and stopped at naming the gap because deciding it needed the domain types. `KWB-3` and
`KWB-4` have closed, so it is decided here.

**KWB does not need a database.** Not as an aspiration — as a consequence of three things this
ecosystem already has: content-derived identity, persistent data structures, and an append-only
journal. Seven of `D-008`'s eight requirements are met, met differently, or routed elsewhere,
and the one genuinely open question is **durability**, which is a backend and not a schema.

**Most of the mechanism belongs to XVPE, and most of it already exists there.** This repository
owns the catalogue: what a concept, claim and assertion are, and what it means for one to be
current. It does not own versioned state, and should not write one.

## The measurement this rests on

Against xvpe `dev` `a7eee3c6e`, 2026-09-12.

**`xvpe-collections-persistent`** ships `HamtMap` and `RrbVector`, and describes itself exactly
as the requirement needs: *"operations return a new version that shares most of its internal
nodes with the previous version. The old version is unchanged and remains valid; **both versions
can be queried independently**."* Update cost is logarithmic in depth, not size.

**`xvpe-event-journal`** is an append-only record, one JSON object per line — and it was
**extracted from the reference miner's `RunJournal` on 2026-09-10** precisely because the part
that does not know what is being journalled was trapped above the engine where nothing else
could reach it. The prototype's own `ROADMAP.md` deferred full event sourcing and said *"an
append-only derivation ledger is the right scope"*; this is that, already built, already shared.

Adoption cost, measured with `cargo tree --no-default-features -e normal`:

| Crate | Third-party in its closure |
|---|---|
| `xvpe-collections-persistent` | `hashbrown`, `smallvec` |
| `xvpe-event-journal` | `web-time` (via `xvpe-clock`) |

**Three third-party crates.** More than the zero `D-007` measured for the ingestion candidates,
and recorded plainly rather than elided: this is a real cost and it is small, and all three are
ordinary, widely-vendored, `no_std`-friendly crates. `web-time` is a `wasm` shim for
`std::time::Instant`, which this repository has no use for and inherits anyway.

## The eight requirements, answered

### 3 and 4 — which-world reads, and temporal reconstruction — **met, and met structurally**

This is the strongest result and the reason the rest follows.

`D19-B` was a global query filter that rewrote every query, including ones written by someone
who had never heard of it, so `merge-audit` resolved none of the merge log's identifiers,
printed *"nothing has been merged away"* and exited `0`. The prototype's answer was two
interfaces and a discipline about which one you depend on.

A persistent structure makes the discipline unnecessary. **A version is a value.** The current
graph and last Tuesday's graph are two values a caller holds, both queryable, and there is no
filter to bypass and none to forget. `IConceptRepository` versus `IConceptHistory` stops being
two interfaces over one mutable store and becomes *which value you were handed*.

`KWB-6` asks for current-version and all-versions reads as **distinct types rather than one type
with a flag**. Over a persistent structure they are distinct *values* of a versioned type, which
is stronger than distinct types — a flag cannot be forgotten because there is no flag, and a
caller cannot ask the wrong world because it does not hold it.

Temporal reconstruction (`AsOf`) is the same mechanism: the version published at an instant is a
value that was kept, not a query that opts out of a filter.

### 1 and 5 — one liveness expression, and refusing contradictory states — **KWB's, in types**

Both are domain rules, not storage mechanisms, and both stay here.

Liveness must be **one expression**, and this repository has the harder version of that problem:
the prototype's index enforced half the rule for three weeks and nothing failed, because a
half-rule is a weaker constraint. It is owed, and it is `KWB-4`'s successor rather than this
record's.

**Discharged.**

**Condition met:** `KWB-5`

`KWB-4`'s successor built it. `Standing::Is_Current` is that one expression — a single definition, and `kwb-domain/tests/one_liveness.rs` scans this crate's own
source and fails on a second. The composed form is the graph's: a claim is current when its own
standing is current **and its concept's is**, which `KWB-65` found a renderer getting wrong and
is the half-rule this paragraph warns about, caught rather than shipped.

Refusing contradictory states was six SQL `CHECK` constraints in the prototype. Here it is what
this repository already does twice: `Coverage` cannot represent *never ran* as *ran and found
nothing* without fabricating a number, and `Written` cannot exist without the bytes it
describes. A constraint in the type is checked everywhere; a constraint in the schema is checked
by the one store that has it.

### 2 — uniqueness under liveness — **deferred, and it is not a storage question here**

The prototype needed a partial unique index because the database was the arbiter of which
concepts exist. Here identity is derived from content, so two concepts cannot collide on
identity, and two *names* colliding is a question about whether they are the same concept —
which `D-011` routed to the relation vocabulary's decision procedure. Nothing a store can
answer. *Deferred to `D-011`'s successor.*

### 6 — idempotent admission, and the lease — **half met, half routed away**

Idempotence is **already met and met better**. Content addressing makes a re-offered document
one document with no caller-supplied key, and `Admission::AlreadyPresent` says so. An
idempotency key can be forgotten; content cannot.

The **lease** is not a storage requirement at all. It belongs to the work ledger, and that is
shared between this repository and Nomos — which makes it `D-135`'s subject and not this
record's. *Routed to the ledger's own decision; declined here.*

### 7 — ranked retrieval from a stored vector — **deferred, with the warning attached**

No embeddings exist in this repository and nothing here should invent them. The requirement
stands and the deferral is explicit.

`D-008`'s warning attaches to whatever settles it, and is repeated because it is the one that
cannot be caught later: `ts_rank_cd` over an expression index measured **38 ms against 1.3 ms**
at 7,914 chunks, and **both forms returned byte-identical rows**. A retrieval shape's cost is
invisible to every correctness test. *A choice made here cannot be validated by the tests
passing.*

### 8 — delete-and-write atomicity — **met by absence, twice**

In `kwb-store` there is no overwrite, so there is no pair to make atomic. In a persistent
structure there is no in-place edit: a new version is published or it is not. `ReplaceAllAsync`
deleting unconditionally and writing conditionally is not a mistake either mechanism can make.

### Durability — **the one genuinely open question**

`kwb-store` holds documents in memory for the length of a process, and a persistent structure
holds versions the same way. Nothing here survives the process.

This is deliberately not decided, and the reason is that it is a **backend** question. The
Nomos precedent is the shape: XVPE owns the mechanism and the backend crate beside it, and the
product owns its catalogue and names no library at all. *Settled by:* an item that decides where
durable bytes land, which needs a consumer that actually loses something when the process ends —
and nothing here has one yet, because nothing here runs for longer than a test.

## Ownership, and how adoption happens

**XVPE owns the mechanisms.** Versioned state and the append-only journal are not knowledge; a
renderer and a linter want the same two things, and `xvpe-event-journal` demonstrates the point
by having come out of the miner rather than out of KWB.

**This repository owns the catalogue**: what a version contains, what makes a concept current,
and what an assertion is.

**Adoption is by git reference and commit SHA, into a crate that quarantines it, never by
`path`** — `D-007`, unchanged and load-bearing here. The workspace manifest already reserves
`kwb-platform-xvpe` for exactly this, and creating it is an item rather than a side effect of
this record.

## Consequences

- **`KWB-6` is unblocked, and its requirement changes shape.** It should be built against a
  version *value* rather than two types over one store. Building it before this record would
  have produced two types that differ in name and not behaviour.
- **`D-008`'s two-stores finding stands and is now answered.** The second store is a persistent
  graph value plus a journal, not a database, and most of it is adopted rather than written.
- **The bands table needs a row** for whatever quarantines the XVPE dependency, and `README.md`
  currently says no crate here depends on XVPE. Both are corrections owed once the crate exists,
  not before.
- **Three third-party crates enter this repository's closure** when adoption happens. Recorded
  so that the first person to see `hashbrown` in a lockfile finds the decision rather than a
  surprise.

## Alternatives Considered

**A database.** Rejected on the prototype's own evidence rather than on taste: 91,527 of its
151,392 `src/` lines are generated migrations, and the requirements that motivated it —
which-world reads, temporal reconstruction — are met more strongly by a value that cannot be
queried wrongly than by a filter that can be bypassed. The requirement a database would still
answer is durability, and that is a backend, not a schema.

**Writing a versioned graph here.** Rejected as the thing `D-135` exists to prevent, and as
waste: `xvpe-collections-persistent` ships the structures with `O(log n)` updates and structural
sharing, and this repository would be writing a worse one to avoid three dependencies.

**Deferring the whole decision until durability is settled.** Rejected because durability is the
*only* open part, and holding the other seven behind it is what left `KWB-6` looking ready while
being unbuildable.

**Taking the mechanisms by `path` since they are measured clean.** Rejected by `D-007`, whose
reasoning is untouched by these crates being small: a `path` edge binds this repository's
reproducibility to another repository's working tree, and that tree moved by five crates in a
day.

## Amendment: Durability Is No Longer Open, 2026-09-12

The section above calls durability *"the one genuinely open question"* and says nothing here
survives the process. Both were true when written.

`D-014` settled it, in the sequence this record asked for: it named the condition — a consumer
that actually loses something when the process ends — and `KWB-26` built that consumer before
the decision was taken. The answer is two mechanisms rather than one, because the two halves are
lost differently: documents to a content-addressed file store written temp-then-rename, and the
graph to an append-only record of publications, replayed. `KWB-30`, `KWB-33` and `KWB-34` built
them.

What this record decided is untouched. Only its open question has closed.

## Amendment: Requirement 4 Was Met In Mechanism And Unreachable, 2026-09-12

The section above puts requirements 3 and 4 under **met, and met structurally**, and says
temporal reconstruction is the same mechanism as a which-world read: *the version published at
an instant is a value that was kept*. `D-014` then said the temporal requirement **falls out**,
the graph as of an instant being a prefix of the record.

Both were true about the mechanism and false about the repository.

**Measured 2026-09-12.** `Replay` was called in five places and every one passed the whole log.
Nothing took a prefix, so no user could reach the graph as of anything. *Falls out* described
what the shape permits; it was read as something the repository does, which is the one-crate-away
gap this repository has now found four times — here inside a canonical record, as a completed
requirement.

`KWB-60` built the consumer: `kwb history --store <dir> [--through <count>]` replays a prefix
through the same `Replay` every other caller uses, and refuses a count past the end of the log
rather than quietly returning everything.

**And *an instant* is the wrong word, which matters more than it looks.** A publication record
carries a kind, a standing, a successor, a reason and the entity, and **no time at all**. This
record's own consequences say `kwb-platform` is owed *a clock for the record's own timestamps*,
and `kwb-platform`'s module documentation says the clock, lock and process ports are not here
yet. So an instant cannot be asked for in any form. What a prefix answers is **as of the first N
publications**, and that is what the command offers.

That is not a lesser thing in the case it was wanted for. `D19-B` is the incident where a merge
audit resolved none of the merge log's identifiers and printed *"nothing has been merged away"*.
`kwb-mcp` answers `merge_losers`, which says what was merged away; this says what the graph
looked like before it. Together they are the audit that incident could not perform.

*Settled by, for a real instant:* the clock port. A time on a record needs one, and a timestamp
invented without a clock is a field that reports the good case until somebody notices — `D20`.

**Condition met:** `KWB-64`

`KWB-64` adopted it rather than declaring one. A clock has no knowledge-domain semantics — its
definition never mentions a claim, a concept or a source — so `D-135` puts it in XVPE and XVPE
has one; `kwb-platform-xvpe` re-exports `xvpe-clock`'s as `PublicationClock` and its hosted
implementation as `SystemClock`, the way it already re-exports the persistent map. No clock port
was declared here, because declaring a second authority for something XVPE owns is the defect
this repository spent `KWB-49`, `KWB-55` and `KWB-61` removing from three other places.

A publication now carries the time it was published, and `kwb history --as-of <unix seconds>`
answers a real instant. **So the sentence above — that an instant cannot be asked for in any
form — is no longer true**, and the paragraph it sits in should be read as the state before
`KWB-64` rather than as a description of this repository.

The time is a *trailing, optional* field. `Replay` dispatches on how many fields a record has,
so appending kept every log written before `KWB-64` replayable, and `Option` rather than a zero
keeps *unknown* distinguishable from *a value* — the distinction `Coverage` is this
repository's long argument about. A log in which nothing is timestamped refuses `--as-of`
rather than answering it, because every answer would otherwise be the same answer whatever was
asked.

What this record decided is untouched. Requirement 4's answer is narrowed to what it always
was, and now has a consumer.

## Amendment: The Database Claim Is Narrower Than It Reads, 2026-09-13

This record's decision opens **KWB does not need a database.** That is stronger than the evidence
below it supports, and left alone it becomes an ideological rule rather than a finding.

**What the evidence supports** is narrower and survives scrutiny: the prototype's EF and Postgres
architecture was **not itself a requirement**, and the requirements that justified it — which-world
reads, temporal reconstruction, uniqueness, idempotence, atomicity — are currently met more
strongly by this design than they were by that one. Every one of those is answered above on the
prototype's own measurements, and that argument is untouched.

**What it does not support** is a permanent answer. Rejecting 91,527 lines of generated migrations
is a verdict on *that* architecture reached for *those* requirements, not a standing position
against a class of tool. A repository that reads its own rejection as *never* would meet the next
real requirement with an argument instead of a measurement.

*Reopened by:* a requirement this design does not meet — scale that makes a linear replay
expensive, concurrent querying, analytics over the corpus, or retrieval pressure once embeddings
are real. `D-008`'s seventh requirement is already the shape of the last one, and this record
already defers it. Any of those is a measurement, not a preference, and would be answered here on
the same footing as the eight above.

Nothing decided above changes. Only the sentence that reads like a rule.

## Amendment: Both Owed Corrections Are Settled, And One Had Been For Days, 2026-09-13

The Consequences list above says:

> **The bands table needs a row** for whatever quarantines the XVPE dependency, and `README.md`
> currently says no crate here depends on XVPE. Both are corrections owed once the crate exists,
> not before.

The crate has existed since `KWB-24`. Measured for `KWB-80`, the two halves were in opposite
states and had been for some time.

**The bands row was discharged and nothing said so.** `KWB-75` made the bands table a projection
of the crate manifests, so `kwb-platform-xvpe` appears in it with what it adopts — not because
anybody acted on this bullet, but because the table stopped being hand-maintained and the crate
had a manifest. The obligation was met as a side effect of removing the class of obligation.

**The README half was never done.** It still read *No crate here depends on it yet*, thirty lines
under the generated table naming the crate that does. `KWB-80` corrected it, and the README now
carries the qualifier `D-007` actually decided: no crate depends on XVPE **by `path`**.

The reason neither half got attention is the shape worth recording. A bullet naming two
obligations is checked by reading it, and a reader who confirms the first one is true has no
prompt to check the second. **Phantom debt and real debt in one sentence hide each other**:
`KWB-76` swept this repository for obligations that had been discharged without being closed and
would have closed this bullet outright; a sweep for obligations still outstanding would have
found the README and left the bands row. Neither sweep reads a mixed bullet correctly, and this
one survived both.

So the rule this record can offer is narrow and mechanical: **one obligation per bullet.** A
consequence that names two things cannot be settled, only half-settled, and half-settled reads
as settled from either end.

Nothing decided above changes. The Consequences list is left as written, because a superseded
claim stays visible beside its correction here; this amendment is the correction.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `D-013`
- `D-014`
