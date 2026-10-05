# KWB record reservation repair

Measured 2026-10-04 against the board after 80741b1, using Nomos schema 7. This is a dated repair measurement; nomos work list remains the live work authority.

Twelve unclaimed legacy decision items reserved the entire docs/records directory. Adding an amendment to the published D-018 was refused as reserved by KWB-126, despite no live writer. The identifier-allocation guard was acting on those authored territories as written.

Each original is retained as Declined, naming its replacement. Each replacement keeps the exact title, complete acceptance condition, kind, origin, verification argv and timeout. Its rationale retains the complete original after a supersession prefix. Non-directory territory and patterns are preserved. Dependencies are unchanged except for replacing IDs within the twelve-item closure.

| Original | Replacement | Allocated record |
|---|---|---|
| KWB-126 | REL09-REPLACE-KWB-126 | docs/records/D-028 |
| KWB-129 | REL09-REPLACE-KWB-129 | docs/records/D-029 |
| KWB-130 | REL09-REPLACE-KWB-130 | docs/records/D-030 |
| KWB-131 | REL09-REPLACE-KWB-131 | docs/records/D-031 |
| KWB-132 | REL09-REPLACE-KWB-132 | docs/records/D-032 |
| KWB-133 | REL09-REPLACE-KWB-133 | docs/records/D-033 |
| KWB-134 | REL09-REPLACE-KWB-134 | docs/records/D-034 |
| KWB-143 | REL09-REPLACE-KWB-143 | docs/records/D-035 |
| KWB-160 | REL09-REPLACE-KWB-160 | docs/records/D-036 |
| KWB-161 | REL09-REPLACE-KWB-161 | docs/records/D-037 |
| KWB-162 | REL09-REPLACE-KWB-162 | docs/records/D-038 |
| KWB-185 | REL09-REPLACE-KWB-185 | docs/records/D-039 |

Identifiers were checked against published records and open explicit reservations before any supersession. All twelve items were Ready and unclaimed; the full dependent closure was exactly these twelve, so no outside obligation was stranded. Verification compared every preserved field and remapped dependency against the pre-mutation snapshot. The ledger validates, and no open item retains the directory reservation.

These identifiers reserve future record files; this repair publishes no decision and fulfils none of their product obligations. A future decision must reserve any additional existing record it actually needs to amend through the ordinary amendment and territory workflow. Its record number may not be silently changed.

No production source, Cargo metadata, published record, original data, allocation guard or correctness rule changed. Claim exclusion still serializes overlapping writers. The D-018 amendment and identity adoption remain unfinished product work, now able to reserve their own precise territory.

Validation: nomos work validate --root . and cargo test --locked --offline -p kwb-contract-tests --no-fail-fast. The cleanup finishes through the repository lint step as well.
