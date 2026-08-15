---
id: D-001
type: decision
title: This repository is a ground-up Rust rewrite, and the .NET repository is prototype material
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - bootstrap
  - ecosystem
relations: []
---

# This repository is a ground-up Rust rewrite, and the .NET repository is prototype material

## Decision

`f:/repos/kwb` is KnowledgeWorkbench rebuilt from scratch in Rust. The prior
implementation, in C# at `C:/Users/kmett/source/repos/KnowledgeWorkbench`, is not ported.
It is prototype material: read for its domain model, its documented data-loss incidents,
and its own accepted decisions, and answered requirement by requirement — met, diverges,
deferred, or declined — as this repository is built.

This decision and its rationale are recorded on the Nomos side first, as `D-136` in
`f:/repos/nomos`, because at the time it was made this repository did not yet exist to
record it. This record is that decision's KWB-side acknowledgement, not a second
authorship of it — where the two disagree, `D-136` is the one that was actually
deliberated, and this record should be corrected to match it rather than the reverse.

## Rationale

The .NET repository's own governing decision,
`docs/adr/0001-implementation-language.md`, rejected Rust "for now," reasoning that most
of its code is EF Core and expression-tree schema mapping, migrations and dependency
injection that does not survive translation. That decision named its own trigger for
revisiting: "if the domain's invariant density grows faster than its plumbing." `D-136`
judged that trigger already met, given a working precedent for exactly this class of
problem now exists at `f:/repos/nomos` — a records-driven governance skeleton, a
territory-based work ledger, a content-addressed store, and a fact-oriented shared-analysis
substrate, over a domain just as provenance-heavy as this one's own epistemic kernel.

## Consequences

This repository's bands, `AGENTS.md`/`CLAUDE.md` shape, and `tests/contract` pattern are
lifted from `f:/repos/nomos` deliberately — the same reasoning that makes Nomos's own
prototype relationship to `code-standards` a precedent for this one makes Nomos's own
*current* shape a precedent for this repository's skeleton, since both are the same kind
of thing: bootstrap scaffolding proven once, not domain logic specific to software
analysis.

The .NET repository's domain model is read, not translated: its universal epistemic
kernel, its content-derived claim and concept identity (the actual cross-source dedup
mechanism), its `CoverageOutcome` and derivation-ledger primitives, and the five recorded
data-loss incidents (`D17`-`D21` in its own `AGENTS.md`) that name the failure classes
this repository's own types must make unrepresentable.

## Alternatives Considered

See `D-136` in `f:/repos/nomos`, which considered and rejected continuing the .NET
implementation and mechanically translating it, for reasons that apply unchanged here.
