# The audit register

This register records which `Done` items a design pass has audited, under `D-022` as amended
on 2026-10-03. It is the authority for that question: nothing else answers it, not commit
subjects, hashes or times.

An item is **unaudited** when all three of these hold:

- its state is `Done`;
- its `verified.verified_at` is later than `1790495654` (2026-09-27, when `D-022` first
  landed; `KWB-122` reached back past it);
- no row below names it.

Each audit appends one section. Rows are never edited or removed: an audit that changes an
earlier verdict appends a new row. A verdict is `accepted` or `sent back`.

- For an item sent back, *carried by* names the items that hold the defect.
- For an item accepted, it names any follow-up that is not a defect of the item.

## 2026-09-27: the first audit

| item | verdict | carried by |
|---|---|---|
| `KWB-103` | accepted | |
| `KWB-109` | accepted | |

## 2026-09-27: the pre-loop audit, under `KWB-122`

| item | verdict | carried by |
|---|---|---|
| `KWB-102` | accepted | |
| `KWB-104` | accepted | |
| `KWB-122` | accepted | the audit's own item, recorded by the pass that did it rather than reviewed by another; follow-ups `KWB-135`, `KWB-136`, `KWB-137` |

## 2026-09-28: the second audit

| item | verdict | carried by |
|---|---|---|
| `KWB-100` | sent back | `KWB-139` |
| `KWB-101` | accepted | |
| `KWB-105` | accepted | |
| `KWB-108` | accepted | follow-up `KWB-138`: the guard kept its own copy of the source walk, outside its territory to remove |
| `KWB-110` | sent back | `KWB-138` |

## 2026-10-03: the third audit

This audit reached back past an empty audit commit. The 2026-10-01 history rewrite had
deleted every `Audited:` commit, so that commit's watermark could see none of the first five
rows below.

| item | verdict | carried by |
|---|---|---|
| `KWB-107` | accepted | follow-up `KWB-175`: OD-GATE-002 closes |
| `KWB-112` | accepted | follow-ups `KWB-176`, `KWB-177`, `KWB-178`: the visitor page and `README.md` never said `--serve` |
| `KWB-113` | accepted | |
| `KWB-138` | accepted | |
| `KWB-139` | accepted | |
| `KWB-125` | sent back | `KWB-174` |
| `KWB-127` | sent back | `KWB-173`; `KWB-167`, `KWB-168` and `KWB-170` declined for it |
| `KWB-159` | accepted | |

## 2026-10-03: the fourth audit

The first audit under `D-022` as amended, asked for by the pass that wrote the item.

| item | verdict | carried by |
|---|---|---|
| `KWB-124` | accepted | follow-up XVPE step X10, written `ready` in XVPE's `kwb-parity.md` |

## 2026-10-03: the fifth audit

By the pass that holds the design lane, of the three items the fourth audit's pass finished, so
none of them is audited by its author. Each was measured rather than read: `KWB-175`'s five
acting items are `Done` and its observation reads `closed`, version 3; `KWB-176`'s figures
reproduce at `0073ae2` — 14 workspace members, 26,917 lines of tracked `.rs`, 25 records, and
453 tests passing as measured that day; `KWB-179`'s `1790495654` is the reflog time of `905dede`,
the commit that landed `D-021` and `D-022`.

| item | verdict | carried by |
|---|---|---|
| `KWB-175` | accepted | |
| `KWB-176` | accepted | |
| `KWB-179` | accepted | |

## 2026-10-03: the sixth audit

Of the two items the fifth audit's pass finished, by the pass that sent them back, so neither is
audited by its author. `KWB-173`'s new codes were measured against both binaries, which use 0, 1
and 2 and nothing else, and its re-authored `KWB-180` to `KWB-182` carry the table. Its reading of
the owner's 2026-10-03 instruction was checked against the instruction itself: "do everything that
doesnt require running the actual program".

| item | verdict | carried by |
|---|---|---|
| `KWB-173` | accepted | |
| `KWB-174` | sent back | `KWB-184`: the amendment calls C-12's verdict unchanged while moving it from met to diverges; every other row, and `KWB-183`'s hold on G2, is sound |

## 2026-10-03: the seventh audit

Of the two items the sixth audit's sender then finished, by the pass that audited the last two.
`D-027`'s decisions are sound:
- grounding per proposition, adopted from XVPE's quote check at K3 rather than written twice;
- a model's judgement kept as a reading;
- a run the model caller stops records its unread documents Unmet and exits 1;
- no hold lifted.

Two things in it go back. `docs/corpus/core-invariant-audit.md` measured F19 with hand-typed
`--says` and no model, and left it unowned, yet the record routes the repeat rule to the work
owed at K3. And it defers C-47 to C-52 and C-54 on triggers without naming their measurement or
whether a scratch copy could supply it.

| item | verdict | carried by |
|---|---|---|
| `KWB-184` | accepted | |
| `KWB-128` | sent back | `KWB-186`: F19's repeat, implemented now; `KWB-187`: `D-027` names the relation kernel's measurements and says the repeat is no longer owed at K3 |

## 2026-10-03: the eighth audit

`D-027` v2:

- names, row by row, what each held relation row waits on, and that a scratch copy can supply it
  once K3 composes reading and model runs are permitted, with none waiting on G8;
- routes the repeat to `KWB-186`, so `KWB-185` owes only grounding and the failed-run exit;
- lifts no hold, and sits at version 2 with one amendment.

| item | verdict | carried by |
|---|---|---|
| `KWB-187` | accepted | |
