---
id: OD-GATE-002
type: observation
title: All three gate steps are red, and two of the three failures are guards that went stale against the tree they watch
status: open
version: 2
authority: observation
tags:
  - gate
  - process
relations:
  - target: OD-GATE-001
    type: relates-to
---

# All three gate steps are red, and two of the three failures are guards that went stale against the tree they watch

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

This observation exists because the table above is not one finding, and because the obvious
reading of it is wrong. Four failures across three steps resolve into **one ordinary defect and
two guards that stopped seeing the tree they watch** — and the two guards went blind in the same
commit, to the same structural sweep, by two different mechanisms.

## Cause one: a crate's own source, and a path literal that did not learn about it

`tests/guards` became a workspace member at `386a432`, on 2026-09-15 — two days after
`OD-GATE-001`'s table was measured. Its source trips three findings the workspace sets to warn
at clippy's pedantic level and the gate raises to deny: `missing_panics_doc` on `Crate_Sources`,
and `must_use_candidate` on `Crate_Sources` and on `Mutating_Public_Methods`. That is the Lint
step, and it is a straightforward defect in the crate.

**The two contract failures are not.** They read as the same defect — a crate that joined
without a band or a README row — and they are not that at all. Reading them that way is easy
and it is wrong, which is worth saying because getting it wrong sends the repair to `README.md`,
where there is nothing to fix.

`Test_Every_Workspace_Member_Should_Appear_In_The_Readme_Table` and
`Test_Every_Crate_Should_Declare_Which_Band_It_Is_In` both report `["guards"]`, and both take
their member list from `Member_Name_On` in `tests/contract/src/lib.rs`, which refuses two paths
by literal:

```rust
if path == "tests/contract" || path == "tests/integration"
```

Its stated reason covers exactly this case — *"the two test crates are refused here [...] because
they are a property of what a member is for this reader: `tests/contract` asserts the tables and
`tests/integration` is its sibling, so neither is a row in one."* `tests/guards` is a third test
crate, reached only through `dev-dependencies`, and by that reason it is not a row in the table
either.

Two things follow. `README.md` is **correct as it stands**: its bands table is projected from the
manifests, and both tests that check the projection itself pass. And the list refuses
`tests/integration`, which is not a workspace member and not a directory — `tests/` holds
`contract` and `guards` and nothing else. So the literal is stale in both directions at once: it
excludes something that does not exist and fails to exclude the thing that does.

What looked like a crate arriving unfinished is a hardcoded path list that did not learn about a
new sibling. That is a guard going stale against the tree, which is cause two's family, not this
one's.

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

### Why this is the worse of the two blind guards

Both went blind to the same sweep, and the difference is what each one does when blind. The path
literal in cause one produces a **false positive**: it names a crate that is fine, loudly, and
the failure is unmissable even though its message points at the wrong repair. This one produces
the other kind.

The guard states its own reason for existing, and it is not a style preference:

> The prototype had the rule in four places — a Postgres global query filter, an in-memory
> repository, a JSON repository and a partial unique index — and the providers came to disagree
> about which concepts exist.
>
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

## What else was checked, and the one latent instance it found

Two guards blinded by one sweep is a class, not a coincidence, so the class was censused rather
than left for the next accident to find. Both mechanisms were searched across the workspace on
2026-09-21.

**Path literals selecting what a guard looks at.** One, and it is the one above. No other test
compares a path against `tests/…` or `crates/…` by literal.

**Non-recursive `read_dir` over a source tree.** Beyond the liveness guard's own copy, one:
`kwb_source_guards::Crate_Sources`, the shared reader, which takes a crate's manifest directory
and reads `src` at a single level. Two guards are built on it — `kwb-store`'s one-door test and
`kwb-retrieval`'s read-only test — and both pair it with `Mutating_Public_Methods` to assert that
**no** public method takes `&mut self`.

Both of those crates' `src` directories are flat today, so neither guard is currently blind. This
is a hazard rather than a failure, and it is recorded because it is the dangerous polarity of the
same defect:

| guard | asserts | what partial blindness does |
|---|---|---|
| liveness | the count is exactly one | fails loudly, as it did |
| one door, read only | the count is zero | **passes** |

The reader does carry a floor — `assert!(!sources.is_empty(), "no sources were scanned, so this
test proves nothing")` — and that floor catches a scan that found *nothing*. It cannot catch a
scan that found the top level and missed a subdirectory, which is exactly what a file split
produces, and `kwb-domain`, `kwb-ingest` and others already hold subdirectories under `src`. The
arrangement this reader assumes is one the workspace has already stopped keeping.

`KWB-110` holds it.

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

It proposes no remedy and fixes nothing. `KWB-103` holds the clippy findings, which are the only
part of this that is a defect in a crate rather than in something watching one. `KWB-109` holds
the stale path literal and `KWB-108` the non-recursive scan.

`README.md` needs no change, and saying so is the point of recording this rather than leaving the
failure to be read at face value: its bands table is projected from the manifests, and both tests
that check that projection pass. An item pointed at `README.md` here would have edited a correct
file to satisfy a guard that was wrong, which is how a stale guard converts into a real defect.

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
