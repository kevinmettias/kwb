---
id: D-019
type: decision
title: The document store and the publication log are kept through XVPE's store and log, behind kwb-platform-std's own types, and a torn tail is a refusal
status: accepted
version: 3
authority: canonical-normative-record
tags:
  - storage
  - platform
  - xvpe
  - ownership
relations:
  - target: D-014
    type: relates-to
---

# The document store and the publication log are kept through XVPE's store and log, behind kwb-platform-std's own types, and a torn tail is a refusal

## Decision

`D-014` decided the two halves -- a content-addressed file store for documents, an append-only
record of publications for the graph -- and `kwb-platform-std` built both over `std::fs`
(`KWB-30`, `KWB-33`). XVPE has since generalized both from these donors into mechanisms of its own,
`xvpe-content-store` and `xvpe-record-log`, over an injected file-system port. `D-135` places code
with no knowledge semantics in XVPE, and neither a directory of hash-named files nor a file of lines
mentions a claim, a concept or a source. So:

- **`kwb-platform-std`'s `FileRecordLog` and `DirectoryContentStore` keep their names, their
  constructors and the `kwb-platform` ports they implement, and keep their files through XVPE's.**
  An adapter, not a replacement: `kwb-platform`'s traits and `StorageError` do not change, and no
  caller above `kwb-platform-std` learns that anything moved.
- **XVPE is reached only through `kwb-platform-xvpe`'s re-exports**, as every adoption is
  (`D-007`'s quarantine, pinned by `Test_Only_The_Quarantine_Crate_Should_Name_Xvpe`), at one pinned
  rev. `kwb-platform-xvpe` adds no behaviour; the adapters live in `kwb-platform-std`.
- **The log is adopted first, at `c700bcf83`** (`KWB-113`'s rev), because nothing in XVPE blocks it.
- **The store follows, at the first XVPE revision in which "held" means a file is there** -- see
  below -- and not before, with its own pin move measured alone.

## The measurement, 2026-09-25

Read clause by clause: `kwb-platform-std/src/file_record_log.rs` and `directory_content_store.rs`
against `xvpe-record-log` at `c700bcf83` and `xvpe-content-store` at XVPE's working revision.

| KWB's clause | XVPE's | What follows |
|---|---|---|
| A put is atomic (temporary, then rename) | Kept, and stronger: a unique exclusive temporary, synchronized before the rename | nothing to adapt |
| A second put of a held address writes nothing | Kept | nothing to adapt |
| **Held means a regular file is there** (`is_file`), so a directory at an address is not held and a put onto it fails | **Broken at XVPE's working revision:** any existing path counted, so a directory read as held and a put onto it reported success having written nothing -- `one_door.rs`'s report-must-not-outrun-the-work test would fail under it | **Fixed in XVPE on 2026-09-25**, where `D-135` says a gap in a shared mechanism belongs: `DirectoryContentStore` now asks the port what stands at the path, refuses a put onto a directory, and refuses rather than guesses on a medium that cannot tell. The store's adoption waits for a pinned revision carrying it |
| A failed put keeps `<address>.staged` as evidence (the prototype's `D17`, `directory_content_store.rs`'s own note) | The temporary is removed on failure | **Decided below** |
| An address is any string | An address is a `ContentIdentity` | **Decided below** |
| `At` creates the log's directory and file | The file is created on first append | the adapter keeps `At`'s behaviour, through the port |
| A record contains no newline, *guaranteed by the domain* | **Enforced**: an empty record or a line break is refused | stronger; mapped to `Refused` |
| **A last line with no terminator is returned as a record** | Reported as a torn tail, with the complete records and the fragment | **Decided below** |
| A missing log after `At` is a refusal | A missing log is empty | reachable only by deleting the file behind a running process; accepted |
| One writer, one process (`D-014`) | One write per append, unique temporaries | stronger; unchanged assumption |

**Bytes already on disk are compatible in both halves.** Documents are `<root>/<64 lowercase hex>`
on both sides, holding the raw bytes, and XVPE pins that layout with a test of its own. A log line
is `record\n` on both sides, both skip a blank line, and both remove a carriage return before the
terminator. A store and a log written by today's implementation read back unchanged.

## What this decides that the adoption does not settle by itself

- **A torn tail is a refusal.** Today's log returns an unterminated last line as a record. That is
  not harmless: a fragment cut inside a trailing field still parses -- as a shorter concept name, or
  a different time -- so a crash mid-append publishes something nobody published. The adapter maps
  XVPE's torn tail to `StorageError::Refused`, naming how many complete records precede it. Every
  append path in this repository reads the log first, so a torn log stops the next append rather
  than extending the fragment.
- **Nothing closes a torn tail.** XVPE's later revisions offer to terminate a fragment and keep it
  as a record. Doing so would publish the fragment on the next replay. Repairing a torn log is a
  decision about what a crash means for the graph, which no item here makes; until one does, the
  refusal stands and the fragment stays where a person can see it.
- **The evidence of a failed put is the refusal, not a kept temporary.** The prototype kept
  `<address>.staged` so a person could inspect what failed. The refusal already reaches the exit
  code (`D19`: a report must not outrun the work), a kept temporary under a fixed name is the race
  XVPE's unique temporaries remove, and nothing ever read one. This supersedes the kept-`.staged`
  note in `directory_content_store.rs`. A `.staged` file an earlier run left stays where it is; the
  store never lists its directory, so it is inert.
- **An address is a rendered content identity** -- which is what `kwb-model` mints (`D-018`), so no
  caller changes. An address that does not parse as one is a defect upstream, and the adapter
  refuses it (`Refused`) rather than calling it absent: answering "not held" would hide the caller's
  bug behind a true-sounding answer.

## Alternatives Considered

**Keep the `std::fs` implementations.** Refused: they are the donors of XVPE's two mechanisms, so
keeping them is keeping a second copy of each, and the second copy is the one that stays behind when
the first is corrected -- as the atomic-put race already shows.

**Work around the held defect in the adapter**, asking the disk whether the path is a file.
Refused: that is a second route to the disk beside the port, and it would leave XVPE's store wrong
for every other product. The gap was XVPE's to close, and it is closed.

**Adopt both halves at once.** Refused: the store needs a pin move and the log does not, and
`D-007`'s amendment measures a pin move alone, so its effect can be told from the adoption's.

**Adopt `xvpe-event-journal` for the publications**, the route `D-014` left open. Not needed: the
record log keeps an opaque line and leaves the record's shape to `kwb-domain`'s `publication.rs`,
so the question `D-014` deferred -- whether a mining run's record shape fits a publication -- does
not arise. The journal's refusal (`D-014`'s amendments) stands and is untouched here.

## Consequences

- `KWB-116` adopts the log at `c700bcf83`. The store's adoption, and the pin move it needs, are
  authored once the XVPE revision carrying the held fix is published and its closure measured.
- `kwb-platform-std` depends on `kwb-platform-xvpe`: a same-band edge, which the band rules allow.
- A process that crashes mid-append now stops the next run at the torn line instead of publishing
  the fragment. That is the intended behaviour, and the refusal names what it found.

## Amendment: The Store Waits For A Port That Can Make A Directory, 2026-09-26

Measured at XVPE `d6f075c14`, the first published revision carrying the held fix (`9e8e90d1a`), on
an exported copy of this repository with every pin moved there:

- **The held clause holds.** `DirectoryContentStore` asks the port what stands at an address,
  counts only a regular file as held, and refuses a put onto a directory before writing anything.
- **The store would no longer write to standard error.** Until 2026-09-26 XVPE's system backend
  reported a temporary it could not remove on stderr. That is output from a library, and
  `standards.json`'s `telemetry.none` allows output only from this repository's hosts. XVPE now
  carries the failure inside the error the caller already receives, and `d6f075c14` has it.
- **A clause the table above did not list does not hold.** `Under` creates the store's directory,
  and `Test_Under_Should_Create_The_Directory_It_Is_Given` pins it: a store's root is known to be
  usable when the store is opened, not at its first write. XVPE's file-system port had no way to
  create a directory with nothing in it -- `Replace_Atomically` and `Append` create only the
  directory a file they write sits in -- so the adapter could have kept that promise only through
  `std::fs`, beside the port. That is the second route to the disk this record refused for the held
  check, and it is refused here for the same reason. XVPE closed the gap the same day
  (`FileSystemStrategy::Create_Directory`); `d6f075c14` predates it.
- **Closure.** With `KWB-114`'s and `KWB-116`'s adoptions in place, adopting `xvpe-content-store` at
  `d6f075c14` adds exactly that one package.

So the store half moves once more. It follows at the first published XVPE revision carrying
`Create_Directory`, and that revision's pin move is measured alone, as this record already requires.
`Under` keeps its promise, through the port. `KWB-117` moves the pin to `d6f075c14` in the meantime,
because that revision measures clean and carries what `D-020` adopts; the store adapter is authored
once the later revision is published.

## Amendment: The Store's Revision Is f45a7b085, And Its Pin Move Renames Three Surfaces, 2026-09-27

`f45a7b085` is the first published XVPE revision carrying `FileSystemStrategy::Create_Directory`. It
was measured on copies of this repository at `b8729cd`, exported with `git archive`, with the state
`KWB-112` through `KWB-118` leave simulated in both: every pin at `d6f075c14`, the crates those items
adopt declared (`xvpe-remote-call`, `xvpe-remote-call-backend-json`, `xvpe-content-identity`,
`xvpe-record-log`, `xvpe-file-system`, `xvpe-file-system-backend-system`, `xvpe-evidence`), and
`KWB-117`'s one test line made. One copy was then moved to `f45a7b085`. Toolchain 1.87.0 throughout.

- **The port can make a directory.** `Create_Directory` creates a directory and every missing one
  above it, and a directory already there is not an error. `Under` keeps its promise through it.
- **Closure.** Adopting `xvpe-content-store` at `f45a7b085` adds exactly that package. Without it,
  the move changes no package by name and version: 68 at either revision.
- **Three surfaces this repository names were renamed, and nothing else it names changed.** XVPE's
  own standards pass made them, and each is a name, not a behaviour:
  - `xvpe_ai_inference::strategies` is private. `ReplayInference` and `ReplayRecording` are
    re-exported at the crate root, which is where `kwb-platform-xvpe` must name them.
  - `InferenceRequest::New` takes a `RequestTerms` -- the model, role, cacheable instructions,
    schema, effort and output ceiling as named fields -- and the content, instead of seven
    positional arguments. `kwb-extract`'s request builder names the fields, and
    `kwb-platform-xvpe` re-exports `RequestTerms`.
  - `Passage::Requires_Page_Images` is `Is_Page_Image_Required`.

  With those made, the workspace compiles, and `cargo test --workspace` passes 436 and fails the same
  three tests the `d6f075c14` copy fails (`one_liveness`, the README member table, the band
  declaration).
- **What a request renders as did not change.** The fingerprint of the extractor's own request,
  schema included, built from the same source at both revisions, rendered
  `1af0b0c1ed896c7eccaec57f7aa22caa68f355fa9dc13104f2138170f1092383` at each. XVPE restructured the
  fingerprint's encoding in the same pass, and it writes the same bytes.

So the store half moves at `f45a7b085`. `KWB-119` moves the pin, with the three renames and nothing
else. `KWB-120` adopts the store behind `kwb-platform-std`'s own `DirectoryContentStore`, as the
decision above has it.
