---
id: D-021
type: decision
title: The migration runs to nine goals, every prototype capability is verdicted inside one of them, and the last is the owner's to open
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - bootstrap
  - ecosystem
  - process
relations:
  - target: D-001
    type: relates-to
  - target: D-003
    type: relates-to
  - target: D-004
    type: relates-to
  - target: D-022
    type: relates-to
---

# The migration runs to nine goals, every prototype capability is verdicted inside one of them, and the last is the owner's to open

## Decision

The migration from `C:/Users/kmett/source/repos/KnowledgeWorkbench` is **complete** when the
nine goals below are closed. Until then the development loop `D-022` describes keeps running,
and "done" means exactly this record's closure test, not an empty board.

Two populations make up the goals, and both are in scope. This was the owner's choice on
2026-09-27, over parity alone:

- **Parity.** Every capability the prototype could reach — the 81 rows of
  `docs/corpus/prototype-capabilities.md`, which routes all 78 of its commands, its 9 MCP tools and
  its 14 enrichment stages — is verdicted **met, diverges, deferred or declined**, as `D-001`
  requires, and every verdict of met or diverges is built.
- **The back half.** The corpus's operations the prototype never built — revise, compare,
  reconstruct, assess, synthesize at document grain, and artifact engineering — are designed and
  built, because `what-kwb-is-for.md` measured them as the half of the product that is missing.

**The owner's hold stands.** On 2026-09-24 the owner said not to run KWB or the migrators against
real data until they say so, and on 2026-09-27 kept that hold for this loop. Nothing the loop does
writes a durable record into a real store: every test is offline, and a measurement runs against a
copy in a scratch directory, which the owner's own words allow. Running against the real books is
goal G8, and only the owner opens it.

## The goals

Each goal has an **opening item**: a `Decision` item whose record verdicts the goal's catalogue
rows and whose authoring puts the implementation items for them on the board. A goal is therefore
not a list maintained here. It is a claim about the board and the records that a design pass can
check, and that is what keeps this record from becoming a second board.

| Goal | What is true when it closes | Catalogue rows | Opening items | Already on the board |
|---|---|---|---|---|
| **G0** — the loop can run | The gate's three steps pass at `HEAD`. `AGENTS.md` routes this record and `D-022`, and `tests/contract` holds the route. The anomalies found before the loop began are resolved | — | `KWB-121`, `KWB-122` | `KWB-101`, `KWB-102`, `KWB-103`, `KWB-107`, `KWB-108`, `KWB-109`, `KWB-110`, `KWB-115` |
| **G1** — the substrate is XVPE's | Every XVPE dependency pins one published revision that carries the store fixes. The store and the publication log are kept through XVPE's (`D-019`). A citation can be followed from any process. The placement contradictions XVPE recorded are answered here | C-14, C-65, C-81 | `KWB-123`, `KWB-124` | `KWB-100`, `KWB-104`, `KWB-111`, `KWB-113`, `KWB-114`, `KWB-116`, `KWB-117`, `KWB-118`, `KWB-119`, `KWB-120` |
| **G2** — KWB reads real sources | A PDF, a folder and a text file are admitted end to end, offline, through replayed model answers. An unreadable page is recorded as such rather than as an absence | C-01 – C-12 | `KWB-125` | — |
| **G3** — the query surface | `kwb-mcp` speaks MCP over stdio, every reply is bounded, and search is ranked, semantic and hybrid. Every tool the prototype had is built or verdicted | C-55 – C-67 | `KWB-126` | `KWB-112` |
| **G4** — curation, coverage and audit | Coverage is recorded per source and audited against an independent expectation. Merges are logged, audited and undoable. Derivation lineage and findings exist, and every model call is accounted for | C-13, C-24 – C-35 | `KWB-127` | — |
| **G5** — the epistemic layer | Adjudication, grounding, corroboration with its trust, calibration, the relation kernel and its algebra, negative knowledge and hypotheses are built, or each is verdicted with the `D-004` measurement it waits on | C-15 – C-22, C-47 – C-54 | `KWB-128` | `KWB-105` |
| **G6** — enrichment, code and project context | The enrichment stages the verdicts keep are built. Whether KWB owns a code entity is decided, and its rows follow the decision | C-23, C-36 – C-45, C-68 – C-72 | `KWB-129`, `KWB-130` | — |
| **G7** — the back half | Each of revise, compare, reconstruct, assess, document synthesis and artifact engineering is built, or is verdicted by a record that says why not | C-46, C-73 – C-80 | `KWB-131`, `KWB-132`, `KWB-133`, `KWB-134` | — |
| **G8** — the real books | The owner has released the hold, in words an amendment here records. KWB has then been run over the real sources on copies, and its results were checked against the prototype's recorded measurements | — | none until the owner speaks | — |

A goal may open before the one above it closes. Order is carried by `depends_on` on the items, and
only where a dependency is real. G0 comes first in practice, because `nomos work finish` runs the
gate's lint step before any predicate, so while the gate is red nothing on the board can finish.

## When a goal is closed

A design pass may call a goal closed when all four of these hold, and records it in the audit
commit `D-022` describes:

1. Every catalogue row the goal owns is verdicted by name in a record its opening item wrote.
2. Every item authored for those verdicts is `Done`, and was audited after it finished.
3. The goal's sentence in the table above is true at `HEAD`, measured rather than inferred from
   titles. `D-003` is why: implementation presence is not evidence of semantic validity.
4. No `Ready` or `Claimed` item names the goal in its `why`, and every item the table lists
   for the goal as already on the board is `Done` or `Declined`.

The migration is complete when G0 through G8 are closed. A design pass that finds every goal but
G8 closed reports that the owner is needed. It does not report completion.

## What is held, and by whom

- **The owner's hold** — real data. It covers G8, and any measurement that cannot be taken on a
  scratch copy.
- **`D-004`'s holds** — decisions that wait on the reference miner's measurements. They stay held
  until that record's own revision trigger fires. An opening item verdicts a held row as *deferred*,
  and names the measurement it waits on and whether a scratch copy can supply it. It does not
  decide the held question in order to close its goal.
- **XVPE** — a capability whose mechanism is domain-neutral is proposed in XVPE (`D-135` in
  Nomos). An opening item writes that as a step in XVPE's
  `docs/plans/programs/ecosystem-substrate/kwb-parity.md`, and the item here that consumes it
  waits on the pin move that brings it in. XVPE's steps are XVPE's board, not this one.

## Why

`D-001` requires each requirement to be met, diverge, be deferred or be declined — explicitly, as
this repository is built. Before this record, nothing said when that was finished, so "the
prototype has been migrated" was a feeling, the same defect `KWB-9` removed from "the prototype
has been read". An autonomous loop is exactly the reader who cannot act on a feeling. It needs a
closure test it can evaluate, or it will stop at the first empty board, or never.

Measured 2026-09-27, the directory-grain inventory had routed the prototype's surfaces and
misplaced most of what it could do. The 78 commands sat in one class-4 row, noted as "≥27", with
*corpus* as their destination, and a destination of *corpus* has no owner. The capability
catalogue is the missing denominator, and the goals are its owners.

## Alternatives Considered

**A milestone list beside the ledger.** Refused. `work/ledger.json` is the only board, and a
second list of what is left would drift from it the first time an item was re-authored. The goals
here name catalogue rows and opening items. What is left is always read off the board.

**Author every implementation item now.** Refused. Most verdicts are decisions nobody has made:
whether a provider enters the workspace, whether KWB owns a code entity, what reconstruction even
means. An item authored ahead of its decision states requirements nobody has decided, and a
`done_when` cannot be edited once it is stored. The opening items put each decision immediately
before the work it shapes.

**Parity only.** Offered and not chosen. The owner took parity and the back half together.

**Lift the real-data hold for the loop.** Offered and not chosen.

## Consequences

- `docs/corpus/prototype-capabilities.md` is the denominator for parity, and the audit reads it.
- Every item this goal structure creates names its goal in its `why`, so rule 4 above can be
  checked by reading the board.
- An amendment here is the only way a goal's scope changes, including the owner's release of G8.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `D-022`
