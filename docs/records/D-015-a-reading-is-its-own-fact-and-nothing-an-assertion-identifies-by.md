---
id: D-015
type: decision
title: A reading is its own fact, and an assertion never identifies by one
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - provenance
  - domain
  - ingestion
relations:
  - target: D-010
    type: relates-to
  - target: D-002
    type: relates-to
  - target: D-014
    type: relates-to
---

# A reading is its own fact, and an assertion never identifies by one

## Decision

`KWB-47` declared an extraction contract whose output carries **where in a source** something was
read and **under what protocol**. `KWB-53` established that admission discards both, so the
requirement that every admitted claim identify its source occurrence and its extraction protocol
is unmet and had no record deciding how it should be met.

**Neither a location nor a lineage may enter an assertion's identity.** Not now, and not when a
model-backed reader arrives. An assertion answers *which source asserted this claim, at what
scope*, and that is the question corroboration is counted by.

**Neither may be carried on an assertion as a non-identifying field either.**

**A reading is a separate fact about an assertion**, and it is built when something produces
readings that differ. That condition was named below and has since been met by `KWB-66`; what
still holds the build is that no provider exists, which is a different and weaker reason. The
amendment at the end says so.

## The measurement

This repository counts corroboration by assertions. That is not an inference from the types; it
is what the test exercising the mechanism every identity decision here was made to support says
in its own words:

> Two different documents, the same claim text: one claim, two assertions, each naming the
> address of the document it was read out of.

So the count of assertions on a claim **is** the count of sources that assert it. Everything
below follows from that one sentence, and any placement that changes what an assertion counts
changes what that number means.

## Why not in the identity

If lineage participated in an assertion's identity, then re-reading one book under a new prompt
would derive a second assertion: same claim, same source, same scope, different protocol. The
claim would then carry two assertions and the number that says *two sources agree* would say two
where **one source was read twice**.

That is the reference miner's measured failure, reached from the other side. Its own claim
identity absorbs the source path and the page window, which is why `D-004` version 2 records
cross-source corroboration as **stranded** there: two sources' assertions can never meet, because
the thing that would make them meet is inside what distinguishes them. Putting a reading's
protocol into an assertion's identity does the same damage one level up — it cannot stop two
sources meeting, but it stops one source from being counted once.

Location fails the same test for the same reason. A book that says something on page 10 and again
on page 200 is still **one source asserting it**. Two assertions there would be a corroboration
count that grew because a passage repeated itself.

## Why not as a field either

The remaining option is to carry them on the assertion without identifying by them. That is worse
than it looks, and this repository has already paid for it once.

Two readings of the same source, proposing the same claim at the same scope, derive **one**
assertion. That assertion would hold one location and one lineage — whichever reading reached it
first. `KWB-32` is the same defect: `Concept` and `Claim` stored text *as given* while deriving
identity from it *normalized*, so a value disagreed with its own address and insertion order
decided what a reader got back. A non-identifying field on a content-derived value is a value
that can disagree with itself, and the disagreement is invisible because both readings are
truthful.

## So a reading is its own fact

What is left is the shape the evidence points at: the reading is a fact **about** an assertion
rather than a part of one. One assertion, any number of readings that produced it, each with its
own location, protocol and reader. Corroboration stays a count of assertions; provenance becomes
a count of readings; neither number answers the other's question.

This also keeps `D-010` intact rather than reinterpreting it. `D-010` put scope on the assertion
because *how far a source claims to reach* is a property of the assertion and not of the
proposition. *How somebody came to read it* is a property of neither — it is a property of the
act — and the reason it does not go where scope went is that scope is something the **source**
says and lineage is something the **reader** did.

## What this does not decide, and what settles it

- **Whether a reading is published, and in what order.** `D-014` made the graph durable by
  replaying an append-only record of publications, and a reading would be a fourth kind. Replay
  refuses a record naming something no earlier record published, so a reading would follow its
  assertion — but whether readings belong in that log at all, or beside it, is not settled here.
  *Settled by:* the first reader whose readings differ.
- **Whether a reading has an identity of its own, and what derives it.** Every identity in this
  repository is derived from content (`D-002`), and what a reading's content *is* — the passage,
  the answer, both — is a question nothing can answer without a reader that produces one.
  *Settled by:* the same condition.
- **Whether the extraction contract's one-location-per-reading shape survives.** ~~Today
  `ProposedReading` carries one location for any number of proposals~~ — it still does, and a
  *read* now returns one reading per passage instead, so each location describes the passage it
  came from rather than an average of several. *Settled by:* a reader that splits. **Settled by
  `KWB-66`**, which is that reader; `ExtractionStrategy::Read` returns `Vec<ProposedReading>` and
  its documentation carries the reasoning. A failed passage refuses the whole read, because a
  source two thirds read that reported the shape of a source fully read is the prototype's 1,367
  rows again.

## Why it is not built here

**Condition met:** `KWB-66`

This section said, in three sentences: *nothing in this repository reads a source*; *every
reading is a person, through `Stated`*; *today there is exactly one reading per assertion and no
two of them can differ*. `KWB-66` falsified all three within the hour. `kwb-extract` reads
sources, `ReadsText` is a reader that is not a person, and a read now returns one reading per
passage — so readings differ by passage, and two readers over different models differ by lineage
as well.

All three conditions above therefore fired, and this record mentioned `KWB-66` nowhere until
`KWB-68`. The third was settled in the extraction contract's own documentation *by that item*
and the record was never told, which is the gap between a decision and the code that answers it,
one crate apart.

**The honest reason it is still not built is a different one.** No provider exists in this
workspace and none is coming into it, so nothing yet produces a claim from a model: every
reading is still exercised through recordings. A mechanism for many readings of one assertion
would have a second case to justify its shape — that objection is gone — and would have no
traffic. That is a weaker reason than the one this section used to give, and it is the true one.

**What it costs while it waits, stated because it is now live.** A claim admitted from a model
enters the graph with no record of which model or which passage produced it. `Admit` carries a
reading's location and lineage to the door and drops both; an `Assertion` holds a source, a claim
and a scope. So *every admitted claim identifies its source occurrence and extraction protocol*
is unmet in practice rather than in theory, and the first corpus read by a provider is the first
corpus that cannot answer where its claims came from.

**The decision above is untouched.** A reading is still its own fact, and an assertion still
identifies by neither location nor lineage, because the corroboration-count argument that settled
that turns on how corroboration is counted and not on what reads a source.

## Consequences

- `Assertion` is unchanged, and that is now a decision rather than an omission. `KWB-53`'s
  documentation saying location and lineage stop at the door is correct and stays.
- The requirement that an admitted claim identify its source occurrence and extraction protocol
  is **deferred with a named condition**, not met and not abandoned. What is met today is the
  source: every assertion carries the content address of the document it was read out of.
- An extractor implementation must not be allowed to close this gap on its own by putting lineage
  somewhere convenient. The contract already refuses the two placements that would matter, and
  this record is why.

## Alternatives Considered

**Lineage in the assertion's identity.** Rejected on the corroboration count: one source read
twice would be indistinguishable from two sources agreeing, in the one number this repository
exists to produce.

**Lineage as a non-identifying field.** Rejected as `KWB-32` in a new place: two truthful readings
meeting at one assertion that holds one of their lineages is a value disagreeing with itself,
decided by insertion order.

**Lineage on the claim.** Rejected by `D-002` and `D-010` together, and not seriously considered:
the source is excluded from a claim's derivation so that two books asserting one thing are one
claim, and a protocol is further from the proposition than the source is.

**Deciding nothing until a reader exists.** Rejected because the obvious design is already
available to whoever writes that reader, and it is wrong for a reason that is measurable today.
A decision taken after the code is an excavation.
