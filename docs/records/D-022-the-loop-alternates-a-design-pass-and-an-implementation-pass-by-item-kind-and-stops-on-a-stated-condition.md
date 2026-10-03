---
id: D-022
type: decision
title: The loop alternates a design pass and an implementation pass by item kind, audits by commit watermark, and stops on a stated condition
status: accepted
version: 2
authority: canonical-normative-record
tags:
  - process
  - ledger
relations:
  - target: D-021
    type: relates-to
  - target: D-005
    type: relates-to
---

# The loop alternates a design pass and an implementation pass by item kind, audits by commit watermark, and stops on a stated condition

## Decision

Work on this repository runs as a loop of **passes**. Each pass is one headless Claude Code
session, started by the owner's driver (`~/.claude/tools/kwb-loop`) or by hand. It does one
half of the work, then ends with a status line the driver reads.

| Pass | Provider | Takes | Never |
|---|---|---|---|
| **design** | Anthropic — `ANTHROPIC_BASE_URL` empty | the audit, then items of kind `Decision` | implements: the phase guard refuses its writes outside `docs/**` and `work/ledger.json` |
| **implementation** | DeepSeek — `ANTHROPIC_BASE_URL` names `deepseek` | items of every kind except `Decision` | decides: an ambiguity stops it (below), it does not resolve one |

The item's `kind` is the whole routing rule. A `Decision` item's work is a record and the items
the record implies. Every other kind — `Capability`, `Correction`, `Cleanup`, `Validation` — is
bounded work inside a territory. An item that needs both halves is two items, joined by
`depends_on`.

The loop runs until `D-021`'s closure test holds, or until one of the stopping conditions below
fires.

## The implementation pass

1. `git log --oneline -5` and `git status`. Read `AGENTS.md`.
2. **The gate first.** Run `cargo clippy --workspace --all-targets -- -D warnings` and
   `cargo test -p kwb-contract-tests --no-fail-fast`. `nomos work finish` runs the first before
   any predicate and most predicates include the second, so while either is red no other item
   can finish. If one is red, take the `ready` item whose `done_when` names the failing test or
   the failing command. If none does, end the pass `blocked` and name the failure. Owning it is
   a design pass's job.
3. Otherwise, `nomos work list`, through the copy of the binary the driver names in the prompt.
   Never through `cargo run`: a predicate that runs `cargo test` would rebuild the binary running
   the verb. Take the item on the `next:` line if its kind is not `Decision`. Otherwise take the
   first `ready` row, in listing order, whose kind is not `Decision`.
4. `nomos work claim --item <id> --holder deepseek`. **Exit 0 is the only thing that means you
   have it.**
5. `nomos work show --item <id>`. Read the whole `why` and `done_when`, the territory, the
   predicate, and every record they name, **before editing anything**. A `done_when` says what is
   forbidden as often as what is required, and the prohibitions are the part that cannot be
   re-derived.
6. Implement inside the territory, and nothing outside it. Make the smallest change that
   satisfies the `done_when`. Where the `done_when` asks for a mutation proof, perform it, and
   restore the mutated file byte-for-byte.
7. Run the item's predicate and `cargo clippy --workspace --all-targets -- -D warnings`.
8. `nomos work finish --item <id> --holder deepseek`. Then commit the paths you touched: named
   on `git add`, never `-A`, with a message written to a file and a bare `git commit -F`. The
   subject begins with the item id.
9. Return to step 2 until the pass's item budget is spent or nothing is claimable.

**Stop, and do not improvise, when** an item is ambiguous, contradicts a record, needs a path
its territory does not reserve, or has a predicate that is red for a reason outside its
territory. Run `nomos work abandon --item <id> --holder deepseek --reason "<the exact
ambiguity>"`, restore every file you changed for it, and end the pass `blocked`. The abandon
reason is the message to the next design pass. Make it the sentence you would need to act on it.

## The design pass

1. `git log --oneline -5` and `git status`.
2. **Audit.** The watermark is the newest commit whose subject starts with `Audited:`. If there
   is none, it is the commit that landed this record. An item is **unaudited** when its state is
   `Done` and its `verified.verified_at` is later than the watermark commit's time. For each
   unaudited item, read its `done_when` and the commits whose subject names it, and judge the
   diff against meaning rather than green tests. Was the requirement met for the intended
   reason? Were the load-bearing mutation proofs actually performed? Are the records and
   documents still true? Does the product behave as the item says, measured? A defect becomes a
   new `Correction` item that names the audited item and the evidence. A `Done` item cannot be
   reopened, and pretending otherwise would make the board lie. Also read every abandon reason
   recorded since the watermark. Answer each by clarifying through a record, or by declining the
   item and authoring its successor. When the gate is red and no `ready` item's `done_when`
   names the failure, author the item that owns it: that is the one thing that unblocks every
   other pass.
3. Commit the audit. The subject is `Audited: <ids> — <n> accepted, <m> sent back`. The body
   names any goal of `D-021` the audit found closed, with the four checks that closed it. If the
   audit changed no file, the commit is `--allow-empty`. The commit **is** the watermark, so it
   is never skipped.
4. **Decide.** Claim the next `ready` item of kind `Decision` with `--holder fable`, do it, run
   its predicate, finish it and commit it. While the gate is red, `finish` refuses a `Decision`
   item too, so do not claim one then: the audit and the authoring of step 2 are the whole pass. An opening item from `D-021` writes its verdict record
   and authors the implementation items the verdicts imply. Each of those carries a territory,
   a `done_when` that names what is forbidden, and a predicate that selects every crate it
   changes, green at `HEAD` before the item is stored or depending on the item that makes it so.
   Where a verdict needs an XVPE mechanism, the pass writes the step into XVPE's
   `docs/plans/programs/ecosystem-substrate/kwb-parity.md`.
5. Repeat step 4 while the pass's budget allows.
6. Evaluate `D-021`'s closure test, and end with the status line.

A design pass never edits source, and the phase guard enforces that. When the next useful act
is implementation, the pass ends. It does not continue into the other half because the work is
small.

## The status line

The last line of every pass's final message is

```text
LOOP-STATUS: <word> — <one sentence>
```

| Word | Implementation pass | Design pass |
|---|---|---|
| `progress` | finished at least one item | audited, decided or authored something |
| `exhausted` | nothing claimable of its kinds | nothing to audit and no `Decision` item ready |
| `blocked` | abandoned an item for a stated ambiguity | cannot proceed for a reason the next pass cannot fix |
| `owner` | — | every remaining item or goal needs the owner (G8, or a hold only the owner lifts) |
| `complete` | — | `D-021`'s closure test holds for G0 through G8 |

## What stops the loop

The driver stops, and says which condition fired, when:

1. a design pass reports `complete` or `owner`;
2. two consecutive cycles pass in which no pass reported `progress`, because a loop that changes
   nothing is only spending money;
3. two consecutive implementation passes report `blocked` on the same item;
4. two consecutive passes exit non-zero or end without a status line;
5. a cap is reached — cycles, wall-clock hours, or dollars across passes;
6. the file `STOP` appears in the driver's state directory.

The driver never pushes, never runs `cargo fmt`, and never edits either repository itself. It
reads the board to choose a pass and reads the status line to choose the next.

## XVPE's steps

A mechanism `D-021` places in XVPE is a **step** in XVPE's
`docs/plans/programs/ecosystem-substrate/kwb-parity.md`, not an item here. XVPE has no ledger,
so that document carries its own claim protocol, and an XVPE implementation pass follows it
rather than this record. The design pass audits a finished step as it audits an item. The KWB
item that consumes the step depends on a pin-move item. That item measures the move alone, at a
revision published on XVPE's `origin/dev` (`D-007`'s amendment).

## Why

**Cost, enforced rather than remembered.** Work that can be re-derived from a prompt should not
be bought at the design model's price. Work that cannot be re-derived — what a requirement
means, what is forbidden — should not be handed to the cheaper one. On 2026-09-13 a single
session authored and implemented five items in one autonomous loop. The phase guard exists
because of that, and this record is the other half of the same control: it tells an unattended
session which half it is in, using a field every item already carries.

**An audit that leaves no trace cannot be resumed.** The ledger has no audited field. A commit
watermark is derivable from `git log` alone, survives any session, and costs one commit per
design pass.

**A loop needs a reason to stop that is not an empty board.** The board empties briefly after
every design pass is consumed. `D-021`'s closure test, the owner's hold, and the no-progress rule
are what separate "finished" from "waiting" from "stuck".

## Alternatives Considered

**One session doing both halves.** Refused, for the reason above. It is the incident the guard
records.

**The driver deciding what to work on.** Refused. The driver counts `ready` rows by kind to pick
the next pass and does nothing else. Choosing an item is `nomos work list`'s job, and the
reasons it chooses are the ledger's.

**An audited flag on the item.** Not available. The schema is Nomos's (`D-005`) and has no such
field. Adding one to a borrowed schema would be a second authority for what Nomos defines.

## Consequences

- `AGENTS.md` routes to this record and to `D-021`; `KWB-121` adds the route and a test that
  holds it.
- The first audit's watermark is the commit that lands this record. Items finished before it
  were audited as they went, and what those audits missed is `KWB-122`'s subject.
- While the gate is red, `nomos work finish` refuses every item before its predicate runs. The
  first implementation passes will therefore be the gate's repairs, `KWB-103` and `KWB-109`, and
  every item authored for `D-021` depends on both.

## Amendment: The audit register is the watermark, because a history rewrite erased every commit that held it, 2026-10-03

This amendment replaces step 2's watermark.

**What happened.** On 2026-10-01 this repository's history was squashed into one commit, and
the reflog shows a reset to `refs/reauthored/dev`, so history rewrites here are a recurring
fact. The squash deleted every `Audited:` commit. The rule's fallback, the commit that landed
this record, became the squash commit itself, which is later than every item finished before
it. As a result:

- five items finished between the last audit and the squash read as audited: `KWB-107`,
  `KWB-112`, `KWB-113`, `KWB-138` and `KWB-139`;
- the next audit recorded that it had nothing to audit.

A watermark kept in commit metadata does not survive the operation most likely to happen to
commits.

**The register.** `docs/audits.md` is the audit's authority. It is a file in the tree, so a
rewrite carries it like any other content.

- Each audit appends one section: its date, then one row per item audited.
- A row gives the item id, its verdict (`accepted` or `sent back`), and the items that carry a
  defect sent back.
- A row is never edited or removed. An audit that changes an earlier verdict appends a new row.

**Unaudited, restated.** An item is unaudited when all three of these hold:

- its state is `Done`;
- its `verified.verified_at` is later than `1790495654`, the time this record first landed on
  2026-09-27 (`KWB-122` reached back past it);
- no row of the register names it.

The rule reads no commit subject, hash or time.

**So the design pass's steps read:**

- **Step 2.** Its first sentences become: *The register is `docs/audits.md`. An item is
  unaudited when it is `Done`, its `verified_at` is later than `1790495654`, and no row of the
  register names it.* The rest of step 2 stands.
- **Step 3.** It becomes: *Append the audit's section to the register and commit it.* The
  subject `Audited: <ids> — <n> accepted, <m> sent back` stays as a convention for a reader of
  the log, but nothing reads it as the watermark. An audit that found nothing to audit appends
  nothing, and needs no commit.

**Every other rule in this record stands as written.** The *Why* paragraph on an audit that
leaves no trace still holds: the register is derivable without any session, as the commit
watermark was. What it drops is the assumption that history is never rewritten.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `D-021`
