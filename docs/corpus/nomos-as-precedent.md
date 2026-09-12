# Nomos as precedent: the XVPE crossing and the work ledger

Measured 2026-09-12 against `f:/repos/nomos` and `f:/repos/xvpe`.

`D-001` records that this repository's bands, `AGENTS.md`/`CLAUDE.md` shape and
`tests/contract` pattern were lifted from `f:/repos/nomos` deliberately. Two questions that
lifting left open are now live on this board: how a product in this ecosystem depends on
XVPE (`KWB-8`), and where this repository's ledger tooling comes from (`KWB-7`).

This is a corpus artifact: evidence about two sibling repositories, gathered by observation.
It decides nothing — `KWB-7` and `KWB-8` are the items that decide — and it is not a second
authority for anything nomos's own records already state. Where it cites nomos, the citation
is the authority and the quotation is a pointer to it.

Observation is permitted while the reconciliation is open; see `D-004`.

## 1. The reason this repository gives for refusing XVPE has been retired next door

`AGENTS.md`'s operating hazard and `README.md`'s ecosystem section both refuse a path
dependency on XVPE "for the same reason Nomos's own `D-130` gives." `KWB-8` already records
that the compile-failure half of that premise no longer reproduces. It does not record the
other half.

**`OD-PLATFORM-003` retired the no-dependency clause for the XVPE crossing**, and its
decision section is unambiguous:

> **No, for the XVPE crossing. Nomos is built on top of XVPE.**
>
> XVPE is the engine. This workspace is an application over it, **in the same sense the
> knowledge workbench is.** The clause was written on the premise of two peer systems that
> must not entangle; that is not the relationship these two have.

That is nomos's own governing record characterising *this* repository's relationship to
XVPE, not merely its own.

### What that record does not say, and it matters

It closes one crossing explicitly and names it:

> **The KWB crossing is untouched.** `Test_No_Crate_May_Name_The_Sibling_Knowledge_Workbench`
> still fails the build if any crate reaches a `kwb-` prefix, with an empty
> `KNOWLEDGE_ADAPTER`. Nothing here says this workspace may name `kwb-`; both systems being
> built on XVPE says nothing about either naming the other.

So the closed crossing is **nomos → kwb**. Nothing in that record, and nothing found in
nomos's record set, closes **kwb → xvpe**. A reader who takes "the KWB crossing is still
closed" as bearing on this item has read the direction backwards.

### And a divergence in nomos that this repository is currently citing the wrong side of

| Measured | Value |
|---|---|
| `D-130`'s `status` field | `accepted`, version 2 |
| `D-130`'s text | adoption is "by git reference and commit SHA into one quarantined crate, `nomos-platform-xvpe`. It is **never** a `path` dependency." |
| Nomos practice, per its own `AGENTS.md` | "Six crates name an `xvpe-*` dependency by a relative path that climbs out of this repository… and ten reach one transitively (measured 2026-09-11)." |
| `OD-PLATFORM-003`'s `relations` | targets `AGT-006`, `OD-PLATFORM-001`, `OD-EXECUTOR-001`, `OD-EXECUTOR-004` — **not** `D-130` |
| `ARC-CONNECTOR-001` | "`D-130` is untouched." |

Part of `D-130`'s shape survived — `nomos-platform-xvpe` exists as a named adapter crate —
and two specifics of it did not: the dependencies are by path, and more than one crate takes
them.

**This is nomos's divergence to resolve, not this repository's.** What belongs to this
repository is the consequence: `AGENTS.md` and `README.md` here cite `D-130` as their reason,
and `D-130` is the side of that divergence practice has left behind. `KWB-8`'s `done_when`
already requires that the hazard "either cites a premise that currently reproduces or routes
to that record instead of restating one," and this is a second premise that does not
reproduce.

### Mechanics, so the option is costed rather than imagined

```
/f/repos/kwb   /f/repos/nomos   /f/repos/xvpe      siblings
```

Nomos's edges are spelled `path = "../../../../xvpe/crates/…"`, climbing out of the repo into
the sibling. `cargo metadata --no-deps` on nomos exits 0, so the pattern resolves today. The
same spelling would resolve identically from here. The cost `OD-PLATFORM-003` accepted
deliberately is stated in nomos's `AGENTS.md`: the workspace "does not build without the
`xvpe` checkout beside it," and a missing or broken one fails at manifest resolution "before
any code of this workspace's is read, which reads nothing like the real cause."

## 2. How nomos is built on XVPE, which is the shape to copy or refuse

The pattern is consistent across four adoptions: **XVPE owns the mechanism and the backend;
nomos owns the catalogue, and names no protocol library at all.**

| nomos crate | XVPE crates it names | What nomos kept for itself |
|---|---|---|
| `nomos-mcp` | `xvpe-remote-call`, `xvpe-remote-call-backend-json`, `xvpe-primitives` | `ServedTool` — which four tools exist, the sentence each publishes, the schema each accepts |
| `nomos-api-transport` | `xvpe-remote-call`, `xvpe-primitives` | `ServedMethod` — which four verbs are served, and which twenty-one are excluded |
| `nomos-lsp` | `xvpe-diagnostics`, `xvpe-language-server-backend-lsp`, `xvpe-primitives` | `Severity_Of` — what its own `GateCategory` and `Applicability` deserve |
| `nomos-agent-executor-claude-code` | `xvpe-agent-execution`, `xvpe-agent-backend-claude-code` | the structural capability boundary (`OD-EXECUTOR-001`) |
| `nomos-platform-xvpe` | — | one adapter implementing nomos's **own** port traits over XVPE |

`nomos-mcp`'s own band-table entry states the test of whether the split was drawn correctly:
the whole `initialize` / `tools/list` / `tools/call` / `ping` handshake, notification
suppression and framing are XVPE's, so **"this crate names no protocol library at all."**
`nomos-lsp`'s says the same of LSP.

**This bears directly on `KWB-6`.** That item requires a read-only query surface and an MCP
tool surface built only against it. Under this pattern the protocol is XVPE's and what this
repository would hold is the catalogue plus the read-only-ness — which is the part `KWB-6` is
actually about. The prototype's nine tools (`search`, `get_concept`, `neighbours`, `path`,
`proofs_for`, `claims_for`, `code_for`, `gaps_in_source`, `list_connection_hypotheses`) are a
catalogue, not a protocol.

**And it converges with something this repository already decided independently.**
`nomos-contracts` is band 0, depends on `serde` and nothing else, and is that way because
peers that never compile it must still agree with it. `kwb-contracts` has the identical
charter in this repository's own bands table, and `D-002` reached the identical conclusion
about `KnowledgeReferenceId` from the other side. Two products arriving at the same rule for
the same stated reason is evidence about the seam, not a coincidence worth flattening.

One caution carried from nomos's own experience, because it is the expensive kind: a shared
*vocabulary* agreeing does not mean a shared *trait* means the same thing. Nomos and XVPE
both have a `Strategy` with the same three enums and a conformance test proving the
vocabulary agrees — and the two workspaces mean different things by it (a per-implementation
promise against a published classification of an execution domain). The test pins the
difference rather than erasing it.

## 3. The work ledger, and what it would cost this repository

`nomos-ledger` is **14,008 lines**, zone `Repo Tooling`. Its `lib.rs` names three rules:

- **Territory is the unit of exclusion, not the item.** Two items are concurrently claimable
  exactly when their territories are *provably* disjoint — "an unanswerable overlap question
  refuses the claim rather than granting it, because unknown independence is not safe
  parallelism."
- **A claim is a lease, not a lock. It lapses.** An agent that dies holding one stops
  excluding others when the lease runs out, and the lapsed claim stays visible "so a person
  can see the work was abandoned rather than never started." `list` reports it as `lapsed`,
  not `claimed`. Recovery is `Take_Over`, never `Claim`, and the displaced claim moves to
  `LedgerItem::displaced` rather than being overwritten.
- **Finishing runs a predicate.** `done_when` is prose for a human; `VerificationPredicate`
  is an argument vector that gets executed with no shell between what was written and what
  runs. It keeps three answers apart — the predicate failed, the predicate could not be
  started or timed out, and **there is no predicate at all**, "which must never read like
  everything checked out."

Two verbs that are not degrees of one thing: `abandon` ends a *claim* and returns the item to
the board with a reason; `decline` ends the *item*. `OD-LEDGER-019` measures what having only
the first one cost — a superseded item went back to `Ready` and was offered again, "at the
cost of a whole session's run."

Exit codes are a contract because agents branch on them: `0` ok, `1` validation, `2` usage,
`3` claim unavailable or dependency unfinished (**retryable**), `4` conflict a human must
resolve, `5` the ledger or its lock could not be used at all. The distinction nomos calls out
as earning its own code is 3 against 5.

### The measured gap between this repository's ledger and that crate

This repository's `work/ledger.json` was written by hand against no tool. Pointing nomos's
own binary at it answers, exactly, how far from the real thing it is:

```
$ cd /f/repos/kwb && /f/repos/nomos/nomos.exe work validate
ledger is malformed: work\ledger.json: missing field `kind` at line 26 column 5
exit=5

$ cd /f/repos/nomos && ./nomos.exe work validate
ledger is valid (schema 5, and this build understands 5)
exit=0
```

It read this repository's file — `work\ledger.json` resolved relative to the working
directory, and line 26 is the end of `KWB-1` — parsed it far enough to reach the first
missing field, and refused with the documented code for an unusable ledger.

Field by field, the schemas are the same one:

| | nomos `LedgerItem` | this repository |
|---|---|---|
| shared | `id`, `title`, `why`, `done_when`, `territory`, `state`, `depends_on`, `blocked`, `claim`, `verification`, `verified`, `abandoned`, `displaced`, `declined` | all present |
| **missing here** | `kind` — `capability`\|`decision`\|`validation`\|`correction`\|`cleanup` | — |
| **missing here** | `origin` — `required`\|`proposed` | — |
| `schema_version` | `5` | `1` |
| `Territory` | `resolution`, `paths`, `patterns` | field-identical |
| `ItemState` | `Ready`, `Claimed`, `Blocked`, `Done`, `Declined` | all five used or usable |

Both containers carry `#[serde(deny_unknown_fields)]`, decided in `OD-LEDGER-008`: "a build
that cannot account for every key in the ledger does not get to write the ledger back."
That is why the refusal is total and why it is the right kind of refusal — the failure it
replaced was a stale binary silently dropping a field it did not know at exit 0, so "a lossy
write looked exactly like a clean one."

**The whole gap is two closed-set fields and a version number.** `kind` and `origin` are
deliberately not `#[serde(default)]`: `OD-LEDGER-024` refuses "a field empty on a hundred rows
and set on the next," and every item already on that board was given both in the commit that
added them.

### Which record governs the choice, and it is not the one `KWB-7` names

`KWB-7` frames the decision around `D-135`'s admission test. `D-135` decides where **new**
domain-neutral code is authored the first time, and is explicit that it "does not weaken the
proof requirement for code moving after the fact." `nomos-ledger` is not new code. The clause
that governs moving it is the one `D-135` preserves — `D-122`, adopted into
`ARC-ECOSYSTEM-001`:

> code that began product-specific and is later suspected of being generic still needs that
> proof — two products demonstrating materially identical domain-neutral semantics — before
> it moves, because "both products would use it" is the reuse argument `ARC-ECOSYSTEM-001`
> already refuses.

This repository is positioned to be that second product, and the table above is close to the
evidence `D-122` asks for: not two products that *would* use one mechanism, but two boards
already written in one schema. That is a materially stronger position than the hypothetical
`D-122` exists to refuse — and it is still an argument for `KWB-7` to make or reject, not one
this file makes.

Two facts that bound it:

- **XVPE has no work ledger today.** Its nearest names are not the same thing:
  `xvpe-frame-ledger` is timing, `xvpe-task-contract` and `xvpe-task-host` are runtime
  concurrency. Nothing there does territory-based exclusion over durable units of work.
- **Nomos already measured `Territory` as a migration candidate and rejected it**, under
  `OD-PLATFORM-003`'s criterion — a capability moves down when a general capability sitting
  up in nomos is unreachable by an XVPE site doing the same thing worse. No XVPE site was
  found doing it worse. That criterion is about what XVPE is missing; it is silent on what a
  *second product* needs, which is the question `KWB-7` holds.

There is also a fourth option `KWB-7` does not currently name, and it is the cheapest: adopt
the *schema* without adopting the *dependency*. Two fields and a version number make this
board readable by a tool that already exists, independently of whether this repository ever
takes an edge to nomos or to XVPE.

## 4. The lease model is the thing this repository most concretely lacks

`KWB-9` is parked in `Claimed` and cannot be transitioned, because `AGENTS.md` forbids
hand-editing an item to Done and no tool exists to do it otherwise. Under nomos's model that
state is not terminal: the claim is a lease, it lapses, `list` reports it `lapsed`, and
`Take_Over` recovers it in one deliberate command while preserving who held it and when.

The absence also produced a second, sharper failure here. `KWB-9`'s territory was authored as
`docs/corpus`, `docs/records`, `work` — which is exactly `KWB-7`'s territory — so the item
that would build the tool that could finish `KWB-9` was excluded by `KWB-9`. One item out of
twelve was claimable. `OD-LEDGER-001` (`status: open`) names that authoring mistake and
records it recurring three times running in nomos, with the same shape each time:

> the item named where the *thinking* would happen and not where the *writing* would.

`OD-LEDGER-001`'s larger finding applies here unchanged and is worth carrying whether or not
any tooling is adopted: **territory is declared, not enforced.** Claiming compares
territories; nothing checks that the work then stayed inside one. "The territory is a promise
about where it intends to write, and a promise is what this system exists to stop relying
on."

## What would falsify this file

1. **A nomos record that supersedes `OD-PLATFORM-003`, or one that formally supersedes
   `D-130`.** The divergence in §1 rests on `D-130` reading `accepted` while practice
   contradicts two of its specifics; a record resolving that makes §1 historical. Check
   `f:/repos/nomos/docs/records/` rather than this file.
2. **A change to either ledger schema.** §3's gap is `nomos-ledger` at `SCHEMA_VERSION = 5`
   against this board at `1`. Re-measure by running `nomos work validate` from this
   repository's root, which is one command and does not modify anything.
3. **A `kwb-` crossing record.** §1 asserts that nothing closes kwb → xvpe. It rests on a
   reading of `OD-PLATFORM-003` plus a search of nomos's record set; a record naming that
   direction falsifies it.
4. **An XVPE crate that does territory-based work exclusion.** §3 asserts there is none, from
   a name-based search of `f:/repos/xvpe/crates`. A crate doing it under a name that search
   missed falsifies that bound.
