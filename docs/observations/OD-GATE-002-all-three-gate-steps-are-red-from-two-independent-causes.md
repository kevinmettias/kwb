---
id: OD-GATE-002
type: observation
title: All three gate steps are red from two independent causes, and the one that matters is a guard that stopped reaching its subject
status: open
version: 1
authority: observation
tags:
  - gate
  - process
relations:
  - target: OD-GATE-001
    type: relates-to
---

# All three gate steps are red from two independent causes, and the one that matters is a guard that stopped reaching its subject

`OD-GATE-001` ran every step of `gate.yml` by hand on 2026-09-13 and recorded a table in which
all three pass. That table is the load-bearing part of its argument — *"the gap is not that the
gate is wrong. Run by hand, it is green."*

All three steps are red. Measured 2026-09-21 on Windows, at `da5fd1e`, before anything in this
observation's own commit.

| step | command | result |
|---|---|---|
| Lint | `cargo clippy --workspace --all-targets -- -D warnings` | exit 101, three findings, all in `tests/guards/src/lib.rs` |
| Test | `cargo test --workspace --no-fail-fast` | three targets failed: `kwb-contract-tests` `boundaries`, `kwb-contract-tests` `projections`, `kwb-domain` `one_liveness` |
| Contract | `cargo test -p kwb-contract-tests --no-fail-fast` | two targets failed: `boundaries`, `projections` |

This observation exists because the table above is not one finding. It is two, they have nothing
to do with each other, and reading the first as the whole of it would leave the second to be
found again later.

## Cause one: a crate joined the workspace and the job was not finished

`tests/guards` became a workspace member at `386a432`, on 2026-09-15 — two days after
`OD-GATE-001`'s table was measured. It arrived without a band in its manifest and without a row
in `README.md`'s table, and its own source trips three findings the workspace sets to warn at
clippy's pedantic level and the gate raises to deny: `missing_panics_doc` on `Crate_Sources`,
and `must_use_candidate` on `Crate_Sources` and on `Mutating_Public_Methods`.

Three gate classes, one crate, one commit. `Test_Every_Workspace_Member_Should_Appear_In_The_Readme_Table`
and `Test_Every_Crate_Should_Declare_Which_Band_It_Is_In` both report the same one-element list,
`["guards"]`, which is what makes the attribution certain rather than inferred.

This is the ordinary kind of breakage. It is visible the moment anything runs, it names itself,
and it would have been caught by the first push had a push been able to run anything.

## Cause two: a guard reports zero and calls it a measurement

`kwb-domain`'s `Test_The_Crate_Should_Define_Liveness_Exactly_Once` fails with `left: 0`,
`right: 1`, and an empty list.

Zero is not a second copy of the rule. Zero is the guard having stopped finding the first one.

The mechanism, measured rather than supposed. `Liveness_Definitions` matches a function named
`Is_Current`. The `Crate_Sources` that feeds it — the local one at `one_liveness.rs:77`, not the
`kwb-source-guards` function of the same name — reads `CARGO_MANIFEST_DIR/src` with a single
non-recursive `read_dir`. The top level of that directory holds four files: `epistemic.rs`,
`graph.rs`, `lib.rs` and `versioning.rs`. `Is_Current` is defined at
`src/epistemic/standing.rs:84`, one level down, where that scan does not look.

The decomposition sweep that moved it is the same sweep that landed `tests/guards`. Nothing
about the liveness rule changed; what changed is where the file sits.

### Why this is the one that matters

The guard states its own reason for existing, and it is not a style preference:

> The prototype had the rule in four places — a Postgres global query filter, an in-memory
> repository, a JSON repository and a partial unique index — and the providers came to disagree
> about which concepts exist.

> `IX_Concepts_CanonicalName_Unique_Active` was created filtering only on
> `Status <> 'Deprecated'` and corrected three weeks later to also filter on
> `ValidUntil IS NULL`. **For three weeks the index enforced half the rule while the code
> documented all of it, and nothing failed**, because a half-rule is a weaker constraint and
> weaker constraints raise no errors.

A guard built because a half-enforced rule raises no errors is now enforcing none of it, and the
only reason anybody knows is a detail of how it was written: it asserts the count is **exactly**
one. Had it asserted *at most* one — the spelling that reads as more permissive and more
defensive — it would have passed, silently, for as long as the file stayed where it now is.

`OD-GATE-001` named this shape in its closing paragraph, as an analogy for the gate itself:
*"the same shape as a guard whose probe never reached its subject, and the reason that shape
keeps earning its own items on this board."* It is no longer an analogy.

## How long, and what was running in the meantime

Thirty-three commits landed after `386a432` and before this was measured, with the suite red
throughout.

None of them went through the board. The last revision the ledger records as verified is
`d034196`, which is `KWB-99`, forty-two commits back. Items `KWB-100` and `KWB-101` were authored
in that span and neither was finished.

So the sequence is not that a check failed and was ignored. It is that nothing ran a check at
all: CI could not, the ledger's `finish` — which runs the gate's lint step before any predicate —
was not invoked, and the two artifacts that would each have caught this independently were both
idle for six days.

## What made it visible

`KWB-102`'s `finish`, on 2026-09-21. `nomos work finish` derives its lint step from `gate.yml`
and runs it before the item's own predicate, so the first attempt to finish an item since
`KWB-99` was also the first execution of that step this repository has ever had. It refused, and
the item it refused was unrelated to anything broken.

`OD-GATE-001` observed that `finish`'s exit-4 refusal is satisfied by `gate.yml` being
**readable**, and that seventy-seven items had been finished against a check that never ran. The
converse has now happened too: the first time the check did run, it was red, and the item it
stopped had nothing to do with why.

## What this observation does not do

It proposes no remedy and fixes nothing. `KWB-103` holds cause one's clippy third. The band and
README row are the same cause but not the same territory — `README.md` is reserved by `KWB-101`,
and the bands table is projected from crate manifests, so whoever holds that path fixes both by
declaring the band and re-rendering. Cause two has no item yet.

It also does not revise `OD-GATE-001`. That table was true on 2026-09-13 and a measurement
outlives the decision it informed; what it lacked was any way for a reader to learn it had been
superseded, which the relation between these two documents now supplies.

This observation stays `open` until something acts on it.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `OD-GATE-001`
