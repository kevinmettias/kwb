---
id: D-017
type: decision
title: Domain vocabulary has one semantic authority, and executable code refers to it rather than restating it
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - domain
  - ownership
  - prototype
relations:
  - target: D-011
    type: relates-to
---

# Domain vocabulary has one semantic authority, and executable code refers to it rather than restating it

## Decision

One authoritative declaration owns a domain's declarative vocabulary: its entity kinds, its
relation kinds, the algebraic properties those commit to, their representations, and their
external identifiers.

Executable code — validators, inference providers, extractors, decomposers — and every derived
artifact — generated types, indexes, registries, rendered documentation, projections — may
implement or project those semantics. Each refers to them by stable identity. None of them
declares a vocabulary of its own that overlaps the authoritative one.

This decides authority and nothing else. The carrier, the serialization, the package shape, the
loading mechanism and the generated representation all remain undecided, and `D-011` Part C is
the reason they are not settled here.

## This is B7 one size larger

`D-011` B7 already decided this for one case. Its requirement:

> **a graph algorithm must not be able to receive a relation whose algebra it has not checked.**
> In this repository that is a statement about types, not about initialisation order.

That was written about the algebra, because the prototype's algebra lived in a global mutable
registry populated by a static constructor and mutated again at dependency-injection time, so
*"a traversal in a host that never loaded packs consults a different algebra than one that did,
and nothing says so."*

The algebra is not special. It was simply the instance that had already been measured. The rule
underneath it — that the declaration owns the vocabulary and the code consults it rather than
carrying a copy — has never been written down here, and the case B7 covers is one of several.

## What the absence cost the prototype

Measured on 2026-09-21 in `C:/Users/kmett/source/repos/KnowledgeWorkbench`, which `D-001` makes
prototype material rather than an implementation to port.

Nine disciplines are each described twice. `src/KnowledgeWorkbench.Domain/Schemas/*.yml` carries
the type hierarchy and the relation algebra. `src/KnowledgeWorkbench.Domain/Modules/*.cs` carries
a hand-written `IDomainModule` per discipline with decomposition, validation and inference rules.
`tests/KnowledgeWorkbench.Domain.Tests/Modules/ModulePackDriftTests.cs` exists to watch the two,
and it reports what happened:

> `physics.yml` once declared six entity types against the module's sixteen, and `history.yml`
> had lost the module's `historically_unconnected_to` entirely.

Three details are worth more than the incident itself.

**Neither side failed.** Each was internally coherent; six types is a valid declaration and so is
sixteen. Nothing in either artifact could detect the divergence, because a second authority does
not look wrong from inside the first.

**The compensating artifact came after.** The drift test was written because the drift existed. It
asserts that neither side holds a type or relation the other cannot account for — a consistency
check between two authorities, which is the thing needed exactly when there are two and not
otherwise.

**It resolves nothing, and says so.** The test *"report[s] divergence rather than resolving it:
the modules are kept because their rules are not yet expressible in YAML."* That is a correct
judgement about behaviour and the wrong one about vocabulary, and the two were never separated —
the modules carried both, and only the vocabulary half was duplicated.

## Behaviour may remain code

This record does not push algorithms into configuration to avoid having code. Decomposition,
validation, inference and extraction stay executable, and the prototype's reason for keeping its
modules — rules a declaration could not express — remains a good reason to keep executable
providers.

What such a provider may not do is carry its own account of what exists. A physics inference
provider implements a named rule; it does not also publish a list of which physics types there
are. The separation the prototype never made is the whole content of this decision.

## What this does not decide

The carrier, the serialization format, the package shape, the loading mechanism and the generated
representation. This record names no type, no file format and no loader.

That restraint is not caution, it is `D-011` Part C applied one step back:

> **The pack format is downstream of all of it** [...] A format designed first would fix the
> declaration surface before the vocabulary knew what it had to be able to say — and B4 is a
> measured instance of exactly that having happened once already.

A rule phrased as *the authoritative object is a versioned package-definition contract* would fix
the carrier before the vocabulary knows what it must express, which is the same error B4 records,
committed one level further out. Whatever eventually carries the declaration inherits this
decision; it does not get to be named by it.

## Consequences

- `KWB-105` builds the guard. A record is not a guard, and this one is unenforceable today
  because no domain package exists. What that guard can prove before the first package is that
  the mechanism is non-vacuous — which is why its `done_when` requires injected refusals rather
  than a green run, in the form `tests/contract/tests/boundaries.rs` already uses on its synthetic
  `D-900`.
- `D-011`'s two open questions are untouched. Layer inheritance in A3 and the
  closure-versus-unclassified choice in B3 are questions about what the vocabulary says. This is a
  question about who says it, and answering one does not answer the other.
- A future pack format is constrained in exactly one way: whatever it is, it is the authority, and
  the code that consumes it refers back to it.

## Alternatives Considered

**Deferring until the first domain package exists.** Rejected. The prototype is the evidence: the
duplication was not a decision anybody made, it was what happened when two artifacts each needed
to know the vocabulary and neither was named as the one that owned it. Stating the rule costs a
record now and a migration later, and the later cost is paid in exactly the artifacts hardest to
change.

**Naming the carrier, so the rule has something concrete to attach to.** Rejected under `D-011`
Part C, above. The concreteness is the defect.

**Recording this as an amendment to `D-011`.** Rejected. An amendment here leaves a superseded
claim visible beside its correction, and there is nothing in B7 to supersede — B7 is correct and
stays correct. This generalizes it. `D-011`'s version does not move, and the relation between the
two records is `relates-to` rather than `amends`, which in this repository has one use and means
a later decision modifying an earlier one's content.

## Referenced By


*Written by hand, and checked by `tests/contract` in both directions: a declared relation with
no entry here fails, and an entry here that nothing declares a relation to fails too. Either
end may be a record or an observation, since `KWB-86`. A relation is declared in the
frontmatter of the document that makes it; this is the other end, so that a reader of this
record can reach the ones that answer, amend or build on it. Before `KWB-38`, 24 of 27
relations were reachable from one side only — which is how three records came to assert things
this repository had stopped doing.*

- `D-011`
