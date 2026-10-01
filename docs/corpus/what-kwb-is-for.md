# What KWB is for, and which half of it the prototype built

Two readings placed beside each other: what the design corpus says KnowledgeWorkbench is, and
what the .NET prototype actually shipped. Measured 2026-09-12.

`docs/corpus/prototype-inventory.md` classified the prototype's **code surfaces** — 104
directories, by class and evidentiary level. That answers *what exists*. It does not answer
*what the system does*, and it never touched the question of what the system is **for**. This
file is that question, and the answer changes what the board should hold.

A corpus artifact under `ARC-ECOSYSTEM-002`: evidence about content. It proposes no crates.
`README.md` is the authority for bands and `tests/contract` asserts it; where this file reports
a gap against the README, the gap is the finding, not a proposal.

## The target, from the design corpus

Citations are topic-file numbers in `C:/Users/kmett/Downloads/Brainstorming/KWB/kwb full game plan`.

**The mission** (422):

> To make complex knowledge explicit, durable, comparable, auditable, and operationally useful
> without erasing uncertainty, disagreement, context, or provenance.

**The core identity** (412). KWB takes books, papers, code, documentation, arguments,
historical sources, engineering incidents, conversations, experiments and organizational
decisions, and converts them into concepts, claims, definitions, arguments, evidence, events,
processes, methods, examples, failures, decisions, contexts, relations, uncertainties and
derivations — then supports operations over that model. *"KWB is not primarily a note-taking
application, chatbot, graph viewer, or ontology editor. Those can all be interfaces or
applications built on top of it."*

**Nine operations** (415) — the workbench's whole surface:

```text
compile   check   link   compare   reconstruct   query   synthesize   assess   revise
```

**Six layers** (414):

```text
1  source and evidence          what material do we actually have?
2  semantic compilation         what does the source appear to say or contain?
3  ontology and language        what kinds of entities and relationships are these?
4  epistemic                    why should anything here be believed, and how strongly?
5  reasoning and analysis       what follows, conflicts, depends on, or connects to what?
6  projection and application   how should this be presented or used for a particular task?
```

**Six exclusions** (417). Not merely RAG — *"RAG retrieves passages; KWB constructs persistent
typed knowledge with identity, provenance, and dependencies."* Not merely a knowledge graph,
not merely an LLM application, not merely an ontology, **not a universal truth machine** —
*"it should not flatten contested domains into one authoritative graph"* — and not a
replacement for original sources.

**The platform realization** (436). KWB is LLVM- or Git-shaped: an embeddable runtime with a
CLI, an MCP surface, an SDK and editor integrations above it. *"The core isn't the interface.
The core is the engine."* The corpus proposes naming the runtime separately from the platform
so that every interface — a person, an IDE, XVPE, an autonomous agent — uses one semantic
engine rather than reimplementing KWB's logic.

**The admission rule** (421), which is what keeps the above from becoming everything software:

> A KWB feature must operate on explicit semantic knowledge, preserve provenance, or improve
> the compilation, validation, comparison, reconstruction, or application of that knowledge.

The prototype's own `ROADMAP.md` Part 0 already carries the nine verbs, the six layers and this
rule, plus a second test worth keeping: *if the model were replaced by a perfect human
annotator, would this feature still make sense?* Yes → it is KWB. No → it is an AI capability
KWB happens to use.

## What the prototype built

`docs/FEATURE_MATRIX.md` is the prototype's own feature list: **161 rows** across fifteen
numbered sections, each with four independent evidence columns (implementation, unit,
integration, end-to-end) and two stated rules — *a checkbox is a claim about evidence, not
intent*, and *a schema is not a feature*.

Counted mechanically: **143 of 161 rows are implemented; 55 reach end-to-end.**

| Section | Rows | Impl | E2E |
|---|--:|--:|--:|
| 1. Ingestion | 9 | 8 | 7 |
| 2. Embeddings and search | 8 | 7 | 5 |
| 3. Chat providers | 10 | 10 | 1 |
| 4. Enrichment — concepts | 9 | 9 | 2 |
| 5. Enrichment — synthesis | 9 | 9 | 1 |
| 6. Formal knowledge — proofs, rigor | 18 | 17 | 5 |
| 7. Subject taxonomy | 7 | 7 | 6 |
| 8. Extraction thoroughness | 8 | 7 | 3 |
| 9. Examples and exercises | 5 | 3 | 2 |
| 10. Graph traversal | 8 | 7 | 1 |
| 10a. Provenance — citation, coverage, overlap | 14 | 13 | 5 |
| 11. Platform | 16 | 16 | 3 |
| 12. Consumers — human and agentic | 14 | 8 | 2 |
| 13. Admission protocol | 11 | 7 | 2 |
| 13a. Code as a first-class node | 6 | 6 | 6 |
| 14. Performance and complexity | 9 | 9 | 4 |

## The reconciliation

| Operation | Where the prototype has it | State |
|---|---|---|
| **compile** | §1 ingestion, §2 embeddings, §4 concepts, §8 thoroughness, §13 admission, §13a code | **built**, 44 of 51 rows implemented |
| **link** | §4 normalisation and identity judging, §7 taxonomy, §10 traversal | **built** |
| **query** | §2 keyword/semantic/hybrid search, §10 neighbourhood and path, §12 MCP tools | **built** |
| **synthesize** | §5 consensus, unique contributions, conflicts, misconceptions; §6 proofs and rigor | **built at concept grain, absent at document grain** — see the correction below |
| **check** | §8 structural recall audit and golden set, §10a coverage audit, §11 negative-knowledge check and unified findings | **partial**, and audit-shaped: it checks what a run did, not whether an argument holds |
| **compare** | §10a pairwise reference overlap — **one row** | **absent in substance** |
| **reconstruct** | — | **absent**; `kw reconstruct` is named absent in the prototype's own roadmap |
| **assess** | §9, 3 of 5 rows; generated exercises and verified solutions are both unticked in every column | **barely started** |
| **revise** | — | **absent**; `DerivationDependency` and staleness propagation are named absent |

**Four of the nine operations have essentially no implementation, and they are the same four in
both directions.** The prototype's own roadmap lists thirteen gaps "each verified absent from
the source tree, not inferred," and they land almost entirely in that half: semantic
reconstruction, semantic source diff, the assessment engine, graph versions and snapshots,
merge preview→validate→apply, and `DerivationDependency` staleness propagation are six of the
thirteen; most of the remainder are `check`.

### Said plainly

**The prototype is a compiler front end with a synthesis stage.** It compiles sources into a
typed graph, links concepts across them, answers queries over the result, and synthesizes
consensus and conflict. That part is real, measured, and in places operationally validated.

The back half of the workbench — comparing two bodies of knowledge, reconstructing a source
from the model, assessing what a person or agent understands, and revising the model when its
inputs change — was designed in the corpus and not built. Which means the corpus is not a
wish-list layered over a finished system; **it is the half that is missing, described in
detail.**

## What this changes here

`AGENTS.md` names `README.md` as the authority for what exists and what owns what. It says:

> KnowledgeWorkbench (KWB) ingests references and builds a durable graph of the concepts they
> teach, the claims they assert, and the code that implements them.

That is *compile* and *link*, and a graph. It is an accurate description of **what the
prototype does**, and it is roughly two of six layers and three of nine operations. Nothing in
this repository states the mission, the nine operations, the six layers, the exclusions, or the
runtime-versus-platform split — verified by search, zero occurrences outside this artifact.

The bands table was drawn against that README and `tests/contract` has been faithfully
asserting it ever since:

| Layer (414) | Crate in the bands table |
|---|---|
| 1 source and evidence | `kwb-store` |
| 2 semantic compilation | `kwb-ingest` |
| 3 ontology and language | `kwb-domain` |
| 4 epistemic | `kwb-domain`, nominally — it is named as owning evidence, coverage and the derivation ledger |
| 5 reasoning and analysis | **none** |
| 6 projection and application | **none** — `kwb-cli` and `kwb-mcp` are hosts, not a projection layer |

| Operation | Owner in the bands table |
|---|---|
| compile, link | `kwb-ingest` |
| query | `kwb-retrieval` |
| check | `kwb-domain`, arguably |
| **compare, reconstruct, synthesize, assess, revise** | **none** |

So the repository has inherited the prototype's scope rather than the corpus's target — which
is the expected outcome of a bootstrap written before the corpus had been read, and is
precisely the kind of thing a reconciliation pass exists to surface. It is **not** evidence
that the bands table is wrong: a band with no crate may be a band whose crate has not been
written yet, and `D-004` holds several of these subjects pending the reference miner. What it
is evidence of is that **no artifact in this repository currently states what KWB is for**, so
nothing would notice if the board drifted further toward the half that was already built.

Naming that gap is this file's whole job. Deciding what to do about it — whether the README
grows, whether bands are added, whether a runtime/platform split is adopted — is a decision,
and a decision belongs to a ledger item and a record, not here.

## Correction, and a subsystem neither reading above found

Added 2026-09-12 after reading `essay.txt` — the 78 KB file sitting beside the corpus,
unsplit and absent from its `_index.md`, which the prototype inventory recorded as *present and
unread*. It is the most recent design material in the corpus and it is explicitly about the
Rust KWB, so it postdates `D-136`. Reading it changed two things in this file.

### The correction: `synthesize` is built at the wrong grain

The table above originally read `synthesize` as **built** — 26 of 27 rows, the strongest part
of the tree. That was wrong, and the essay names why:

> The search does find `ConceptSynthesis` and a `ConceptSynthesisPipeline`, but that is
> synthesis **about a concept**, not document composition.

The corpus's own definition of the verb (415) is *"generate grounded explanations, documents,
curricula, and support content."* §5 generates none of those. It generates a consolidated
definition, consensus statements, unique contributions, conflicts and misconceptions — all of
them properties **of one concept**. So `synthesize` is built at concept grain and absent at
document grain, and the count of five-of-nine implemented in this file was an overcount of one.

This is `D-003`'s own rule turned on this artifact: a section being fully implemented is not
evidence it implements the operation it was filed under.

### The subsystem: composition and revision

The essay's central claim is that the prototype's roadmap — the thing I read as authoritative
about what was missing — **underrepresents composition and revision**. The implementation is
only:

```text
source → semantic graph        and        semantic graph → concept synthesis
```

and there is no first-class equivalent of `DocumentIntent`, `Outline`, `SectionIntent`,
`ClaimPlacement`, `EvidenceSelection`, `CompositionPlan`, `DraftArtifact`, `RevisionFinding`,
`RevisionCandidate`, `RevisionDecision` or `RevisionLineage`. *"This is the biggest thing I
would add to the design."*

The boundary it draws is the load-bearing part, and it is one line:

```text
Enrichment     source    → knowledge
Composition    knowledge → authored artifact
```

Which makes KWB two halves rather than one pipeline:

```text
KNOWLEDGE COMPILATION                 ARTIFACT ENGINEERING
Sources                               Artifact Intent
Documents                             Document Architecture
Chunks                                Composition Plan
Concepts / Claims        ────────→    Evidence Assignment
Proof / Conflict                      Argument Plan
Epistemic Graph                       Draft Artifact
                                      Evaluation Findings
                                      Revision Plan
                                      Candidate Revision
                                      Validation
                                      Human Decision
                                      Artifact Version
```

with provenance, derivation, coverage, execution traces, human decisions, versioning,
snapshots, model routing and cost accounting running alongside both.

The synthesis it states, which is the clearest one-sentence statement of KWB's shape anywhere
in the corpus:

> **KWB knows. Composition decides what knowledge belongs in an artifact and how it should be
> structured. Evaluation diagnoses the artifact against explicit goals. Revision proposes
> controlled transformations. Essay is one policy/plugin package that configures those general
> mechanisms.**

And the rule that keeps the halves apart: *"KWB core knows about knowledge and evidence, not
essays."* An essay must not leak `ThesisParagraph` or `ConclusionSection` into the core
semantic model; document type is a profile, not an engine. The essay lists fifteen further
artifact types the same engine would serve — design documents, requirements, API docs,
tutorials, manuals, reports — and treats the essay case as one specialization among them.

**Everything in the right-hand column is absent from the bands table**, which has no crate on
the artifact-engineering side at all. The essay proposes names for them (`kwb-composition`,
`kwb-evaluation`, `kwb-revision`, or a `kwb-authoring` façade over the three). Those are
recorded here as *what the corpus proposes*, not as a layout: `README.md` is the authority for
bands, `tests/contract` asserts it, and changing it is a decision with its own item.

### What this does to the central product loop

The corpus's ten-step product loop (054) is entirely within the left-hand column. It ends at
*"users compare current and historical interpretations"* — it never reaches an authored
artifact. So the loop and the two-half architecture are not the same claim at different sizes:
**the loop is the knowledge-compilation half, and the artifact-engineering half has no loop
stated for it at all.** A reader who takes 054 as "the full KWB workflow" is taking half of it.

## What would falsify this file

1. **A statement of the target already in this repository that the search missed.** The claim
   rests on grepping `f:/repos/kwb` for the verbs, the layer names, and "semantic workbench",
   which returned nothing outside `docs/corpus/`. A statement in other words falsifies it.
2. **A prototype implementation of compare, reconstruct, assess or revise that
   `FEATURE_MATRIX.md` does not list.** The reconciliation trusts that file's own claim to be
   the feature list. It is a document, and `D-003` applies to it: a row's checkbox is a claim
   about evidence. Re-derive from `src/` if a row looks wrong.
3. **A different mapping of sections to operations.** The table above is a judgement about
   which of nine verbs each section serves, made from section titles and row names. A defensible
   alternative mapping that moves a section into `compare`, `reconstruct`, `assess` or `revise`
   weakens the central finding, and the finding is only as good as that mapping.
4. **Corpus material contradicting the nine-operation framing.** The framing is taken from topic
   415 and corroborated by the prototype's own `ROADMAP.md` Part 0. 718 indexed topic files
   exist and were navigated by index, not read exhaustively. `essay.txt` has now been read and
   is folded in above; it already falsified one row of the table, which is the best available
   evidence that the remaining unread corpus can falsify others.
5. **The two-half framing itself.** It comes from one source — `essay.txt` — which is unindexed,
   undated in its own text, and was written about three repositories rather than as a statement
   of KWB's architecture. It is corroborated by the nine operations needing somewhere for
   `compare`, `reconstruct` and document-grain `synthesize` to live, and by the prototype having
   none of them. It is not corroborated by a second independent source, and it should not
   harden into a band boundary on one file's authority.
