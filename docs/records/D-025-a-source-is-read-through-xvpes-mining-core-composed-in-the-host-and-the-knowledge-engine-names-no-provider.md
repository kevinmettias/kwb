---
id: D-025
type: decision
title: A source is read through XVPE's mining core composed in the host, the knowledge engine names no provider, and only fidelity produces Skipped
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - ingestion
  - reading
  - xvpe
  - coverage
relations:
  - target: D-024
    type: relates-to
  - target: D-004
    type: relates-to
---

# A source is read through XVPE's mining core composed in the host, the knowledge engine names no provider, and only fidelity produces Skipped

## Decision

This record verdicts rows C-01 to C-12 of `docs/corpus/prototype-capabilities.md` for goal G2 of
`D-021`.

**Where reading happens.** Model-backed reading of a PDF, a folder or a web page runs through
XVPE's mining core — its pipeline driver, its corpus fronts, its inference backends and its record
and replay — composed in this repository's host band (`kwb-cli`) behind a cargo feature that is not
a default. KWB supplies the two ends only XVPE cannot: what a passage is asked and how its answer
becomes a proposition (the extraction contract `kwb-ingest` declares), and where the readings go
(admission into the graph, which is XVPE's mining-core step K4). The knowledge-engine crates —
`kwb-model`, `kwb-store`, `kwb-domain`, `kwb-ingest`, `kwb-extract`, `kwb-retrieval` — name no
provider, read no credential and open no socket, and every test in the workspace stays offline,
replaying recorded answers. `README.md`'s statement that no provider is in this workspace is amended
by the item that composes it, to say exactly that: a provider is in the host, behind a feature,
and nowhere else.

**When.** The adoption is XVPE's step K3, which XVPE's own plan holds as not yet authorable: the
surfaces it would adopt are being hardened and renamed this week, and an adoption is measured at one
published revision (`D-007`). So every verdict below that needs the core is **met, deferred to K3**,
and the items that implement them are authored when K3 is, against the pin that brings the core in.
Building a reader pipeline here meanwhile would be the defect XVPE names *wired twice*.

**What KWB decides now, because no adoption changes it.**

- **The unit of admission is a document: the bytes of one file, or of one fetched snapshot,
  content-addressed.** A folder is many documents, one admission each, in one run. A web page is the
  snapshot's bytes; its URL, like a file's path or a catalogue key, is evidence about where the bytes
  came from and never part of an identity (`D-002`).
- **An unreadable passage is Skipped, and an unreadable document is Unmet.** A passage whose
  fidelity needs a page image — sparse or image-only — is not sent to a text protocol; it is recorded
  as `Skipped` with the reason that it needs a page image, and counted, so a partly scanned book
  reports how much of it was not read. A document none of whose passages is readable as text is
  `Unmet`, naming that, and never `Barren`: nothing was examined. Both are coverage records
  (`D-024`).
- **Only fidelity produces Skipped.** Classifying passages as code, contents or front matter, so as
  to skip them, decides what is worth reading — admission input `D-004` holds — and the prototype's
  classifier wrote off a theorem's own proof as a fragment, by 0.061 of a letter-fraction threshold,
  in a run that reported clean. Until that hold lifts, every passage readable as text is read.
- **A reading protocol's version is derived from what the reader sends.** The instructions, the
  answer schema and the passage and answer budgets, hashed through this repository's derivation, so
  a prompt edit is a new protocol by construction rather than by someone remembering to bump a string.
  The model is not folded in: it is the reader, recorded beside the protocol on every coverage record
  (`D-024`), so a change of model and a change of prompt stay distinguishable.
  Coverage recorded under one protocol is stale under another (`D-024`'s coverage audit) and the
  document is read again. The prototype hashed the schema alone for two of its stages, and an
  instruction edit changed nothing they recorded.
- **Text quality is reported before money is spent, and gates nothing.** The core's per-passage
  fidelity and, once XVPE step X1 measures it, the glued-word rate, are stated before the first
  request. A gate on an unmeasured quality threshold would write off content on a number.

### The verdicts

| Row | Verdict |
|---|---|
| C-01 | **met** in the form above; the composition is deferred to K3. No source registry: a source is its document's identity, and paths, URLs and catalogue keys are evidence. |
| C-02 | **met, deferred to K3** for PDF text through XVPE's PDFium reader, and to XVPE step X1 for word reconstruction, which first measures whether PDFium glues words at all — the 4.21% the prototype measured was another library's. |
| C-03 | **met.** UTF-8 text is read today through `kwb-extract`; at K3 the core's front reads it. |
| C-04 | **met, deferred** — folders to K3 (XVPE's file library), web pages to XVPE step X3. The snapshot is the bytes read, kept as the document; the prototype stored stripped text, refetched on every ingest, and never read its snapshot again. |
| C-05 | **met** in the form above. Surfacing an unreadable source in every query reply is the query surface's, `KWB-126`. |
| C-06 | **diverges.** Passages are the core's page runs, split below the page by XVPE step X2 so that no passage exceeds its budget and every character is in exactly one passage. A passage's location is its page span and character range, true (`KWB-156`, `D-015`). No chunk entity and no chunk identity are stored: a passage is re-derived from its document and its protocol. The budget is a stated working value until a measurement on the real corpus, which the owner holds. |
| C-07 | **deferred** under `D-004`'s admission hold, for the reason above. |
| C-08 | **met** in the form above: reported, never a gate. |
| C-09 | **deferred to G8.** Seeding sources from the reference library's catalogue records the real library, which the owner holds (`D-021`). XVPE step X4 stays `needed` until then. |
| C-10 | **met** in the form above, **deferred to K3.** One stage, extraction, so per-stage routing has nothing to route yet; prompt caching is the backends'. |
| C-11 | **met**: record and replay are XVPE's, a miss is a failure, and the host gains `--record` and `--replay` with K3. |
| C-12 | **met** in the form above. `KWB-172` derives the version for the reader this repository has today. |

## Why

**One reading pipeline, not two.** The owner set the direction on 2026-09-24: the two miners and
KWB are one architecture with different front and back ports into one core. XVPE has since built that
core — the driver, the corpus fronts, the extraction core schema, grounding, coverage, record and
replay — and is hardening it. A reader pipeline written here would duplicate all of it, and would
have to be torn out at K3.

**The knowledge engine stays provider-free, and that is cheap to keep.** The extraction contract
already says a model-backed implementation belongs outside the knowledge engine. A host-band feature
satisfies it without a second repository, and keeps every engine crate's closure and every test free
of credentials and network.

**Each decision now is one the prototype paid for.** Whole-source readability averaged a folder's
scans with its text files; a classifier's write-off was final and self-confirming; a schema-only
version hid instruction edits; a quality score gated nothing and was not read before paying.

## Alternatives Considered

**A provider adapter in another repository.** Refused: it adds a repository to keep in step for a
separation the host band already gives.

**Building the reader pipeline here now, and adopting XVPE's later.** Refused: wired twice, and the
second wiring is the expensive one.

**A passage classifier now, recording non-prose as Skipped rather than Barren.** Refused while
`D-004` holds admission: Skipped would keep the write-off reversible, but deciding what is worth
reading is exactly the input that hold waits on.

## Consequences

- `KWB-172`: the reading protocol's version derived from what the reader sends.
- XVPE steps X1 (word reconstruction, measured first), X2 (splitting below the page) and X3 (a web
  page as a snapshot) are specified in XVPE's `docs/plans/programs/ecosystem-substrate/kwb-parity.md`.
- The G2 proof — a PDF, a folder and a text file admitted end to end through replayed answers,
  offline — is authored with K3, as are the composition items, because every one of them names a
  surface K3 adopts.
- `D-024`'s coverage record carries the Skipped and Unmet cases above unchanged.
