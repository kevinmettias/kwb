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
| What was measured, and on what? | `docs/corpus/` — surveys of the prototype and of Nomos. Evidence the records cite, kept apart from them because a measurement outlives the decision it informed |
| What has this repository learned about how it works? | `docs/observations/` — findings about the process rather than the product. Open until something acts on them, and they are not decisions |
| What does KWB own versus Nomos, XVPE, or repository tooling? | `f:/repos/nomos/docs/records/ARC-ECOSYSTEM-001-...md` — this repository does not restate it |
| Where does code that is *not* about knowledge get written in the first place? | `f:/repos/nomos/docs/records/D-135-...md` — the later record, and it narrows the one above |
| What work is available, claimed, blocked, or already refused? | `work/ledger.json` |
| What must pass before I finish? | the claimed item's own verification predicate |
| Which files am I allowed to change? | the territory of the item you hold, and nothing else |

When two of those disagree, the mechanical one wins and the disagreement is a defect worth
an item.

**The two ownership rows are not one row, and reading only the first one gets it wrong.**
`ARC-ECOSYSTEM-001` decides ownership *by semantics* and adopts `D-122`, which it quotes: a
shared mechanism moves to XVPE only after two products have demonstrated materially identical
domain-neutral semantics. `D-135` is later and narrows that gate to code which *started*
product-specific. Code designed for shared use from the outset is proposed in XVPE **the first
time it is written**, and `D-135` names this repository as one of the two products that
narrowing is for.

The second row exists because the first one cannot be followed to the second. `D-135` declares
a relation to `ARC-ECOSYSTEM-001`; `ARC-ECOSYSTEM-001` does not mention `D-135`. A session that
followed the route and stopped would read the un-narrowed gate and build a domain-neutral
mechanism here to wait for a second product — the cost `D-135` exists to avoid. Making that
relation reachable from both ends belongs to the sibling repository and is owed there;
`KWB-38` is what fixing it looks like, and it could only fix the records inside this one.

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

**A record's `version` is one plus the number of amendments it carries.** An amendment is
what a new version of a record is here: this repository leaves a superseded claim visible
beside its correction rather than replacing it, so the unit that goes out of date is a
paragraph and not a record. Add an `## Amendment` section, bump `version`. The frontmatter's
other standing field, `status`, reads `accepted` on every record because no record has yet
been superseded *wholesale* — that is the condition under which it would change, and until
one is, the field carries no signal to read. `tests/contract/tests/boundaries.rs` enforces
the first of those and states both; it is the authority, and this paragraph is the route to
it.

**That paragraph is about `docs/records/` only. An observation's two fields mean different
things, and carrying the record rule across gets both wrong.** An observation's `version`
counts revisions made **in place**: a finding that overstated itself should simply say what
was measured, so there is nothing to keep visible beside it and no `## Amendment` section to
count — `OD-LEDGER-001` is at version 2 with none, correctly, because `KWB-43` rewrote it.
The difference is not stylistic: a record's superseded claim stays because somebody may have
acted on it, and an observation is a finding nobody has acted on yet. Its `status` is
correspondingly the field that *does* carry a signal — `open` until something acts on the
finding, where a record's `status` carries none — which is the reverse of the record case and
the reason both are spelled out rather than left to symmetry.

## Operating hazards

**No path dependency on `f:/repos/xvpe`.** `D-007` is why, and it is worth reading
before acting on this line, because the reason is not the one this file used to give.
The premise stated here until 2026-09-12 — that XVPE's foundations tier does not
compile — was false when it was read, and had been retired by Nomos's own source months
earlier. What survives is a claim about coupling rather than breakage: a `path` edge
binds this repository's reproducibility to another repository's working tree. Adoption
is by git reference and commit SHA, into a crate that exists to quarantine it.

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
