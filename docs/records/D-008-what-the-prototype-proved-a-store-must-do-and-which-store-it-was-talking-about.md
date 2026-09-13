---
id: D-008
type: decision
title: What the prototype proved a store must do, and which store it was talking about
status: accepted
version: 2
authority: canonical-normative-record
tags:
  - prototype
  - storage
  - requirements
relations:
  - target: D-003
    type: relates-to
  - target: D-007
    type: relates-to
---

# What the prototype proved a store must do, and which store it was talking about

## Decision

Eight storage requirements are established by the prototype's own evidence, and each is
recorded below with what established it. Separately recorded is what was Postgres rather
than a requirement, because the two are easy to carry across together and only one of them
should be.

The finding that matters more than any individual requirement is the last section:
**seven of the eight are requirements on a mutable versioned graph, and the bands table's
only store is an immutable content-addressed document store.** They are not the same store,
the table names one, and this record does not create the other — naming a crate is what
`KWB-10` explicitly forbids and what a later decision must do with its own evidence.

No implementation is named and no crate is proposed.

## Required of any store

Each row is a requirement because the prototype paid to learn it, not because it is
good practice.

### 1. Liveness is one expression, and a second copy of it drifts

`ConceptLiveness.IsCurrentExpression` exists because the rule had three hand-written copies —
the Postgres global query filter, the in-memory repository, the JSON repository — and the
providers came to disagree about which concepts exist.

The proof is inside the prototype's own migrations. `IX_Concepts_CanonicalName_Unique_Active`
was created on 2026-07-09 as:

```sql
WHERE "Status" <> 'Deprecated'
```

and corrected on 2026-08-01 to:

```sql
WHERE "Status" <> 'Deprecated' AND "ValidUntil" IS NULL
```

For three weeks the index enforced **half** the liveness rule while `ConceptLiveness`
documented the whole of it. Nothing failed, because a half-rule is a weaker constraint and
weaker constraints do not raise errors. **A liveness rule restated in a second language
drifts, and the drift is silent.** Any store here either applies the one expression or is a
second copy.

### 2. Uniqueness must be expressible *under* liveness, not over the whole extent

That index is *partial* for a reason. A merge loser keeps its row — `D17` requires it and
`IConceptHistory` reads it — so a store enforcing canonical-name uniqueness across all rows
could not retain merge losers at all. The requirement is not "unique names". It is
**uniqueness over the subset a predicate selects**, with the predicate being requirement 1.

The prototype states the same shape again on a different column:
`HasIndex(ExternalKey).IsUnique().HasFilter("\"ExternalKey\" IS NOT NULL")`, with the
comment that a null external key "is a legitimate state — it says the source has not been
given a key — and many sources share it."

### 3. A read says which world it is asking about

`D19-B`. Concepts are read through two interfaces and which one a caller depends on is the
whole design: `IConceptRepository` returns current concepts, `IConceptHistory` returns every
version including merge losers.

What established it: `merge-audit` resolved the merge log's keeper and loser identifiers
against the concepts it could see. Against the current view it resolved **none**, printed
*"nothing has been merged away"*, and exited **0**. Every merge on record could have been
wrong and the gate would have passed.

Both available defaults fail. A global filter rewrites queries written by someone who has
never heard of it; no filter at all means new code that forgets the predicate reports merge
losers as live. The requirement is that the choice is **visible at the call site** and
reachable by neither forgetting.

### 4. Temporal reads reconstruct the graph as of an instant

`TemporalQueryExtensions.AsOf<T>` over `ITemporalEntity`:

```csharp
ValidFrom <= pointInTime && (ValidUntil == null || ValidUntil > pointInTime)
```

explicitly bypassing the default filter. The cost is part of the requirement and should be
recorded with it: **every entity this applies to carries a validity interval**, which is a
claim about the shape of every row, not a feature bolted to a query.

### 5. The store refuses states the liveness rule cannot answer

`AddTemporalConsistencyConstraints` adds six CHECK constraints, among them:

```sql
CHECK ("ValidUntil" IS NULL OR "Status" = 'Deprecated')
CHECK ("SupersededBy" IS NULL OR "ValidUntil" IS NOT NULL)
```

`ConceptLiveness` names the point exactly: *"this predicate cannot be asked a question that
has two answers."* Two columns encode one fact from two sides, and a row where they
contradict is not an invalid answer, it is an unanswerable one.

This is `D20` in storage form. A contradictory state made unrepresentable beats a validation
convention, for the same reason an enum with no zero value beats remembering to set a field.

### 6. Admission is idempotent under a key, and claimed work is held by a lease

`AddJobLeaseAndIdempotency`: a partial unique index `UX_PendingJobs_IdempotencyKey` over
non-null keys, a `LeaseExpiresAtUtc` column, and `IX_PendingJobs_Claimable` over
`(Status, LeaseExpiresAtUtc, EnqueuedAtUtc)` serving *"queued jobs, and running ones whose
lease has lapsed."*

This is `D19`'s remediation. Work handed to a queue is not work done, and a lease is how a
claim whose holder died is recovered rather than lost. Worth recording plainly because this
repository learned the same lesson from the other end: its own board had no lease until the
schema-5 migration, and an item sat permanently stuck in `Claimed` as a result. The
prototype had solved this for jobs before this repository hit it for items.

### 7. Ranked retrieval ranks from a stored vector and never derives one per query

The sharpest measurement in the prototype, from
`PostgresKeywordSearchTests.cs`:

The original index was an *expression* index over `to_tsvector(...)`. Matching was fine.
Ranking was pathological, because `ts_rank_cd`'s first argument was the expression too, so
scoring **re-tokenised the text of every matching chunk, per query** — and keyword queries
OR their terms, so a query of common words matched a large slice of the corpus and
re-tokenised all of it. Cost scaled with rows matched rather than with the `topK` wanted.

Measured live at 7,914 chunks: **38 ms, against 1.3 ms for vector search over the same
data.** And the line that makes this a requirement rather than an optimisation:

> Both forms return byte-identical rows, so no correctness test could ever see it.

A ranking that recomputes its input is invisible to every test that checks answers. It has
to be excluded by the shape of the store.

### 8. Delete-and-write is one act, or a failure between them destroys

`D17`. `ReplaceAllAsync` deleted unconditionally and wrote conditionally, so an
empty-but-valid reply erased a concept's entire synthesis. The requirement is atomicity
across the pair, and the stronger form — the one `D17` actually states — is that *the moment
a model stands between the delete and the write, write first or do not delete*.

## What was Postgres, and is not a requirement

Recorded because these travel with the requirements above and must not be carried as if they
were the requirements.

| Carried in the prototype | What it actually was |
|---|---|
| the **global query filter** | one implementation of requirement 3, and a bad one — `D19-B` is the incident it caused, not a feature it provided |
| `to_tsvector`, GIN, `ts_rank_cd` | one vector and one ranking function. Requirement 7 says *stored vector*, not *tsvector* |
| expression indexes over `LOWER(...)` | one way to make a comparison index-visible |
| `timestamp with time zone`, `DateTimeOffset` | a column type, not requirement 4 |
| `DbContext`, EF migrations | **91,527 of the prototype's 151,392 `src/` lines are generated migrations.** Sixty per cent of the source is an artifact of the mapping, and none of it is a requirement |
| materialised views `ConceptDegrees`, `SourceTrust` | a caching strategy for derived values |
| whether the planner *uses* an index | the prototype's own test refuses to assert this, and says why: *"asserting a plan shape over a synthetic fixture asserts the fixture, not the code"* |

## What the bands table covers, and what it does not

The table's row is: `kwb-store` — *"The content-addressed document store; one write door."*
Measured against the eight requirements as `kwb-store` stands after `KWB-2`:

**Covered, and covered more strongly than the prototype covered it.**

- **Requirement 8** cannot fail here. An address is derived from content, so an address is
  only ever offered the content it names; there is no overwrite, so there is no delete-write
  pair to make atomic. The failure mode is absent rather than guarded.
- **Requirement 6, the idempotence half.** Content addressing gives idempotent admission
  *without a caller-supplied key*, and `Admission::AlreadyPresent` reports it rather than
  folding it into success. This is stronger than `UX_PendingJobs_IdempotencyKey`, because a
  key can be forgotten and content cannot.

**Not covered, and covered by nothing else in this repository.**

Requirements 1, 2, 3, 4, 5, the lease half of 6, and 7. Also durability in any form:
`kwb-store` holds documents in memory for the length of a process, which `KWB-2` recorded as
a deliberate deferral to this item rather than an oversight.

**And the reason that list is so long is the finding.**

Seven of the eight requirements are about **entities that change** — concepts that have
versions, statuses, successors, validity intervals, and a liveness predicate over them.
`kwb-store` holds **immutable content that cannot change by construction**. Liveness,
supersession, temporal reconstruction and which-world reads are not things a content-addressed
document store is missing. They are questions about a different store, and asking `kwb-store`
to answer them would be asking it to stop being content-addressed.

So the bands table names one store where the prototype's evidence describes two:

- an **immutable artifact store**, which `kwb-store` is, and which `KWB-2` completed for
  everything except durability; and
- a **mutable versioned graph**, which nothing in this workspace has, which no row in the
  bands table names, and which is where requirements 1 through 7 actually land.

`kwb-domain`'s row names *"claims, concepts, argumentation, evidence, coverage, the
derivation ledger"* — the types — and says nothing about where they live or what must be
true of the place. That silence is where the second store is currently hiding.

This record states the gap and stops. Whether the second store is a crate, a band, a set of
port traits, or a decision that KWB does not need mutable storage at all is not settled by
the prototype's evidence, because the prototype never asked the question — it had Postgres
from the first migration and every requirement above is a thing it discovered *inside* that
choice. Deciding it needs its own item, and that item needs an input this one does not have:
what `kwb-domain`'s types are, which is `KWB-3` and `KWB-4`.

## Consequences

- `kwb-store`'s scope is now bounded by evidence rather than by not having been extended
  yet. It is the immutable half, and the requirements it does not meet are largely not its
  to meet.
- Durability **is** its to meet, and remains open. Nothing here decides it; requirement 7's
  lesson — that the cost of a retrieval shape is invisible to correctness tests — is the
  warning that should attach to whatever decides it.
- A second storage decision is owed, and is blocked on the domain types rather than on more
  prototype reading. The prototype has been read for this question and the reading is above.
- Requirement 1 is the one most likely to be violated by accident here, because this
  repository will have at least two ways to ask whether something is current the moment it
  has a second representation of anything.

## Alternatives Considered

**Extending `kwb-store` to cover the versioned requirements** was rejected as the thing this
item exists to prevent. It would make a content-addressed store carry mutable state, which
removes the property that makes requirement 8 unfailable.

**Recording the eight requirements without the two-stores finding** was rejected because the
coverage answer the item asks for is unreadable without it. "Seven of eight uncovered" reads
as a store that is badly behind; "seven of eight are about a different store" is the actual
state and leads somewhere different.

**Deciding the second store here** was rejected: `KWB-10` says the record names no
implementation and proposes no crate, and the input that decision needs does not exist yet.

## Amendment: The Durability Row Is Answered, 2026-09-12

The uncovered list above says *"durability in any form: `kwb-store` holds documents in memory
for the length of a process"*. That was true when written and stopped being true the same day.

`D-012` answered seven of the eight requirements and left durability; `D-014` then decided it,
on a measurement that changed the answer — a document is perfectly reproducible from its source
and a graph is not, so they are a cache and a ledger rather than one problem. `KWB-30` built the
content-addressed file store behind this crate's existing write door, `KWB-33` the publication
record, `KWB-34` the replay. Documents and knowledge both survive a process now.

The sentence is left standing rather than edited, because *when* a thing stopped being true is
what a reader of a dated record needs, and a silent correction destroys exactly that.

## Referenced By


*Generated by `tests/contract`, which fails if it is wrong. A relation is declared in
the frontmatter of the record that makes it; this is the other end, so that a reader of
this record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of
27 relations were reachable from one side only — which is how three records came to
assert things this repository had stopped doing.*

- `D-009`
- `D-012`
- `D-014`
