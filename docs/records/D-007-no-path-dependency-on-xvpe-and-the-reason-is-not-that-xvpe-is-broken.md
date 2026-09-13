---
id: D-007
type: decision
title: This repository takes no path dependency on XVPE, and the reason is coupling rather than breakage
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - ecosystem
  - dependencies
  - platform
relations:
  - target: D-004
    type: relates-to
  - target: D-005
    type: relates-to
---

# This repository takes no path dependency on XVPE, and the reason is coupling rather than breakage

## Decision

No crate here depends on `f:/repos/xvpe` by `path`. When a dependency is taken it is by git
reference and commit SHA, into a crate that exists to quarantine it, on the model
`nomos-platform-xvpe` sets and for the reason `D-130` gives: **a commit SHA is the difference
between adopting a body of work and adopting whatever that work becomes.**

What changes with this record is the *reason*, and the change is not cosmetic. The position
no longer rests on XVPE being unfit to depend on. It is not. It rests on what a `path` edge
does to this repository's reproducibility, which is a different claim, survives measurement,
and points at a different remedy.

`AGENTS.md` routes here rather than restating any of it.

## Rationale

### The premise that is gone

`AGENTS.md` said, and this repository inherited from `D-130`, that XVPE's foundations tier
"does not currently compile (`xvpe-dataflow` fails at 62 errors)."

Measured 2026-09-12 against xvpe `dev` at `a7eee3c6e361371b269217201233a2e7d7e36122`:

```
cargo check -p xvpe-dataflow
```

succeeds. 106 warnings, zero errors. The named crate compiles, and Nomos had already
recorded the same result on its own side in `D-130`'s amendment — checked there on
2026-08-18 across five crates, all clean. **This repository was carrying, as a live hazard,
a premise its own source had already retired.** A session reading `AGENTS.md` before today
was misinformed by it.

That is the failure worth naming: not that the fact went stale, which facts do, but that it
was written somewhere that does not get re-measured, in a file whose own text says it holds
how to act safely and not what is true.

### `D-130`'s surviving reasons, evaluated here rather than inherited

`D-130`'s amendment is explicit that a clean compile does not lift its gate, and gives three
reasons that are untouched by build stability. Each is a claim about **Nomos and what Nomos
wanted**, and this repository is obliged to ask whether it holds for **KWB and what KWB
wants**. Two do not.

**"No storage crate and no package crate."** This does not transfer. It describes what Nomos
needed and XVPE lacked. KWB needed a content-addressed document store and `KWB-2` built one;
`kwb-store` is this repository's own, and nothing about XVPE's inventory bears on it.

**"`xvpe-telemetry` names `xvpe-os-backend-desktop` as a direct dependency."** True of that
crate, and irrelevant to the crates KWB would want, which are not that crate. The relevant
measurement is below, and it is decisive.

**"A `path` dependency does not select the crates that happen to compile today; it selects
the workspace."** This is the reason that survives, and it survives in a **weaker and more
precise form than it is stated in**. Mechanically the sentence is not true — Cargo resolves
a path dependency to the named crate and its transitive dependencies, not to the workspace
containing it, and the measurement below is that closure. What is true, and what the
sentence is reaching for, is a claim about *coupling*: a path edge binds this repository's
build and its reproducibility to another repository's **working tree**, at whatever state
that tree is in.

### What was measured, exactly

On 2026-09-12, against xvpe `dev` at `a7eee3c6e361371b269217201233a2e7d7e36122`.

The candidate crates are chosen, not swept: they are the ones a KWB ingestion path would
actually reach for, and they are the mechanism half of the same pairing the Nomos precedent
records — XVPE owns the mechanism and the backend, the product owns its catalogue.

```
cargo tree -p xvpe-json-text    --no-default-features -e normal
cargo tree -p xvpe-corpus-text  --no-default-features -e normal
cargo tree -p xvpe-ai-inference --no-default-features -e normal
cargo check -p xvpe-json-text -p xvpe-corpus-text -p xvpe-ai-inference -p xvpe-primitives
```

| Crate | What it holds | Declared dependency closure |
|---|---|---|
| `xvpe-json-text` | minimal JSON text surface, `no_std` | **nothing at all** |
| `xvpe-corpus-text` | page fidelity, chunking | `xvpe-primitives` |
| `xvpe-ai-inference` | model surface, integer money | `xvpe-primitives`, `xvpe-json-text` |

Four crates in the union. **Zero third-party packages.** No GPU crate, no windowing crate,
no `xvpe-os-backend-desktop`, nothing resembling the transitive pull `D-130` warns about.
All four compile clean. Every inter-crate edge carries `default-features = false`.

Two of `D-130`'s structural descriptions are also simply out of date and should not be
re-cited from here: XVPE is no longer organized as `crates/engine/{foundations, simulation,
runtime, presentation}` — there is no `crates/engine` — and the workspace is 258 members
across eleven tiers rather than the 112 that record measured.

So the technical objection does not survive. **For the crates KWB would want, XVPE is
already fit to depend on.**

### The reason that does survive, and the evidence for it

The coupling claim, stated as what it is: a `path` edge makes this repository's build a
function of another repository's working tree, and that tree is measurably in motion.

The sharpest available evidence is from inside XVPE itself, 2026-09-12. Its reference miner
records a run, then replays it to re-evaluate deterministic downstream processing. Five
crates were added to the workspace — 253 to 258 — and the replay of a recording written the
day before matched **171 attempts, 171 not-recorded, zero tokens**. Nothing about the
prompt, the schema, the chunking, the models or the source had changed. Workspace drift the
caller never touched defeated a deterministic re-run.

That is not a compile failure and no `cargo check` would have caught it. It is exactly the
class of harm a path edge imports: not *"the dependency is broken"* but *"the dependency is
not the same dependency it was yesterday, and nothing said so."* A repository whose whole
premise is content-derived identity and reproducible derivation cannot take that edge
casually. `D-006` computes staleness by comparing a recorded input identity against a
recomputed one; a path dependency is an input with no identity to record.

A commit SHA gives that input an identity. That is the same argument this repository already
makes about everything else it stores, which is why the remedy is not novel here.

## Consequences

The gate is no longer *"wait until XVPE is fit to depend on."* That question is answered.
Two conditions replace it, and they are different in kind:

- **Mechanical.** Adoption is by git reference and commit SHA, into one crate that exists to
  hold it. Never by `path`. When such a crate is created it takes the `kwb-platform-xvpe`
  name the workspace manifest already reserves a comment for, and its creation is an item.
- **Open, and held elsewhere.** *Which* requirements XVPE is the right authority for under
  `D-135` is not settled by any of the above, and is on `D-004`'s held list. This record
  deliberately does not answer it. Nothing here proposes adopting any crate; it measures that
  three named crates would be cheap to adopt, which is not the same statement and must not be
  read as one.

`AGENTS.md`'s hazard is replaced by a pointer to this record. It no longer carries a
technical premise of its own, because a premise stated in a file nobody re-measures is how
this one went stale.

## What this measurement does not cover

Listed because a measurement whose limits are unstated invites being read as more than it is,
and `D-003` is this repository's record that presence is not validity.

- **The closure was measured in XVPE's own workspace, not in this one.** Cargo unifies
  features across a workspace, so a closure measured there is not the closure a KWB member
  would resolve to. XVPE has been bitten by exactly this — enabling one feature on one crate
  turned it on for six — and a dependency edge missing `default-features = false` has
  collapsed a `no_std` floor there before. The figures above are a lower bound on what
  adoption would pull.
- **No KWB crate has ever named an XVPE crate.** Nothing here has been built from a consumer
  in this repository. This is a reading of manifests and a check in the other tree.
- **Nothing about API stability, licensing, or `no_std` posture under this repository's own
  floor** was examined.
- **The 253-to-258 replay figure is inherited, not re-derived.** It is recorded in XVPE's
  reference-miner architecture document; the run journals behind it are on no path this
  machine could find, so it could not be reproduced here. It is cited as the clearest
  statement of a mechanism, and the mechanism does not depend on the exact count.
- **Whether these three are the right crates is not addressed**, only what they would cost.

## Alternatives Considered

**Lifting the hazard entirely and permitting a `path` dependency**, on the ground that the
compile premise is dead and the closure is four crates with no third-party packages. Rejected
because the premise that died is not the reason that remains, and the replay measurement is a
demonstration that reproducibility damage is invisible to the check that would have been
offered as reassurance.

**Leaving `AGENTS.md`'s hazard as written until a crate is actually wanted.** Rejected: the
hazard is not inert while unused. It is read by every session before it does anything, and it
was telling each one something false.

**Restating the corrected premise in `AGENTS.md` instead of routing here.** Rejected for the
reason the stale premise existed at all. `AGENTS.md` says of itself that it holds how to act
and not what is true, and a dated measurement is a claim about the world that will go stale
again.

## Referenced By


*Generated by `tests/contract`, which fails if it is wrong. A relation is declared in
the frontmatter of the record that makes it; this is the other end, so that a reader of
this record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of
27 relations were reachable from one side only — which is how three records came to
assert things this repository had stopped doing.*

- `D-008`
- `D-012`
