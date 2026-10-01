# What the prototype could do, and which goal owns each capability

`prototype-inventory.md` classifies the prototype's **directories**. This file catalogues what
the prototype could **do** — every command, tool and pipeline stage a person or an agent could
reach — because a directory-grain inventory let most of the operational surface through
unrouted: the whole `Cli/Commands` row, 31 files, went to *corpus* with a note of "≥27 named
subcommands", and there are 78.

It is a corpus artifact: evidence, not a decision. It verdicts nothing. Each row names the goal
of `D-021` that owns it, and that goal's opening item writes the record that verdicts the row —
met, diverges, deferred or declined, per `D-001`. The row itself never changes to say how far the
work got; the board and the records say that.

Measured 2026-09-27 against the prototype at `f22f255`, its last commit (2026-08-05). The
prototype has not changed since, so this is a final measurement rather than a snapshot.

## The denominator

| Surface | Count | How it was taken |
|---|--:|---|
| Top-level `kw` commands | 78 | `root.AddCommand` calls under `src/KnowledgeWorkbench.Cli/Commands` |
| Command-name constants | 80 | `const string Cmd… = "…"` there: the 78, plus `proofs for-concept` and `proofs get` |
| MCP tools | 9 | `McpServerTool(Name = …)` under `src/KnowledgeWorkbench.Mcp` |
| HTTP routes | 79 | `.MapGet/.MapPost/.MapPut/.MapPatch/.MapDelete` under `src/KnowledgeWorkbench.Api`: 46 GET, 28 POST, 3 DELETE, 1 PUT, 1 PATCH |
| Enrichment stages | 14 | `EnrichmentPipeline.RunAsync`, lines 122–135. The class's own doc comment lists seven and is stale |

**The claim:** every one of the 78 commands, 9 tools and 14 stages names a row below — see
*Every surface, routed* at the foot of this file. A name that appears there with no row, or a
row whose goal is not one of `D-021`'s, is a defect in this file. The 79 routes are one row
(C-66): each handler calls the same services a command does, so the capability behind a route
is the row its command names, and the API as a surface is the thing C-66 verdicts.

## The columns

*Reach* is where the prototype exposed the capability: **CLI**, **API**, **MCP**, a
**stage** inside a chain, or **declared** — registered or typed with no caller in any host.
*kwb today* is measured against `f:/repos/kwb` at `b8729cd`. *Goal* is `D-021`'s.

## A. Sources and reading

| # | Capability | Reach | kwb today | Goal |
|---|---|---|---|---|
| C-01 | Register a source from a file, a folder or a URL, and ingest it | CLI `add-source`, `list-sources`, `ingest`; API | `kwb admit <file>`: one path, one byte blob | G2 |
| C-02 | PDF text with word reconstruction from glyph geometry — glued words of 12+ characters fell from 4.21% to 0.94% | stage (`Infrastructure/Parsing`) | UTF-8 only; other bytes are `ReaderFailed`. XVPE has a PDFium reader and no reconstruction pass | G2 |
| C-03 | Plain text and source files (`.txt .md .cs .rs .hlsl .glsl`) | stage | UTF-8 text through `ReadsText` | G2 |
| C-04 | Folder walk and web fetch, with a local snapshot | CLI, API | none. XVPE has a folder walk (`xvpe-corpus-front`) and no web fetcher | G2 |
| C-05 | An unreadable source is its own outcome, surfaced in every search reply | stage, API `/sources/unreadable`, MCP `search` | `ExtractionError::CannotRead { Visual }` at admission; not recorded, not surfaced | G2 |
| C-06 | Structure-aware chunking with section path, page spans and content-addressed chunk ids | stage | XVPE's page-run chunker at 4,000 characters / 1 page, **unmeasured**; no chunk identity is kept | G2 |
| C-07 | Passage classification — prose, code, navigation, fragment — so non-prose is `Skipped`, not `Barren` | stage | `Coverage::Skipped` is consumed everywhere and produced nowhere | G2 |
| C-08 | Mangled-text diagnostics before any model is paid | CLI `text-quality` | XVPE `PageFidelity` grading only | G2 |
| C-09 | Seeding sources and their stable external keys from XVPE's `catalog.toml` and `reference_registry.toml` | CLI `seed-catalog` | none. XVPE holds the data and Go tools, no Rust reader | G2 |
| C-10 | Model-backed reading through a real provider: Anthropic, Ollama, OpenAI, Gemini; structured outputs; per-stage model routing; prompt caching | stage | `kwb-extract` exists and no command reaches it; no provider is in the workspace | G2 |
| C-11 | Record and replay of model calls, a miss being a failure | stage (config) | met in tests through XVPE `ReplayInference` | G2 |
| C-12 | A prompt change reopens coverage: the extraction version is `model@prompt-hash` | stage | the reading protocol is a constant string, `read-born-digital-text-v1` | G2 |

## B. Admission and claims

| # | Capability | Reach | kwb today | Goal |
|---|---|---|---|---|
| C-14 | Content-derived claim identity, so two books asserting one thing make one claim | stage | met — `kwb-model`, `KWB-1`, `KWB-15` | G1 |
| C-15 | Admission against the concept's claim neighbourhood, with four verdicts — new, duplicate, refinement, contradiction — on a closed candidate list | CLI `admit` | none; `D-004` holds admission | G5 |
| C-16 | A grounding gate (the quote must be found), intra-source duplicate refusal, identity overruling a mispointed duplicate | stage | none. XVPE has `ClaimGrounding` and `QuoteVerification` | G5 |
| C-17 | Claim status — asserted, corroborated, disputed, superseded — with supersession withheld (`D17`) | CLI `claims --disputed` | `Standing` exists for concepts only | G5 |
| C-18 | Support counted over distinct sources and never shown without its trust: certified, candidate, suspect | CLI `claims --corroborations`, MCP `claims_for` | assertions accumulate on one claim; nothing counts or grades them. XVPE has `ClaimLedger::Corroborated` | G5 |
| C-19 | Calibrating the adjudicator: a worksheet, reference verdicts that must name their labeler, scoring | CLI `judgements` | none | G5 |
| C-20 | Measuring a model before trusting it: several models over one passage set; a probe that refuted the embedding redundancy gate | CLI `adjudicator-bench`, `admission-probe` | none | G5 |
| C-21 | A run's derived health, where a suspected model outage exits 1 | stage | none | G5 |
| C-22 | A model's verdict is carried as a model's verdict, in the type | stage (`Core/Claims`) | a reading's lineage is dropped at admission (`D-015`) | G5 |

## C. Curation, coverage and audit

| # | Capability | Reach | kwb today | Goal |
|---|---|---|---|---|
| C-13 | Source quality ratings and precise locations (page, anchor, line range) | CLI `set-source-quality`, `get-source-quality`, `add-source-location`, `list-source-locations`; API | none | G4 |
| C-24 | Deterministic normalization: alias derivation (case, spacing, hyphen, plural, acronym), an ambiguity guard, no union-find bridging, and a merge log | CLI `normalize-concepts`, `harvest-aliases`; stages 5–6 | grouping by identity equality only; alias derivation refused until `D-011` B4 can certify transitivity | G4 |
| C-25 | A merge audit that re-derives every logged merge independently, and undoes one | CLI `merge-audit [--undo]` | none | G4 |
| C-26 | Closing a concept with a reason, its claims following the successor | stage (`ConceptLiveness`) | `kwb retire` and `kwb supersede` with `--because`; claims are not reassigned | G4 |
| C-27 | Hand curation: concepts, relations, chunk links, domains, annotations and summaries, added and listed | CLI `add-concept`, `list-concepts`, `add-relation`, `list-chunk-concepts`, `list-concept-chunks`, `add-domain`, `list-domains`, `add-concept-domain`, `list-concept-domains`, `add-annotation`, `list-annotations`, `list-summaries`; API | none | G4 |
| C-28 | Coverage recorded per unit, stage and model version, each outcome with its reason | stage; API `/sources/{id}/coverage` | computed per run and printed; never persisted | G4 |
| C-29 | A coverage audit that re-checks each recorded reason, reports its denominator, and reopens what went stale | CLI `coverage-audit [--reopen]` | none | G4 |
| C-30 | Append-only derivation lineage and its audit | CLI `derivation-audit`; API `/claims/{id}/derivation` | the publication log is append-only; nothing records how a claim was derived | G4 |
| C-31 | One finding model with a computed verdict — blind, failed, vacuous, clean — and exit codes 0, 1, 2 | CLI `findings` | none | G4 |
| C-32 | Corpus health and quality reports | CLI `corpus-health`; API `/quality/report`, `/runs`, dashboard | none | G4 |
| C-33 | A flight recorder and cost ledger for every model call | stage; API `/runs` | `TokenUsage` is re-exported and never recorded. XVPE `xvpe-ai-pipeline` has call accounting | G4 |
| C-34 | Golden-set scoring of extraction recall and precision | declared (tests, gated on a live flag) | none; XVPE has none | G4 |
| C-35 | Rebuilding the derived layer while keeping chunks and embeddings | CLI `rebuild-derived`; scripts | the graph is rebuilt by replaying the publication log | G4 |
| C-81 | Concurrent writers: optimistic concept creation, and a stage lock around graph building | stage | none; two processes may write one store. XVPE's cross-process lock is not started | G1 |

## D. Enrichment

| # | Capability | Reach | kwb today | Goal |
|---|---|---|---|---|
| C-23 | Linking concept mentions per chunk, resumable from a chunk ledger | CLI `link-concepts`; stage 4 | a reader proposes concept and claim together; `Link_Concepts` attaches within one admission | G6 |
| C-36 | The fourteen-stage enrichment chain, incremental and resumable, each stage committing alone | CLI `enrich`, `enrich-doc`; API `/enrich` | none | G6 |
| C-37 | Chunk annotations and six kinds of summary | CLI `summarize`; stages 2–3 | none | G6 |
| C-38 | Typed concept edges with a prerequisite projection and cycle breaking | CLI `build-graph`; stage 7 | none | G6 |
| C-39 | Concept identity judged into reviewable `SameConceptAs` edges, never merges | stage 8 | none | G6 |
| C-40 | Concept kind classification, gating proof extraction | stage 9 | none | G6 |
| C-41 | Placing each concept in a subject taxonomy | CLI `list-taxonomy`; stage 10 | none | G6 |
| C-42 | Proofs: six kinds, rigor and accessibility as independent axes, prerequisites, proof relationships, a proof graph | CLI `proofs`, `proofs for-concept`, `proofs get`; stages 11–12; MCP `proofs_for`; API | none | G6 |
| C-43 | Extracting exercises and worked examples | stage 13 | none | G6 |
| C-44 | Concept synthesis: definition, consensus, unique contributions, conflicts, misconceptions | CLI `synthesise-concepts`, `synthesise-concept`; stage 14 | none | G6 |
| C-45 | Evidence weighting by source quality, and the evidence pack for a concept | CLI `weight-evidence`, `weight-concept`, `evidence-pack` | none | G6 |

## E. The epistemic kernel

| # | Capability | Reach | kwb today | Goal |
|---|---|---|---|---|
| C-47 | A universal type kernel and 21 relation kinds, each with an executable constraint (symmetry, transitivity, negative knowledge) | CLI `universal-kernel` | `D-011` states requirements; no type exists | G5 |
| C-48 | Nine domain packs validated against the kernel on load | CLI `domain-packs` | none | G5 |
| C-49 | Inference over the relation algebra, writing hypotheses and never edges | CLI `infer-relations` | none | G5 |
| C-50 | One read model over five relation families | CLI `relations` | none | G5 |
| C-51 | Negative assertions that require a falsifier | CLI `negative-assert`, `negative-assertions` | none | G5 |
| C-52 | Connection discovery across domains, with hypotheses held for review | CLI `discover-connections`, `list-connection-hypotheses`; MCP `list_connection_hypotheses`; API | none | G5 |
| C-53 | Theory containers and values, and scoping a claim to a framework | CLI `theory-add`, `theory-list`, `theory-scope`, `value-add`, `value-list` | a scope is `Named` or `Unstated`; promotion is deferred by `D-010` | G5 |
| C-54 | The transitive closure of transitive families, with fingerprint staleness | stage (`EfConceptClosure`) | none | G5 |

## F. The query surface

| # | Capability | Reach | kwb today | Goal |
|---|---|---|---|---|
| C-55 | A subgraph by subject, domain, rigor or confidence | CLI `subgraph` | none | G3 |
| C-56 | Ranked keyword search | CLI `search`; API; MCP `search` | every query word must appear; unranked | G3 |
| C-57 | Embeddings and semantic search | CLI `embed`; stage 1; MCP `search` | none; `KWB-82`. XVPE has no embedding API and no vector index | G3 |
| C-58 | Hybrid search, with each hit carrying its chunk id and catalogue key | CLI `search`; MCP `search` | none | G3 |
| C-59 | Neighbourhood with a truncation flag, and shortest paths between concepts | MCP `get_concept`, `neighbours`, `path` | `get_concept` and `neighbours` met; `path` absent — it needs typed relations | G3 |
| C-60 | An MCP server over stdio whose every reply is bounded and says when it trimmed | MCP | five tools over argv; no wire (`KWB-112`); replies unbounded | G3 |
| C-61 | A claim's support, status and catalogue keys | MCP `claims_for` | `neighbours` lists a concept's claims and their assertions | G3 |
| C-62 | The gaps in one source | MCP `gaps_in_source` | none; needs coverage stored per source (C-28) | G3 |
| C-63 | Historical reads: what was merged away, what it held, the graph as of a moment | declared (`IConceptHistory`, no user-facing as-of) | **beyond the prototype**: `merge_losers`, `held_neighbours`, `kwb history --through/--as-of` | G3 |
| C-64 | Citations with edition, per-source contributions, a source's subgraph | API provenance group | `neighbours` prints each citing source's address and scope | G3 |
| C-65 | Following a citation to the exact bytes it cites | — (the prototype stored paths) | `README.md` claims it; no command does it, and `DocumentStore::Read` consults memory only, so a fresh process reports `NoSuchDocument` for a document on disk | G1 |
| C-66 | An HTTP API — 79 routes, reader and writer keys, rate limiting, readiness | API | none | G3 |
| C-67 | A durable job queue with retry and dead-letter | CLI `enqueue-fetch`, `enqueue-enrich`, `job-status`; API | none, and deliberately: `prototype-incidents.md` D19 (a) records that admission hands its work to no queue | G3 |

## G. Code and project context

| # | Capability | Reach | kwb today | Goal |
|---|---|---|---|---|
| C-68 | Code entities extracted from a book behind a verbatim guard | CLI `extract-code` | none; `README.md` says a code entity is intended and undecided | G6 |
| C-69 | Concept-to-code links by deterministic name matching | CLI `link-code` | none | G6 |
| C-70 | Code gaps: what a book explains and never implements, and prints and never explains | CLI `code --gaps`; MCP `code_for` | none | G6 |
| C-71 | Project spaces and decision notes | CLI `add-space`, `list-spaces`, `add-decision`, `list-decisions`; API | none | G6 |
| C-72 | Linked repositories and a drift check | CLI `link-repo`, `list-repos`, `drift-check`; API | none | G6 |

## H. The back half, which the prototype did not build either

`what-kwb-is-for.md` measured four of the corpus's nine operations as essentially absent from
the prototype, and the essay found a fifth built at the wrong grain. They are rows here because
`D-021` makes them part of completion, not because the prototype could do them.

| # | Capability | Reach | kwb today | Goal |
|---|---|---|---|---|
| C-46 | Comparing two concepts | CLI `compare-concepts`; API | none | G7 |
| C-73 | Learning paths from seed concepts | CLI `build-learning-path`, `list-learning-paths`; API | none | G7 |
| C-74 | **revise**: a derived artifact declares what it was derived from, and staleness is computed when an input changes or is retracted — the product loop's steps 7 and 8 | absent — the prototype's roadmap names it absent | `D-006` states the requirement; nothing computes it | G7 |
| C-75 | Graph versions and snapshots; merge as preview, validate, apply | absent | publications replay to any prefix (`kwb history`); no merge preview | G7 |
| C-76 | **compare**: two bodies of knowledge, and a semantic diff of two sources | one row — pairwise overlap, API `/sources/overlap` | none | G7 |
| C-77 | **reconstruct**: a source rebuilt from the model | absent | none | G7 |
| C-78 | **assess**: what a person or agent understands; generated exercises and verified solutions | 3 of 5 feature rows, the last two unticked in every column | none | G7 |
| C-79 | **synthesize** at document grain: grounded explanations, documents, curricula | absent — the prototype synthesizes about one concept | none | G7 |
| C-80 | Artifact engineering: composition, evaluation and revision, an essay being one profile over them | absent | none | G7 |

81 rows, C-01 to C-81, each once. Numbers were assigned in drafting order and the sections group
by subject, so a number can sit outside its neighbours' section (C-13 is curation, C-46 and C-73
are the back half). By goal: G1 3 · G2 12 · G3 12 · G4 13 · G5 16 · G6 16 · G7 9. G0 owns the loop
and G8 the owner's release; neither owns a capability.

## Every surface, routed

**Commands (78).** `add-source` C-01 · `list-sources` C-01 · `ingest` C-01 · `seed-catalog` C-09 ·
`text-quality` C-08 · `embed` C-57 · `search` C-56 · `admit` C-15 · `admission-probe` C-20 ·
`adjudicator-bench` C-20 · `claims` C-17 · `judgements` C-19 · `enrich` C-36 · `enrich-doc` C-36 ·
`summarize` C-37 · `link-concepts` C-23 · `harvest-aliases` C-24 · `normalize-concepts` C-24 ·
`merge-audit` C-25 · `coverage-audit` C-29 · `derivation-audit` C-30 · `findings` C-31 ·
`corpus-health` C-32 · `rebuild-derived` C-35 · `add-concept` C-27 · `list-concepts` C-27 ·
`add-relation` C-27 · `list-chunk-concepts` C-27 · `list-concept-chunks` C-27 · `add-domain` C-27 ·
`list-domains` C-27 · `add-concept-domain` C-27 · `list-concept-domains` C-27 · `list-taxonomy` C-41 ·
`add-annotation` C-27 · `list-annotations` C-27 · `list-summaries` C-27 · `set-source-quality` C-13 ·
`get-source-quality` C-13 · `add-source-location` C-13 · `list-source-locations` C-13 ·
`build-graph` C-38 · `proofs` C-42 · `synthesise-concepts` C-44 · `synthesise-concept` C-44 ·
`weight-evidence` C-45 · `weight-concept` C-45 · `evidence-pack` C-45 · `compare-concepts` C-46 ·
`universal-kernel` C-47 · `domain-packs` C-48 · `infer-relations` C-49 · `relations` C-50 ·
`negative-assert` C-51 · `negative-assertions` C-51 · `discover-connections` C-52 ·
`list-connection-hypotheses` C-52 · `theory-add` C-53 · `theory-list` C-53 · `theory-scope` C-53 ·
`value-add` C-53 · `value-list` C-53 · `subgraph` C-55 · `extract-code` C-68 · `link-code` C-69 ·
`code` C-70 · `add-space` C-71 · `list-spaces` C-71 · `add-decision` C-71 · `list-decisions` C-71 ·
`link-repo` C-72 · `list-repos` C-72 · `drift-check` C-72 · `enqueue-fetch` C-67 ·
`enqueue-enrich` C-67 · `job-status` C-67 · `build-learning-path` C-73 · `list-learning-paths` C-73.

**MCP tools (9).** `search` C-56 · `get_concept` C-59 · `neighbours` C-59 · `path` C-59 ·
`proofs_for` C-42 · `claims_for` C-61 · `code_for` C-70 · `gaps_in_source` C-62 ·
`list_connection_hypotheses` C-52.

**Stages (14).** Embed C-57 · Annotate C-37 · Summarize C-37 · ConceptLink C-23 · AliasHarvest
C-24 · ConceptNormalize C-24 · ConceptGraph C-38 · ConceptIdentity C-39 · ConceptKind C-40 ·
SubjectClassify C-41 · ProofExtract C-42 · ProofGraph C-42 · ExerciseExtract C-43 ·
ConceptSynthesis C-44.

## What would falsify this file

1. **A command, tool or stage the counts above miss.** Re-run each count; a number other than
   78, 9 or 14 means the prototype holds a surface this file does not route.
2. **A capability the prototype exercised that no row describes.** The grain here is a
   capability, and a row may bundle several commands; a reader who finds a behaviour none of
   the bundled rows would carry has found a missing row.
3. **A "kwb today" cell that is wrong.** Each was read from source at `b8729cd`, not from a
   record's statement of intent — `D-003` is why that distinction is the whole point.
