---
id: D-007
type: decision
title: This repository takes no path dependency on XVPE, and the reason is coupling rather than breakage
status: accepted
version: 6
authority: canonical-normative-record
tags:
  - ecosystem
  - dependencies
  - platform
relations:
  - target: D-004
    type: relates-to
  - target: D-005
    type: relates-to
---

# This repository takes no path dependency on XVPE, and the reason is coupling rather than breakage

## Decision

No crate here depends on `f:/repos/xvpe` by `path`. When a dependency is taken it is by git
reference and commit SHA, into a crate that exists to quarantine it, on the model
`nomos-platform-xvpe` sets and for the reason `D-130` gives: **a commit SHA is the difference
between adopting a body of work and adopting whatever that work becomes.**

What changes with this record is the *reason*, and the change is not cosmetic. The position
no longer rests on XVPE being unfit to depend on. It is not. It rests on what a `path` edge
does to this repository's reproducibility, which is a different claim, survives measurement,
and points at a different remedy.

`AGENTS.md` routes here rather than restating any of it.

## Rationale

### The premise that is gone

`AGENTS.md` said, and this repository inherited from `D-130`, that XVPE's foundations tier
"does not currently compile (`xvpe-dataflow` fails at 62 errors)."

Measured 2026-09-12 against xvpe `dev` at `a7eee3c6e361371b269217201233a2e7d7e36122`:

```
cargo check -p xvpe-dataflow
```

succeeds. 106 warnings, zero errors. The named crate compiles, and Nomos had already
recorded the same result on its own side in `D-130`'s amendment — checked there on
2026-08-18 across five crates, all clean. **This repository was carrying, as a live hazard,
a premise its own source had already retired.** A session reading `AGENTS.md` before today
was misinformed by it.

That is the failure worth naming: not that the fact went stale, which facts do, but that it
was written somewhere that does not get re-measured, in a file whose own text says it holds
how to act safely and not what is true.

### `D-130`'s surviving reasons, evaluated here rather than inherited

`D-130`'s amendment is explicit that a clean compile does not lift its gate, and gives three
reasons that are untouched by build stability. Each is a claim about **Nomos and what Nomos
wanted**, and this repository is obliged to ask whether it holds for **KWB and what KWB
wants**. Two do not.

**"No storage crate and no package crate."** This does not transfer. It describes what Nomos
needed and XVPE lacked. KWB needed a content-addressed document store and `KWB-2` built one;
`kwb-store` is this repository's own, and nothing about XVPE's inventory bears on it.

**"`xvpe-telemetry` names `xvpe-os-backend-desktop` as a direct dependency."** True of that
crate, and irrelevant to the crates KWB would want, which are not that crate. The relevant
measurement is below, and it is decisive.

**"A `path` dependency does not select the crates that happen to compile today; it selects
the workspace."** This is the reason that survives, and it survives in a **weaker and more
precise form than it is stated in**. Mechanically the sentence is not true — Cargo resolves
a path dependency to the named crate and its transitive dependencies, not to the workspace
containing it, and the measurement below is that closure. What is true, and what the
sentence is reaching for, is a claim about *coupling*: a path edge binds this repository's
build and its reproducibility to another repository's **working tree**, at whatever state
that tree is in.

### What was measured, exactly

On 2026-09-12, against xvpe `dev` at `a7eee3c6e361371b269217201233a2e7d7e36122`.

The candidate crates are chosen, not swept: they are the ones a KWB ingestion path would
actually reach for, and they are the mechanism half of the same pairing the Nomos precedent
records — XVPE owns the mechanism and the backend, the product owns its catalogue.

```
cargo tree -p xvpe-json-text    --no-default-features -e normal
cargo tree -p xvpe-corpus-text  --no-default-features -e normal
cargo tree -p xvpe-ai-inference --no-default-features -e normal
cargo check -p xvpe-json-text -p xvpe-corpus-text -p xvpe-ai-inference -p xvpe-primitives
```

| Crate | What it holds | Declared dependency closure |
|---|---|---|
| `xvpe-json-text` | minimal JSON text surface, `no_std` | **nothing at all** |
| `xvpe-corpus-text` | page fidelity, chunking | `xvpe-primitives` |
| `xvpe-ai-inference` | model surface, integer money | `xvpe-primitives`, `xvpe-json-text` |

Four crates in the union. **Zero third-party packages.** No GPU crate, no windowing crate,
no `xvpe-os-backend-desktop`, nothing resembling the transitive pull `D-130` warns about.
All four compile clean. Every inter-crate edge carries `default-features = false`.

Two of `D-130`'s structural descriptions are also simply out of date and should not be
re-cited from here: XVPE is no longer organized as `crates/engine/{foundations, simulation,
runtime, presentation}` — there is no `crates/engine` — and the workspace is 258 members
across eleven tiers rather than the 112 that record measured.

So the technical objection does not survive. **For the crates KWB would want, XVPE is
already fit to depend on.**

### The reason that does survive, and the evidence for it

The coupling claim, stated as what it is: a `path` edge makes this repository's build a
function of another repository's working tree, and that tree is measurably in motion.

The sharpest available evidence is from inside XVPE itself, 2026-09-12. Its reference miner
records a run, then replays it to re-evaluate deterministic downstream processing. Five
crates were added to the workspace — 253 to 258 — and the replay of a recording written the
day before matched **171 attempts, 171 not-recorded, zero tokens**. Nothing about the
prompt, the schema, the chunking, the models or the source had changed. Workspace drift the
caller never touched defeated a deterministic re-run.

That is not a compile failure and no `cargo check` would have caught it. It is exactly the
class of harm a path edge imports: not *"the dependency is broken"* but *"the dependency is
not the same dependency it was yesterday, and nothing said so."* A repository whose whole
premise is content-derived identity and reproducible derivation cannot take that edge
casually. `D-006` computes staleness by comparing a recorded input identity against a
recomputed one; a path dependency is an input with no identity to record.

A commit SHA gives that input an identity. That is the same argument this repository already
makes about everything else it stores, which is why the remedy is not novel here.

## Consequences

The gate is no longer *"wait until XVPE is fit to depend on."* That question is answered.
Two conditions replace it, and they are different in kind:

- **Mechanical.** Adoption is by git reference and commit SHA, into one crate that exists to
  hold it. Never by `path`. When such a crate is created it takes the `kwb-platform-xvpe`
  name the workspace manifest already reserves a comment for, and its creation is an item.
- **Open, and held elsewhere.** *Which* requirements XVPE is the right authority for under
  `D-135` is not settled by any of the above, and is on `D-004`'s held list. This record
  deliberately does not answer it. Nothing here proposes adopting any crate; it measures that
  three named crates would be cheap to adopt, which is not the same statement and must not be
  read as one.

`AGENTS.md`'s hazard is replaced by a pointer to this record. It no longer carries a
technical premise of its own, because a premise stated in a file nobody re-measures is how
this one went stale.

## What this measurement does not cover

Listed because a measurement whose limits are unstated invites being read as more than it is,
and `D-003` is this repository's record that presence is not validity.

- **The closure was measured in XVPE's own workspace, not in this one.** Cargo unifies
  features across a workspace, so a closure measured there is not the closure a KWB member
  would resolve to. XVPE has been bitten by exactly this — enabling one feature on one crate
  turned it on for six — and a dependency edge missing `default-features = false` has
  collapsed a `no_std` floor there before. The figures above are a lower bound on what
  adoption would pull.
- **No KWB crate has ever named an XVPE crate.** Nothing here has been built from a consumer
  in this repository. This is a reading of manifests and a check in the other tree.
- **Nothing about API stability, licensing, or `no_std` posture under this repository's own
  floor** was examined.
- **The 253-to-258 replay figure is inherited, not re-derived.** It is recorded in XVPE's
  reference-miner architecture document; the run journals behind it are on no path this
  machine could find, so it could not be reproduced here. It is cited as the clearest
  statement of a mechanism, and the mechanism does not depend on the exact count.
- **Whether these three are the right crates is not addressed**, only what they would cost.

## Amendment: The Pin Moved, and the Closure Is Measurable Here Now, 2026-09-13

Two of the limits stated above under *What this measurement does not cover* have expired, and
closing them answers a question the manifest has been asking since `KWB-24`.

Neither needed a new measurement of XVPE. They needed this repository to have a consumer, and
`KWB-24` built one: *no KWB crate has ever named an XVPE crate* stopped being true then, and
with a consumer here `Cargo.lock` resolves the closure **in this workspace**, which is the other
limit. The figures in version 1 were a lower bound taken in XVPE's own tree. These are not.

### The manifest names four crates and this workspace compiles nine

Named, each pinned by `rev`: `xvpe-collections-persistent`, `xvpe-clock`, `xvpe-corpus-text`,
`xvpe-ai-inference`.

Reached through them and named nowhere in this repository: `xvpe-primitives`,
`xvpe-collections-map`, `xvpe-collections-sequence`, `xvpe-collections-handle` and
`xvpe-json-text`. `Cargo.lock` carries all nine at one commit.

**The five unnamed crates are not a lesser part of the adoption, and this repository has been
surprised by them twice.** `KWB-90` found the build floor at 1.87.0 and traced it to
`integer_sign_cast` in `xvpe-collections-map` — a crate reached through
`xvpe-collections-persistent`, so the quarantine confined the dependency *edge* to one manifest
while the toolchain floor it carried reached every crate above band 1p. `D-016` records that.
The second is below. Both were found by going and looking, and nothing would have reported
either.

### What the bump changed, which is what a pinned dependency exists to make visible

The manifest says beside the pin that bumping the `rev` is a decision rather than maintenance.
The pin moved from `a7eee3c6e361371b269217201233a2e7d7e36122` to `8ff98a8fd` at `KWB-72`, whose
subject was `D-014`'s last deferral, and no record says it moved. `8ff98a8fd` appears in no
record at all; `OD-GATE-001` names it, to say the gate cannot fetch it.

Measured between the two commits, 2026-09-13:

- **None of the four named crates changed.**
- **`xvpe-json-text` did**, which this workspace compiles and does not name. Two constants
  widened from private to `pub(crate)`, and `JsonRecordReader` gained one method —
  `Flag(document, name) -> Result<bool, FieldDefect>`, which those two constants serve — with
  tests. Additive and visibility-only; nothing was removed or changed in behaviour.
- Nine crates changed in XVPE between the two commits. One of them is in this closure.

So the bump was behaviourally inert here. **That is the measurement's result and not its
justification** — it was inert as a matter of fact, not by anything this repository did, and
nothing here established it at the time or since. A bump that had changed behaviour would have
looked identical from inside this repository.

One thing deliberately not treated as a defect: the abbreviated `rev`. `Cargo.lock` resolves it
to `8ff98a8fd478750ca4f8a96ffd7174db902ffd0f`, so the abbreviation is latent rather than live,
and rewriting it would churn the lock for no measured gain.

### The rule this forces

**The surface a `rev` bump must be measured against is the closure `Cargo.lock` resolves, not
the crates the manifest names.** A bump measured against the four named crates would have
reported no change here and been wrong about the closure, which is the same error version 1
guarded against in the other direction by calling its XVPE-side figures a lower bound.

### What is guarded, and what is not

`KWB-96` adds `Test_Every_Xvpe_Dependency_Should_Pin_The_Same_Commit` to `tests/contract`. The
four pins must name one commit, so a partial bump — the shape a hand edit inside an item about
something else invites — fails instead of adopting two states of another repository at once. It
was proven by mutating this repository's real manifest, in both directions: one pin left behind
fails it, and a manifest whose shape moves under the parser fails it too rather than passing on
nothing.

It does **not** check which commit is pinned. That is a decision and this record is where it
belongs; a test demanding a particular SHA would fail on every deliberate bump, which is how a
guard gets switched off.

It also does not notice that a bump happened, or measure what one changed. Nothing here can:
`D-007` adopts XVPE by git reference precisely so that no test in this repository reads its
working tree. That measurement is a person's, and this section is what one is owed.

## Amendment: The Closure Is A Set Of Features As Well As A Set Of Crates, 2026-09-13

The amendment above says the surface a `rev` bump must be measured against is the closure
`Cargo.lock` resolves. That is half of it. *Which features* resolve is the other half, and this
repository had never measured it — version 1's figures were taken under `--no-default-features`,
and three of the four real edges do not set that.

Measured at `8ff98a8fd` with `cargo tree -p kwb-platform-xvpe -f "{p} [{f}]" -e normal`:

| crate | features resolved |
|---|---|
| `xvpe-ai-inference` | `default`, `std` |
| `xvpe-clock` | `default`, `std` |
| `xvpe-corpus-text` | `default`, `std` |
| `xvpe-collections-persistent` | none |
| `xvpe-collections-map`, `-sequence`, `-handle` | none |
| `xvpe-primitives` | `std` |

Three things follow, and they are not one thing said three ways.

**`xvpe-primitives` carries `std` whatever any single edge asks for.** The clock,
`xvpe-corpus-text` and `xvpe-ai-inference` each declare `std = ["xvpe-primitives/std"]`, and
Cargo unifies features across the graph, so the one edge that disables defaults does not keep
`std` off the shared foundation. It was never going to. `default-features = false` on one
dependency is not a workspace-wide `no_std` posture, and reading it as one would be the mistake
available here.

**For two of the four edges the setting is inert.** `xvpe-corpus-text` and `xvpe-ai-inference`
declare `std = ["xvpe-primitives/std"]` and nothing else, and each documents itself as needing
nothing from std. Turning their defaults off would change nothing about their own code, so their
manifest entries should not grow a feature justification by analogy with the clock's.

**For the other two it is load-bearing, and unequally protected.** `xvpe-clock`'s defaults stay
on because `HostedWallClock` is behind `std`, and
`pub use xvpe_clock::HostedWallClock as SystemClock` in `kwb-platform-xvpe/src/lib.rs` stops
compiling if they go off — a guard by construction, which is what that manifest comment means by
*proved by naming the types*. `xvpe-collections-persistent`'s `default-features = false` compiles
it and `xvpe-collections-map` freestanding and keeps `StandardHashMap` off the surface, and
**nothing protects that one**: removing the line compiles, turns `std` on, and adds a type
nothing here names. `KWB-98` wrote the reason into the manifest and stated the missing guard
rather than papering over it with a tautological one.

All four settings are correct as they stand. What was missing was any record that they are
choices at all, which is the same gap the amendment above found for the crate list.

## Amendment: The Closure's First Third-Party Packages, 2026-09-28

`KWB-112` names two more crates — `xvpe-remote-call` and `xvpe-remote-call-backend-json` — and
**the pin does not move.** The manifest grew two edges at `8ff98a8fd`, which is the commit every
existing pin already named. `Test_Every_Xvpe_Dependency_Should_Pin_The_Same_Commit` accordingly
reads seven pins where it read four and passes, because its floor is a minimum and not an
equality: what it fails on is seven pins naming two commits, or a manifest whose shape moved
under the parser. Growth at one commit is neither, and it was never meant to be.

### What the closure gained, measured

Measured 2026-09-28, as the difference between the package names in `git show HEAD:Cargo.lock`
and those in the working `Cargo.lock`:

| package | reached through |
|---|---|
| `xvpe-remote-call` | the new manifest edge |
| `xvpe-remote-call-backend-json` | the new manifest edge |
| `serde_json` | `xvpe-remote-call-backend-json` |
| `itoa`, `memchr`, `zmij` | `serde_json` |

Nothing left the closure. Measured with `cargo tree -p kwb-platform-xvpe -f "{p} [{f}]"`, the
features resolved are none for the two XVPE crates and `default,std` for `serde_json`.

### What is left of "zero third-party packages"

Version 1's table ends: *"Four crates in the union. **Zero third-party packages.**"* That figure
was true of the union it measured and has never been true of the closure this repository
resolves, which version 1 also said, under *What this measurement does not cover* — the closure
was measured in XVPE's own workspace and the figures were called a lower bound on what adoption
would pull. This is that lower bound being cashed, and only one half of it is new.

**Already there before this item:** `web-time` through `xvpe-clock`, and `hashbrown` and
`smallvec` through `xvpe-collections-persistent`. They are in the closure and the diff above
does not list them, which is what says they are not this item's doing.

**New, and different in kind:** the packages this adoption gained are not support for a clock or
a container. `serde_json` is a document reader and a document writer. `itoa`, `memchr` and `zmij`
are its own dependencies and were not chosen here at all. That distinction is the one worth
keeping: a third-party package reached because a data structure needs it is a cost inherited
with the crate, and a document model is a cost this repository accepted on purpose, for a wire
it decided to adopt. `D-135` is where that kind of decision belongs and that record is not
amended here; the justification is in the manifest beside the edge, which is where a reader
meeting the dependency is standing.

### `xvpe-primitives` stops being reached and unnamed

The amendment above lists `xvpe-primitives` among five crates *reached through them and named
nowhere in this repository*, and calls the unnamed ones a lesser part of the adoption that had
surprised this repository twice. `KWB-112` names it, for the narrowest reason available: a
`use` cannot name what the manifest does not declare. The seam re-exports four of its types, so
the edge had to be written down. That is the same shape of fact the amendment above records —
a crate depended on with nothing in the manifest saying so — with the difference that this one
was noticed while it was being written rather than after a toolchain floor moved.

### What this amendment does not decide

Whether `xvpe-remote-call` is a requirement XVPE is the right authority for under `D-135` is not
settled here or by this item. The boundary this adoption draws — the mechanism and the wire in
XVPE, the catalogue in the product — is an application of `D-135`, not a finding about it, and
the question of *which* requirements are XVPE's remains on `D-004`'s held list where the
Consequences section above put it.

## Amendment: The Pinned Commit Left Every Remote, So The Pin Moves To `7471a1c9d`, 2026-09-30

XVPE's history was restarted on 2026-09-30. `github.com/kevinmettias/xvpe` holds a new history,
and every XVPE commit this repository has pinned or measured against — `a7eee3c6e`,
`8ff98a8fd`, `c700bcf83`, `d6f075c14`, `f45a7b085` — left GitHub with the old one. The
revisions the records cite stay as they are: each is a measurement dated where it was made, and
none is a pin. The pin is different. `c700bcf83` now resolves only where cargo's git cache still
holds its checkout, so this workspace builds on the machine that built it last and a fresh clone
cannot build at all. XVPE is private until its owner publishes it, which bounds who can fetch
any revision; it does not change that the old one can never be fetched again.

**The pin moves to `7471a1c9d`**, XVPE's `dev` at the time of writing. That is one move, and it
replaces the two `KWB-117` and `KWB-119` planned, because both of their target commits are among
the ones that left. `KWB-140` carries it.

### What the move changes, measured

Measured 2026-09-30 on a scratch clone of `a3725be`, toolchain 1.87.0, with every pin at
`7471a1c9d`. Every change the source needs was already measured by `KWB-117` and `KWB-119`
except one:

- `RequestFingerprint::Of_Request` returns a `Result`, and the replay fixture in
  `kwb-extract/src/tests.rs` expects it (`KWB-117`).
- `xvpe_ai_inference::strategies` is private, and `ReplayInference` and `ReplayRecording` are
  re-exported at the crate root. `InferenceRequest::New` takes a `RequestTerms` and the content.
  `Passage::Requires_Page_Images` is `Is_Page_Image_Required` (`KWB-119`).
- **New:** `HostedWallClock` is no longer in `xvpe-clock`. XVPE moved every host clock into
  `xvpe-host-clock` on 2026-09-29, so that one crate is the only place that reads the machine's
  clock. `SystemClock` has to be re-exported from there, which means adopting a crate this
  repository has not named before.

With those changes made, `cargo test --workspace` passes 452 tests and fails one:
`Test_The_Adopted_Revision_Should_Be_Named_By_A_Record`. This amendment is what makes that test
pass. The extractor's tests still replay the recordings committed before the move, and replay
finds a recording by its request's fingerprint, so no fingerprint moved.

### What adopting `xvpe-host-clock` costs, and why it is nothing

At `e87f438fd`, the first commit of the new history, `xvpe-host-clock` also held the deadline
clocks that XVPE's concurrent collections and TTL cache take as strategies. Adopting it for a
wall clock would have brought five more XVPE crates into this closure (`xvpe-collections-concurrent`,
`xvpe-collections-cache`, `xvpe-collections-bits`, `xvpe-collections-probabilistic` and
`xvpe-sync-primitives`), along with nine third-party packages: `parking_lot`,
`parking_lot_core`, `lock_api`, `crossbeam-epoch`, `crossbeam-utils`, `scopeguard`, `bitflags`,
`redox_syscall` and `windows-link`. XVPE put those clocks behind an
opt-in `collections` feature at `7471a1c9d`. With default features, which enable nothing,
the crate's closure is `xvpe-clock`, `xvpe-primitives` and `web-time`. This closure already
holds all three.

Compared by name and version, `Cargo.lock` gains exactly three packages: `xvpe-host-clock`, and
the `xvpe-content-identity` and `xvpe-algorithms-hashing` that `KWB-117` already measured
arriving through `xvpe-ai-inference`. **No third-party package is gained, lost or re-versioned.**
One trap: while cargo re-resolves the move, it raises `js-sys` and the `wasm-bindgen` family by
one patch version without being asked. Nothing requires that. Held at the versions the lock
already carries, they resolve and the workspace builds, and that is the delta above.

### What this corrects in the amendment on features

That amendment says `xvpe-clock`'s default features stay on because `HostedWallClock` is behind
`std`, and that the `SystemClock` re-export guards the setting by construction. **Both halves
are no longer true.** At `7471a1c9d`, `xvpe-clock`'s `std` feature does three things. It lets
the crate link `std`. It exposes `ControllableClock` and its companion types. And it turns on
`xvpe-primitives/std`, which other edges unify on anyway. Nothing in this repository names
`ControllableClock` or any companion. So the setting stops being load-bearing here, the same as
`xvpe-corpus-text`'s and `xvpe-ai-inference`'s. The guard by
construction now protects the `xvpe-host-clock` edge instead: turning on its `collections`
feature would still compile, and would bring back the nine packages above. The setting stays as
it is. Changing it is not the move's business, and a setting that no longer matters needs no
defence. Only the comment beside it has to stop giving a reason that no longer applies.

### The revisions other records measured against

`D-018`, `D-019` and `D-020` measured the crates they adopt at `c700bcf83`, `d6f075c14` and
`f45a7b085`. At `7471a1c9d` those crates are as follows:

- `xvpe-content-identity`'s source is identical to `d6f075c14`.
- `xvpe-content-store`, `xvpe-file-system` and `xvpe-file-system-backend-system` are identical
  in source to `f45a7b085`.
- `xvpe-evidence` differs from `d6f075c14` by one doc comment moved into a markdown include.
- `xvpe-record-log` differs from `c700bcf83` only by additions: `ErasedRecordLog`, and a
  `Close_Torn_Tail` method on its strategy trait. The trait method matters only to a type in
  this repository that implements that trait.

XVPE's tier migration moved several of these crates to new paths. A git dependency names a crate,
not a path, so that is invisible here. Those records' measurements therefore hold at the new pin,
and they are not amended.

## Amendment: `7471a1c9d` Left GitHub Too, So The Pin Moves To `726aa3fee`, 2026-10-01

XVPE's history was folded into a single commit on 2026-10-01 and its GitHub repository was
re-created, so `7471a1c9d` — the target the 2026-09-30 amendment chose — left GitHub before
`KWB-140` carried it out. XVPE's `dev` and `main` are now one commit, `726aa3fee`. The fold was
first published as `aa9833f2b` and re-made the same day with a byte-identical tree, so every
comparison below holds for `726aa3fee` exactly as it was measured for `aa9833f2b`.

**The pin moves to `726aa3fee`** instead, and `KWB-140` carries it. Nothing else in the
2026-09-30 amendment changes, because nothing it measured changed:

- Every XVPE crate in this workspace's `Cargo.lock` — `xvpe-ai-inference`,
  `xvpe-collections-handle`, `xvpe-collections-map`, `xvpe-collections-persistent`,
  `xvpe-collections-sequence`, `xvpe-corpus-text`, `xvpe-json-text`, `xvpe-primitives`,
  `xvpe-remote-call` and `xvpe-remote-call-backend-json` — and the three the move adds —
  `xvpe-host-clock`, `xvpe-content-identity` and `xvpe-algorithms-hashing` — is byte-identical at
  `7471a1c9d` and `726aa3fee`.
- `xvpe-clock` differs only in documentation wording: four files under its `docs/` and one
  comment in its `impact-map.toml`. No source file, manifest or feature changed, so what the
  2026-09-30 amendment says of its `std` feature stands.
- The crates `D-018`, `D-019` and `D-020` adopt — `xvpe-content-store`, `xvpe-file-system`,
  `xvpe-file-system-backend-system`, `xvpe-evidence` and `xvpe-record-log` — are byte-identical
  too, so the previous section's account of them holds at the new pin.

Measured 2026-10-01 by diffing each crate's directory between the two commits, both of which a
local archive of XVPE's history still holds. The build itself was not re-run here; `KWB-140`'s
predicate runs it. XVPE is still private, so the 2026-09-30 amendment's note on
`CARGO_NET_GIT_FETCH_WITH_CLI` applies unchanged; `726aa3fee` is already in this machine's cargo
git cache.

## Alternatives Considered

**Lifting the hazard entirely and permitting a `path` dependency**, on the ground that the
compile premise is dead and the closure is four crates with no third-party packages. Rejected
because the premise that died is not the reason that remains, and the replay measurement is a
demonstration that reproducibility damage is invisible to the check that would have been
offered as reassurance.

**Leaving `AGENTS.md`'s hazard as written until a crate is actually wanted.** Rejected: the
hazard is not inert while unused. It is read by every session before it does anything, and it
was telling each one something false.

**Restating the corrected premise in `AGENTS.md` instead of routing here.** Rejected for the
reason the stale premise existed at all. `AGENTS.md` says of itself that it holds how to act
and not what is true, and a dated measurement is a claim about the world that will go stale
again.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `D-008`
- `D-012`
- `D-016`
