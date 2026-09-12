# Prototype requirement inventory

A classification of every scoped surface of the .NET prototype
(`C:/Users/kmett/source/repos/KnowledgeWorkbench`) and of the KWB design corpus, so that
"the prototype has been read" becomes a checkable claim rather than a feeling.

This is a corpus artifact under `ARC-ECOSYSTEM-002`: evidence about content. It is not a
design document, it does not decide anything, and no crate layout appears in it — `README.md`
is the authority for bands and `tests/contract` asserts it. Where an entry implies work, the
work is a ledger item and the row names it. Where an entry implies a decision that outlives
the item, the decision is a record in `docs/records/` and the row names it.

Measured 2026-09-12 against the prototype at its then-current working tree.

## What this inventory claims

> Every scoped prototype surface was inventoried, every candidate was classified, and no
> artifact in a retained class remains unrouted.

It does not claim that every artifact was read line by line, that the classification is the
only defensible one, or that the design corpus was exhausted. See *What would falsify this*.

## The denominator

An inventory with no denominator is a coverage report that cannot say what it skipped.

**A scoped surface is one directory of the prototype's `src/` tree, at the second level of a
project (or a project's own root), that contains at least one hand-written `.cs` or `.yml`
file.** Generated EF Core migrations, `obj/`, `bin/`, and `Properties/` are excluded — the
first because `ARC-ECOSYSTEM-002` excludes schema mapping from what is extracted, the rest
because they are build output.

Recount it with:

```sh
cd C:/Users/kmett/source/repos/KnowledgeWorkbench
for p in src/*/; do
  find "$p" -maxdepth 1 -name '*.cs' | grep -q . && echo "$p(root)"
  find "$p" -mindepth 1 -maxdepth 1 -type d \
    -not -name obj -not -name bin -not -name Migrations -not -name Properties \
  | while read d; do
      find "$d" -type f \( -name '*.cs' -o -name '*.yml' \) \
        -not -path '*/obj/*' -not -path '*/bin/*' | grep -q . && echo "$d"
    done
done | wc -l
```

**That count is 104.** Six further second-level directories exist and hold no source at all —
`Api/data`, `Api/data-backups`, `Core/Projects`, `Enrichment/Ollama`,
`Infrastructure.EfCore/Configurations`, `Infrastructure/Jobs` — and are listed here so a later
reader who finds 110 directories knows which six were excluded and why. The table below has
exactly 104 rows; a row count that disagrees with the command above is a defect in this file.

Scale, for budgeting a re-read: `src/` is 699 files and 151,392 lines, of which **80 files and
91,527 lines are generated migrations**. Hand-written non-test source is therefore 619 files
and **59,865 lines** — `Domain` 13,386, `Enrichment` 15,506. Tests are 325 files and **53,802
lines**, 304 test classes, 1,947 `[Fact]`/`[Theory]` attributes. The prototype's own
`ROADMAP.md` records 2,690 tests green as of 2026-08-01; the attribute count is lower because a
`[Theory]` yields many cases.

Outside `src/`, four further surfaces are in scope and are inventoried below the table:
`docs/`, the prototype's `AGENTS.md` incident register, `scripts/` and `infra/`, and the KWB
design corpus at `C:/Users/kmett/Downloads/Brainstorming/KWB`.

## The evidentiary ladder

A type existing in the prototype proves the first two of these and nothing further.

| Level | Means |
|---|---|
| **D** Declared | The type or rule exists. |
| **I** Implemented | It has a body, a table, a migration. |
| **E** Exercised | Something in the pipeline, a host, or a shipping command actually calls it. |
| **O** Operationally validated | It ran against the real corpus and the result was checked. |

> **Implementation presence is not evidence of semantic validity.**

That rule is recorded as `D-003`, because it governs every future extraction from this tree and
not only this pass. The prototype states its own version of it, in its own words, in
`docs/FEATURE_MATRIX.md`: *"A schema is not a feature. Several tables below have existed since
the initial migration with nothing writing to them."*

**The measured case that proves it.** The universal epistemic kernel is Declared and
Implemented and, in its node types, nothing more. 27 types in `Domain/` implement
`IEpistemicNode`, and **twelve of them are byte-identical apart from the type name** — verified
by normalising the type name out of each file and hashing the remainder, which yields one
digest for all twelve:

```
AgentNode  ArgumentNode  ArtifactNode  EconomicFactorNode  EpistemicValue  EventNode
EvidenceNode  ExplanationNode  InstitutionNode  MethodNode  ProcessNode  TechnologyNode
```

Each is 33 lines. Each has 5–6 EF Core files behind it. Six of the twelve — `AgentNode`,
`ArtifactNode`, `EvidenceNode`, `MethodNode`, `ProcessNode`, `EpistemicValue` — appear in **no
extraction result, no pipeline stage, no host, and no repository read path**; the other six
appear only as collections on `Core/Pipeline/EpistemicGraphExtractionResult`. `KnowledgeLayer`
is 1 file in `Domain/Universal`, referenced by 3 files in `Domain/`, 0 in EfCore, 0 in any
host.

So those twelve are not twelve mature concepts awaiting Rust representations. They are **one
architectural hypothesis expressed twelve times and never validated operationally** — which is
why `KWB-3` is right to demand the relation vocabulary be re-derived rather than carried.

**What did reach Operationally validated**, with consumers and real data behind it:

| Surface | Reach |
|---|---|
| `Domain/Algebra/` — `RelationAlgebra`, `RelationAlgebraRegistry`, `RelationCompositionRegistry`, `RelationInferenceEngine` | 658 lines; read by `Core/Concepts/Graph/ConceptPathAssembler`, `Core/Semantics/NegativeKnowledgeAudit`, and four EfCore query/repository files; driven by `kw infer-relations` |
| `ConnectionHypothesis` | 5 `Domain/` files, 14 EfCore, 3 Enrichment, 4 host |
| `NegativeAssertion` | 4 `Domain/`, 8 EfCore, 2 host; `kw negative-assert` requires `--falsifier` |
| `CoverageOutcome` | 10 `Domain/`, 3 EfCore, 12 Enrichment, 2 host, 16 test files |
| `ConceptMerge` / `AliasDerivation` | 20 EfCore / 3 Enrichment + 2 host; the post-D18 merge path |
| `DeterministicId` | 9 call sites, all in `Domain/` |

## The classification scheme

Every scoped surface is classified as exactly one of:

| # | Class |
|---|---|
| 1 | proven requirement or invariant |
| 2 | measured failure or incident lesson |
| 3 | accepted historical decision |
| 4 | operational lesson |
| 5 | useful corpus or reference material |
| 6 | implementation detail explicitly rejected |
| 7 | obsolete or superseded |
| 8 | unresolved question requiring a ledger item |

**The grain rule, stated so the "exactly one" is honest:** a surface holds many artifacts, so a
surface is classified by *the highest-value class any artifact in it reaches*, and the row's
note names that artifact. Lower-value material in the same directory is accounted for by that
surface's row and is not separately routed. Classes rank 1 > 2 > 3 > 4 > 5 > 8 > 7 > 6 for this
purpose — 8 sits below 5 because an unresolved question is worth less than material already
usable, and above 7 because it is still live.

Every surface in classes 1–5 and 8 carries a destination, per `ARC-ECOSYSTEM-002`:

- **`KWB-n`** — a ledger item on this board owns it.
- **record** — a governing record here owns it.
- **corpus** — it lands as corpus material or a fixture, here in `docs/corpus/`, and asserts
  nothing about architecture.
- **upward** — it is a KWB-domain finding (rationale, semantic intent, requirements
  elicitation) awaiting the crossing `P11-ECOSYSTEM-UPWARD` governs. That crossing is **not
  open**, so the finding is written down where it was found and is not asserted as KWB
  knowledge on its own authority.

Classes 6 and 7 are not retained and carry no destination; `—` is correct for them and is not
an omission.

## The inventory

Columns: files, lines, class, evidentiary level, destination.

| Surface | F | L | C | Lvl | Destination · note |
|---|--:|--:|:-:|:-:|---|
| `Api/(root)` | 2 | 119 | 6 | I | — composition root; organization, excluded by `ARC-ECOSYSTEM-002` |
| `Api/Authentication` | 3 | 147 | 4 | O | corpus · API-key auth with no read/write distinction; the corpus flags it (game plan 009) |
| `Api/Contracts` | 2 | 126 | 1 | O | KWB-4 · `UnreadableCoverage`: an unreadable source is its own outcome, not an absence |
| `Api/Endpoints` | 16 | 2410 | 1 | O | KWB-6 · the read surface is separable from the write surface at the host boundary |
| `Api/Extensions` | 5 | 679 | 6 | I | — DI and options wiring |
| `Api/Infrastructure` | 21 | 1187 | 2 | O | KWB-2 · durable job scheduler; D19's dead-letter remediation and the 37-attempt retry leak |
| `Cli/(root)` | 3 | 198 | 4 | O | corpus · the CLI read `Persistence:Provider` only after a command had silently written to files |
| `Cli/Commands` | 31 | 6219 | 4 | O | corpus · ≥27 named subcommands; `merge-audit` and `coverage-audit` are the audit-with-independent-expectation pattern |
| `Core/(root)` | 3 | 112 | 6 | I | — |
| `Core/Annotations` | 3 | 78 | 6 | I | — port interfaces |
| `Core/Claims` | 2 | 105 | 1 | E | KWB-5 · a model's verdict is carried as a model's verdict, in the type |
| `Core/Code` | 2 | 96 | 5 | E | corpus · code-graph query surface (S14) |
| `Core/Concepts` | 32 | 1758 | 1 | O | KWB-6 · `IConceptRepository` vs `IConceptHistory`: which world you read is a constructor dependency |
| `Core/Coverage` | 8 | 391 | 1 | O | KWB-4 · the coverage ledger interfaces and `CoverageAudit`'s independent expectation |
| `Core/Documents` | 6 | 206 | 5 | O | corpus · keyword tokenizer and relevance scorer |
| `Core/Findings` | 2 | 522 | 5 | E | corpus · `GraphFinding`; the unified finding model the prototype's own roadmap still lists as missing |
| `Core/Infrastructure` | 8 | 297 | 1 | E | KWB-2 · `IAtomicWriteCoordinator`, `ISerializableWriteCoordinator`, `IStageLock` — the write door's ancestors |
| `Core/Learning` | 1 | 28 | 5 | D | corpus |
| `Core/Observability` | 13 | 414 | 2 | O | KWB-2 · `PipelineOutcome` is D20 remediated (zero is now `Unknown`); `LlmCallOutcome`'s zero is still `Succeeded` |
| `Core/Pipeline` | 4 | 184 | 1 | E | KWB-3 · `EpistemicGraphExtractionResult` names the 26 collections extraction actually asks for |
| `Core/Proofs` | 3 | 129 | 5 | E | corpus |
| `Core/Provenance` | 9 | 553 | 1 | O | KWB-4 · `AppendOnlyRepository`, `ConceptCitation`, `SourceOverlap`; "append-only was a word in a doc comment" (C4) |
| `Core/Semantics` | 1 | 157 | 1 | E | KWB-3 · `NegativeKnowledgeAudit`: a negative edge must not be traversable as a positive one |
| `Core/Sources` | 4 | 116 | 4 | O | corpus · `ISourceFailureJournal`; a source's identity is its path, not the string someone typed |
| `Core/Synthesis` | 2 | 57 | 5 | D | corpus |
| `Domain/(root)` | 4 | 186 | 1 | O | KWB-1 · `DeterministicId` — the entire cross-source dedup mechanism |
| `Domain/Admission` | 2 | 208 | 1 | E | KWB-5 · `AdmissionProbe`, `VectorSimilarity` |
| `Domain/Agents` | 1 | 33 | 6 | I | — one of the twelve identical nodes |
| `Domain/Algebra` | 4 | 658 | 1 | O | KWB-3 · the prototype's generalized answer to D18 |
| `Domain/Annotations` | 3 | 87 | 5 | O | corpus |
| `Domain/Argumentation` | 2 | 83 | 6 | I | — identical node plus its relation |
| `Domain/Artifacts` | 1 | 33 | 6 | I | — |
| `Domain/Claims` | 9 | 813 | 1 | O | KWB-4 · `Claim`, `ClaimIdentity`, `ClaimGrounding`, `CorroborationAssessment`, `GroundingPolicy` |
| `Domain/Code` | 6 | 410 | 5 | O | corpus · code as a first-class node (S14) |
| `Domain/Concepts` | 22 | 1064 | 1 | O | KWB-5 · `AliasDerivation` is D18's fix; `ConceptNameRules`, `ConceptStatus`, `ConceptMerge` |
| `Domain/Conceptual` | 13 | 803 | 1 | E | KWB-3 · facets, granularity levels, theory containers, external-ontology anchoring |
| `Domain/ConnectionDiscovery` | 23 | 1309 | 1 | O | KWB-3 · the nine discriminations applied; adversarial tester; negative-assertion oracle |
| `Domain/Context` | 2 | 103 | 8 | I | KWB-3 · "context participates in identity" is asserted as an invariant and implemented as a node with no reader |
| `Domain/Coverage` | 10 | 470 | 1 | O | KWB-4 · `CoverageOutcome`'s four values, `Unmet` included |
| `Domain/Documents` | 8 | 589 | 1 | O | KWB-4 · `Chunk`, `ChunkIdentity`, `PassageKind`, `TextQuality` |
| `Domain/Epistemic` | 3 | 202 | 8 | I | KWB-3 · `IEpistemicNode`, 27 implementors, one unvalidated hypothesis |
| `Domain/Events` | 1 | 33 | 6 | I | — |
| `Domain/Evidence` | 1 | 33 | 8 | D | KWB-3 · **one file, 33 lines.** There is no evidence subsystem here to preserve; see *What is held* |
| `Domain/Explanation` | 2 | 82 | 6 | I | — |
| `Domain/Historical` | 4 | 151 | 6 | I | — three of the twelve, plus `HistoricalRelation` |
| `Domain/Hyperedges` | 7 | 276 | 1 | E | KWB-3 · reified relations declaring required participants; incomplete ones dropped with a reason |
| `Domain/Hypotheses` | 2 | 58 | 1 | O | KWB-3 · a hypothesis is not a fact and never becomes one by accumulation |
| `Domain/Jobs` | 3 | 142 | 2 | O | KWB-2 · `PendingJobStatus.DeadLettered`, and why success-deletes-the-row was wrong |
| `Domain/Knowledge` | 4 | 148 | 5 | O | corpus · the taxonomy seeded from the xvpe catalogue (D3, D7) |
| `Domain/Learning` | 8 | 224 | 5 | E | corpus |
| `Domain/Ledger` | 2 | 70 | 1 | E | KWB-4 · `DerivationLedger`, `DerivationRecord` |
| `Domain/Lexical` | 2 | 100 | 1 | E | KWB-3 · the lexical layer of the six |
| `Domain/Methods` | 1 | 33 | 6 | I | — |
| `Domain/Modules` | 19 | 1430 | 1 | O | KWB-3 · pack schema, loader, registry, validator — **and nine hand-written `IDomainModule` twins with zero consumers** |
| `Domain/Observability` | 7 | 541 | 4 | O | corpus · `LlmCallRecord`, `ModelPricing`, `GoldenSetScorer`; the flight recorder wired to one provider |
| `Domain/Processes` | 1 | 33 | 6 | I | — |
| `Domain/Projects` | 3 | 72 | 5 | D | corpus |
| `Domain/Proofs` | 7 | 413 | 1 | O | KWB-4 · the two-axis rigor model (D2): rigor and accessibility are independent |
| `Domain/Propositional` | 2 | 133 | 1 | E | KWB-3 |
| `Domain/Representational` | 2 | 108 | 1 | E | KWB-3 |
| `Domain/Schemas` | 9 | 1497 | 1 | O | KWB-3 · the nine discipline packs; **only their relation algebra reaches the runtime** |
| `Domain/Scoring` | 3 | 108 | 5 | E | corpus |
| `Domain/Search` | 2 | 42 | 5 | D | corpus |
| `Domain/Semantic` | 4 | 108 | 8 | I | KWB-3 · `SemanticRelationType` is a **second** relation vocabulary alongside `UniversalRelationKind` |
| `Domain/Semantics` | 2 | 293 | 1 | O | KWB-3 · `NegativeAssertion`'s mandatory falsifier; `RelationAssertion` as a read-only projection |
| `Domain/Sources` | 7 | 178 | 1 | O | KWB-4 · `Source`, `SourceQuality`, `SourceStatus` |
| `Domain/Structure` | 4 | 298 | 1 | O | KWB-4 · `StructuralArtifactScanner` produces the `Unmet` outcome (D16) |
| `Domain/Synthesis` | 8 | 323 | 5 | O | corpus |
| `Domain/Universal` | 5 | 457 | 8 | I | KWB-3 · **21** relation kinds and their constraint table; see the correction below |
| `Domain/Validation` | 1 | 130 | 4 | E | corpus · `DomainPolicyEnforcer` |
| `Domain/Values` | 7 | 318 | 1 | O | KWB-1 · `Confidence`, `RigorLevel`, `AccessibilityLevel` and their JSON converters — the persistence-boundary lesson |
| `Enrichment/(root)` | 3 | 599 | 4 | O | corpus · `AddDomainPacks` is the single wire that makes the packs load-bearing at all |
| `Enrichment/Annotations` | 19 | 5022 | 1 | O | KWB-5 · 17 model stages plus `ExactAliasConceptNormalizer` (643 lines), D18's fix |
| `Enrichment/ConnectionDiscovery` | 6 | 922 | 1 | O | KWB-3 · the model-side half of connection discovery |
| `Enrichment/Discovery` | 4 | 198 | 7 | I | — **superseded by the two `ConnectionDiscovery` surfaces; zero production consumers, one green test file** |
| `Enrichment/ExternalOntology` | 1 | 170 | 5 | E | corpus · `WikidataSparqlResolver` |
| `Enrichment/Observability` | 4 | 251 | 4 | O | corpus · the flight recorder's provider gap |
| `Enrichment/Options` | 4 | 154 | 6 | I | — |
| `Enrichment/Pipeline` | 31 | 6591 | 1 | O | KWB-5 · the admission pipeline; **`AdmitChunk` is still unconsumed**, disarmed by a config default |
| `Enrichment/Providers` | 11 | 1599 | 1 | O | KWB-5 · four providers behind one port, plus **record/replay** (`RecordedCompletionLibrary`, `ReplayChatCompletionClient`) |
| `Infrastructure.EfCore/(root)` | 10 | 2260 | 8 | O | KWB-10 · `DbContext`, CHECK constraints, the partial unique index — storage semantics, not a port target |
| `Infrastructure.EfCore/ConnectionDiscovery` | 2 | 94 | 5 | E | corpus |
| `Infrastructure.EfCore/Ledger` | 1 | 42 | 1 | E | KWB-4 |
| `Infrastructure.EfCore/Queries` | 6 | 1201 | 8 | O | KWB-10 · `GraphAnalytics`, `TemporalQueryExtensions`, `DerivedViews`, `AnalyticalQueries` |
| `Infrastructure.EfCore/Repositories` | 18 | 2800 | 8 | O | KWB-10 · the filter split lives here; `EfWeightedConceptSearch` is the measured retrieval path |
| `Infrastructure/(root)` | 1 | 383 | 6 | I | — |
| `Infrastructure/Code` | 1 | 213 | 5 | E | corpus |
| `Infrastructure/ConnectionDiscovery` | 1 | 21 | 5 | E | corpus |
| `Infrastructure/Coverage` | 5 | 503 | 1 | O | KWB-4 · the coverage ledger implementations and `CoverageAudit` |
| `Infrastructure/Fetching` | 3 | 283 | 4 | O | corpus · file, folder and web fetchers; the library-moved incident |
| `Infrastructure/Hashing` | 1 | 21 | 1 | E | KWB-1 · `Sha256HashService` |
| `Infrastructure/Ledger` | 1 | 46 | 1 | E | KWB-4 |
| `Infrastructure/Null` | 14 | 241 | 1 | O | KWB-5 · fourteen null objects: the corpus's design test — *remove the model, does the feature still make sense* — made mechanical |
| `Infrastructure/Observability` | 7 | 497 | 4 | O | corpus |
| `Infrastructure/Parsing` | 3 | 751 | 4 | O | corpus · PdfPig plus `WordReconstruction`; PDF text-layer repair is real, hard-won work |
| `Infrastructure/Persistence` | 4 | 820 | 1 | O | KWB-2 · `JsonAtomicWriteCoordinator`; the `.bak` files under `Api/data-backups` are its output |
| `Infrastructure/Repositories` | 19 | 1485 | 8 | O | KWB-10 · three backends behind one interface — the claim ADR 0001 rests on, and the one its own successor doubts |
| `Infrastructure/Sources` | 1 | 67 | 4 | E | corpus |
| `Infrastructure/Text` | 1 | 251 | 5 | E | corpus |
| `Ingestion/(root)` | 3 | 592 | 5 | O | corpus |
| `Ingestion/ReferenceCatalog` | 3 | 724 | 5 | O | corpus · the xvpe `catalog.toml` / `reference_registry.toml` bridge (D7) |
| `Mcp/(root)` | 4 | 647 | 1 | O | KWB-6 · **nine** read-only tools, every reply bounded, mutation excluded from the surface |
| `ProjectIntegration/(root)` | 2 | 129 | 5 | D | corpus |
| `Retrieval/(root)` | 5 | 614 | 1 | O | KWB-6 · keyword, semantic and hybrid search |

104 rows. Class totals: **1** × 43, **2** × 3, **3** × 0, **4** × 12, **5** × 22, **6** × 14,
**7** × 1, **8** × 9. Classes 1–5 and 8 total **89**, and each of those 89 rows carries a
destination. Recount the tally with:

```sh
grep -oE '^\| `[A-Za-z.]+/[A-Za-z()]+` \| [0-9]+ \| [0-9]+ \| [0-9] \|' \
  docs/corpus/prototype-inventory.md | grep -oE '\| [0-9] \|$' | tr -d '| ' | sort | uniq -c
```

Class 3 — *accepted historical decision* — is empty in this table because a decision is not a
directory. The prototype's accepted decisions live in `docs/adr/` and in `FEATURE_MATRIX.md`'s
*Open decisions*, and are inventoried immediately below.

### Surfaces outside `src/`

| Surface | Lines | C | Lvl | Destination · note |
|---|--:|:-:|:-:|---|
| `AGENTS.md` incident register | 269 | 2 | O | `prototype-incidents.md` — answered incident by incident |
| `docs/FEATURE_MATRIX.md` | 2028 | 2 | O | corpus · the densest measurement log in the tree; 161 evidence rows over four independent columns |
| `docs/ROADMAP.md` | 336 | 1 | O | corpus · Part 0's admission rule, Part 2's fifteen invariants, Part 1's thirteen verified-absent gaps |
| `docs/ARCHITECTURE.md` | 174 | 1 | O | corpus · the only prose statement of the universal kernel's intent; **verdict its claims against the table above, not the reverse** |
| `docs/ADMISSION.md` | 910 | 1 | O | KWB-5 · the admission protocol, read before write |
| `docs/adr/0001-implementation-language.md` | 228 | 3 | O | record · superseded by `D-136`/`D-001`; retained because its *reasoning* is evidence, and one strand of it is contradicted from inside the same corpus |
| `docs/adr/0002-target-framework-version.md` | 222 | 6 | I | — runtime selection for a runtime this repository does not have |
| `docs/CALIBRATION_WORKSHEET.md` | 1262 | 5 | O | corpus · threshold calibration against a real corpus; the overfitting hazard the game plan names (005) |
| `docs/WORKLOG.md` | 1106 | 4 | O | corpus · dated operational history |
| `docs/language-selection.md` | 810 | 3 | O | corpus · the general framework ADR 0001 applies |
| `docs/CODE_GRAPH.md` | 180 | 5 | E | corpus |
| `docs/OPERATIONS.md`, `RUNNING.md`, `load-testing/`, `oidc/` | 538 | 4 | O | corpus · deployment and operations |
| `docs/incidents/admission_failure_outages.md` | 22 | 2 | O | corpus |
| `scripts/`, `infra/` | — | 4 | O | corpus · smoke and e2e scripts; AKS, Helm, Terraform — deployment shape, not architecture |
| `calibration.md` (repo root) | 10090 | 5 | O | corpus · raw calibration output |

### The design corpus

`C:/Users/kmett/Downloads/Brainstorming/KWB`. Navigate by `_index.md`; do not read the raw
transcripts linearly.

| Source | Size | C | Destination · note |
|---|---|:-:|---|
| `kwb full game plan` | 676 topic files from 321 turns; 26,102 lines | 1 | corpus · the primary design corpus |
| `KWB_epistemic_graph_layering` | 20 topic files | 1 | KWB-3 · the prototype's `Domain/` tree *is* this document |
| `KWB_database_depth_assessment` | 7 topic files | 8 | KWB-10 · argues the end state needs *more* in the database |
| `KWB_project_assessment_overview` | 15 topic files | 4 | corpus |
| `essay.txt` | 77,932 bytes, **unsplit and absent from `_index.md`** | 8 | corpus · see below |

**A completeness defect in the corpus index itself.** `_index.md` says "4 conversations, 4 of
them split into 718 topic files" and lists four. A fifth file, `essay.txt`, sits beside them,
is not split, is not indexed, and was last modified 2026-09-12 — the day of this inventory. It
concerns two further repositories (`MultiModelEssayReviser`, `essay-enricher`) and their
relation to what it calls "the KnowledgeWorkbench composition/revision concept", which is
KWB-domain material. **It is recorded here as present and unread**, not silently omitted. A
later session that reads it is reading corpus this inventory did not cover.

**Superseded: topics 664–676**, which conclude *"stay with .NET for the main platform."* Dated
2026-08-24 and closed by `D-136`. Do not treat it as authority and do not argue with it.

**The supersession boundary is not file-aligned.** Topic **665**,
`the-main-architectural-rule`, carries live and superseded material in one file: its first half
states the separation of *semantic truth* from *physical execution strategy* — the single most
reusable idea in that stretch — and its second half is the .NET recommendation. Do not discard
665 with its neighbours.

**Live, and named so a later reader can find them:** the six semantic layers, the universal
kernel, relation algebra, negative knowledge, connection hypotheses and domain packs
(`KWB_epistemic_graph_layering.txt`, of which the prototype's `Domain/` tree is the
implementation); the ontological admission rule (61); the feature admission rule (421);
semantic truth versus physical execution strategy (665); KWB as LLVM- and Git-shaped
infrastructure (412–436); the Knowledge IR (84, 105, 315); the three-plane boundary and the
constraints that KWB is optional and never required infrastructure, must not own runtime state,
and holds knowledge that must not become `xvpe-primitive` (`XVPE/video 6.txt` §§5–9, 16–20);
and the knowledge-scope hierarchy (`XVPE/video 6.txt` §17).

## What the inventory changes on the board

### Corrections to existing items

**`KWB-3` is wrong about the size of the relation vocabulary, in the direction that matters.**
It says "the nine relation kinds" and its `done_when` asks whether "the nine discriminations
are complete and mutually exclusive." `Domain/Universal/UniversalRelationKind` has **21**
members: nine discovery discriminations and twelve structural families — `Taxonomic`,
`Compositional`, `Dependency`, `Logical`, `Evidential`, `Normative`, `Representational`,
`Functional`, `Temporal`, `Contextual`, `Contradiction`, `Contrast`. An implementer who
re-derives only the nine re-derives the vocabulary of *cross-domain discovery* and inherits,
unexamined, the vocabulary that types every relation the packs actually declare. The item text
is corrected accordingly.

**The prototype's real answer to D18 is not the nine kinds; it is the constraint table, and it
is executable.** `UniversalRelationKindExtensions.Constraint()` records, per kind, whether
symmetry is required or forbidden, whether transitivity is forbidden, and whether negative
knowledge is required; `AlgebraViolations` reports every way a declared relation contradicts
its kind; and `DomainPackValidator.ValidateRelationAlgebra` calls it on the pack-load path
(`DomainPackValidator.cs:196`, reached from `DomainPackRegistry.cs:86`). Its comment on
`Identity` states the D18 lesson directly: *"Identity is a genuine equivalence relation; that
is precisely why it is the only kind safe to feed to a union-find."* That is the artifact worth
re-deriving. It is Exercised: shipped packs are asserted clean by
`ShippedDomainPackTests.EveryShippedPack_LayersOntoTheUniversalKernel`.

**Nine hand-written `IDomainModule` implementations have no consumer at all.**
`PhysicsModule`, `MathematicsModule`, `BiologyModule`, `HistoryModule`, `ArtModule`,
`PhilosophyModule`, `PsychologyModule`, `EngineeringModule`, `ComputerScienceModule` — 1,430
lines across `Domain/Modules` — are referenced by **zero** files outside their own directory,
and by exactly one test file each: `ModulePackDriftTests`, which keeps them in sync with the
YAML packs. Two full descriptions of nine disciplines, a test enforcing agreement between them,
and nothing that reads either. Do not rebuild the twin.

**A pack's seven contributions do not reach the runtime equally.** Only the relation algebra
crosses: `AddDomainPacks` pushes each pack's declared algebra into `RelationAlgebraRegistry`,
which the traversal and closure code reads. Entity types, decomposition rules, validation
constraints, inference rules, representations and external identifiers have no runtime
consumer — no enrichment stage, no prompt, no query resolves against them.
`ShippedDomainPackTests.EveryPack_ContributesAllSevenThings` asserts that each pack *declares*
all seven, which is a real check and not the same check.

**`ValidationConstraint` cannot fire, and not for the reason the architecture doc gives.**
`Domain/Modules/Rules/ValidationConstraint` is `{ Name, Description, TargetEntityType?,
TargetRelationType? }` — a prose description and a target, with no predicate. `ARCHITECTURE.md`
observes that a constraint naming a relation the pack does not declare can never fire; the
stronger fact is that **no** validation constraint can fire, because none of them is
executable. They are documentation carried in a type.

**`KWB-1`'s `done_when` is right, and the prototype is its counterexample.**
`DeterministicId.From(params string?[])` is a bare static that accepts arbitrary strings —
exactly the construction the item forbids. Its design is worth carrying whole: a name-based
UUID over SHA-256 tagged **version 8** (RFC 9562's custom slot) rather than impersonating a v5;
an ASCII unit separator `0x1F` chosen because it cannot survive the whitespace normalisation
applied to every text part, so a delimiter can never be forged from content; whitespace
normalised but **case deliberately not**, because a claim that changed its capitalisation
changed what it says; and a null part treated as an empty field in a fixed layout, so
`(a, null, b)` cannot collide with `(a, b, null)`.

**Identity discipline was implemented once and applied almost nowhere.** **87** types in
`Domain/` default their `Id` to `Guid.NewGuid()`. **Nine** files call `DeterministicId`. The
twelve identical kernel nodes are all in the first group, so the newest tier of the graph mints
random identity while the doc comment on `DeterministicId` explains that random identity is
what rots a citation. This is the sharpest available argument for `KWB-1`'s type-level
requirement: a discipline a caller must remember is a discipline 87 callers forgot.

**`KWB-8`'s stated precondition now has an item.** Its `done_when` says the precondition is
written in prose "rather than encoded as a dependency edge the ledger does not actually have;
when an item for it exists, this sentence is replaced by a real `depends_on`." `KWB-9` is that
item, and the edge is added.

### Uncovered ground, which becomes new items

- **`KWB-10` — storage semantics.** The question is *what storage semantics did the prototype
  prove are required*, not how to port EF and Postgres.
- **`KWB-11` — the Knowledge IR.** Absent from the prototype's source entirely — but **not**
  absent from its record: `ROADMAP.md` carries it as item **I1** with the exact stage list from
  corpus topics 84 and 315, and a closing predicate (*a wrong merge is localised to a named
  stage by inspecting IRs, not by reading SQL rows*). The item establishes what representations
  must survive between stages; it does not invent a compiler.
- **`KWB-12` — knowledge scopes.** Absent from the prototype and from the board.

### Findings routed *upward*, and therefore parked

These are KWB-domain findings — statements about knowledge, not about this repository's
architecture. `P11-ECOSYSTEM-UPWARD` is not open, so they are written here and asserted
nowhere:

- The scope hierarchy of `XVPE/video 6.txt` §17, and §16's promotion rule: a project decision
  is recorded as evidence under stated conditions, and only accumulated evidence forms a
  broader conditional claim.
- The ontological admission rule (topic 61): a distinction earns its place by preventing an
  invalid merge, enabling a query, changing inference, improving provenance, exposing
  disagreement, supporting domain validation, improving connection discovery, or preventing a
  misleading synthesis — and a distinction that never changes behaviour only adds vocabulary.
- The feature admission rule (topic 421) as `ROADMAP.md` Part 0 restates it, together with its
  design test: *if the model were replaced by a perfect human annotator, would this feature
  still make sense?*

## What is held, and what is safe to proceed on

`D-004` holds both lists, so this file does not become a second authority for them. The
governing rule while anything remains held is narrower than "nothing moves": **observation can
proceed; a decision whose answer depends on the inventory cannot.** This inventory is itself an
observation, which is why it could be made while the reference miner runs.

The clearest held case is the one the table already shows: `Domain/Evidence` is **one file, 33
lines**. There is no mature evidence subsystem in the prototype to preserve, and the
distinctions the miner is still discovering are exactly the ones an evidence model would have
to encode. Designing it against the prototype would replace one speculative model with another.

## What would falsify this inventory

A completeness claim that cannot be attacked is not a completeness claim.

1. **A load-bearing requirement in the .NET tree that this inventory does not list.** Check it
   by running the denominator command; if it returns a surface absent from the table, or a
   count other than 104, the claim is false. This is the check the denominator exists to make
   possible.
2. **A surface whose class is wrong in the direction that loses evidence** — anything filed 6
   or 7 that a later reader shows is load-bearing. The 14 rows in class 6 and the 1 in class 7
   are the whole exposure, and they are listed rather than summarised so the attack surface is
   finite. `Enrichment/Discovery` is the single class-7 entry and the argument for it is one
   command: zero production consumers, one test file.
3. **An evidentiary level assigned too high.** Every `O` asserts that something ran against the
   real corpus and the result was checked. No run was performed here: `O` means *the prototype
   recorded a checked result*. Where that rests on the prototype's own reporting it inherits
   the prototype's own warning — *a zero is not a result, and a count is not a check* — and a
   reader who finds the underlying report is a count rather than a reason has falsified that
   row.
4. **A destination that routes a retained artifact nowhere.** 89 rows are in retained classes
   and 89 carry a destination; a row in classes 1–5 or 8 with an empty destination column is a
   defect.
5. **The grain rule hiding something.** A surface is classified by its best artifact, so a
   class-1 directory may contain material worth class 6 and a class-6 directory may contain one
   overlooked gem. The second is the dangerous direction. A reader who finds a retained
   requirement inside a class-6 surface has falsified that row, and the fix is to reclassify
   the row, not to widen the rule.
6. **The corpus, which is explicitly not exhausted.** 718 indexed topic files were navigated by
   index and named selectively; `essay.txt` was not read at all and is recorded as unread. A
   requirement found in unread corpus does not falsify the *prototype* inventory — its
   denominator is the `src/` tree — but it does falsify any reading of this file as "the design
   corpus has been consumed." It has not been.
7. **A later prototype commit.** This is a measurement of a working tree on 2026-09-12. The
   prototype is not frozen. If it changes, the counts here are historical, and the denominator
   command is how a reader finds that out.

## Cited measurements, and how each was taken

| Claim | How |
|---|---|
| 104 scoped surfaces, 6 empty directories | the denominator command above |
| 699 files / 151,392 lines in `src/`; 80 / 91,527 generated migrations | `find`/`wc` per project; migrations counted by path |
| 325 test files / 53,802 lines; 304 test classes; 1,947 `[Fact]`/`[Theory]` | `find`/`grep -c` over `tests/` |
| twelve byte-identical node types | each file's type name normalised to `TYPE`, whitespace stripped, `sha256sum`; one digest for twelve files |
| 27 `IEpistemicNode` implementors | `grep -rl ': IEpistemicNode' src/KnowledgeWorkbench.Domain` |
| 21 `UniversalRelationKind` members | enumerated from the enum body |
| 87 types defaulting `Id` to `Guid.NewGuid()` against 9 `DeterministicId` call sites | `grep -rl` on each pattern |
| 40 types with `Confidence { get; set; } = 1.0` (41 counting one `init`) | `grep -rl` per accessor form |
| nine `IDomainModule` implementations with zero consumers outside `Domain/Modules` | `grep -rl` per class name, excluding that directory |
| nine MCP tools | `[McpServerTool(Name = …)]` attributes in `Mcp/KnowledgeTools.cs` |
| `AdmitChunk` still unconsumed | `AdmissionPipeline.cs:76-81`, `AdmissionWorker.cs:81,92` |
| 1.3 ms vector against 38 ms keyword at 7,914 chunks | the prototype's own `PostgresKeywordSearchTests.cs:146` |
