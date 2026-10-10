---
id: TC-031
title: "Verify function-application oracle generation, agreement, and static location tagging"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---
# TC-031: Verify function-application oracle generation, agreement, and static location tagging

## Description

Verify CG's target-neutral function-body graph analysis, flat SSA schedule,
caller-selected work and source-byte limits, compact location records, typed
refusals and deterministic claim ordering. Concrete oracle execution is a
separate planned leg against the QSL-owned application API after
QSL-358/IR-349. The old Contract Runtime `exact` corpus remains historical
evidence for the behavioral obligations retained in FR-021; it is not a
required dependency or target path.

## Test Procedure

1. Build an admitted `quire.checked-package/v2` package under QSpec FR-322.
   Bind each requested function to its admitted V2 `function` declaration
   node (whose body is the root), ordered `value`/`parameter` node ids and
   declared types. Build scalar, equality and nested-call expression bodies
   from separate V2 expression nodes, each joined by `reference` leaves; a
   call's argument 0 references its exact function declaration, followed by
   value arguments in source order. Include a 100,000-node unary-call chain
   over a simple identity function, with each nested application its own V2
   expression node and no shared expression node. Supply its lowered graph at
   the generator seam to isolate CG from upstream reader/lowering limits
   (FR-021-AC-7 and AC-27).
2. At the same seam, vary one edge at a time: missing child, expression cycle,
   expression node reached by two parents, unbound parameter, wrong callee
   declaration, wrong operation identity, wrong operand type and wrong result
   type. Assert the typed refusal occurs before any instruction or location
   entry for that function, while an unrelated healthy binding stays
   scheduled (AC-1, AC-12 and AC-27). Retain capability-gated, reference-typed,
   model/relation, state/temporal/protocol and unsupported `Negate` cases and
   assert their distinct existing blockers (AC-6, AC-10, AC-11 and AC-21).
3. Generate the main corpus twice and under permuted request order. Compare
   schedule source, claim and location bytes; inspect ordering by call-node id,
   declaring function-node id, argument source ids and final byte-wise name
   tie-break. Assert each scheduled item names the request's own function
   identity and source ordering, never an identity read back from emitted
   source (AC-3, AC-13 and AC-24).
4. Count one generation charge for each entered node, followed child edge,
   emitted instruction and location record, sharing one per-function counter.
   Under a limit equal to the measured count, assert complete schedule output;
   one unit less returns `GenerationWorkExhausted` with the first denied count
   before the action and publishes no partial root or location record. A
   separately failed Contract IR lowering record retains its work or byte
   refusal before any generation charge; a failed call-node record wins over
   the body record. Supply `u64::MAX` as the setting and assert
   `InvalidGenerationWorkLimit` before lowering; at `u64::MAX - 1`, assert the
   first denied `u64` count is `u64::MAX` without overflow (AC-23, AC-25,
   AC-30).
5. Measure the 100,000-node body's flat source bytes. Under the default
   1,048,576-byte setting, assert `SourceTooLarge` names that limit if the
   source needs more. Raise the caller limit to the measured size, generate
   on a 512 KiB stack and compile the complete schedule crate; with one byte less, assert
   refusal before append and no partial artifact. Verify every ordinal-range
   module fits its publication artifact byte limit and the bundle fits its
   bundle limit. Scan the table: each SSA slot is unique, each operand slot
   precedes its user, and no Rust expression/closure nests in proportion to
   V2 graph depth (AC-7, AC-14, AC-28).
6. Inspect the location map for the shallow scalar, equality and nested-call
   corpus and independently walk its V2 graph to derive each path. Assert
   function origin and child ordinals match exactly. For the 100,000-call
   chain, assert exactly 100,000 constant-size parent-linked records, no eager
   full paths, and linear source/map size. Query its deepest path with
   sufficient work and byte limits and compare all 100,000 ordinals; one
   query unit less refuses without a partial path. Independently compare the
   origin with the QSL-owned checker's refusal when that API exists
   (AC-15, AC-17 and AC-29; origin cross-check PLANNED QSL-358/IR-349).
7. On a shallow body with separately observable left and right charge
   points, inspect branch/stop instructions. A denied left charge must branch
   out before any right charge, and an admitted left charge must precede the
   right and parent operation. The eventual concrete oracle shall be run
   with both denial injections and compared with an independently assembled
   shallow body (AC-5, AC-9 and AC-26; execution PLANNED QSL-358/IR-349).
8. Reuse the six duplicate-declaring-node fixtures and permutations of
   FR-021-AC-22: both admissible; one Stage-1 refusal; shared name plus a
   distinct-node declaration; a nested caller; two duplicate groups holding
   one name; and two items on one call node, including a repeated item. Assert
   the shared-id declarations appear in neither instruction nor location
   table, every item has the exact `DuplicateDeclaringNode` precedence and
   smallest applicable id, nested callers get `UnknownCallee`, and unrelated
   claims remain identical with the pair removed.
9. Reuse the seven unknown-name and duplicate-pair cases of FR-021-AC-24:
   one unknown, two different unknowns in both orders, one unknown twice,
   known plus unknown, case-sensitive names, two pair members under both name
   orders, and differing declaring ids. Assert separate entries, each one's
   own refusal, byte-wise name tie-break only after node ids, and stable
   output under permutation.
10. Retain the `Failed`-record seam for work, bytes and unrecognised lowering
    limit kinds; assert the mapping of FR-021-AC-23, body/call-node priority,
    per-function isolation and no panic. Scan generator and schedule source
    for the panic tokens AC-19 and AC-21 forbid, and assert `Negate` has no
    schedule root. The schedule and claim/location maps shall not read or
    depend on dynamic invocation location or losses (AC-16, AC-19 to AC-21).
11. After QSL-358/IR-349 publishes its callable and admission API, emit the
    concrete owner-path crate from the same schedule, compile it, and execute
    both the main corpus and the 100,000-node case on a 512 KiB stack under
    caller limits raised to fit. Compare outcomes, typed input refusals,
    charge sequences and consumed counters with an independently assembled
    QSL-owned package and shallow equivalent; deny each charge in turn and
    assert `Incomplete` before the denied charge. Inspect linked-mode
    admission, no kernel-mode application, exhaustive owner-result handling,
    no literal outcomes/charges and no dependence on dynamic location/losses.
    This is the distinct PLANNED execution gate for AC-2, AC-4, AC-5, AC-8,
    AC-9, AC-16 through AC-20, AC-26 and AC-31; schedule compilation in step
    5 does not satisfy it.
12. Inspect the schedule crate and eventual owner-path emission for any
    `quire_contract_runtime::exact` dependency, alias, wrapper or compatibility
    surface. Assert none is required. The owner-path leg remains PLANNED until
    QSL-358/IR-349 defines the API (AC-32).

## Expected Results

Every admitted body has a deterministic flat schedule, compact location map
and one item disposition. Deep generation succeeds under raised work and
source-byte limits on a small stack, compiles as a schedule crate and has no
depth refusal. The first denied work or byte charge refuses before action or
partial publication. Existing duplicate, unknown-name, lowering and blocker
precedence remains testable. Concrete QSL-owner-path compilation and execution
is a separate planned gate; no Contract Runtime `exact` path is required.
