---
id: D-023
type: decision
title: A standing changes only by a closing act on a current concept, and a reading never reopens one
status: accepted
version: 2
authority: canonical-normative-record
tags:
  - domain
  - epistemic
  - closing
relations:
  - target: D-014
    type: relates-to
---

# A standing changes only by a closing act on a current concept, and a reading never reopens one

## Decision

A concept's standing is changed by `kwb retire` and `kwb supersede` and by nothing else. Each
carries the reason `D17` requires, which `KWB-37` made a condition of constructing one. An
admission, a reading, and a model's answer or silence change no standing.

**An admission that names a closed concept keeps what the source said and leaves the concept
closed.** The claims and assertions the reading carries are published, because the citation is
the source's evidence and dropping it would lose content a later reader cannot recover. The
concept is not published again, so its latest version stays the closed one. A claim published
this way is held and is current exactly as any other claim of a closed concept is; what closing a
concept does to its claims is `KWB-127`'s to decide, not this record's. The claim is never attached
to the successor: a successor is reached by following a relation, and following a relation nobody
certified is the inference `D18` forbids. The admission report says how many of the concepts the run
named were closed, in a field printed on every run, so a reader is told rather than left to notice.

**A closing act applies only to a current concept.** `retire` and `supersede` refuse a concept the
graph does not hold and a concept already closed; `supersede` also refuses a successor that is not
held or not current, naming the standing it has. Re-closing is refused because it would replace the
reason, and for a supersession the successor, that an earlier act recorded — destroying the
evidence `D17` exists to keep.

**No cycle of supersession can form, as a consequence rather than a separate check.** Each
supersession needs a current successor and leaves the superseded concept closed. A cycle would need
its last step's successor to be current when its first step has already closed it.

Every refusal exits 1, because the command line was well formed and the act failed against the
store, and leaves the publication log's bytes unchanged.

**Nothing reopens a closed concept.** Reopening is a change of standing, so it would need a reason
of its own, and no act carrying one is decided. Until a record decides one, a closed concept stays
closed.

**A log written before this record is read unchanged.** Replay keeps each concept's newest version
(`D-014`), so a re-assertion already published remains that concept's latest version. Nothing is
migrated or rewritten.

## Why

Measured 2026-10-03 at `17f82cf` (`docs/corpus/core-invariant-audit.md`, F1 and F14), with the
debug binaries against scratch stores:

- After `kwb supersede phlogiston --into oxidation --because Lavoisier`, `kwb-mcp merge_losers`
  lists phlogiston. A later `kwb admit` of another file that `--says phlogiston …` empties that list,
  because admission publishes every concept it names as asserted and replay keeps the newest version.
  A closure that carried evidence was undone by a reading that carried none. Once a model is the
  reader, a model's label undoes a person's evidence. That is `D17` reversed.
- Superseding alpha into beta and then beta into alpha exits 0 twice, and `kwb history` then reports
  0 concepts and 0 claims current. Two acts, each individually checked by `KWB-37`'s rules, made the
  whole graph invisible to every current query.
- `kwb retire x` on a store holding no concept named x exits 0 and records the closure.

`KWB-37` decided two refusals — a supersession into itself, and into a concept the graph does not
hold — and checked the successor only. This record extends the same reasoning to the concept being
closed and to the successor's standing.

## Alternatives Considered

**Refuse the whole reading that names a closed concept.** Refused. A new source restating a
superseded idea is evidence about that source, and the claim may be the very thing a later reader
needs in order to see why the concept was closed. Refusing it loses content to protect a standing
that publishing the claim without re-asserting the concept protects just as well.

**Attach the claim to the successor.** Refused. That is grouping over a relation nobody certified
transitive, and the prototype's 144 wrong merges of 579 are what that costs (`D18`).

**Let a re-assertion reopen the concept, as the code did.** Refused, for the reason the first finding
above gives.

**Decide a reopening act now.** Not asked, and not needed for anything measured. It is left open
rather than declined, and this record says so in the decision above.

**Refuse cycles by searching the successor chain.** Unnecessary. The current-successor rule makes a
cycle impossible to form without walking anything.

## Consequences

- `KWB-163` makes admission leave a closed concept closed and report the count; `KWB-164` makes the
  closing verbs refuse what this record refuses. Both carry the audit's F1 scenario and F14 cycle as
  tests through the built binary.
- `KWB-146` already forbids asserting any of these cases, so its tests stand unchanged under either
  item.
- `KWB-127` still decides what closing a concept does to its claims, and this record does not
  presume its answer.

## Amendment: A closed concept can be reopened, by an act that carries a reason, 2026-10-03

The decision above says nothing reopens a closed concept until a record decides an act carrying a
reason of its own. `D-024` decides it: `kwb reopen <concept> --store <dir> --because <reason>`
publishes the concept's next version under the standing `Reopened { because }`, which is current to
every query. So the sentence *nothing reopens a closed concept* now reads *nothing but `kwb reopen`
reopens a closed concept*, and every other rule here stands as written:

- an admission still never reopens one — reopening is a person's act with a reason, which is the
  whole of `D17`, and a reading carries none;
- a reopened concept is current, so the closing rules apply to it again exactly as to an asserted
  one;
- reopening a concept that is current, one the graph does not hold, or with a reason that normalizes
  to nothing is refused, exit 1, the log unchanged;
- the closing record stays in the log. Reopening adds a version; it removes nothing.

`D-024` also answers what closing does to a concept's claims, which this record left to `KWB-127`:
nothing. They stay with the concept they were asserted of.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `D-024`
