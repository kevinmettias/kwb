---
id: D-003
type: decision
title: Implementation presence is not evidence of semantic validity, and an extracted requirement carries the level it actually reached
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - prototype
  - evidence
  - ecosystem
relations:
  - target: D-001
    type: relates-to
---

# Implementation presence is not evidence of semantic validity, and an extracted requirement carries the level it actually reached

## Decision

Every requirement extracted from the .NET prototype is classified by the highest of four levels
it actually reached, and that level travels with it into whatever item or record consumes it.

```text
Declared                  the type or rule exists
Implemented               it has a body, a table, a migration
Exercised                 something in the pipeline, a host, or a shipping command calls it
Operationally validated   it ran against the real corpus and the result was checked
```

A type existing in the prototype proves the first two and nothing further. Stated as the rule
it is:

> **Implementation presence is not evidence of semantic validity.**

A requirement offered to this repository without its level is incomplete, and an item that acts
on a *Declared* requirement as though it were *Operationally validated* is acting on a
hypothesis it has mistaken for a finding.

The companion classification — which of eight kinds an extracted artifact is, and where it
lands — is applied in `docs/corpus/prototype-inventory.md` and is not restated here. That file
is evidence; this record is the rule it applies.

## Rationale

`D-136` and `ARC-ECOSYSTEM-002` already decided that the prototype is read rather than ported,
and that what survives is "the findings the code paid to learn." Neither says how to tell a
finding from a shape that merely exists, and without an answer the default behaviour for a
codebase that does roughly the right thing is to treat every type in it as a requirement. That
is the port this repository declined, arriving one type at a time.

The prototype supplies its own version of the rule and its own counterexample.

**Its version.** `docs/FEATURE_MATRIX.md` reads its columns as four independent claims —
implementation, unit tests, integration tests, end-to-end — and states two rules for itself: *a
checkbox is a claim about evidence, not intent*, and *a schema is not a feature: several tables
below have existed since the initial migration with nothing writing to them.* The ladder above
is that structure generalised past test categories to reach *ran against the corpus and the
result was checked*, which is the level the prototype's own hardest lessons were learned at.

**Its counterexample.** The universal epistemic kernel — the piece the prototype's own
architecture treats as most load-bearing — is Declared and Implemented and, in its node types,
nothing more. Twenty-seven types implement `IEpistemicNode`; **twelve are byte-identical apart
from the type name**, each 33 lines, each with five or six EF Core files behind it. Six of the
twelve reach no extraction result, no pipeline stage, no host and no repository read path.
`KnowledgeLayer` is one file read by three others and by nothing outside `Domain/`.

Read by presence, that is twelve mature concepts requiring Rust representations. Read by level,
it is **one architectural hypothesis expressed twelve times and never validated
operationally** — and the two readings produce entirely different work. The measurements are in
`docs/corpus/prototype-inventory.md`, with the commands that produced them.

The same test, applied to the other direction, is what makes the rule useful rather than merely
sceptical: `Domain/Algebra/` reaches *Operationally validated* with consumers in four EfCore
files, two `Core` files and a shipping command, so its content is evidence and not hypothesis.
The rule does not discount the prototype. It says which parts of it are load-bearing.

## Consequences

An item that cites the prototype states the level of what it cites. `KWB-3` is the immediate
case: the node types it would be natural to carry are *Declared*, while the relation algebra and
its constraint table are *Exercised*, and those are different kinds of input to the same item.

A level is a claim about the prototype and inherits the prototype's own warning about claims —
*a zero is not a result, and a count is not a check.* `Operationally validated` asserted on the
strength of a prototype report that turns out to be a count is wrong, and correcting it is a
defect fix, not a re-interpretation.

This record governs extraction from the .NET tree. `ARC-ECOSYSTEM-002` observes that this repository
will meet the same shape again — Nomos has exactly this relationship to `code-standards` — but
generalising the rule beyond this tree is not this record's to do, and doing it here would
assert ecosystem policy from inside one product.

## Alternatives Considered

**Classifying by confidence instead of by reach** — high / medium / low on each extracted
requirement — was rejected because it is unfalsifiable. Whether something was called by a
shipping code path is a fact a later reader can check with one command; whether a previous
reader felt confident is not, and the disagreement it produces cannot be settled.

**Not classifying at all, and relying on each item's author to judge** was rejected for the
reason `D-001` gives for answering the prototype requirement by requirement rather than
wholesale: the judgement is cheap to make once, per artifact, at the point of extraction, and
expensive to remake inside every item that later depends on it.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that no record declares a relation to fails too. A
relation is declared in the frontmatter of the record that makes it; this is the other end, so
that a reader of this record can reach the ones that answer, amend or build on it. Before
`KWB-38`, 24 of 27 relations were reachable from one side only — which is how three records
came to assert things this repository had stopped doing.*

- `D-004`
- `D-006`
- `D-008`
- `D-011`
