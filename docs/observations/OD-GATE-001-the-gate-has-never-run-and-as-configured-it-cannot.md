---
id: OD-GATE-001
type: observation
title: Six pushes have run the gate and no step has ever executed, and independently of that the gate as configured cannot resolve the dependency D-007 adopts
status: open
version: 1
authority: observation
tags:
  - gate
  - process
relations:
  - target: D-005
    type: relates-to
---

# Six pushes have run the gate and no step has ever executed, and independently of that the gate as configured cannot resolve the dependency D-007 adopts

Measured on 2026-09-13, against the remote and against a Linux machine, while checking whether
twelve commits that had been reported unpushed had reached CI. Written as an observation
because it is a finding about how work is checked here, not a decision about what this
repository is. It decides nothing, and the thing it is about cannot be decided by a session.

There are two facts, and either one alone stops the gate. They are separate, and reading only
the first would leave the second to be found again later.

## No step has ever executed

Six workflow runs exist on `dev` — `34736068864`, `34742201791`, `34742849247`, `34743331845`,
`34745931127` and `34766510019`. Every one is `completed failure` in four or five seconds, and
every one carries an empty `steps` array: the job was created and never started. GitHub's
annotation is the same on each, and it is not about this repository's code:

> The job was not started because recent account payments have failed or your spending limit
> needs to be increased.

That is an account condition. Nothing in this repository can change it, and no amount of
correct configuration would have made those six runs pass.

## As configured, it could not have passed anyway

This is the half that outlives the billing, and it is the reason this observation exists rather
than a note in a session log.

`kwb-platform-xvpe` adopts four crates from `github.com/kevinmettias/xvpe`, by git reference and
commit SHA, which is what `D-007` decided. That repository is **private**.
`.github/workflows/gate.yml` provides no credential for it: no `token:` on the checkout step, no
ssh-agent, no `net.git-fetch-with-cli`, and this workspace has no `.cargo/config.toml` at all.
The token `actions/checkout@v4` installs is scoped to this repository, and cargo does not use it
when resolving a git dependency.

Reproduced on Linux rather than reasoned about:

```console
$ cargo fetch
    Updating git repository `https://github.com/kevinmettias/xvpe.git`
error: failed to get `xvpe-ai-inference` as a dependency of package `kwb-platform-xvpe`
  failed to authenticate when downloading repository
```

The pin is not at fault, and that was checked rather than assumed: `8ff98a8fd` is present on
xvpe's `origin/dev`. A clean runner fails on credentials, not on a bad reference.

## What the gate would have reported, measured by hand

With xvpe's git database seeded from a machine that does hold credentials, and with a target
directory of its own so nothing contended, all three of `gate.yml`'s steps pass on Linux:

| step | command | result |
|---|---|---|
| Lint | `cargo clippy --workspace --all-targets -- -D warnings` | clean, stable 1.98.0 |
| Test | `cargo test --workspace --no-fail-fast` | 210 passed, 0 failed |
| Contract | `cargo test -p kwb-contract-tests --no-fail-fast` | exit 0 |

The count is identical to Windows, and so are the test *name* sets, compared directly rather
than inferred from the totals. The workspace declares no `cfg(unix)`, `cfg(windows)` or
`cfg(target_os)` anywhere, so there was nothing platform-conditional to diverge. This is the
first time this workspace has been built on Linux.

**One number in that table was wrong before it was right, and the failure is the instructive
part.** The first Linux run reported 230. Two cargo processes were running against one
`CARGO_TARGET_DIR` and writing one log path, and their interleaved output summed to more tests
than exist. Re-measured in isolation it is 210, twice. A measurement taken while something else
is building is not a measurement, and the wrong number was the more interesting-looking one.

## Why it is worth recording

`D-005` adopted the nomos ledger partly on the strength of having a gate. `nomos work finish`
refuses at exit 4 when `gate.yml` cannot be read, on the stated grounds that an item finished
against a check weaker than the gate is the defect that refusal exists to prevent. That refusal
is satisfied by the file being **readable**. Seventy-seven items have now been finished against
it, and it has never proved anything, because the check it stands for has not run once.

The gap is not that the gate is wrong. Run by hand, it is green. The gap is that a file has
been standing in for a check, and nothing here could tell the difference — which is the same
shape as a guard whose probe never reached its subject, and the reason that shape keeps earning
its own items on this board.

## What this observation does not do

It proposes no remedy. Restoring the gate needs an account change, and then a decision about
granting continuous integration read access to a private repository — which is a credential
decision with consequences beyond this workspace, and the maintainer's to make rather than a
session's. This observation stays `open` until something acts on it.
