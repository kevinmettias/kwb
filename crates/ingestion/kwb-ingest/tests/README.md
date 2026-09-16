# kwb-ingest's test suite

<!-- folder-organization: coherent: one file per test unit, and the flat layer is what makes each of them a unit -- cargo compiles every `tests/*.rs` as its own crate and compiles no test target from a subfolder, so grouping these into subfolders would either stop them running or merge several units into one binary. Being a separate crate is load-bearing in the other direction too: it is what limits a file here to kwb-ingest's public surface, which is the property these tests exist to assert. Each file's stem is the unit's name, and that stem is what `check-test-coverage` pairs with a source file under `src/`. -->

Fourteen files, one per unit, every one of them named for a source file -- though not all of those
sources sit directly under `src/`: `src/concepts`, `src/readings` and `src/extractor` carry theirs
as submodules, and a file here is still one file per unit rather than one per folder.
