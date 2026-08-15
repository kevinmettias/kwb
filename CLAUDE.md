# Claude Code in this repository

@AGENTS.md

`AGENTS.md` is the operating contract and it is not repeated here. This file holds only
what is specific to Claude Code.

## Subagents

Work is decomposed by territory, not by topic. Give each worker exactly one claimed item:
its id, its territory, the authorities its task actually reaches, and its verification
predicate.

## The obvious mistakes

Do not create a task list under `.claude` or anywhere else. `work/ledger.json` is the
board.

Do not `Write` a file you have not read in this session.

Scope every commit to explicit paths. `git add -A` sweeps up work that belongs to
somebody else once more than one session works this tree.
