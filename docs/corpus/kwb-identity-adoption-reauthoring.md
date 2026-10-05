# Identity adoption re-authoring at the current published pin

Measured 2026-10-04 from committed export 811353c at unchanged XVPE pin 726aa3fee. D-018 version 3 holds the normative amendment; this file records its dependency effect and work mapping.

The simulated manifest-only adoption resolves 66 packages before and 57 after. Comparing every package by name, version and source finds zero gains, no retained changes, and exactly nine registry removals: block-buffer 0.10.4; cpufeatures 0.2.17; crypto-common 0.1.7; digest 0.10.7; generic-array 0.14.7; libc 0.2.189; sha2 0.10.9; typenum 1.20.1; version_check 0.9.5. This confirms the earlier a88e1b3 measurement. No source migration or compilation is claimed by this metadata experiment.

Eighteen Ready, unclaimed items comprised the full open dependent closure of KWB-114. Each original remains in Declined history naming its replacement. Every title, kind, origin, territory, predicate and timeout is unchanged. Every acceptance condition is unchanged except the root adoption clause, whose obsolete two-package gain is replaced by the exact measured delta. Dependencies are remapped only within this closure; original rationales remain after supersession prefixes. Direct comparison against the original snapshot verifies these statements.

| Original | Replacement |
|---|---|
| KWB-114 | REL10-ADOPT-XVPE-CONTENT-IDENTITY |
| KWB-116 | REL10-AFTER-IDENTITY-KWB-116 |
| KWB-141 | REL10-AFTER-IDENTITY-KWB-141 |
| KWB-142 | REL10-AFTER-IDENTITY-KWB-142 |
| KWB-148 | REL10-AFTER-IDENTITY-KWB-148 |
| KWB-150 | REL10-AFTER-IDENTITY-KWB-150 |
| KWB-151 | REL10-AFTER-IDENTITY-KWB-151 |
| KWB-152 | REL10-AFTER-IDENTITY-KWB-152 |
| KWB-153 | REL10-AFTER-IDENTITY-KWB-153 |
| KWB-158 | REL10-AFTER-IDENTITY-KWB-158 |
| KWB-166 | REL10-AFTER-IDENTITY-KWB-166 |
| KWB-172 | REL10-AFTER-IDENTITY-KWB-172-EXACT-ARGV |
| KWB-182 | REL10-AFTER-IDENTITY-KWB-182 |
| KWB-188 | REL10-AFTER-IDENTITY-KWB-188 |
| REL09-REPLACE-KWB-143 | REL10-AFTER-IDENTITY-REL09-REPLACE-KWB-143 |
| REL09-REPLACE-KWB-160 | REL10-AFTER-IDENTITY-REL09-REPLACE-KWB-160 |
| REL09-REPLACE-KWB-161 | REL10-AFTER-IDENTITY-REL09-REPLACE-KWB-161 |
| REL09-REPLACE-KWB-162 | REL10-AFTER-IDENTITY-REL09-REPLACE-KWB-162 |

The preservation check caught PowerShell 5 stripping literal quotation marks from the first KWB-172 replacement. That intermediate item remains Declined and names the corrected exact-argv replacement. The final item preserves the complete original text, including quotation marks. No mismatch is accepted as completion.

Identity feature isolation, the current pin, InputTooLong propagation, fixed identity/rendering fixtures, and replay obligations remain intact. No product work is declared complete by re-authoring it. No production source, Cargo manifest, lockfile, original store or live model is changed. The board validates, and the decision finishes through workspace lint and repository contract tests.

Local reproducibility artifacts are under the ignored XVPE release-loop task directory: kwb-identity-current.zip, kwb-identity-current, kwb-identity-current-before.json and kwb-identity-current-after.json. Reproduce from the named commit and manifest changes documented in kwb-identity-adoption-at-726aa3fee.md; local exports are not dependency paths or published source.
