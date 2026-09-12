---
id: D-005
type: decision
title: This repository adopts nomos-ledger's schema and its binary, and defers owning any ledger code
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - work-ledger
  - ecosystem
  - tooling
relations:
  - target: D-004
    type: relates-to
  - target: D-001
    type: relates-to
---

# This repository adopts nomos-ledger's schema and its binary, and defers owning any ledger code

## Decision

`work/ledger.json` adopts `nomos-ledger`'s document schema exactly, and this repository's
ledger verbs are `nomos-ledger`'s own binary run against that file. No ledger code is
vendored, forked, or depended on, and no crate is added.

Concretely:

- The board is migrated to `schema_version` 5, with `kind` and `origin` on every item, drawn
  from that crate's closed sets (`Capability`, `Decision`, `Validation`, `Correction`,
  `Cleanup`; `Required`, `Proposed`). `KWB-13` carries the migration.
- `claim`, `renew`, `takeover`, `finish`, `abandon`, `decline`, `list`, `validate` and `audit`
  are performed by running the `nomos` executable from this repository's root. It resolves
  `work/ledger.json` relative to the working directory, so it operates this board without
  knowing anything about this repository.
- **Nothing in this repository's build graph changes.** No `Cargo.toml` gains an edge, no
  crate names `nomos-` or `xvpe-`, and `tests/contract`'s bands table is untouched. This is
  tool use and a file format, not a dependency.
- The question of who should *own* a work ledger in this ecosystem is deferred, not answered.
  This record names what would reopen it.

## Rationale

### The three options this item named are all the same option

`KWB-7` asked whether this repository's ledger tooling "vendors `nomos-ledger`, depends on a
shared XVPE crate `D-135` would place it in, or forks one." All three are answers to *who
owns the code*. Measurement showed that is not the question this repository currently has.

What this repository has is a board no tool can operate, which has already cost one deadlock:
`KWB-9` completed its work and could not be transitioned, and while it held a territory drawn
one path too wide it excluded `KWB-7` — the item that would have fixed that — leaving one of
twelve items claimable. That is a tooling problem, and owning code is only one of several ways
to solve it.

The measurement, recorded in `docs/corpus/nomos-as-precedent.md`: running `nomos work validate`
from this repository's root reads this board, parses it, and refuses it at exit 5 with
`missing field kind`. The two schemas are one schema. `Territory` is field-identical
(`resolution`, `paths`, `patterns`); `ItemState` already contains `Claimed`; `abandoned`,
`displaced` and `declined` are already here. Applied to a throwaway copy, the migration
validated at exit 0 and `work list` computed the whole board — `ready` and `waiting` from the
dependency graph, a `next:` line, and `KWB-9` reported **`lapsed`** rather than stuck, which is
the lease behaviour whose absence caused the deadlock.

So the cost of a working board is two closed-set fields and a version number. Against that,
every option that involves owning code is expensive.

### Why not vendor

`nomos-ledger` is 14,008 lines. This repository's entire `crates/` tree is 144 lines across ten
stubs. Vendoring would make the copied crate two orders of magnitude larger than everything
this repository has written, and it would arrive with dependencies of its own —
`nomos-scope-verification` for `Territory` and `VerificationPredicate`, `nomos-model` for
`SubjectSet`.

It is also the move `ARC-ECOSYSTEM-002` exists to refuse, one repository over. That record's
reasoning about the C# prototype — that importing an implementation imports the boundary it was
designed against, and then you argue about the new boundary from inside the old one — does not
stop being true because the source is a sibling written in the right language. A vendored copy
is a fork that has not admitted it yet: it drifts on the first divergent need, and nothing
notices.

### Why not fork

A fork is a vendor with the reconciliation abandoned up front. It has vendoring's costs and
gives up the thing that makes this decision valuable: **two products sharing one schema is
evidence, and a fork spends it.** See below.

### Why not an XVPE crate — and why that is still the right eventual answer

`KWB-7` frames this as an application of `D-135`'s admission test. That is the wrong test, and
the record says so itself: `D-135` decides where *new* domain-neutral code is authored the first
time, and states explicitly that it "does not weaken the proof requirement for code moving after
the fact." `nomos-ledger` is not new code. The clause governing a move is the one `D-135`
preserves — `D-122`, adopted into `ARC-ECOSYSTEM-001` — which requires *two products
demonstrating materially identical domain-neutral semantics* before code moves, because "both
products would use it" is the reuse argument that record already refuses.

A work ledger passes the domain-neutrality half easily. Its definition — territory-based mutual
exclusion over durable units of work, with leases and executed verification predicates —
mentions no crate, no rule, no finding, no claim and no concept. `ARC-ECOSYSTEM-001` already
names `nomos-ledger` a candidate "whose subject does not require the thing being coordinated to
be software."

Two things nonetheless make it untakeable by this item:

1. **It is not this repository's move to make.** Authoring `xvpe-work-ledger` is work in two
   other repositories, neither of which this item has territory in. And depending on it would
   take the XVPE dependency `KWB-8` holds and has not decided. An item cannot resolve another
   item's question by consuming its answer.
2. **Nobody in XVPE is waiting for it.** XVPE has no work-ledger crate today —
   `xvpe-frame-ledger` is timing, `xvpe-task-contract` and `xvpe-task-host` are runtime
   concurrency. Nomos already evaluated `Territory` as a migration candidate under
   `OD-PLATFORM-003`'s criterion (a capability moves down when an XVPE site is doing the same
   thing worse) and rejected it, because no such site was found. That criterion asks what XVPE
   is *missing* and is silent on what a *second product* needs, so the rejection does not settle
   the question — but it does mean the move has no pull from below.

**And this decision improves that position rather than foreclosing it.** `D-122` wants two
products with materially identical domain-neutral semantics. Adopting the schema produces
exactly that, in the strongest available form: not two products that *would* use one mechanism,
but two boards written in one schema and operated by one tool. A fork would have destroyed that
evidence; a vendored copy would have started eroding it. This is the option that accumulates the
proof the right eventual answer requires.

### What adopting the schema costs, stated rather than implied

- **This repository inherits a vocabulary it did not choose.** `kind`'s five values and
  `origin`'s two are nomos's, decided in its `OD-LEDGER-024` for reasons argued against nomos's
  own board. They are work classifications with no software-engineering semantics, so they carry
  nothing this repository must refuse — but they were not derived here, and a future need for a
  sixth kind is a conversation with another repository rather than a local edit.
- **The board becomes operable only by someone who has nomos built.** This is a real coupling.
  It is operational rather than structural — it constrains who can run a verb, not what this
  repository compiles — and it is accepted because the alternative on offer today is a board
  nobody can operate at all, which is the state this decision is being made from.
- **A stale binary is a hazard with a known shape.** `OD-LEDGER-008` made the refusal total: a
  build that cannot account for every key refuses to read the ledger rather than silently
  dropping the key it does not know. That guard is why this option is safe to take; the failure
  it replaced — a lossy write that looked exactly like a clean one — is the reason to build the
  binary fresh rather than trusting an old copy.
- **`finish` requires a gate.** It refuses at exit 4 when `.github/workflows/gate.yml` cannot be
  read, on the grounds that "an item finished against a check weaker than the gate is the defect
  this refusal exists to prevent." This repository has no CI at all today. That is a real
  precondition of the decision working, and it is a precondition worth having independently:
  `D-004`'s loop already requires the repository gates beside each item's own predicate, and
  this repository currently has no second half to that.

## Consequences

`KWB-13` carries the migration, the gate workflow, and the one reshaping the schema forces: a
`Claim` is `{holder, acquired_at, lease_expires_at}` with integer timestamps, so the prose notes
currently carried in `KWB-7`'s and `KWB-9`'s hand-written `claim` objects are not representable
and move into each item's `why`.

Once `KWB-13` lands, `KWB-7` and `KWB-9` — both parked in `Claimed` because nothing could
transition them — are finished through the tool rather than by hand, which is the first time
this repository's own `AGENTS.md` loop can be completed as written.

The lease arrives with the schema, and it is the part that changes behaviour most. A claim
lapses; a session that dies stops excluding others; `list` reports `lapsed` rather than
`claimed`; and `takeover` recovers an item while preserving the displaced claim rather than
overwriting it. The deadlock this repository has already hit becomes a condition that clears
itself.

`OD-LEDGER-001` is adopted as a known limitation rather than solved: territory is declared, not
enforced. Claiming compares territories; nothing checks the work stayed inside one. That record
is `status: open` in nomos and records the failure recurring three times there, each time as the
same authoring mistake — an item named where the *thinking* would happen rather than where the
*writing* would. This repository has now made it once, on `KWB-9`. Adopting the tool does not
fix it and must not be read as fixing it.

`KWB-8` is unaffected in substance and slightly clarified in framing: this decision deliberately
takes no dependency, so it neither pre-empts that item nor supplies it an answer. If `KWB-8`
later admits an XVPE dependency, the eventual-home question reopens on better evidence than it
has today.

## Alternatives Considered

Vendoring, forking, and an XVPE crate are treated above; each is refused with its reason, and
the third is refused *for now* rather than on the merits.

**Writing a small ledger tool here from scratch** was considered and rejected as the worst of
the options, though it is the one that looks cheapest at the start. The verbs are easy and the
semantics are not: a lease that lapses, an overlap predicate that refuses an unanswerable
question rather than granting it, a takeover that preserves the claim it displaces, three
distinguishable failures of a verification predicate, and an exit-code contract agents branch
on. Every one of those exists in `nomos-ledger` because something went wrong without it —
`OD-LEDGER-008`, `OD-LEDGER-012`, `OD-LEDGER-013`, `OD-LEDGER-019` and `OD-LEDGER-024` each
record the specific loss. Re-deriving them here would mean paying for them again, and this
repository's own `D-003` is a record about exactly that mistake in a different tree.

**Doing nothing and continuing to hand-edit** is what produced the deadlock this decision is
being made from, and `AGENTS.md` forbids the one hand-edit that matters.

## Revisit This Decision If

- **`KWB-8` admits an XVPE dependency.** The eventual-home question reopens immediately, on
  stronger `D-122` evidence than exists today.
- **This repository needs a verb, a state, or a field `nomos-ledger` does not have.** That is
  the point at which sharing a schema starts costing something, and the point at which owning
  code starts being worth its price. Until then the absence of such a need is evidence for this
  decision, not an accident of it.
- **The operational coupling becomes intolerable** — a contributor who cannot build nomos, or a
  CI environment that has no reason to check it out.
- **Nomos's schema changes in a direction this repository cannot follow.** The refusal is total
  and loud, so this failure announces itself at exit 5 rather than corrupting anything, which is
  what makes the coupling survivable in the meantime.
