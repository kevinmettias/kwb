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
