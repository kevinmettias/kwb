# The prototype's incident register, answered

The prototype's `AGENTS.md` records data-loss incidents in its own voice. They are the strongest
material in that tree, because each was paid for with destroyed data, and a lesson bought that
way outranks any abstract recommendation.

This file answers four questions about each one, so the register becomes design pressure on
this repository rather than a reading list:

```text
What incident established it?
Where does the prototype still violate it?
Does the Rust KWB already encode it, and in what — a type, a runtime check, or a convention?
If not, which ledger item owns it?
```

The third question has the same answer everywhere today — **nothing; this repository has no
code yet** — so it is answered as *what the encoding would have to be*, which is the part a
ledger item can actually hold. Saying "not yet" nine times would be true and useless.

A corpus artifact under `ARC-ECOSYSTEM-002`. It decides nothing.

## The register is not five incidents. It is six, and two of them are numbered `D19`

`AGENTS.md` carries six incident sections. Two are headed `D19` and describe entirely different
failures:

| Heading | Subject |
|---|---|
| D17 | Destruction requires evidence |
| D18 | A rule proved pairwise is not proved of the algorithm that uses it |
| **D19 (a)** | Work handed to a queue is not work done |
| D20 | A default is a claim, and an enum's zero value is a default |
| D21 | A second implementation of the stage is not the stage |
| **D19 (b)** | A filter that applies itself is a check nobody asked for |

The prototype's `ROADMAP.md` Part 2 lists five invariants from the register and carries only
D19 (a). **D19 (b) is therefore an incident with a colliding identifier that the prototype's own
roadmap does not carry forward** — the one most likely to be lost, and the one whose subject
(which world a read returns) is closest to a query surface this repository has not built yet.
It is answered below as **D19-B**. This is a finding about the register, not a criticism of it.

---

## D17 — Destruction requires evidence

**What established it.** Seven stages deleted data on a belief nobody had checked, and every one
reported success. A merge fused `inertia tensor` into `aerodynamic tensor` and `speed` into
`velocity vector` — from the very passage explaining those two are different — and deprecated
**302 concepts**. A sweep deleted a frontier model's proofs because a *weaker* model examined
the same concepts and stayed silent. A `ReplaceAllAsync` deleted unconditionally and wrote
conditionally, so an empty-but-valid reply erased a concept's entire synthesis.

**The rules it produced.** Before a delete, deprecate, supersede or overwrite: *what belief
authorises this, and what would falsify it?* Then: the absence of an extraction is not evidence
against an artefact; delete-and-rebuild is safe only when the rebuild is deterministic and its
input complete — **the moment a model stands between the delete and the write, write first or do
not delete**; and a model's label, verdict or silence is not evidence.

**Where the prototype still violates it.**

- **`CoverageOutcome`'s zero value is `Yielded`.** The type that exists to separate *ran and
  found nothing* from *never ran* is itself defaulted to *ran and found something*. An unset
  coverage row reads as the most productive outcome available. This is D20 applied to D17's own
  primitive, and the two incidents were found on the same day without this crossing being noted.
- `ReplaceAllAsync` survives as a private helper in
  `Enrichment/Pipeline/ConceptSynthesisPipeline.cs:197`; 58 delete-shaped call sites remain
  across non-migration `src/`. That is not itself a defect — the rule is about evidence, not
  about the absence of deletes — but it is the surface the rule has to cover.

**What would encode it here.** Not a convention. A coverage type whose *unknown* value is its
default and whose `Barren` and `Skipped` cannot be the same value; and a delete path that
cannot be constructed without the evidence authorising it. The first is a type; the second is
the shape `kwb-store`'s write door already exists to impose, read in the destructive direction.

**Owned by** `KWB-4` (the coverage type) and `KWB-2` (the write door).

---

## D18 — A rule proved pairwise is not proved of the algorithm that uses it

**What established it.** `AliasDerivation.IsVariantOf` is correct, and has a test asserting in as
many words that `C++` and `C` are not variants. That test passes. `C` was merged into `C++` on
the live corpus anyway, along with `Nilpotent matrix` → `Newton's method`, `Taylor's theorem` →
`Total time`, and 141 others: **144 of 579 merges, 25%**, fusing ideas the rule explicitly
rejects.

The rule was consulted. Its answers went into a **union-find**, which computes a transitive
closure, and *is a variant of* is not transitive:

```text
IsVariantOf("z m",         "zero mass")   = true     a legitimate acronym of it
IsVariantOf("z m",         "zero matrix") = true     and a legitimate acronym of that too
IsVariantOf("zero matrix", "zero mass")   = FALSE    it said no. Nobody asked it.
```

Every ambiguous abbreviation became a **bridge**, and 1,878 green tests could not see it,
because all of them asserted properties of the **predicate** while the defect was in the
**structure that consumed it**.

**This is close to a canonical design case, and it outranks any abstract recommendation — from
GraphRAG or anywhere else — about merging similar entities.** An external recommendation to
merge similar things is advice; this is a measurement of what happens when you do.

**What the prototype built in response, and it is the part worth carrying.**
`Domain/Algebra/` is the generalized answer: `RelationAlgebra` records twelve algebraic
properties per relation (symmetry, antisymmetry, transitivity, reflexivity, functionality,
temporality, contextuality, definitionality, destructiveness, confidence-bearing, negative
knowledge), `RelationAlgebraRegistry` assigns one to every declared relation type, and
`UniversalRelationKindExtensions.Constraint()` states what each universal kind commits to so a
pack cannot declare an algebra its kind forbids. Its comment on `Identity` is the lesson in one
sentence:

> Identity is a genuine equivalence relation; that is precisely why it is the only kind safe to
> feed to a union-find.

`AlgebraViolations` is executable and runs on the pack-load path
(`DomainPackValidator.cs:196` ← `DomainPackRegistry.cs:86`), and `kw merge-audit` re-asks the
rule about every merge already on record, so a destructive step stays falsifiable *after* it has
run.

**Where the prototype still violates it.** It does not, in the merge path — the post-2026-07-12
normaliser buckets on canonical names and merges only names derivable from one another, with no
model and no alias in the path. The residual exposure is that the algebra is a *global mutable
registry* (`ConcurrentDictionary` keyed by `object`, populated by a static constructor and
mutated further by `AddDomainPacks` at DI time), so what algebra a relation has depends on
whether pack loading happened. A traversal in a host that never loaded packs consults a
different algebra than one that did, and nothing says so.

**What would encode it here.** The algebra belongs *to* the relation kind, not to a registry
consulted by convention — so a graph algorithm cannot be handed a relation whose algebra it has
not checked. The nine discovery discriminations and the twelve structural families are 21 kinds,
not nine, and the closed-set argument has to be made for all 21.

**Owned by** `KWB-3` (the vocabulary and its algebra) and `KWB-5` (each stage's tests exercise
the algorithmic property that stage depends on).

---

## D19 (a) — Work handed to a queue is not work done

**What established it.** `kw admit` queued every passage of every book as a `PendingJob` of kind
`AdmitChunk`. Nothing consumed that kind. The scheduler's `default:` arm **deleted** the job and
returned normally, so it was marked `Succeeded`; the run reported `chunks admitted 0`,
`claims created 0`, and printed it under the heading `Admitted`. Four green components, one
destroyed corpus, and a screen of zeros indistinguishable from a clean run of an empty book.

**The rules it produced.** Never enqueue for a handler that does not exist — a producer and its
consumer belong in the same commit. An unhandled message is retained, never deleted: "this
process does not know this kind" is a fact about the handler table, not about the work. And the
corollary that made it invisible: **a report may only count what it can see**; a stage that
defers work must say how much it deferred.

**Where the prototype still violates it.** `AdmitChunk` **is still unconsumed.** The remediation
disarmed the path rather than removing it: `AdmissionPipeline.cs:76-81` documents a flag that
"defaults to `true`, and must stay that way while no worker consumes the `AdmitChunk` kind," and
`AdmissionWorker.cs:81,92` carries the same caveat. The producer survives, the consumer was
never written, and what stands between them is a default value in configuration. That is an
uncompleted architectural obligation, not a closed incident.

What *was* built is worth carrying: `PendingJobStatus` now distinguishes `Queued`, `Running`,
`Succeeded`, `Failed` and `DeadLettered`, and its doc comment records a second paid-for lesson —
success used to delete the row, so `GET /jobs/{id}` answered `404` both for a job that had
finished perfectly and for an id that never existed. `DeadLettered` records a third: one job
reached **37 attempts across a day**, re-enqueued on every restart, incrementing a counter
nobody read.

**What would encode it here.** A job kind that cannot be enqueued without a registered handler —
the producer/consumer pairing made a type-level fact rather than a commit-discipline request.
Failing that, a status enum whose zero value is not success and whose unhandled arm retains.

**Owned by** `KWB-2`.

---

## D19-B — A filter that applies itself is a check nobody asked for

**What established it.** Concepts are read through two interfaces, and which one a caller depends
on is the whole design. `IConceptRepository` returns **current** concepts; `IConceptHistory`
returns **every version**, including merge losers. The split exists because the alternative
failed silently in both directions:

- A global query filter rewrites every query, including ones written by someone who has never
  heard of it. `merge-audit` resolves the merge log's keeper and loser ids against the concepts
  it can see; against the current view it resolves **none**, prints "nothing has been merged
  away", and exits `0`. Every merge on record could be wrong and the gate would pass.
- No filter at all fails the other way: new code that forgets `Status != Deprecated` reports
  merge losers as live.

**The rules it produced.** *Say which world you read* — needing retired rows is a dependency on
`IConceptHistory`, visible in a constructor, and neither answer may be arrived at by forgetting.
*Liveness is one expression* — `ConceptLiveness.IsCurrentExpression` is the rule, and the query
filter, the in-memory and JSON repositories, and the partial unique index all apply *that*, not
a copy; three hand-written copies is how the providers came to disagree about which concepts
exist. *An audit needs an independent expectation* — `merge-audit` checks its count against the
`ConceptMerge` log and **throws** if it can account for fewer losers than the log records.
Blindness must be loud; a zero is not a result.

And the two-authority rule underneath: `Status` says *no longer asserted*, `ValidUntil` says
*this version closed at a known instant*, and the database holds the implication between them
(`CK_Concepts_Closed_Implies_Deprecated`, `CK_Concepts_Successor_Implies_Closed`), so a write
path that sets one and forgets the other fails at commit. Setting `SupersededBy` without
`ValidUntil` is what made every `AsOf(t)` return the present, for every `t`, for the life of the
corpus.

**Where the prototype still violates it.** Only in the letter, not the substance.
`EfConceptHistory`'s own docstring says `IgnoreQueryFilters` "appears in this file and nowhere
else in the concept read path," and it appears in a second file,
`Queries/TemporalQueryExtensions.cs:17`. That helper is reachable only from `EfConceptHistory`,
so the invariant holds — but it is stated as a fact about one file and enforced by nothing, which
is the shape of an invariant that drifts. A one-line mechanical check would close it, and none
exists.

**What would encode it here.** Two distinct read types, not one type with a flag — which is the
same discipline `KWB-6` states for mutation and the same one `D-133` states in Nomos for a read
surface that assembles its store and names what is missing. And liveness as one expression with
a test that no second copy exists.

**Owned by** `KWB-6`. It is not currently named there, and the item's `why` is extended to name
it.

---

## D20 — A default is a claim, and an enum's zero value is a default

**What established it.** `PipelineOutcome` was added to distinguish `Clean` / `Partial` /
`Degraded` / `Failed`. Every one of its 19 uses was `Clean`, and `AdmissionReport.Outcome` was a
settable field nothing set — so it read `Clean` on a dead model, because `Clean` was the enum's
zero value. A field that reports success until someone remembers to say otherwise is not an
unfinished feature, it is a false one.

**The rule it produced.** Derive a status from the evidence rather than assigning it: a computed
property cannot fall out of step with the counts it describes, and it leaves no setter for a
caller to forget. If it must be assigned, order the enum so the zero value is the *unknown* or
*worst* case, never the good one.

**Where the prototype still violates it.** In three measured places, and the first is the
largest:

1. **`public double Confidence { get; set; } = 1.0` in 40 `Domain` types** (41 counting the one
   that uses `init`). An unset confidence reads as *certain*. The types affected include every
   one of the twelve identical kernel nodes, `Claim`, `ConceptRelationship`, `NegativeAssertion`,
   `RelationAssertion` and all six reified hyperedges. This is not a historical anecdote in that
   tree — it is an **uncompleted architectural obligation**, and it is also the unfinished half
   of ADR 0001's own *required follow-up*, which named `Confidence` as one of the value objects
   that would close "the principal gap between C# and Rust for this domain." `Confidence` as a
   value object exists in `Domain/Values`; the bare `double` defaulted to `1.0` exists in 40
   types beside it.
2. **`EpistemicReviewState.Asserted = 0`**, explicitly assigned. Every epistemic node and edge
   defaults its review state to the *strongest* value the lifecycle offers, ahead of `Inferred`,
   `Suggested`, `Corroborated`, `Disputed`, `Reviewed` and `Deprecated`. An unset review state
   claims the thing was asserted by a source.
3. **`LlmCallOutcome`'s zero value is `Succeeded`** — D20's exact original shape, in the
   observability type that records whether model calls worked.

`PipelineOutcome` itself was remediated: its zero value is now `Unknown`. `CoverageOutcome`'s
was not; see D17.

**What would encode it here.** The rule generalises past enums: a default is a claim wherever it
appears, so the encoding is a confidence type with no default at all — an unstated confidence is
absent, not `1.0` — and review-state and outcome enums whose least-committal member is the one
a forgotten field produces. Rust has no `default(T)` hole to aim; the mitigation ADR 0001
describes (store the value offset from its minimum so `default` is the weakest claim) is
unnecessary here, and that is a genuine difference rather than a stylistic one.

**Owned by** `KWB-1` (the identity and value types) and `KWB-4` (the coverage type). No item
currently names the 40-type `Confidence` measurement; `KWB-1`'s `why` is extended to carry it.

---

## D21 — A second implementation of the stage is not the stage

**What established it.** The fix for D18 was "test the stage, not only the rule." The test file
written to do that asserted its properties against a bucketer **defined inside the test file**,
and could not have done otherwise: it sat in a project that does not reference the one the
normaliser lives in. It would have stayed green if the production guard were deleted. When the
real test was finally written it **failed immediately**, on a keeper tie-break that fell through
to a randomly minted id.

**The rule it produced.** Before trusting a property test, name the object it calls. If that
object was written in the test, the test constrains the specification and not the system. Put it
where it can call the real thing, and say so in the docstring of whatever is left behind — the
one that reads as coverage is the dangerous one.

**Where the prototype still violates it.** `Enrichment/Discovery` — `ConnectionDiscoveryPipeline`,
`IConnectionDiscoveryPipeline`, `ConnectionCandidate`, `StructuralSignature`, 198 lines — has
**zero production consumers** and exactly one referencing file,
`tests/KnowledgeWorkbench.Enrichment.Tests/Discovery/ConnectionDiscoveryPipelineTests.cs`. The
live connection-discovery implementation is `Domain/ConnectionDiscovery` (23 files) plus
`Enrichment/ConnectionDiscovery` (6 files). This is D21 one turn further out: not a stage
reimplemented inside a test, but a superseded stage kept alive *by* its test, contributing green
checks for code nothing runs. It is the single class-7 entry in the inventory.

**What would encode it here.** A test-layout rule cannot be a convention, because D21 is
precisely the failure of a convention. The mechanical form is a check that every production
module has at least one non-test consumer or is declared as a leaf — which is close enough to
what `tests/contract` already does for the bands table that it is worth asking whether the same
mechanism extends.

**Owned by** nothing yet, and deliberately: it is a repository-hygiene mechanism, not a
knowledge requirement, and `KWB-7`'s territory is the nearest thing on the board. It is recorded
here rather than turned into an item, because inventing a testing-policy item before this
repository has a second crate would reserve ground nobody is standing on.

---

## The reasoned divergence, verdicted

`Domain/Semantics/RelationAssertion` is deliberately **a read-only projection over five typed
relation tables, not a storage model**, and its docstring says why:

> This is deliberately a *view*, not a replacement. Each typed relation continues to enforce its
> own invariants at write time; `RelationAssertion` exists so cross-cutting analysis (inference,
> ranking, MCP surface, graph exports) can operate over one shape instead of five, without
> collapsing them into a single monolithic table on disk.

**Verdict: met, and it is the standard for how a departure is recorded here.** It names the
alternative it rejected (one table), the cost it refused to pay (losing per-family write-time
invariants), and the benefit it bought (one shape for five readers). A later reader can disagree
with it on its own terms, which is what makes it a decision rather than a preference.

Two qualifications a Rust encoding has to carry, because the prototype's own version does not:

- The projection is read-only **by intent and by naming**, not by construction: it is a `record`
  with `init` accessors and no write path, which is a convention a future method could break.
  `KWB-6`'s requirement that a query type have no write path is exactly this made structural.
- It carries `Confidence = 1.0` and `Status = Asserted` as defaults — so the projection restates
  D20's defect for relations that had it, and *introduces* it for any that did not.

---

## What this register does not cover

The prototype's `docs/FEATURE_MATRIX.md` records further failures that never became numbered
incidents but were measured the same way: hand-written SQL wrong three ways (2026-08-01); two
safety properties written but never asserted; the flight recorder wired to one provider only, so
every published measurement against a local model — the ~41% qwen fold error included — had no
artefact behind it; a merge-safety claim that was "true of the design and false of the code, and
stayed that way for two months while the checkbox said otherwise." That last one is the register's
own thesis restated: a safety property is a claim, and a claim needs evidence, exactly like a
checkbox does.

Those are inventoried as corpus in `prototype-inventory.md` rather than answered here, because
answering them four ways would assert more structure than the prototype gave them.
