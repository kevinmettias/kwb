---
id: D-020
type: decision
title: A rule's coverage is XVPE's evidence type, re-exported where kwb-domain declared its own, and the reasons for its shape stay here
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - domain
  - epistemic
  - xvpe
  - ownership
relations:
  - target: D-004
    type: relates-to
---

# A rule's coverage is XVPE's evidence type, re-exported where kwb-domain declared its own, and the reasons for its shape stay here

## Decision

`kwb-domain`'s `Coverage` becomes a re-export of `xvpe_evidence::Coverage`, reached only through
`kwb-platform-xvpe` (`D-007`'s quarantine), at the pin `KWB-117` sets. `kwb_domain::Coverage` keeps
its path, its four variants, their fields, its five methods and the names it renders. There is one
type: no wrapper, no newtype, no conversion between two.

## Why XVPE's

`D-135` places code with no knowledge-domain semantics in XVPE. `Coverage` says whether a rule ran
over some material and what it found, and its definition never mentions a claim, a concept or a
source.

XVPE rebuilt it from this repository's type on 2026-09-26, name for name, for its two mining
products, whose journals had to tell a look that found nothing from a look that never happened --
the distinction this type exists to draw. That left two copies of one type. `D-017` names the
condition for vocabulary: two artifacts each describing the same thing, neither detecting when the
other drifts. The same holds for a mechanism, and `KWB-111` measured it happening to a copied
normalizer here. The copy that stays is the one every product can reach.

## The measurement, 2026-09-26

Read side by side: `kwb-domain/src/epistemic/coverage.rs` at this repository's head against
`xvpe-evidence/src/coverage.rs` at XVPE `d6f075c14`.

- The same four variants with the same fields: `Yielded { findings: NonZeroUsize }`,
  `Barren { examined: NonZeroUsize }`, `Skipped { because: &'static str }`,
  `Unmet { prerequisite: &'static str }`.
- The same five methods, with the same signatures and the same bodies: `Of_Run`, `Has_Run`,
  `Is_Evidence_Of_Absence`, `Findings` and `Name`, which renders `yielded`, `barren`, `skipped` and
  `unmet`.
- XVPE adds `Hash` to the derives, and the four rendered names as public constants. Nothing this
  repository does changes under either.
- `xvpe-evidence` is `no_std` and depends on nothing. Adopting it adds exactly one package to
  `Cargo.lock`.

## What stays here

The type's reasons are this repository's history, and they stay in this repository. From now on this
record carries them; `kwb-domain`'s re-export points here.

- **The incident the shape exists for.** The prototype had these four outcomes and still lost data
  with them. It wrote **1,367 `Barren` rows that meant "the prerequisite had not run"**, permanently
  foreclosing two thirds of a book while reporting full coverage. Nothing was broken: `Barren` and
  `Unmet` were plain members of one enum, so writing the wrong one was a typo no type could catch
  and no reader could later tell from the truth.
- **So each variant carries the fact that distinguishes it.** `Barren` requires the amount of
  material examined, non-zero, because a rule that looked at nothing has no evidence of absence to
  offer. `Unmet` requires the prerequisite by name. `Skipped` requires the reason as a
  `&'static str`, a decision written in code rather than a message assembled at runtime. `Yielded`
  requires a non-zero count, because a yield of nothing is `Barren`. Recording an `Unmet` as a
  `Barren` therefore means inventing a number for material never examined -- not a slip.
- **`D17`**: the absence of an extraction is not evidence against an artefact. Only `Barren` is
  evidence of absence, and `Is_Evidence_Of_Absence` is the question a caller asks before sweeping,
  deprecating or deleting on the strength of having found nothing -- three of the four outcomes have
  no findings, and only one of them means there are none.
- **`D20`**: no default, and a derived outcome. A value that reports the good case until somebody
  remembers otherwise is a false feature, so the type has no `Default`, and `Of_Run` computes
  `Yielded` or `Barren` from what the run found rather than letting a caller assert either.
- **`Skipped` is producible and unbuilt.** Nothing in this repository constructs one, because
  nothing here declines to examine anything: `Admit_Source` takes one source and always reads it.
  What would produce it is a driver that walks a corpus and so has occasion to decline a source.
  It is not `kwb-store`'s `StoreError::Collision`, which is unreachable by mathematics; this one is
  reachable and merely waiting.

## Alternatives Considered

**Keep both copies.** Refused. Two copies of one definition drift, and neither looks wrong from
inside itself (`D-017`).

**Convert between them where the two meet.** Refused. Two types with one meaning, and a conversion
to paper over the gap, is the drift with a mechanism added to hide it.

**Keep this repository's as the authority and have XVPE use it.** Not possible, and not wanted. XVPE
depends on nothing of this repository's, and `D-135` places a domain-neutral mechanism there.

## Consequences

- `KWB-118` makes the change, after `KWB-117` moves the pin to a revision that has the type.
- A later change to XVPE's `Coverage` reaches this repository only through a pin move, which is
  measured alone (`D-007`'s amendment), so a change in its meaning cannot arrive unannounced.
