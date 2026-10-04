# Identity adoption at the current published pin

Measured on 2026-10-04 from committed KWB `a88e1b3`, exported with `git archive`
into a scratch directory. Production sources, manifests, the lockfile and the
XVPE pin were unchanged. This is a dependency-resolution measurement, not a
completed source migration or a compilation claim about the simulated adoption.

The exported manifests simulate D-018's approved adoption at `726aa3fee`:

- The quarantine gains optional `xvpe-content-identity` and an identity-only feature.
- Existing quarantine edges are optional behind its preserved default runtime feature;
  the model disables defaults and enables identity only.
- The workspace dependency disables defaults, with existing runtime consumers explicitly
  retaining them. Cargo refuses overriding inherited defaults off otherwise.
- The model and workspace remove `sha2`.

`cargo metadata --offline --format-version 1` resolves 66 packages before and 57
after. Comparing each package's name, version and source yields **zero gains, zero
version/source changes, and nine removals**:

| Removed package | Version |
|---|---|
| block-buffer | 0.10.4 |
| cpufeatures | 0.2.17 |
| crypto-common | 0.1.7 |
| digest | 0.10.7 |
| generic-array | 0.14.7 |
| libc | 0.2.189 |
| sha2 | 0.10.9 |
| typenum | 1.20.1 |
| version_check | 0.9.5 |

`xvpe-content-identity` and `xvpe-algorithms-hashing` already occur in the baseline
closure. All retained packages keep their exact versions and sources; every XVPE
package keeps the existing `726aa3fee` commit. No new third-party dependency enters.

An independent package, outside the exported workspace's consumer feature union,
depends only on the simulated `kwb-model`. Its `cargo tree --offline -f '{p} [{f}]'`
shows the quarantine with **only `content-identity`**, then content identity,
hashing, collections map/handle, primitives and the already-held hashbrown 0.15.5.
There is no clock, inference, chunker or persistent map in this consumer closure.
The probe does not build the production source that still contains the old API.

This differs from D-018's 2026-09-24 measurement at `c700bcf83`, which expected
identity and hashing as two new packages. KWB-114 explicitly treats a different
delta as a stop requiring a separate decision. Its source migration therefore
remains unimplemented; this evidence does not quietly substitute new acceptance
terms. An attempt to reserve the exact D-018 amendment was refused because the
open KWB-126 item reserves the records directory. The amendment and the source
migration remain owed.

Local reproducibility artifacts are under the ignored XVPE task directory:
`kwb-identity-export`, `kwb-identity-before.json`, `kwb-identity-after.json`,
`kwb-identity-probe`, and `kwb-identity-closure.txt`. Recreate the manifests above
from the named commit rather than treating those local scratch files as a
published source or a dependency path. No path edge enters production KWB.
