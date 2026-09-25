<!--
  Visitor landing page. GitHub shows .github/README.md in place of the root README.md, so this is
  what someone arriving at github.com/kevinmettias/kwb reads first. The root README.md is the
  workspace's own authority on what exists and what owns what, and tests/contract checks its
  generated tables; this page says what the project is for and routes there. It restates none of
  those tables, and every count below carries the month it was measured.
-->

# KWB: KnowledgeWorkbench

**A knowledge graph that remembers where every claim came from.** KWB admits reference material
(books, papers, notes) and builds a durable graph of the concepts and claims found in it. When
two sources assert the same thing, KWB stores one claim with two citations, and each citation is
the content address of the exact document it was read from.

Rust · 14 crates · ~25,600 lines · ~440 tests · 18 decision records
<sub>(measured September 2026)</sub>

---

## The problem

Most knowledge tools store *notes*: text somebody wrote, keyed by a title somebody chose. That
breaks down in three ways:

- **Duplicates.** The same idea read in two places becomes two notes that drift apart.
- **Lost provenance.** "Where did this come from?" gets answered from memory, if at all.
- **No history.** When an idea is revised, merged or retired, the record of what it used to
  say is usually overwritten.

KWB treats knowledge the way a version-control system treats code. A claim's identity is
*derived from its content*, not assigned by hand, so the same proposition from two sources
converges on one claim. Sources are stored by content address, so following a citation returns
those exact bytes or fails loudly because they are gone. The graph is an append-only log of
publications, so any past state can be replayed and audited.

The long-term goal is an epistemic memory for rationale, intent and decisions: what somebody
meant, claimed or decided, and why. That memory should outlive the code or project it was
about.

## What works today

```console
$ kwb admit callen.txt --store ./corpus --scope "physical theory" --says entropy "It is non-decreasing in an isolated system."
$ kwb admit kittel.txt --store ./corpus --scope "physical theory" --says entropy "It is non-decreasing in an isolated system."
$ kwb-mcp ./corpus neighbours entropy
concept  entropy
claim    It is non-decreasing in an isolated system.
cited    0cc40cb7fc596e1b5ab40585ef8369a3bfa79eff11096ba46400b5d61df52a2d [physical theory]
cited    cfcb53adb1aec951a6ecefc8a9087dc78f35bb19201d902fab7f4448602ff2aa [physical theory]
```

Two books and two source addresses, but **one claim with two citations**.

- **Admission pipeline.** Concepts are linked and normalized, then admitted through a single
  write door, so every path into the graph obeys the same identity rules.
- **Destruction requires evidence.** Retiring or superseding a concept requires a stated reason,
  and a claim under a superseded concept reports itself as no longer current.
- **Time travel.** `kwb history` replays the publication log and shows the graph as it stood
  after a given number of publications, or at a given time.
- **A read-only tool surface for AI agents.** Five tools: search, get a concept, show a concept's
  neighbours, and two audit tools that list what was merged away and what it used to hold. Each
  tool declares whether it reads the *current* or the *historical* graph, so an agent cannot ask
  a historical question of the current state by mistake. The tools run from the command line
  today. Serving them over the MCP protocol is the next item on the board, reusing the transport
  XVPE already ships.
- **A model-backed reader, without a vendor lock.** `kwb-extract` splits a source into passages,
  asks a language model what each passage asserts under a schema, and feeds the proposals through
  the same identity pipeline as a hand-typed claim. The repository deliberately contains no model
  provider or credential. Every test replays recorded answers, so the suite is offline, free and
  exactly reproducible.

## How it's built

- **A ground-up Rust rewrite of a C# prototype.** The prototype is treated as evidence, not as
  code to port. Its domain model is read, each of its requirements is judged (met, diverges,
  deferred or declined) as the rewrite proceeds, and its five recorded data-loss incidents name
  the failure classes the new types are designed to make unrepresentable.
  ([`D-001`](/docs/records/D-001-this-repository-is-a-rust-rewrite-and-the-net-repository-is-prototype-material.md))
- **A two-phase AI development loop.** A design-and-audit model writes decision records and
  bounded work items: what is required, how completion is verified, and which interpretations
  are forbidden. A cheaper implementation model claims those items and implements them. The
  design model then audits the result against the item's meaning, not just against green tests.
  A hook enforces the split, so the design phase cannot write implementation files. Nearly every
  commit is AI co-authored. My part is the domain, the decisions, the process, and the audit.
- **Every decision is a record** in [`docs/records/`](/docs/records/), and the README's tables
  are generated from the code and checked against it, so the documentation cannot drift silently.
- **Reproducibility is pinned.** The toolchain is pinned to the measured minimum Rust version, and
  the one shared-platform dependency is pinned by commit SHA and quarantined behind a single
  crate.

## Part of a larger system

KWB is one of four products that share one boundary, and ownership is decided by what a
responsibility *means*:

| Project | Owns |
|---|---|
| [**Nomos**](https://github.com/kevinmettias/nomos) | Software engineering: analysis, architecture, rules, gates. KWB coordinates its own work through the same work ledger Nomos uses. |
| **KWB** (this repository) | Knowledge: rationale, intent, claims, and where each claim came from. |
| [**XVPE**](https://github.com/kevinmettias/xvpe) | Generic runtime and platform infrastructure. KWB adopts its persistent map, clock, passage splitter and inference surface through one quarantined crate. |
| Repository tooling | The machinery that exists only to build these repositories. |

## Try it

Requires Rust 1.87 (pinned in [`rust-toolchain.toml`](/rust-toolchain.toml)).

```sh
cargo build --release
./target/release/kwb help
./target/release/kwb-mcp ./corpus    # lists the five tools
```

## Reading further

| If you want | Read |
|---|---|
| What exists, and which crate owns what | [README.md](/README.md), the engineering README |
| Why something was decided | [docs/records/](/docs/records/) |
| How work on this repository is run | [AGENTS.md](/AGENTS.md) |
| What must pass before a change lands | [.github/workflows/gate.yml](/.github/workflows/gate.yml) |
