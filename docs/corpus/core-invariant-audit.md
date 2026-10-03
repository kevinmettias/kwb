# The core invariants, measured against the code that is meant to hold them

The owner asked on 2026-10-03 that everything core to this repository be tested until it cannot
lose content, admit an invented claim, or decompose a source wrongly without a test going red.
This file is the audit that request starts from: each invariant the records and the code state,
the tests that hold it, and what the code actually does where no test looks. It is a corpus
artifact under `ARC-ECOSYSTEM-002`, in the manner of `prototype-incidents.md`: evidence, and it
decides nothing. The items it names decide or do the work.

Measured 2026-10-03 at `17f82cf`, with the XVPE pin at `c700bcf83` resolved from a warm cargo git
cache. Every source and test file in the twelve crates was read. `cargo test --workspace` passed
453 tests and `cargo clippy --workspace --all-targets -- -D warnings` exited 0, so every predicate
below is green at that commit. Behaviour was measured with the debug `kwb` and `kwb-mcp` this
workspace builds, run against scratch stores outside the repository; nothing ran against a real
library or a model. Where a finding rests on reading alone it says so.

## How the identities were pinned

The derivation layout `crates/kernel/kwb-model/src/derivation.rs` documents was reproduced
outside this workspace with an independent SHA-256: each part followed by `0x1F`, beginning
`kind`, `<kind>`, then per field its name and either the normalized text, or `<identity>` and
the 32 raw bytes of the identity, or `<opaque>` and the SHA-256 of the bytes. Nineteen inputs —
concepts in both cases, padded, precomposed and decomposed, CJK, emoji and empty; claims with
reflowed whitespace and empty text; an assertion scoped and unstated; documents of zero bytes,
`0x00`, `0x1F`, invalid UTF-8 and text — were derived both that way and through
`Concept::Named`, `Claim::About`, `Assertion::By` and `Document::Of`. All nineteen agreed. They
are the literals `KWB-150` pins.

One of them is a finding in itself: `"\u{e9}nergie"` and `"e\u{301}nergie"`, which render the
same, derive `ef53c555…2dde` and `d54b4d70…25f3` — two concepts. Nothing decides whether that is
wanted (`KWB-162`).

## Findings, measured

Each is numbered as the audit numbered it, so the items can cite one line.

| # | What happens | How it was established | Owned by |
|---|---|---|---|
| F1 | A later admission naming a superseded concept re-asserts it: after `supersede phlogiston --into oxidation`, admitting a file that `--says phlogiston …` empties `merge_losers` | measured | `KWB-159` |
| F2 | `--because` is recorded raw: a reason holding `U+001F` or a line feed exits 0 and every later `admit` and `history` on that store exits 1, "the publication log cannot be replayed"; a reason of one `U+0001` is accepted as evidence | measured | `KWB-144` |
| F3 | `--says $'\x01' $'\x07'` publishes a concept with an empty name (`33a3fade…46fe`, the identity of the empty string) and a claim with empty text, and reports `yielded` | measured | `KWB-147` |
| F4 | A text source is one passage whatever its size: the file is chunked as one page, and the chunker never splits a page, so the 4,000-character budget is never applied and a book is one request with a ceiling of 32 propositions | read (`reads_text.rs` `Page_Of`, XVPE `page_run.rs`) | input to `KWB-125` |
| F5 | Every passage's location renders `characters 0 to 0 of the text`: `Where_In` formats a page span as a character range | read | `KWB-156` |
| F6 | The 32-proposition ceiling is enforced only by the schema, which a replayed answer never meets; `Proposed_From` takes any count | read; the file's own doc concedes replay bypasses the schema | `KWB-155` |
| F7 | No proposition is checked against its passage; anything the model names is admitted and cited to the document | read | input to `KWB-128` (C-16) |
| F8 | A reader returning no readings is recorded `Barren` with one unit examined, whatever was examined; the reading list does not say how much was read | read | input to `KWB-127` |
| F9 | An answer whose every proposition is blank is recorded `Barren` | read | input to `KWB-127` |
| F10 | Closures are published without a time; `history --as-of` at an admission's second, taken before a supersession two seconds later, already reports the concept closed | measured | `KWB-145` |
| F11 | `history`, `retire`, `supersede` and `kwb-mcp` create a store at any path they are given: a mistyped path reports `through 0 of 0` or `(nothing)` and exits 0, and `retire x` on it records a closure of a concept nobody published | measured | `KWB-148` |
| F12 | `kwb-mcp --serve` with no store serves an empty corpus to an agent with no warning | read | input to `KWB-126` |
| F13 | `std::env::args()` panics on an argument that is not Unicode and `println!` panics on a closed standard output, so a refusal can arrive as exit 101 | read; both are the standard library's documented behaviour | `KWB-149` |
| F14 | `supersede` checks only self-supersession and that the successor is published: a cycle (A into B, then B into A) is accepted, and `history` then reports 0 concepts and 0 claims current | measured | `KWB-159` |
| F15 | A truncated or foreign file at a content address is answered `AlreadyPresent` forever; nothing compares a stored file's bytes with its address | read | `KWB-160` |
| F16 | A log's unterminated last line is returned as a record, and the next append continues it | read; KWB-116's own why | `KWB-116`, then `KWB-151` |
| F19 | Two identical `--says` in one run append the claim and assertion records twice | measured (five records for one claim) | unowned, low: replay folds the duplicates into one claim |
| F20 | `D-015` says the count of assertions on a claim is the count of sources asserting it; scope is in an assertion's identity, so one source admitted with and without `--scope` prints `citations 2` | measured | input to `KWB-128` (C-18) |
| F21 | `kwb-mcp neighbours "oxidation "` answers `(nothing)` where `held_neighbours` with the same argument finds the concept | measured | `KWB-154` |
| F24 | Tests make scratch directories under fixed names, so two concurrent `cargo test` runs collide | read (17 sites in 13 files) | `KWB-158` |
| F26 | Replay is quadratic: 750 records 4.7 ms, 1,500 15.5 ms, 3,000 121.7 ms, 6,000 464.9 ms, 12,000 2,138.6 ms in a release build; `D-014` says it is linear | measured | `KWB-153` |
| F27 | Replay re-derives every record from its text and checks it against nothing, so a one-byte corruption inside a name replays as a different concept | read | `KWB-161` |
| F28 | A run's records are appended one at a time with no boundary, so a run stopped between records leaves claims with no assertion | read | `KWB-161` |

## What the tests hold, and what they do not

- **Identity.** Exactly one identity is pinned to a literal (`kwb-cli/tests/history.rs:234`).
  Every other identity test compares two derivations through `kwb_model`, so a change to the
  separator, a tag, the kind header, a field name or the order of fields moves both sides and
  stays green. `KWB-150` pins the nineteen vectors above.
- **Replay.** `kwb-domain/tests/publication.rs:172` compares concept counts and current
  identities; closed versions, reasons, successors and times are never compared. `KWB-152`.
- **Truncation.** No test cuts a log at more than one offset. `KWB-151`, after `KWB-116`.
- **Closing.** No test runs `retire` or `supersede` to success. `KWB-146`.
- **The store's one door.** The guard counts `pub fn …(&mut self`, and the medium is written
  from `&self`. `KWB-157`.
- **Extraction.** Every test uses one 48-character passage. `KWB-155` replays adversarial
  answers; `F4` and `F7` wait on the decisions that own them.

## What this audit does not cover

It did not run a model, read a real library, or exercise two processes on one store — `KWB-143`
decides what two writers may do, and a test of it before that decision would test a guess. It
read the XVPE crates this workspace adopts only where a finding needed them.
