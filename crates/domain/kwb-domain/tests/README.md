# kwb-domain's test suite

<!-- folder-organization: coherent: one file per test unit, and the flat layer is what makes each of them a unit -- cargo compiles every `tests/*.rs` as its own crate and compiles no test target from a subfolder, so grouping these into subfolders would either stop them running or merge several units into one binary. Being a separate crate is load-bearing in the other direction too: it is what limits a file here to kwb-domain's public surface, which is the property these tests exist to assert. Each file's stem is the unit's name, and that stem is what `check-test-coverage` pairs with a source file under `src/`. -->

Sixteen files, one per unit. Twelve have a stem naming a source file under `src/`; the four
`one_*` files assert a property of the crate as a whole rather than of any one file.
