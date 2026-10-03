---
id: D-027
type: decision
title: A claim a model proposes is admitted only on a quote found in its passage, a model's judgement stays a reading, and the rest of the epistemic layer waits on its measurements
status: accepted
version: 2
authority: canonical-normative-record
tags:
  - epistemic
  - grounding
  - admission
  - xvpe
relations:
  - target: D-025
    type: relates-to
  - target: D-026
    type: relates-to
---

# A claim a model proposes is admitted only on a quote found in its passage, a model's judgement stays a reading, and the rest of the epistemic layer waits on its measurements

## Decision

This record verdicts rows C-15 to C-22 and C-47 to C-54 of `docs/corpus/prototype-capabilities.md`
for goal G5 of `D-021`. Most of them are subjects `D-004` holds or `D-011`'s held half, and they are
verdicted deferred with the measurement each waits on. Three are not held, and are decided here.

**Grounding (C-16): a proposition is admitted only when its quote is found in the passage it was
read from.** Every proposition a model proposes carries the passage text it rests on. If that quote
is found in the passage's text, the proposition proceeds to admission. If it is not, that proposition
— not the reading — is refused, counted, and recorded with its reason, and nothing of it is admitted.
`D-004`'s amendment closed this axis on the miner's measurement: verification is a property of the
evidence available, and *a quote offered but absent from the passage is a refusal rather than a
verification state*. The matching rule is XVPE's — `xvpe-corpus-extraction`'s quote check, which
reads quoting conventions (markdown markers, elisions whose every piece is present in order) and
refuses an invented piece — and the states are `xvpe-evidence`'s, so under `D-026` this repository
adopts grounding with XVPE's mining core rather than writing a second matcher. **What to do with a
quote that cannot be checked** — a passage that is a page image, read by a visual protocol — is
admission policy, which `D-004` holds; no text protocol meets it, because `D-025` reads no such
passage as text.

**A one-source repeat is one assertion (C-16).** A proposition a source states twice is one claim
and one assertion by identity already; within a run, the repeat is refused as a repeat and counted,
never recorded twice. (The audit's F19 found the log recording it twice.) *Identity overruling a
mispointed duplicate* is adjudication, and waits with C-15.

**A run whose model fails is failed (C-21).** When the model caller stops a run — the arm cannot
conform, the spend ceiling is reached, or every attempt failed — every document the run did not read
is recorded `Unmet` with that reason (`D-024`), nothing is admitted from a refused read (the
extraction contract), and the command exits 1. The caller's breaker and budget are
`xvpe-ai-pipeline`'s, adopted by `KWB-171`.

**A model's judgement is carried as a model's judgement (C-22).** Anything a model decides — a
proposition, a verdict on a candidate, a grade — is a reading under `D-015`: its own fact, with the
reader and protocol that produced it, never a field on a claim, an assertion or a standing. A claim
records what a source asserts; a reading records that a model said so.

### The verdicts

| Row | Verdict |
|---|---|
| C-15 | **deferred** under `D-004`'s admission hold. It waits on the miners' adjudication measured on a run whose `adjudications_disagreed` is zero and `candidates_withheld` small — the corpus-exhaustion program's own bar. A scratch copy of a corpus can supply it; beyond a copy it needs model runs, which the owner's instruction of 2026-10-03 withholds for now. Under `D-026`, the base verdicts are XVPE's (`xvpe-corpus-adjudication`) and this repository's catalogue extends them. |
| C-16 | **deferred**, condition K3 (XVPE's `mining-core.md`), its form decided above; nothing reads by model here before then. `KWB-185` holds it. |
| C-17 | **deferred**: *corroborated* and *disputed* are corroboration and adjudication, which `D-004` holds, and a contradiction register is XVPE's corpus-exhaustion step 6, being built. A claim's *superseded* standing follows its concept's, as `D-024` C-26 decided. |
| C-18 | **deferred** under `D-004`'s corroboration hold. It waits on support counted on a run whose adjudication agrees, from a scratch copy, needing model runs as C-15 does. One rule stands now: a support count is never shown without its trust. |
| C-19 | **deferred** with C-15. Scoring is XVPE step X9, in progress; its expectations name their labeller, as a reference verdict must. |
| C-20 | **deferred**: measuring a model needs model runs. XVPE's conformance breaker and pinned sampling are the mechanism, adopted with `KWB-171`. |
| C-21 | **deferred**, condition K3, its form decided above. `KWB-185` holds it. |
| C-22 | **deferred**, condition K3, its form decided above: readings are built when a provider produces them (`D-015`, `D-025`). |
| C-47 – C-52, C-54 | **deferred** under `D-011`'s held half: the relation kernel, its algebra and its constraints, domain packs, inference that writes hypotheses and never edges, the relation read model, negative assertions, cross-domain hypotheses, and the closure of transitive families. They lift on `D-011`'s and `D-004`'s own triggers. |
| C-53 | **deferred**: promotion is `D-010`'s deferral. A scope is `Named` or `Unstated` today. |

## Why

**Grounding is the cheapest guard against an invented claim, and the only one that needs no
judgement.** A proposition whose quoted support is not in the passage it claims to come from is,
at best, a paraphrase nobody can check and, at worst, a fabrication; either way it must not become
a claim with a citation. The miner measured why the rule must be per proposition and must read
quoting conventions: 15 of 68 sparse-page claims there quoted text that was not in the passage, and
two in five quotes refused on a stricter check were formatting, not invention.

**Per proposition, not per reading.** One bad proposition says the model mis-quoted one thing, not
that it read a different passage; refusing the whole reading would lose the propositions that are
grounded, and the count of refusals is itself evidence about the reader.

**Held means held.** Nine of these rows are exactly the subjects `D-004` and `D-011` hold, and the
item that asked for this record forbids lifting a hold on any ground but its own trigger.

## Alternatives Considered

**A quote check written here now, before K3.** Refused: the matcher is XVPE's, and nothing here reads
by model before K3, so there is nothing for it to guard yet; writing it would be wiring it twice.

**Refusing the whole reading on one ungrounded proposition.** Refused, for the reason above.

**Admitting an unverifiable quote with a mark.** Not decided: it is admission policy, held.

## Consequences

- `KWB-185` replaces `KWB-183`, which `KWB-174` authored before this record: the same obligation —
  author G2's composition once K3 is published — with grounding (C-16), the one-source repeat (C-16)
  and the failed run (C-21) added to the composition it must author.
- `D-025` and `D-026` gain back-links.

## Amendment: Each Held Relation Row Names What It Waits On, And The Repeat Needs No Model, 2026-10-03

`KWB-187`, from the audit of `KWB-128`, found two things the decision above left short. Both are
corrected here, and no held subject is lifted or decided by it.

**The relation rows, C-47 to C-52 and C-54, each name the measurement they wait on.** `D-011`
decides the relation algebra's requirements — it is derived from the prototype's pack-load path,
which ran, and `D-004` lists it as operationally validated — and holds the type half. Its two open
questions, layer inheritance (A3) and the choice between closing a hierarchy and leaving a remainder
unclassified (B3), need inputs that do not exist yet: concepts and the relations between them,
admitted from real sources at scale rather than read off the prototype's packs. Row by row:

| Row | Waits on | A scratch copy? |
|---|---|---|
| C-47 | the type half's inputs above, and enumerating this repository's own kinds, which `D-011` rejects doing before them | **yes**: concepts and relations admitted from a scratch copy of the reference library, read through the mining core at K3, supply them — writing no durable record, which `D-021` allows. Beyond a copy it needs K3 and model runs, which the owner's instruction of 2026-10-03 withholds for now |
| C-48 | a pack format, which `D-017` leaves undecided and which `D-011` puts after the kernel | follows C-47 |
| C-49 | relations admitted under C-47's kinds, for inference to run over | follows C-47 |
| C-50 | the same | follows C-47 |
| C-51 | the same, and the falsifier a negative assertion must carry, which `D-011`'s algebra names and C-47's kinds instantiate | follows C-47 |
| C-52 | C-47 and C-48: connections across domains need more than one domain's kinds | follows C-47 and C-48 |
| C-54 | a family certified transitive, which `D-011` B4 says the prototype's vocabulary could not even assert, and C-47's kinds to certify it on | follows C-47 |

So every one of them reduces to one measurement — what a real corpus's admitted relations need of a
type kernel — that a scratch copy can supply once K3 composes reading and the owner permits model
runs on copies. None waits on G8.

**The one-source repeat (C-16) needs no model, and is implemented now.** The decision above routed
the repeat rule through K3 with grounding. It should not have: the audit's F19 measured a repeat with
hand-typed `--says` and no model at all, so the rule binds the reader this repository already has.
`KWB-186` implements it now — within one run an assertion already published is not published again,
and the repeat is counted in the report. `KWB-185`'s composition therefore owes only what needs K3:
grounding and the failed-run exit, and the repeat for readings a model produces is the same rule
`KWB-186` builds, applied to them.
