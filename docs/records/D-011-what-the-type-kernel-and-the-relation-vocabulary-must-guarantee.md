---
id: D-011
type: decision
title: What the type kernel and the relation vocabulary must guarantee, for all twenty-one kinds
status: accepted
version: 1
authority: canonical-normative-record
tags:
  - domain
  - relations
  - prototype
relations:
  - target: D-003
    type: relates-to
  - target: D-004
    type: relates-to
  - target: D-010
    type: relates-to
---

# What the type kernel and the relation vocabulary must guarantee, for all twenty-one kinds

## Decision

Requirements only. No type is declared, no kind is enumerated as this repository's own, and
no pack format is designed — `KWB-3` requires this record *before* a pack format, and `D-004`
holds the kernel because the prototype's evidence for it is weak.

The evidence for the two halves is **not** of equal strength, and this record treats them
differently on purpose. `D-004` lists *"the operationally validated relation algebra"* as safe
to proceed on, and the algebra is executable on the prototype's pack-load path. The type
kernel is on the held list, and the inventory measured why: twelve byte-identical
`IEpistemicNode` classes, **six of which reach no consumer at all**. So the relation half
below is derived from something that ran, and the type half is derived from what a hierarchy
must be true of for the code that consumes it not to break — which is available without
trusting the types themselves.

## Part A — what a `UniversalType` hierarchy must guarantee

### A1. Every parent resolves, and the hierarchy has exactly one root

The prototype states the purpose exactly: without the kernel, *"a pack's `parent:
physical_entity` is an inert string."* A pack declares its own types with a `parent:` that
resolves into the kernel, and that resolution is the entire mechanism by which a physics
`physical_quantity` and a mathematics `structure` both answer a query for `concept`.

Unresolved parent, no guarantee. This is the requirement the rest rest on.

### A2. Acyclicity must be structural, because the consumer of this guarantee is a loop

`UniversalTypeRegistry.AncestorsOf` walks parents:

```csharp
while (current is not null && _byId.TryGetValue(current, out var parent))
{
    ancestors.Add(parent);
    current = parent.Parent;
}
```

There is no visited set. **A cycle is not an error here; it is a non-terminating loop**, and
`IsA` is built on this walk.

The prototype is not naive about it — its registry test file names the hazard in as many
words, *"internally inconsistent — a dangling parent, a cycle"*. But what that test protects
is a **hardcoded static table**, and what the code needs protecting from is the assumption it
makes while walking. Those are different things, and the second is the one that survives into
a system where a hierarchy is assembled from packs rather than compiled in.

The requirement for this repository: a cycle is either **unrepresentable in whatever holds the
hierarchy**, or **detected on the path that walks it**. A test over one instance of the data
is neither.

### A3. Exactly one layer per type — and whether the layer is inherited is undecided

`KnowledgeLayer` has six members — Lexical, Conceptual, Propositional, Representational,
Evidential, Contextual — and the prototype requires every kernel type to belong to exactly
one, so *"a query can ask for 'everything lexical' or 'everything evidential' without knowing
which discipline supplied the node."*

One layer per type is a requirement and it is well-motivated. **What is not decided anywhere,
and must be, is whether a type's layer is a function of its position in the hierarchy.**
`UniversalType` carries `Parent` and `Layer` as independent fields, and nothing forbids a
`Conceptual` type whose parent is `Propositional`.

The consequence is not cosmetic. If layer is free, then *"everything evidential"* is not a
subtree query, `IsA` and `LayerOf` answer differently shaped questions, and a caller who
assumes the layer of a parent tells them anything about its children is wrong in a way nothing
reports. If layer is inherited, the field is derived and should not be storable independently
— `D20`'s rule, that a status is derived from the evidence rather than assigned.

The prototype's own table appears internally consistent on this point; no violation was found.
That is not the same as the question being answered, and an invariant that holds by accident
in one table is exactly what `D-003` warns against reading as validation.

## Part B — what the relation vocabulary must guarantee, for all twenty-one

### B1. There are twenty-one kinds, and a plan naming nine is wrong

Nine **discovery discriminations** — Identity, FormalCorrespondence, CausalExplanation,
HistoricalInfluence, MethodTransfer, Analogy, Interpretation, Metaphor, Coincidence — and
twelve **structural families** — Taxonomic, Compositional, Dependency, Logical, Evidential,
Normative, Representational, Functional, Temporal, Contextual, Contradiction, Contrast.

The nine carry a strong claim: *"a cross-domain connection that cannot be placed in one of
these is not a finding."* The twelve carry a weaker and different one: they are *"the
structural families needed to type the relations disciplines actually declare."* Two different
justifications for two halves of one enum, and only the first is a closure argument.

### B2. They are not mutually exclusive, and the requirement is a decision procedure

`KWB-3` asks for completeness and mutual exclusivity, or a documented reason they are not.
**They are not**, and the reason is worth recording rather than the claim being weakened.

Overlaps visible in the definitions themselves: `Taxonomic` (is-a) against `Logical`
(entailment), since *X is-a Y* entails *X has Y's properties*; `Dependency` (*"requires
another to function or be defined"*) against `Logical` (presupposition), since definitional
dependency is presupposition; `Evidential` against `Logical`, since a proof both entails and
evidences; `Compositional` against `Dependency`, since a part is usually required.

The prototype nonetheless requires that *"every domain-pack relation declares exactly one of
these as its `universal:` parent."* So exclusivity is **asserted at declaration time** by a
vocabulary that does not have it.

That gap is the requirement. **When several kinds fit, something must say which one wins, and
it must be stated rather than left to the pack author.** Two packs classifying the same
relation differently is not a formatting inconsistency — it is a cross-domain query silently
missing edges, which is the failure the universal vocabulary exists to prevent.

### B3. Completeness is unproven, and the set is closed by refusal

`TryParse` returns false for anything not in the enum, so a relation fitting none of the
twenty-one is **rejected**. A closed vocabulary that refuses is a vocabulary whose
completeness matters a great deal, and the twelve structural families are justified
empirically rather than by closure.

The requirement is one of two things, and this record does not choose between them because the
choice needs the pack format that `KWB-3` puts downstream: either a **closure argument** for
the set, or a member meaning *unclassified* so that an unclassifiable relation is **recorded
and visible** rather than refused at the door. Refusal is data loss, and `D17`'s rule is that
destruction requires evidence — a pack author's failure to find a fit is not evidence that
there is nothing there.

### B4. The algebra a kind commits its specializations to — and the nine properties it cannot reach

This is the sharpest finding in the prototype's own code, and it is a gap rather than an error.

`RelationAlgebra` records **twelve** properties: symmetric, antisymmetric, transitive,
reflexive, functional, inverse-functional, temporal, contextual, definitional, destructive,
confidence-bearing, negative-knowledge.

`UniversalRelationConstraint` — what a universal kind may commit its specializations to —
carries **four flags** reaching **three** of them:

```csharp
public readonly record struct UniversalRelationConstraint(
    bool ForbidsTransitivity = false,
    bool RequiresSymmetry    = false,
    bool ForbidsSymmetry     = false,
    bool RequiresNegativeKnowledge = false);
```

**Nine of the twelve algebraic properties are unconstrained by the universal kind.** A pack may
declare any value it likes for antisymmetry, reflexivity, functionality, inverse-functionality,
temporality, contextuality, definitionality, confidence-bearing — and **destructiveness** — and
no universal kind can contradict it. `IsDestructive` is the one that should stop a reader:
`D17` is *destruction requires evidence*, and whether a relation destroys is currently a pack's
free choice that its kind has no power to refuse.

**And there is no `RequiresTransitivity`.** The vocabulary can forbid transitivity and cannot
require it. Set against the constraint the prototype writes for its most important kind:

```csharp
// Identity is a genuine equivalence relation; that is precisely why it is the only
// kind safe to feed to a union-find.
UniversalRelationKind.Identity => new(RequiresSymmetry: true),
```

An equivalence relation is reflexive, symmetric **and transitive**. The constraint requires
symmetry alone. **The single property that makes `Identity` safe to feed to a union-find is the
one property this vocabulary cannot assert** — a pack could specialize `Identity` declaring
`transitive: false` and `AlgebraViolations` would return empty.

This is `D18` with the polarity reversed. `D18` was a merely-symmetric predicate silently
upgraded into an equivalence relation across 144 of 579 merges. The constraint system now
prevents that upgrade, and cannot require the transitivity that makes the one legitimate case
legitimate. Half a guarantee, and the half that is missing is the half the comment is about.

**The requirement: the constraint vocabulary must reach every property the algebra can express,
in both directions — requires and forbids — or state, per property, why one direction is
meaningless.** Not because symmetry is more deserving of guarding than destructiveness, but
because the current shape is an accident of which two properties `D18` happened to be about.

### B5. A new kind must not acquire zero commitments silently

The constraint mapping ends:

```csharp
_ => new()
```

A kind added to the enum falls into the default and acquires **no commitments at all**, with
no compile error. The permissive answer is the zero value, which is `D20` precisely — *a field
that reports success until someone remembers to say otherwise is not an unfinished feature, it
is a false one.*

The requirement: the mapping from kind to commitments is **total by construction**, so that
adding a kind without deciding its algebra does not compile.

### B6. The two vocabularies must not share words with different meanings

`Functional`, `Temporal` and `Contextual` are members of `UniversalRelationKind`, and
`IsFunctional`, `IsTemporal` and `IsContextual` are properties of `RelationAlgebra`. They do
not mean the same thing. `Functional` the kind is *"a mapping, application, or role: computes,
transduces, preserves, participates-in"*; `IsFunctional` the algebra is single-valuedness — at
most one target. A reader who assumes a `Functional` relation is `IsFunctional` has made an
error the names invited.

Either the vocabularies use disjoint words, or a shared word means the same thing in both.

### B7. The algebra belongs to the kind, not to a registry consulted by convention

Already measured and recorded in `docs/corpus/prototype-incidents.md`: the prototype's algebra
lives in a global mutable registry, a `ConcurrentDictionary` keyed by `object`, populated by a
static constructor and mutated further at dependency-injection time by `AddDomainPacks`. So
*"a traversal in a host that never loaded packs consults a different algebra than one that did,
and nothing says so."*

The requirement carries over unchanged: **a graph algorithm must not be able to receive a
relation whose algebra it has not checked.** In this repository that is a statement about
types, not about initialisation order.

## Part C — what this constrains downstream

- **`KWB-4`** builds the domain types. `D-010` decided that a scope belongs to an *assertion*
  and never to a claim; note that `Contextual` here is a **relation kind**, and using a
  `Contextual` edge to carry an assertion's standing would put scope back on the claim by
  another route. The two are compatible and easy to conflate.
- **`KWB-5`** builds the admission pipeline, and `D-009` recorded that which structures may
  consume which assertions is exactly what the algebra says. B4 is therefore a prerequisite
  for that pipeline being safe, not a tidiness concern: the algorithm that consumes a relation
  is the thing the commitments protect.
- **The pack format is downstream of all of it**, which is what `KWB-3` means by *before any
  domain pack format is designed against it*. A format designed first would fix the
  declaration surface before the vocabulary knew what it had to be able to say — and B4 is a
  measured instance of exactly that having happened once already.

## Consequences

- The relation half of this can be built on. `D-004` lists the algebra as operationally
  validated and it runs on the prototype's pack-load path; the findings above are gaps in its
  coverage, not doubts about its substance.
- The type half stays held. Nothing here declares a type, and the two questions it leaves open
  — layer inheritance in A3, and the closure-versus-unclassified choice in B3 — are recorded
  as open rather than answered, because both need inputs that do not exist yet.
- `KWB-4`, `KWB-5` and `KWB-6` were waiting on this item. What they inherit is a set of
  guarantees to satisfy, not a vocabulary to implement.

## Alternatives Considered

**Enumerating this repository's own kinds and types now.** Rejected: it is what `D-004` holds
this subject for, and `KWB-3` asks for guarantees rather than a vocabulary. The prototype's
twenty-one are evidence of what a vocabulary has to be able to express, and `D-003` is the
record that their existence is not evidence they are right.

**Declaring the twenty-one mutually exclusive because the prototype requires one parent per
relation.** Rejected as the error the prototype made: requiring a single choice is not the same
as the choices being disjoint, and the difference is where two packs come to disagree.

**Treating B4's gap as a defect to fix rather than a requirement to state.** Rejected because
the fix belongs to a vocabulary this repository has not designed. What carries across is the
lesson — that the constraint surface was shaped by the one incident that prompted it, and so
reached two properties out of twelve.
