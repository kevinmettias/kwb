---
id: D-018
type: decision
title: kwb-model derives through XVPE's content identity, and the refusal SHA-256 owes is propagated rather than wrapped
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - identity
  - xvpe
  - ownership
relations:
  - target: D-012
    type: relates-to
---

# kwb-model derives through XVPE's content identity, and the refusal SHA-256 owes is propagated rather than wrapped

## Decision

`kwb-model` stops holding a derivation of its own. Its public names — `Derivation`,
`ContentIdentity`, `Exclusion`, `Sealed`, `IdentityError`, `Normalize_Text`, `IDENTITY_BYTES`,
`IDENTITY_CHARACTERS` — become re-exports of XVPE's `xvpe-content-identity`, reached through
`kwb-platform-xvpe` as every XVPE crate is (`D-007`), behind a feature that brings that crate
and nothing else, so the identity kernel's closure gains no clock, inference, chunker or
persistent map.

`Seal` refuses a derivation fed past SHA-256's message domain, and this repository
**propagates** that refusal: the four constructors that seal — `Claim::About`,
`Concept::Named`, `Assertion::By` and `kwb-store`'s `Document::Of` — return a `Result`, and
their callers pass it on.

What this repository derives — every kind and every field — stays here. What moves is the
scheme a derivation is computed by.

## Why the scheme is XVPE's

`xvpe-content-identity` was generalized from this crate on 2026-09-24, and reproduces seven
identities this repository published, through two independent SHA-256 implementations. Its
definition never mentions a claim, a concept or a source, so under `D-135` it is XVPE's to hold.

Two copies of one derivation drift, and this repository already measured the drift: `KWB-111`,
this normalizer deleting a line break standing alone between two words where XVPE's collapses
it. A second copy is where the next one of those would live.

## What was measured, 2026-09-24

- **The surfaces match name for name.** The same items, the same methods, `IdentityError` with
  the same two variants, the same 64-character lowercase rendering. No crate here names
  `Derivation` as a type — XVPE's is generic over its hasher — so nothing but `Seal` changes
  shape. A `KnowledgeReferenceId` (Nomos `D-137`) keeps its value and its shape.
- **The pin can move alone and change nothing.** A copy of HEAD `4b14f8f` with every XVPE pin
  moved from `8ff98a8fd` to `c700bcf83` compiles with no source change, and `Cargo.lock`
  resolves the same 56 packages, none gained, lost or re-versioned. `cargo test --workspace`
  passes 436 tests and fails 3, and the same 3 fail at the old pin in the real checkout
  (`one_liveness`, the README member table, the band declaration), so none is the bump's.
- **Adoption adds two packages.** `xvpe-content-identity` and `xvpe-algorithms-hashing`;
  everything beneath them already arrives through `xvpe-collections-persistent`. `sha2`, which
  only `kwb-model` uses, leaves.
- **The refusal reaches 51 files in eight crates**: 23 production call sites of the four
  constructors and about 209 in tests.

## Why the refusal is propagated

SHA-256 is defined for messages under 2^64 bits, and a derivation is streamed, so it can be fed
past that. What happens then was an open question this crate answered by accident: `sha2`
wraps the length, and two different inputs past the bound would share one identity. XVPE
answers it on purpose, with `InputTooLong`.

There were four ways to meet that here, and three of them are refused by rules this repository
already holds:

- **A panic** — `unwrap`, `expect` — is a panic path, which this workspace denies.
- **A wrapped length**, as `sha2` does, is a silent collision: two contents, one identity.
- **A sentinel** — a fixed or zero identity for any over-long input — is the same collision
  made deliberate.
- **Propagation** costs signatures, and is the only one that says what happened.

The cost is mechanical and one-time; a test builds its fixtures through a helper that expects
with a named message, which is a test's business and not a production path.

## Sequence

The pin moves first and alone, so its effect reads cleanly (`KWB-113`). It follows `KWB-112`,
which reserves `D-007` and forbids moving the rev inside its own scope, and it waits on `KWB-108`
and `KWB-109`, whose tests are the three red at HEAD. The adoption (`KWB-114`) follows the pin
and `KWB-111`, whose tests then pass unmodified against the adopted normalizer — the proof that
adopting it keeps the correction.

Neither item edits this record. A delta that differs from what is measured above is a stop:
this record is wrong, and amending it is a decision rather than an implementation step.

## Consequences

- One derivation and one normalizer serve both products, and a correction to either reaches
  both at the next pin.
- `kwb-model` keeps its name and its role as the identity kernel of this repository; a reader
  who asks where an identity comes from is still sent there.
- The four constructors are fallible in their signatures. That is the refusal being visible,
  not a new failure: the condition existed, and was silent.

## Alternatives Considered

**Keep `kwb-model`'s own derivation.** Refused: it is the second copy `KWB-111` measured drifting,
and `D-135` places a scheme with no knowledge semantics in XVPE.

**Ask XVPE for an infallible `Seal`.** Refused: an infallible `Seal` over an unbounded stream
must panic, wrap or return a sentinel, and each of those is refused above for the same reason
here as there.

**Adopt the scheme without the pin moving alone.** Refused: `D-007`'s amendment measures a bump
against the lock closure, and a bump folded into an adoption is one whose effect cannot be
told from the adoption's.
