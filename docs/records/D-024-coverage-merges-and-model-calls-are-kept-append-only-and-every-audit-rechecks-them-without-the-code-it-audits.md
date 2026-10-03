---
id: D-024
type: decision
title: Coverage, merges and model calls are kept append-only, every audit re-checks them without the code it audits, and nothing is repaired by deletion
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - audit
  - coverage
  - durability
  - epistemic
relations:
  - target: D-004
    type: relates-to
  - target: D-023
    type: relates-to
---

# Coverage, merges and model calls are kept append-only, every audit re-checks them without the code it audits, and nothing is repaired by deletion

## Decision

This record verdicts rows C-13 and C-24 to C-35 of `docs/corpus/prototype-capabilities.md` for goal
G4 of `D-021`. Four rules govern every verdict, and each is the inverse of a way the prototype lost
data while reporting success.

1. **What a run learned is appended, never overwritten.** Coverage, merges and model calls are
   records in append-only logs kept through the same record log as the publications (`D-019`), so a
   torn tail is a refusal there too. The current answer to a question is the latest record in file
   order; earlier ones stay as history.
2. **An audit re-checks a fact without the code that produced it.** It reads the records itself and
   applies the rule a record states, written in the audit, never by calling the stage it audits. An
   audit that asks the producer's own predicate again confirms the producer's defects.
3. **Every audit has a verdict, and the verdict sets the exit code.** `clean` exits 0, `failed`
   exits 1, and `blind` or `vacuous` exits 2. An audit that cannot read its input is `blind`, never a
   crash and never `clean`.
4. **Nothing is repaired by deletion or overwriting.** A repair is a new record: re-reading a
   document appends coverage, reopening a concept appends a publication.

### The verdicts

| Row | Verdict | What is decided |
|---|---|---|
| C-13 | **deferred**, and **diverges** for locations | A source's quality rating is the epistemic-strength model `D-004` holds pending the reference miner's measurements; a scratch copy cannot supply them, because grading needs the real corpus, which the owner holds. When built, a rating names who rated it, and an unrated source has no number — the prototype's two defaults for "unrated", 0.5 and 0, are the failure. A location belongs to a reading (`D-015`), not to a source: `KWB-156` makes it true at page grain, and finer ranges wait on `KWB-125`'s passage decision. No source-location table. |
| C-24 | **deferred** for alias merging; **diverges** for the merge log | Merging by name variants is grouping over a relation that is not transitive — `z m` bridged `zero mass` and `zero matrix`, 144 of 579 merges were wrong — and `D-011` B4 forbids grouping over an uncertified relation. Concepts group by identity equality only. When variant derivation is built, a variant is a *proposal* a person supersedes on, never a merge a stage performs. The merge log is the publication log: every supersession record carries the loser, the successor and the reason (`KWB-37`), and after `KWB-145` its time. A second log would be a second authority, and the prototype's was an editable table. |
| C-25 | **met**, in this form | `kwb audit merges` replays the log through its own reading of the records and checks every supersession at the point it was published against the rules `D-023` states — the concept was current, the successor held and current, the two distinct, the reason non-empty — and against the graph today: a successor since closed leaves its loser pointing at nothing current, which is a finding. **Undo is `kwb reopen <concept> --store <dir> --because <reason>`**: it publishes the concept's next version under a new standing, `Reopened { because }`, which is current to every query. The supersession record stays. Nothing else needs restoring, because a closure moves no claim (C-26), which is exactly why the prototype's undo could not restore claims and this one has nothing to restore. Reopening a current concept, an unknown concept or with an empty reason is refused, exit 1, the log unchanged. |
| C-26 | **diverges** | Closing requires a reason (`KWB-37`), and a closed concept's claims do not follow its successor. They stay with the concept they were asserted of, held, leaving the current world with it and reachable in the historical one with their standing (`KWB-65`). Moving them would re-derive their identities, which is how the prototype's merge deleted their lineage, and would follow a relation nobody certified (`D18`). `D-023` already says supersession attaches nothing to the successor. |
| C-27 | **met** for claims, **declined** for chunk links, **deferred** for the rest | A person states claims with `kwb admit --says`, and a claim is always asserted by a source (`D-002`, `D-010`), so a concept with no assertion — the prototype's `add-concept` — is declined: it has nothing to cite. Chunk links are declined: there is no chunk entity, and a reading's location is `D-015`'s. Relations wait on `D-011`'s held half (`KWB-128`); domains on `KWB-129` (taxonomy, C-41); annotations and summaries on `KWB-129` and `KWB-134`. |
| C-28 | **met**, in this form | `kwb admit` appends one **coverage record** per admission to `coverage.log` in the store root, beside `publications.log`: the document's content address (the unit), the reading protocol and the reader that applied it (the rule and its version), the outcome's name, its count — findings, or material examined — its reason where the outcome carries one (`D-020`'s `&'static str`, never a free-text sentence matched later), and the run's time as evidence. **Material examined is what the reader reports it examined**: the reading result carries the number of passages read, `Barren` records that number, and a reader that examined nothing is `Unmet`, never `Barren` — the audit's F8, and the prototype's 1,367 rows. Whether a *corpus* is exhausted stays held (`D-004`); this records what each admission did, which is the instrument and not the answer. |
| C-29 | **met**, in this form | `kwb audit coverage`'s independent expectation is the content store: every document it holds, enumerated from the store and not from admission, should have a coverage record under the protocol the audit is asked about, and every coverage record should name a document the store holds. Findings: a document never read; a document read only under another protocol; a record naming a document the store no longer holds, which is content lost; a `Barren` with nothing examined. Denominator: documents held. **The audit never writes.** Stale coverage is answered by re-admitting the document, which appends a new record; the prototype's `--reopen` hard-deleted rows, and is declined. |
| C-30 | **deferred** | `D-015`: a reading is its own fact about an assertion, built when a provider produces readings; that waits on `KWB-125`. Until then the publication log is the whole lineage: an assertion names its claim and the document it was read from, and staleness is computed against those inputs (`D-006`). When readings are built, their log is append-only and their audit re-derives, rather than checking that fields are non-blank, which is all the prototype's "replayable" meant. |
| C-31 | **met** | One finding shape for every audit: the subject's identity, the check's name, the reason, and the evidence. The verdict is computed from what was examined, what was eligible and what was found — `blind` (eligible, none examined), `vacuous` (nothing eligible), `failed` (a finding), `clean` — with no default and no severity: every finding fails the audit, because a severity filtered on display while the verdict counted everything is how the prototype's `findings` failed on informational rows. |
| C-32 | **diverges** | `kwb audit all` runs every audit and reports each verdict with its denominator; that is the corpus's health. There is no separate health computation, which in the prototype compared call outcomes against a string the recorder never wrote and counted every call failed. Dashboards and an HTTP surface are `KWB-126`'s (G3). |
| C-33 | **met** by adoption | Every model call `kwb-extract` makes goes through `xvpe-ai-pipeline`'s caller — budget before request, retries, accounting — through `kwb-platform-xvpe`, and each is appended to `calls.log`: role, model, protocol, document, the outcome's name computed from what happened, input, output and cache tokens as the service reported them (an unreported count is recorded absent, never zero), integer cost from XVPE's price rates, and the wait. Prompts and answers are not copied into it: the recorded answers are the replay recording, and a second copy is a second authority. |
| C-34 | **met** through XVPE step X9 | A golden-set scorer is domain-neutral, so it is XVPE's (`D-135`). X9 is specified in XVPE's `kwb-parity.md`: expectations name their labeller; matching is one-to-one under a declared rule, exact normalized identity first; an empty golden set or an empty output yields no score rather than a perfect one; and the scorer reports the expectations missed (content lost) and the productions matched by nothing (claims to examine for invention). The item that scores `kwb-extract` against recorded answers is authored once X9 is published and pinned. |
| C-35 | **diverges** | Nothing derived is stored apart from the publication log; the graph is rebuilt from it on every run (`D-014`), documents stay in the content store, and so a rebuild deletes nothing. The prototype's rebuild deleted first and rebuilt through model stages, and destroyed its calibration set. Embeddings do not exist yet (G3). |

## Why

**Each rule above is a measured prototype failure turned around.** Read at the prototype's last
commit, `f22f255`:

- Its merge log was an editable table, its coverage ledger deleted and re-inserted on every re-run,
  and only claim lineage was append-only — so the history an audit needs was the first thing a re-run
  destroyed.
- Its merge audit re-asked the normaliser's own variant predicate and its coverage audit re-ran the
  stages' own passage classifier. Its output said so: "every merge is one the rule WOULD make — not
  that the rule is right". A predicate defect — `C` merging into `C++` — re-derives to the same
  answer.
- Its coverage, derivation and health audits always exited 0. Its merge audit's blindness was an
  unhandled exception, and a rebuild script admitted anyway after it printed "144 MERGES THE RULE
  WOULD NOT MAKE".
- Its repairs were destructive: `--undo` overwrote concept rows and could not restore claims, and
  `--reopen` hard-deleted coverage rows, each leaving no record that it had happened.
- Its zero values were the good case: `Yielded = 0`, `Succeeded = 0`, and an empty golden set scored
  1.0.

This repository already keeps documents content-addressed and the graph as an append-only log
(`D-014`, `D-019`); the four rules extend the same discipline to the three things the prototype kept
mutable.

## Alternatives Considered

**A separate merge log beside the publication log.** Refused: the supersession records already carry
everything a merge log would, and two logs of one fact drift.

**Coverage as a fourth kind of publication record.** Refused: replay rebuilds the graph, and coverage
is not graph state. Mixing it in would make every replay parse what it does not use, and would make
the graph's log answer a question that is not about the graph.

**An audit that repairs.** Refused for coverage and merges alike: an audit that writes is a stage, and
the next audit would have to audit it.

**Severity on findings.** Refused for now: nothing here produces a finding that should not fail its
audit, and the prototype shows what a severity filtered at display time does to a verdict.

## Consequences

- Implementation items, each with its territory, a `done_when` naming what is forbidden, and a
  predicate: `KWB-165` (the reader reports what it examined), `KWB-166` (`coverage.log`), `KWB-167`
  (the finding shape and `kwb audit`), `KWB-168` (`kwb audit merges`), `KWB-169` (`kwb reopen`),
  `KWB-170` (`kwb audit coverage`) and `KWB-171` (model calls through `xvpe-ai-pipeline` and
  `calls.log`).
- XVPE step X9 is specified in XVPE's `docs/plans/programs/ecosystem-substrate/kwb-parity.md`.
- `D-023` is amended: it left reopening undecided until a record decided an act that carries a
  reason, and C-25 is that act.
- A corpus being exhausted, a source's quality, alias merging and readings stay where `D-004`,
  `D-011` and `D-015` hold them.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `D-025`
