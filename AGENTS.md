# Working this repository as an agent

This file answers **where the truth is and how to act on it safely**. It does not answer
what the truth is. This repository is new; it borrows this shape from
`f:/repos/nomos/AGENTS.md` rather than inventing its own, because the shape — route to
authority, do not restate it — is not specific to what Nomos analyzes, and repeating it
here as a summary would be an unchecked copy of a file this repository can simply point at.

Read the authority. Do not infer the architecture from the code nearest to your task.

## Which authority answers which question

| Question | Read |
|---|---|
| What exists, what owns what, which band may depend on which | `README.md` |
| Is that still true? | `tests/contract/` — it asserts the README's tables against the real workspace, both directions |
| Why was it decided that way? | `docs/records/` — one record per decision, and they are canonical |
| What does KWB own versus Nomos, XVPE, or repository tooling? | `f:/repos/nomos/docs/records/ARC-ECOSYSTEM-001-...md` — this repository does not restate it |
| What work is available, claimed, blocked, or already refused? | `work/ledger.json` |
| What must pass before I finish? | the claimed item's own verification predicate |
| Which files am I allowed to change? | the territory of the item you hold, and nothing else |

When two of those disagree, the mechanical one wins and the disagreement is a defect worth
an item.

## The loop

1. `git log --oneline -5` and `git status`. Other sessions may be working this tree.
2. Read the board. Pick an item, or add one — an item that reserves nothing is refused,
   because it would exclude nobody while looking like work.
3. Claim it.
4. Read only the authorities your item needs.
5. Implement inside your territory. Make the smallest change that satisfies the item's
   `done_when`, and do not introduce a second authority for something already governed.
6. Run the item's predicate yourself, plus the tests your change actually reaches.
7. Finish through the ledger. Do not hand-edit an item to Done.
8. Commit the paths you touched — explicitly, never `git add -A`. Re-read the board.

A decision that outlives your item belongs in a record, not in a comment and not in this
file.

## Operating hazards

**No path dependency on `f:/repos/xvpe`.** Its foundations tier does not currently
compile (`xvpe-dataflow` fails at 62 errors as of the last check from the Nomos side).
Nomos's own `D-130` takes no XVPE dependency before its Phase 5 and never by path, and
this repository takes the same caution for the same reason, without inheriting Nomos's
phase numbers — this repository has not built anything yet to phase.

**`nomos-contracts::KnowledgeReferenceId` is the one thing this repository must not
silently redefine.** Nomos's `D-137` admits an opaque, KWB-minted identifier for a claim,
rationale, or decision, on the understanding that its value is whatever string identity
`kwb-model` derives — not a new wrapper type. If `kwb-model`'s identity scheme changes
shape in a way that stops being a plain opaque string, that is a decision to record here
and reconcile against `D-137`, not a detail to change quietly.

**The .NET prototype is read, not ported.** `D-001` is why. A requirement from
`C:/Users/kmett/source/repos/KnowledgeWorkbench` is met, diverges, is deferred, or is
declined — explicitly, per requirement — as this repository is built. Nothing is carried
over by silently translating a C# class.

## What this file is not

It is not a place to record architecture. If you find yourself about to paste a table of
crates or a list of decisions into this file, the thing you actually want is a link to the
file that already holds it — and if no file holds it, the change belongs there rather than
here.

It is not a task list either. `work/ledger.json` is the only board.
