---
id: D-016
type: decision
title: The toolchain is pinned to the floor this workspace is measured on, and rustfmt is not installed with it
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - build
  - dependencies
  - reproducibility
relations:
  - target: D-007
    type: relates-to
---

# The toolchain is pinned to the floor this workspace is measured on, and rustfmt is not installed with it

## Context

`D-007` adopts XVPE by git reference and commit SHA, into a crate that exists to quarantine it,
so that this repository's reproducibility does not depend on another repository's working tree.
That decision pins the *dependency*. Until now nothing pinned the *toolchain*, which meant the
compiler was whatever `rustup` happened to default to on a given machine — the same class of
dependency on ambient local state that `D-007` refused, arriving through a different door.

Both siblings had already answered this and written their reason beside the answer. XVPE pins
`channel = "nightly"` because its brace style needs an unstable `rustfmt` option. Nomos pins
`channel = "1.88.0"` exactly, citing `OD-GATE-012`, on the stated grounds that it pins a
toolchain the same way it pins everything else it depends on. This repository mirrors Nomos's
conventions deliberately — the lint policy in `Cargo.toml` says so in as many words — and was
the only one of the three pinning nothing.

`KWB-90` is what made the question concrete rather than tidy. The manifest declared
`rust-version = "1.85"` and the workspace did not build on it. The floor is **1.87.0**, and it is
**imported**: `kwb-contracts`, `kwb-model` and `kwb-platform` still compile on 1.85, and what
raises it is `xvpe-collections-map`, reached through `kwb-platform-xvpe`, using
`integer_sign_cast`. So the quarantine crate confines the XVPE dependency *edge* — one manifest
names XVPE and a test proves it — but it does not confine the toolchain floor that edge carries.
That floor reaches every crate above band 1p and moves when the pin moves.

## What was measured

On Linux, with a target directory per toolchain so that no two builds contended:

| toolchain | `cargo clippy --workspace --all-targets -- -D warnings` | `cargo test --workspace` |
|---|---|---|
| 1.85.0 | does not build | does not build |
| 1.86.0 | does not build | does not build |
| 1.87.0 | clean | 210 passed, 0 failed |
| 1.88.0 | clean | 210 passed, 0 failed |
| 1.98.0 (stable) | clean | 210 passed, 0 failed |

Eight releases of stable sit above the floor and nothing fails anywhere in that span. The same
210 tests, with the same names, run on Windows; the workspace declares no `cfg(unix)`,
`cfg(windows)` or `cfg(target_os)` anywhere.

Separately, and this is what decides the `components` field:

```console
$ cargo +1.87.0 fmt --all --check
... 864 diff sites, each collapsing an Allman brace to K&R
```

## Decision

`rust-toolchain.toml` pins `channel = "1.87.0"` with `components = ["clippy"]`.

**The version is the measured floor, not an inferred one.** It is a version this workspace was
actually built, linted and tested on, which is the property `rust-version = "1.85"` lacked when
it was wrong. The two numbers now agree, and they agree because both were measured rather than
because one was copied from the other.

**`rustfmt` is deliberately absent, and the absence is the decision rather than an oversight.**
`cargo fmt` here collapses 864 Allman braces. There is no `rustfmt.toml` to prevent it, and
stable `rustfmt` cannot express the style in any case — XVPE pins nightly for
`brace_style = "AlwaysNextLine"` precisely because stable cannot. Declaring `rustfmt` in the pin
would install, for every contributor, the one tool that silently reformats the whole workspace
away from its own style. The gate needs `clippy` and nothing here needs `rustfmt`.

## Consequences

Every contributor and every runner compiles this workspace on 1.87.0 unless they override it
deliberately. That is the point: a build that passes here passes for the same reason everywhere.

**The pin is downstream of another repository's pin, and that is now visible instead of
implicit.** When `kwb-platform-xvpe`'s XVPE SHA moves, the floor can move with it. The number in
`rust-toolchain.toml` is then re-measured rather than assumed to still hold, and this record is
the thing that says so.

**Pinning the floor means routine builds no longer exercise a current stable.** That is a real
cost and it is accepted with its mitigation named: the span above the floor was measured green
to 1.98.0 at the time of this decision, and it is re-measured when the XVPE pin moves, which is
the same event that can move the floor.

**This record does not fix `cargo fmt`.** Leaving `rustfmt` out of `components` does not stop a
contributor whose global toolchain already has it. Both siblings say so in prose and this
repository says it nowhere. That gap is real, it is named here so that it is not mistaken for
closed by this record, and it belongs to its own item.

**`OD-GATE-001` is relevant and does not justify this decision.** The gate has never executed a
step, so no pin has ever been exercised by continuous integration. A toolchain pinned to serve
CI would be serving something that does not currently run. This is pinned to serve
reproducibility between contributors and between platforms, which is a property that holds
whether or not the gate is ever repaired.

## Alternatives Considered

**Pinning a current stable, such as 1.98.0.** Rejected because it abandons the floor rather than
defending it: `rust-version` would go on claiming 1.87 while nothing ever built on 1.87 again,
which is exactly the arrangement `KWB-90` had to correct. A claim nothing exercises is the
defect this repository keeps finding.

**Pinning `channel = "stable"`.** Rejected for the reason the pin exists. A floating channel is
whatever shipped that morning, so two contributors on the same commit can get different
compilers and a green build is not evidence about anybody else's build.

**Pinning nightly, as XVPE does.** Rejected because the reason XVPE has is a reason this
repository does not: XVPE needs an unstable `rustfmt` option for its brace style, and this
repository has decided not to run `rustfmt` at all. Nothing here needs an unstable feature.

**Continuing to pin nothing.** Rejected, and it is worth saying why it was not simply the
default. The absence of a toolchain file reads as an omission rather than a choice, so every
session that noticed had to re-derive whether it was deliberate. Recording the decision ends
that, and would have been worth doing even if the answer had been not to pin.

**Including `rustfmt` in `components` because both siblings do.** Rejected on the measurement.
Nomos includes it while forbidding `cargo fmt` in prose and carrying a `rustfmt.toml`; this
repository has neither, so the same list here would ship a loaded tool with nothing pointed away
from the trigger.

## Revisit This Decision If

- **`kwb-platform-xvpe`'s XVPE SHA moves.** The floor is imported through it and can move with
  it. Re-measure rather than assume the number still holds.
- **This repository needs an unstable feature.** That converts the channel question from stable
  versus stable into stable versus nightly, which is a different decision with XVPE's precedent
  available.
- **A `rustfmt.toml` lands here that expresses the house style.** The `components` half of this
  decision rests on stable `rustfmt` being unable to, and on there being nothing configured to
  stop it.
- **The gate begins to run.** `OD-GATE-001` records that it never has. A gate that executes would
  give this pin its first independent exercise and might show the span above the floor moving in
  ways a single measurement did not.
