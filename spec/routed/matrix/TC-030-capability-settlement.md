---
id: TC-030
title: "Verify capability settlement at one negotiation point"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-271
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-335
    type: references
---
# TC-030: Verify capability settlement at one negotiation point

## Description

Verify that every requested capability claim is settled by exactly one
`negotiate_*` arm over a closed backend kind, that each FR-290 rule settles its
own row, and that only an item settled `supported` routes a backend.

## Test Procedure

Settle envelopes whose items exercise each rule in FR-290's stated order: an
absent kind; an unknown kind; an absent extent classification; an unknown
backend; a candidate absent from the registered descriptors; a candidate not advertising the
item's kind; an empty candidate set; two candidates with no named backend; and
exactly one candidate under each row of the advertised-mode table, including an
unbounded extent against a `bounded`-only advertisement both with and without a
finite bound available.

Settle the two-candidate item again under the reverse registration order and
compare the disposition, the cause and the order of the named candidates.

Submit one envelope whose `capability_vocabulary` is absent and one whose value
is another identity, and inspect which kinds were read.

Enumerate the closed backend kind's variants and assert that settlement
dispatches an arm for each.

Ask each settlement that is not `supported`
whether it routes a backend at all, and ask a manifest that repeats one backend
identity the same question.

For planned FR-019-AC-15, inspect the driver projection from each QSL
`Registry::descriptors()` value to CG's descriptor. Build two registries separately, each with
an otherwise equal descriptor, one with `Linked` and one with `Process` origin. Project each,
and confirm identity and advertised pairs
are equal in the CG values while origin alone differs. Verify that a linked
Kani descriptor remains `Linked`, the driver conversion exhaustively matches both QSL origin
variants with no wildcard, no identity or manifest byte test chooses
the origin, and CG has no direct `qsl-route` dependency or new FR-331 origin
wire member. Run this row when IR-633 implements the projection.

For planned FR-019-AC-24, use the QSL FR-335 request for the `+` value-validity
claim in `f using v(s: Set<Int[0,9999]>): Integer pure { size(s) + 1 }`.
Supply the already-classified claim item to CG settlement with: (1) an empty
registry and candidate set; (2) one linked Kani candidate advertising only bounded
`value-validity` with `finite_bound_available=true`; (3) a separate bounded
item requested with `ProofBound::Cardinality{maximum: 8}` against that same
sole candidate; and (4) an added unboundable quantity root with
`finite_bound_available=false` against that bounded-only candidate. Preserve
each item's request index across settlement and routing. Inspect warning
cause, artifact absence and Kani outcome absence for unsupported and
requires-bound items; settlement of the bounded supported item makes no
execution-outcome claim.
QSL TC-845 owns compiling the source into the claim and request fixture;
quire-integration owns a composed source-to-claim, CG and driver execution.

## Expected Results

Each item settles exactly one of `supported`, `requires-bound`, `unsupported`
and `invalid-request`, with the cause FR-290 names for its row and naming the
backend or candidates that row requires.

The unbounded extent against a `bounded`-only advertisement settles
`requires-bound` where a finite bound is available and `unsupported`, warned,
with `unsupported_projection`/`unbounded-extent` where none is; it never
settles `supported`.

The two-candidate item settles `invalid-request` with
`invalid_capability`/`ambiguous-backend`, naming both candidates in candidate
order, identically under both registration orders.

Both malformed-vocabulary envelopes refuse as
`invalid_capability`/`unsupported-version` with no kind read.

Every variant of the closed backend kind has a dispatched arm.

No settlement other than `supported` routes a backend, and a manifest repeating an
identity routes none.

Under FR-019-AC-15, the driver preserves the QSL registry origin as a typed
CG value without altering identity or advertised pairs; the CG crate boundary
and FR-331 wire remain as specified.

The planned collection controls settle respectively `unsupported`, warned with
`value-validity` and no artifact or Kani outcome; `requires-bound`, without a
substituted bound, artifact or Kani outcome; `supported` for the independently
indexed bounded request on that same sole candidate, with any later result
joining only the bounded request's index and no Kani execution outcome implied
by settlement; and `unsupported`, warned with
`unsupported_projection`/`unbounded-extent`, without an artifact or Kani
outcome. The unbounded item's disposition is unchanged by the bounded request.
Passing generic mode-table controls alone does not complete this collection case.
