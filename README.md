# KnowledgeWorkbench

KnowledgeWorkbench (KWB) admits references and builds a durable graph of the concepts and
claims a reader finds in them — deduplicated across sources by content-derived identity, not
by an authored key, so two books asserting the same thing become one claim with two citations.
It is one of four products sharing one ecosystem seam; `ARC-ECOSYSTEM-001` in `f:/repos/nomos`
states what each owns. KWB owns knowledge: rationale, semantic intent, requirements
elicitation, decision context, long-term epistemic memory, and rich authored knowledge models.

**What that sentence does and does not say, because this file is what `AGENTS.md` sends a
session to for what exists.** A source is admitted, stored by its content address and kept; the
graph is built, published as an append-only record and rebuilt by replaying it. What a source
*asserts* is supplied by a **reader**, and the only reader this repository has is a person
typing `--says`. Nothing here opens a document and decides what it says. `D-015` and the
extraction contract in `kwb-ingest` are where that changes, and until a reader lands, *the
claims a reference teaches* means *the claims somebody said it teaches*.

**Intended and not built: the code that implements a concept.** The product this is a rewrite
of was meant to link knowledge to the code that realises it, and that intent stands. This
repository has no entity for it — the domain is `Concept`, `Claim` and `Assertion`, and no
record has decided what a code entity would be or whether KWB owns one. Said here rather than
dropped, so that a reader can tell it was meant rather than forgotten.

This repository is a ground-up Rust rewrite. `D-001` records why: the prior
implementation, in C# at `C:/Users/kmett/source/repos/KnowledgeWorkbench`, is prototype
material from this point forward — read for its domain model and its recorded
data-loss incidents, verdicted requirement by requirement as this repository is built,
never ported wholesale. That relationship mirrors the one `f:/repos/nomos` has to its own
prototype, `code-standards`.

## Trying it

Two books asserting the same thing become **one claim with two citations**. That is the
mechanism every identity decision here was made to support, and it is the shortest way to see
whether this repository does what the paragraph above says.

```console
$ kwb admit callen.txt --store ./corpus --scope "physical theory" --says entropy "It is non-decreasing in an isolated system."
source     0cc40cb7fc596e1b5ab40585ef8369a3bfa79eff11096ba46400b5d61df52a2d
coverage   yielded
concepts   1
claims     1
citations  1
refused    0
documents  kept
knowledge  kept

$ kwb admit kittel.txt --store ./corpus --scope "physical theory" --says entropy "It is non-decreasing in an isolated system."
source     cfcb53adb1aec951a6ecefc8a9087dc78f35bb19201d902fab7f4448602ff2aa
concepts   1
claims     1
citations  2
```

A different book, a different source address, and still **one** claim — because a claim's
identity excludes the source it was read from (`D-002`). The second run replays what the first
published before it admits, so the count is the corpus rather than the run.

```console
$ kwb-mcp ./corpus neighbours entropy
concept  entropy
claim    It is non-decreasing in an isolated system.
cited    0cc40cb7fc596e1b5ab40585ef8369a3bfa79eff11096ba46400b5d61df52a2d [physical theory]
cited    cfcb53adb1aec951a6ecefc8a9087dc78f35bb19201d902fab7f4448602ff2aa [physical theory]
```

Each citation is the **content address of the document the claim was read out of**, so following
one returns those exact bytes or fails loudly because they are gone.

`kwb retire` and `kwb supersede` close a concept, and both require `--because`: `D17` is that
destruction requires evidence, and a merge with no recorded reason cannot answer the question it
demands. `kwb-mcp <store>` with no tool lists the four it answers.

**There is no extractor.** Nothing here reads a document and decides what it says — that is what
`--says` is for, and it is the interesting half. `kwb help` has the flags; they are not repeated
here, because a copy of a help text is a copy that drifts.

## Bands

The authoritative statement of what each crate owns. `tests/contract` asserts this table
against the real workspace, both directions.

| Band | Crate | Owns |
|---|---|---|
| 0 | `kwb-contracts` | Protocol vocabulary crossing a product boundary. Depends on `serde` and nothing else. |
| 1 | `kwb-model` | Canonical, content-derived identity. |
| 1 | `kwb-store` | The content-addressed document store; one write door. |
| 1p | `kwb-platform` | The seam an implementation is chosen behind: one port per thing the outside world does for this repository. |
| 1p | `kwb-platform-std` | The standard-library implementation of those traits. |
| 1p | `kwb-platform-xvpe` | The one crate permitted to name XVPE. Adopts the persistent map versioned state is built on, by git reference and commit SHA. `D-007`, `D-012`. |
| 2 | `kwb-domain` | Claims, concepts, argumentation, evidence, coverage, the derivation ledger, and the universal type kernel. |
| 3 | `kwb-ingest` | The admission pipeline: link-concepts, normalize-concepts, admit. |
| 3 | `kwb-retrieval` | Answering questions about the graph, and never changing it. |
| 10 | `kwb-cli` | The `kwb` composition root. |
| 10 | `kwb-mcp` | The MCP host: the read-only tool surface an agent speaks to. |

## Working this repository

The operating contract is `AGENTS.md`. It is not repeated here.

## Ecosystem

- `f:/repos/nomos` — Nomos, the software-engineering authority this product does not
  duplicate.
- `f:/repos/xvpe` — XVPE, the shared application platform. No crate here depends on it
  yet, and `D-007` states the reason and what was measured. In short: not because XVPE is
  broken — the crates this repository would want compile clean and pull no third-party
  package between them — but because a `path` edge would make this repository's
  reproducibility a function of another repository's working tree.
