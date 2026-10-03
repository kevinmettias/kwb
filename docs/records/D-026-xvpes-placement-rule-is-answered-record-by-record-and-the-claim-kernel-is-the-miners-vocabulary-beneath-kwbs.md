---
id: D-026
type: decision
title: XVPE's placement rule is answered record by record, and the claim kernel is the miners' vocabulary beneath KWB's, not a second authority for it
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - ownership
  - xvpe
  - vocabulary
  - placement
relations:
  - target: D-004
    type: relates-to
  - target: D-006
    type: relates-to
  - target: D-014
    type: relates-to
  - target: D-017
    type: relates-to
---

# XVPE's placement rule is answered record by record, and the claim kernel is the miners' vocabulary beneath KWB's, not a second authority for it

## The rule, as XVPE states it

Read at XVPE `ea0d401d08fb5812a58300ca61d8e21fed772954`, the last commit of
`docs/plans/programs/ecosystem-substrate/README.md`:

> **XVPE is the home of infrastructure and of generalizable domain logic. KWB is the home of
> knowledge-graph logic and its own application logic.**

and, of this repository:

> It also contradicts records in both products, which each need an explicit supersession rather than
> a silent override. … KWB: `D-004`'s held items, `D-006` against Nomos's push invalidation,
> `D-014`'s condition and `D-017`'s placement.

The rule is the owner's (2026-09-24). This record does not re-decide it. It says, for each record
XVPE names, what the rule changes and what it does not, and each record carries the change as an
amendment.

## Decision

**`D-004` — superseded in part: placement, not timing.** The rule decides *where* each held subject
will live once its hold lifts: the mechanisms whose definitions name no concept — how an assertion of
a unit is derived, the counting rule for corroboration, evidence and grounding, the base adjudication
verdicts — are XVPE's; binding a claim to a concept, standing and merges, the relation catalogue and
its algebra, the publication log and retrieval stay here. It does not decide *when*: a hold lifts on
its own revision trigger, the reference miner's measurements, and on nothing else. `D-004`'s rule —
observation proceeds, a decision depending on an open input does not — is untouched.

**`D-006` — amended: the definition stays here, an execution strategy may mark.** XVPE's register
answers the question (`q-substrate-staleness-model`, answered 2026-09-24 from the owner's own design
conversations, at the owner's request): staleness is *defined* by comparing recorded inputs, which is
`D-006`; propagated or precomputed marks are a permitted *physical strategy* that must agree with that
definition. So `D-006`'s rule 2 binds the authority — what stale *means* is never a stored flag — and
not every implementation: a shared mechanism may cache a mark, provided a test holds the mark equal
to the comparison and the comparison wins when they differ. This repository's own implementation
keeps computing on read. Rule 6 is unchanged for this repository: a cycle in `derived-from` is a
defect its write path refuses. That Nomos accepts cycles among facts computed from facts
(`OD-ANALYSIS-003`) is Nomos's to decide, and nothing here asks it to change.

**`D-014` — amended: the condition was met, by a different mechanism.** `D-014` refused the event
journal because its write swallowed failure, and named its condition: a write that reports failure.
XVPE closed it with `xvpe-record-log`, whose write fails visibly, and `D-019` adopted that for the
publication log. The event journal's own refusal stands, and is no longer needed: the publications
have a log that keeps them, and the event journal stays an observability record.

**`D-017` — amended: the authority's home follows the rule.** `D-017` decided that a vocabulary has
one authority and named no repository. Under the rule, a vocabulary whose definition names no concept
has its authority in XVPE, and this repository re-exports it, as `D-020` did for `Coverage`; a
vocabulary of the knowledge graph — what a source claims about a concept, at what scope — has its
authority here. `D-017`'s one-authority rule is unchanged.

**The claim kernel — neither a re-export nor a second authority: the layer beneath.** Read side by
side at XVPE `726aa3fee` and `eed12d02` (unchanged between them):

| Here | `xvpe-claim-kernel` | Relation |
|---|---|---|
| `Claim` — kind `claim`: a concept's identity, then the text; source and scope excluded | `PropositionIdentity` — kind `proposition`: the text alone; source excluded | a claim is a proposition bound to a concept |
| `Assertion` — kind `assertion`: a claim's identity, the source as rendered text, the scope; strength excluded | `AssertionIdentity` — kind `assertion`: a unit's identity, then a proposition's; scope excluded | this repository's assertion is a unit's assertion of a claim, at a scope |
| — | `OccurrenceIdentity` — an assertion and a location | `D-015`'s reading, not yet built here |

The derivation scheme is shared and the identities are not, by construction, and that is correct:
the kernel records what any miner can know — a text, a unit, a place — and this repository records
what only a knowledge graph decides — the concept a text is about, and the scope a source asserts it
at. One is the other's input, through admission. So neither re-exports the other, and `D-017` is
kept by layering: each side declares its own terms, and admission is the one mapping between them.
This repository's identities do not change: they are cited (`D-002`), and Nomos holds them (`D-137`).

**What XVPE is asked to change.** Two statements there are false as written, measured above: that
"the miners mint what KWB would mint" (`mining-core.md`) and that the two products "mint identical
identities from identical code" (the substrate README) — the scheme is identical, the identities are
not. The kernel's manifest describes its identities as "KWB's vocabulary". XVPE step X10 corrects all
three to say what is true. One thing is not a documentation fix: both sides derive under the kind
word `assertion` with different fields, so two different facts carry one name. Whether XVPE's
unit-level assertion takes a kind word of its own is XVPE's question, because changing it re-keys
every ledger the miners have written; it is raised there, not decided here.

## Why

**Silence would have been a decision.** XVPE wrote that each contradiction needs an explicit
supersession rather than a silent override, and that is right: four records here say things the
owner's rule now constrains, and a reader of any of them would otherwise apply it unconstrained.

**Placement and timing are separate questions**, and collapsing them is the failure `D-004` was
written to prevent: a hold would end because the rule made a subject's home obvious, while the
measurement that decides its content was still unmade.

**Equal schemes are not equal identities.** The side-by-side reading shows kinds and fields that
differ, so a claim and a proposition of one sentence carry different values. Treating the kernel as
this repository's vocabulary would have made admission look like a no-op when it is the mapping the
whole seam turns on.

## Alternatives Considered

**Re-derive this repository's claim from the kernel's proposition identity.** Refused: it changes
every cited identity (`D-002`), for an equality admission can supply by recording which proposition a
claim was admitted from — which is `D-015`'s reading, deferred with it.

**Adopt the kernel's types as this repository's.** Refused: the kernel has no concept and no scope,
and both are what a knowledge graph exists to add.

**Leave the four records as written and let readers apply the rule.** Refused: the rule's own text
asks for explicit answers.

## Consequences

- `D-004`, `D-006`, `D-014` and `D-017` each carry an amendment pointing here.
- XVPE step X10 is written into XVPE's `docs/plans/programs/ecosystem-substrate/kwb-parity.md`:
  correct the three false statements; raise the shared kind word as an XVPE question.
- `KWB-128` can open: what it decides about corroboration, grounding and the adjudication verdicts
  lives in XVPE under the rule, and what it decides about binding claims to concepts lives here.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `D-027`
