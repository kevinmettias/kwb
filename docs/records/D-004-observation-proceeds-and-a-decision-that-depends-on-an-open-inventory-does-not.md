---
id: D-004
type: decision
title: Observation proceeds, and a decision whose answer depends on an open reconciliation does not
status: accepted
version: 4
authority: canonical-normative-record
tags:
  - prototype
  - sequencing
  - ecosystem
relations:
  - target: D-003
    type: relates-to
  - target: D-001
    type: relates-to
---

# Observation proceeds, and a decision whose answer depends on an open reconciliation does not

## Decision

Two reconciliations feed this repository's design: the prototype requirement inventory, closed
by `KWB-9`, and the reference miner in `f:/repos/xvpe`, which is still running experiments.
While any input remains open, the governing rule is not "nothing moves." It is:

> **Observation can proceed. A decision whose answer depends on the open input cannot.**

So this record names two lists, and neither is a restatement of the other.

> **Version 2, 2026-09-12.** Two lists were not enough, and several entries were on the wrong
> one. The amendment at the end of this record supersedes the sorting below without touching
> the rule above, which nothing measured contradicts. **Read the amendment before acting on
> either list.** The lists are left as written rather than edited in place, because what
> changed is worth being able to see.

> **Version 3, 2026-09-13.** The stranded category is empty. Both entries it still held have
> had the condition version 2 named for them met, in the instrument rather than here. **Read
> the second amendment too**, and read it for the mechanism as much as the sorting: the guard
> `KWB-62` built to stop this list rotting cannot see the half of it that just rotted.

> **Version 4, 2026-09-13.** Version 3 dated one of those two conditions a commit too late. The
> coverage capability is inside the commit this repository **already pins**, not a commit ahead
> of it, so nothing has to move to reach the evidence. The third amendment corrects that and
> says what it cost.

### Safe to proceed on now

The prototype's evidence here is stable at *Exercised* or *Operationally validated* (`D-003`),
and the reference miner's remaining experiments cannot invalidate it:

```text
governing records and ledger tooling
identity mechanics
basic source and document representation
provenance primitives already genuinely established
store boundaries and the write door
deterministic derivation and version mechanisms
validated value types
the operationally validated relation algebra
translation of proven incidents into properties and tests
infrastructure already justified by this repository's own records
```

### Held for reconciliation

The reference miner is actively generating requirements for all of these, and the prototype's
own evidence for them is thin — in the sharpest case, `Domain/Evidence` is one file of 33 lines:

```text
the evidence ontology
a canonical Knowledge IR
knowledge scopes
canonical claim identity
semantic reconciliation
admission
coverage and exhaustion
the confidence / epistemic-strength model
promotion and authorization
cross-source corroboration
the relation kernel, wherever prototype evidence is weak
```

Held means: a record may capture an observation about one of these, and a ledger item may
*reserve* one. What is refused is architectural concretization — a type, a schema, or a closed
vocabulary asserted as the answer — before the inputs stabilise.

The two tracks run concurrently and synchronise only when work approaches that line. `KWB-9`
did not wait for the miner, and nothing else needs to either.

## Rationale

An "open input" is not an excuse to stop, and treating it as one is how a reconciliation pass
becomes a freeze. Most of the work this repository has to do — its records, its ledger tooling,
its identity mechanics, its store boundary — depends on prototype evidence the miner has no
bearing on whatever. Blocking those on an unrelated experiment costs real progress and buys
nothing.

The converse failure is the one this record actually exists to prevent, and it is specific: an
implementing model that starts concretizing architecture while the governing requirement is
still incomplete will produce something plausible, and plausible-and-early is harder to displace
than absent. The held list is exactly the set where that would happen, and the evidence model is
the clearest case — the miner is still discovering what the distinctions have to be
(mechanically verified, mechanically unverifiable, model observed, quote-verification
provenance, abandoned work as interpretation debt, modality fidelity), so designing KWB's
evidence model against 33 lines of prototype would replace one speculative model with another
and give the second one a repository to live in.

The rule draws the line where it is checkable. *Is this an observation, or a decision that the
open input could change?* is a question about the artifact being produced, answerable by looking
at it, rather than a question about how confident its author feels.

`AGENTS.md` already requires that a session not "introduce a second authority for something
already governed," and that is why the two lists live in one record rather than being repeated
in each item that touches them. An item names the list; it does not copy it.

## Consequences

`KWB-10` (storage semantics), `KWB-11` (the Knowledge IR) and `KWB-12` (knowledge scopes) each
reserve held ground. They are real items and may be claimed, but each one's `done_when` is a
*requirement* record — what the prototype proved is necessary, or what must survive between
stages — and not a design. An item on the held list that produces a schema has exceeded itself.

`KWB-8` is the one item whose gate is now mechanical rather than prose: its precondition was the
prototype inventory, `KWB-9` is that inventory, and the dependency edge exists. Whether this
repository depends on XVPE turns on what the Rust KWB will require and which of those
requirements XVPE is the right authority for under `D-135` — and the second half of that is
still partly on the held list, so closing `KWB-9` does not by itself close `KWB-8`.

This record is revised when the miner's relevant experiments close, by moving entries from the
second list to the first. It is not revised by an item that finds one entry inconvenient.

## Amendment: Two Lists Were Not Enough, 2026-09-12

Measured against the reference miner at xvpe `dev` `a7eee3c6e`, and against this repository's
own closed items. The rule this record exists for is unchanged. What was wrong is the sorting
and, in three cases, the dependency an entry was waiting on.

**A held entry can be in four states, not two.** The original sort had no way to say the
difference, and the difference decides whether waiting is a plan.

### Closed — the miner measured it, and these move to *safe to proceed*

**The evidence ontology, on its verification and provenance axis.** Three states are measured
and distinguished: a quote found in the passage's extracted text; a quote that cannot be
checked because the page's words are pixels; and a quote offered but absent from the passage,
which is a refusal rather than a verification state. The load-bearing lesson is sharper than
the trichotomy: **verifiability is a property of the evidence available, not of the delivery
mode, and the two must not share a predicate.** The miner keyed its check on *"what input must
the model be sent"*, which is true of figure-heavy pages as well as scans, and so exempted
**68 of 111** retained claims from a check they could have passed — carrying a value that said
the passage had no extracted text, which was false for all 68. Re-run under the corrected
predicate: **53 verify, 15 do not.** The conflation was withholding real grounding and hiding
real fabrications at the same time.

**Knowledge scopes, on the question of what a scope belongs to.** Closed by `D-010`, and it
needed no miner input at all — it is forced by `D-002`'s source exclusion, which this
repository decided long before. The promotion *threshold* stays open; see below.

### Pending — the miner is generating input, and the original rule applies unchanged

**The confidence and epistemic-strength model.** The decisive measurement so far: across two
corpora under one contract, the model's self-graded placement held flat at 0.33 against 0.34
while independently *resolved* placement fell from 0.50 to 0.07, with 78% of `named` bearings
resolving to nothing on the larger cell. A grade a source assigns its own assertion is not a
measurement of that assertion's standing. This is exactly the kind of result that would have
made an early answer wrong, which is the argument for the hold working as intended.

**Promotion and authorization**, which `D-010` shows depends on the above.

**Admission.** The miner's screening and abandonment data is input to it, and already carries
one correction: every recorded abandonment is attempt 1 of a configured 4, because
`InferenceError::Is_Retryable` is `matches!(Transport | Throttled)` and a nonconforming answer
is terminal. So a 27–36% abandonment rate is one contract failure per chunk on the only
attempt, not a capability ceiling. **Abandoned work is interpretation debt about the contract,
not evidence about the material.**

### Stranded — waiting on an instrument that cannot answer, with what would unstrand each

An entry here is **not** progressing, and the original wording promised it was.

**Cross-source corroboration.** The miner cannot produce this, by construction and by design.
`SourceClaimIdentity::Derive(source, window, claim)` absorbs **the source path and the page
window** along with the claim, so one claim asserted by two sources is two identities that can
never meet, and `FindingIdentity` is derived from that. This is the deliberate **inverse** of
this repository's own scheme, where `D-002` excludes the source precisely so that two books
asserting one claim become one claim with two citations. Neither is wrong for its own job —
the miner is addressing *where a thing was read*, and KWB is addressing *what a thing is*.
*Unstranded by:* the miner acquiring a second, source-excluding claim identity. That is a
change to the instrument, not a longer run of it.

**Coverage and exhaustion.** Both mining products write an append-only journal that **nothing
reads back**; no `coverage`, `resume` or `redundant` symbol exists in either. So *"nobody
looked"* is indistinguishable from *"looked and found nothing"* — which is the `Barren` versus
`Skipped` distinction this repository already owes `KWB-4`, occurring in the instrument that
was supposed to inform it. *Unstranded by:* a coverage ledger that is read, not another sweep.

**A canonical Knowledge IR.** `D-009` records the measurement and the reasoning. The miner's
experiments concern quote verification, fidelity, grounding, bearing grades and abandonment;
none bears on what representations must survive between *this repository's* ingestion stages,
and no length of run would change that. *Unstranded by:* `KWB-3`, now closed, and `KWB-5`.
Both internal, both reachable without the miner.

**Condition met:** `KWB-3`, `KWB-5`

**No longer stranded, as of `KWB-62`.** `KWB-5` closed at unix `1789254256`, **495 seconds after
this amendment was written** — so this entry was correct when authored and false eight minutes
later. That is the failure this amendment predicts by name, arriving faster than anyone could
have re-read the list by hand, which is the argument for the guard `KWB-62` built rather than an
argument about care. The subject is decidable here and is not decided; `D-009`'s own amendment
says what it now is.

### Answered elsewhere — closed by this repository while this record still held it

**Canonical claim identity.** `KWB-1` shipped `ContentIdentity` and `Derivation`, with the
source excluded and the exclusion recorded as a value, and `KWB-15` added the opaque door.
That *is* the canonical claim identity mechanism. It was built while this list said the
subject was held, and nothing bad came of it because the work was the mechanism rather than a
vocabulary — but the record should not have been claiming to hold it.

The lesson is worth more than the correction: **a list of held subjects goes stale in both
directions**, and this one went stale silently because nothing re-reads it against the board.

### Unchanged, and still held

Semantic reconciliation, and the relation kernel wherever prototype evidence is weak — though
`D-011` has now stated what that kernel must guarantee, which is the requirement-record form
this hold has always permitted.

### What this amendment does not do

It does not weaken the rule. *Observation can proceed; a decision whose answer depends on the
open input cannot* survives every measurement above and is what made the stranded entries
findable at all. It designs nothing for any held subject.

And it does not claim the categories are now correct — only that they are now expressible. The
next revision should be triggered by something re-reading this list against the board, because
the failure this amendment corrects was not a wrong judgement. It was a list nobody checked.

## Amendment: The Stranded List Is Empty, 2026-09-13

Measured against xvpe `dev` `7ec036c52`, committed and pushed. The rule is unchanged and no
held subject is decided here.

The previous amendment asked for its own successor in as many words — *the next revision should
be triggered by something re-reading this list against the board*. This is that re-read. It
finds the stranded category empty, and it finds that out by hand, which is the more important
half of what follows.

### Cross-source corroboration — no longer stranded, and now *pending*

*Unstranded by*, as version 2 wrote it: **the miner acquiring a second, source-excluding claim
identity. That is a change to the instrument, not a longer run of it.**

The instrument has one. `xvpe-corpus-ledger` ships `ClaimAssertion::Of(assertion)`, which folds
in the assertion and nothing else — the identity `D-002` decided for this repository, arriving
independently in the tool that feeds it. Its own documentation names `SourceClaimIdentity` as
the contrast and gives this record's reason for the split: two books asserting one thing produce
two identities there and never learn of each other. Beside it, `ClaimLedger` carries
`Support_For`, `Corroborated`, `Novel` and `Restatements` — the support count version 2 called
absent, and the falling rate of new claims it called unmeasurable while every claim is unique by
construction. `xvpe-brainstorm-synthesizer`'s binary reads `Corroborated()`. This is wired, not
merely written.

**Why *pending* and not *safe to proceed*.** The instrument can now answer the question; it has
not yet answered it. The entry directly above this one is the reason that difference is worth a
category: the confidence model's numbers moved sharply between two corpora under one contract,
and a corroboration count read off one small run is exactly the early answer this record exists
to refuse. What ends a hold here is a measurement, never a capability.

### Coverage and exhaustion — no longer stranded, and now *pending*

*Unstranded by*, as version 2 wrote it: **a coverage ledger that is read, not another sweep.**

There is one. `CorpusLedger::Has_Ever_Seen` is version 2's own *nobody looked* versus *looked
and found nothing* distinction, as a method. `Settlement_Of`, `UnitSettlement` and
`SettlementCounts` partition every unit into answered, unanswerable and work remaining — which
is the `Barren` versus `Skipped` distinction `KWB-4` owes, settled in the instrument before it
was settled here. `xvpe-reference-miner`'s binary reads `Settlement_Of`, and
`xvpe-mining-memory` carries resume with tests over it. Version 2's finding that no `coverage`,
`resume` or `redundant` symbol exists in either product was true when written and is now false
of both.

**Why *pending*.** The same reason, and one this entry adds: a coverage ledger reports
exhaustion *against the corpus it was pointed at*. Whether the estate is exhausted is a claim
about the estate, and this is the instrument for making that claim rather than the making of it.

### The part worth keeping is the mechanism, not the sorting

Both entries were correct when written and both went false, and **the guard built to stop
precisely this could not see either of them.** `KWB-62` built
`Test_Every_Condition_A_Record_Calls_Met_Should_Be_Met_On_The_Board`, which reads a line
beginning `**Condition met:**` and holds it against the board. Its own doc comment explains why
it reads nothing else, and that explanation is correct: the remaining condition-like passages
are not conditions, and a guard that is wrong half the time is switched off.

What it leaves uncovered is this entire category. A stranded entry's condition is
`*Unstranded by:*` followed by a change to **another repository** — not a board item, and not
something any test here can reach, because `D-007` adopts xvpe by git reference precisely so
that nothing here reads its working tree. So the covered half of this list is the half that
cannot rot this way, and the uncovered half is the half that just did. That is not a defect in
the guard. It is the guard's scope, stated here so the next reader does not mistake a
machine-checked list for a checked one.

The drift is visible locally even though the instrument is not. This amendment measures at
`7ec036c52`; version 2 measured at `a7eee3c6e`; `kwb-platform-xvpe` pins `8ff98a8fd`. Three
commits, and no record in this repository says the pin ever moved. **A record that states the
SHA it was measured against is making a claim that expires, and nothing here holds that claim
against the pin.** Four records state such a SHA, and three of them — `D-007`, `D-012` and
`D-014` — name `a7eee3c6e` and nothing newer, so every measurement they report was taken
against a commit the workspace no longer builds against. That is a mechanism this repository
lacks and could have; it is named here and not built, because building it is its own item and
this one is a re-sort.

### What this amendment does not do

It does not touch the rule, which nothing measured contradicts, and it decides nothing for any
held subject.

It does not move **semantic reconciliation**, which stays held exactly where version 2 left it.
No measurement above bears on it; nothing in either repository produces an embedding; and
`KWB-82` waits on that instrument and on this hold, in that order, so nothing here reaches it.

It does not promote either entry to *safe to proceed*. The distinction between *can be answered
now* and *has been answered* is the substance of this amendment rather than caution about it,
and collapsing the two is how a hold ends early.

## Amendment: The Coverage Capability Was Already Inside The Pin, 2026-09-13

The amendment above unstranded coverage and exhaustion against `xvpe-corpus-ledger` at
`7ec036c52`. That is correct, and it is late by one commit. The difference is not bookkeeping:
it decides whether this repository has to move in order to see the evidence. It does not.

**Where the capability actually arrived.** `coverage_vocabulary.rs`, `journal_reader.rs` and
`observed_coverage.rs` first appear in `xvpe-event-journal` at **`8ff98a8fd`** — the commit
`kwb-platform-xvpe` has pinned since `KWB-72`. At `a7eee3c6e` no `CoverageVocabulary`,
`ObservedCoverage` or `JournalReader` exists anywhere in XVPE's tools or in that crate. At
`8ff98a8fd` both mining products import all three and both declare a `CLAIM_COVERAGE`
vocabulary, and `xvpe-brainstorm-synthesizer` dispatches a `coverage` command over a
`JournalReader` and an `ObservedCoverage`.

**So version 2's finding was true when it was written, and the bump is what falsified it.**
Version 2 measured at `a7eee3c6e` and reported that nothing read the journals back. That was
accurate. `KWB-72` then moved the pin onto the very commit that answered it, while doing
something else, and nothing connected the two. `KWB-96` records that no record in this
repository mentions `8ff98a8fd` at all. This is what that cost: a record calling a subject
stranded while the thing that unstrands it sat inside the dependency the workspace compiles.

**`xvpe-corpus-ledger` is a second instrument and a later one.** It lands at `7ec036c52`, it is
a claim ledger as well as a coverage one, and version 3's description of it stands as written.
What version 3 got wrong is the date, and therefore the reach — a reader would conclude the
evidence needs a bump to see. **That error is `KWB-95`'s, which is to say mine**, and it is
recorded here rather than edited away because a list of held subjects that quietly corrects its
own dates is worth less than one that shows them.

The entry stays *pending*, for the reason version 3 gave and this correction does not touch: the
instrument can answer the question and has not yet answered it at scale. What changes is that
answering it needs a run, not a bump.

## Alternatives Considered

**Blocking all design work until both reconciliations close** was rejected: it would stop
identity mechanics, the write door and this repository's own records — none of which the miner
touches — and it would make the inventory itself illegal to write, since that is design work by
any rule broad enough to catch the rest.

**Leaving the sequencing in `AGENTS.md`** was rejected because `AGENTS.md` is explicit that it
holds how to act safely and not what is true, and a list of held subjects is a claim about the
state of the design, which will change. `AGENTS.md` points at records; it does not carry them.

**Recording the hold inside `KWB-9`** was rejected because the hold outlives that item. `KWB-9`
closes when the inventory exists; the hold persists until the miner's experiments do.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `D-005`
- `D-006`
- `D-007`
- `D-009`
- `D-010`
- `D-011`
