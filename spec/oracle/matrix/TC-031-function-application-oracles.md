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
separate leg that CG does not claim: QSL exports no application API through
qsl-replay (IR-845). The old Contract Runtime `exact` corpus remains historical
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
   For the defensive Model/State AC-11 classifier case, pair admitted family
   nodes as request roots with call items bound to a different checked source
   function. Assert the generated claim refusals map to distinct blockers,
   and state this source/item mismatch in the test. Keep a valid source-bound
   call in the same case as a positive generator oracle. Do not encode Model/State
   nodes as parameter types or claim this covers applied calls: the source-bound
   applied-call leg is PLANNED pending a reader-admissible upstream
   representation.
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
   query unit less refuses without a partial path. The origin cross-check against a
   QSL-owned checker's refusal is outside CG's boundary (AC-15, AC-17 and
   AC-29; IR-845).
7. On a shallow body with separately observable left and right charge
   points, inspect branch/stop instructions. A denied left charge must branch
   out before any right charge, and an admitted left charge must precede the
   right and parent operation. Running a concrete oracle with both denial
   injections is outside CG's boundary (AC-5, AC-9 and AC-26; IR-845).
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
    per-function isolation and no panic. In the whole-generator byte-ceiling
    fixture, submit Scalar/Equality body roots and requested call-item nodes
    to Contract IR separately; assert actual `Failed` record values and the
    call-node failure's `consumed` taking priority. A checked `Call` body root
    is classified from its admitted graph, not submitted as a body-lowering
    request. Scan generator and schedule source
    for the panic tokens AC-19 and AC-21 forbid, and assert `Negate` has no
    schedule root. The schedule and claim/location maps shall not read or
    depend on dynamic invocation location or losses (AC-16, AC-19 to AC-21).
11. Inspect (AC-31, FR-014-AC-47): QSL exports no callable application API
    through qsl-replay, so CG emits and runs no concrete owner-path oracle
    (IR-845). Scan the 100,000-node case's test names, `Trace:` tags, output
    lines and doc comments, this procedure's steps and the FR-021 rows of the
    oracle matrix; none may claim, or present as evidence, that a generated
    oracle compiled, ran or agreed against a QSL application API. Schedule
    compilation in step 5 is not execution evidence. Linked-mode admission
    (AC-8), no kernel-mode application and no literal outcomes/charges (AC-14)
    are inspected on the schedule and generated source.
12. Inspect the schedule crate and the concrete oracle CG emits today for any
    `quire_contract_runtime::exact` dependency, alias, wrapper or compatibility
    surface. Assert none is required. The current oracle still requires it
    (`src/oracle/function/mod.rs`), so AC-32 stays Planned while it does. CG
    emits no owner-path oracle (AC-32, IR-845).
12a. Inspect (AC-18, FR-014-AC-47): scan the function-application corpus's test
    names, `Trace:` tags, output lines and doc comments, this procedure's steps
    and the FR-021 rows of the oracle matrix. None may claim, or present as
    evidence, agreement with the QSL value authority; text that states the
    boundary (outside CG, owned by IR-842) is permitted.
13. Direct unsupported roots (FR-021-AC-33). Admit formula, fairness and case
    roots through Contract IR's public V2 reader, using QSpec FR-370/FR-440
    for formula/case shapes and Contract IR FR-038-AC-107 for fairness.
    Request each root first as an item's call-node id. For the separate body
    leg, use that root as a declared function's body and supply a distinct,
    IR-admitted `expression`/`call` item naming the function. Verify that the
    call item lowers successfully and has no duplicate, capability or earlier
    item refusal, so item classification reaches the declaration's Stage 1
    body result. Include an unrelated healthy function and call item. Compare
    the refused claim's call-node id with the request, and its cause id/tag
    with the relevant IR function-profile lowering record for the direct
    item or body root; for a lowered case root assert the root id and
    `expression` tag. Assert one `ExactFunctionRefusal::Unsupported`
    claim per affected item, no schedule root, symbol or location entry, no
    panic or omitted claim, and unchanged healthy claim, schedule source and
    location map. Request a temporal clause whose closure reaches formula or
    fairness as a separate QSL-121 blocker control. Preserve the existing
    duplicate, capability and lowering precedence cases.
14. Public admitted chain (FR-021-AC-34; PLANNED IR-768). Build one authentic
    `CheckedPackageV2` with 129 distinct function declarations, each with two
    bound Integer parameters. Give 128 functions separate admitted `expression`/
    `call` body roots whose callee references form one acyclic chain; the last
    function has an admitted Scalar body. Make every call's two value arguments
    reference the caller's ordered parameter ids, and use one admitted call
    item naming the chain head. Read the source through Contract IR's strict
    public reader and assert its canonical size is below 1,048,576 bytes.
    Invoke the public function generator from that checked package with the
    default 1,048,576-byte reader/lowering and generated-source limits; raise
    only the caller generation-work limit to the independently counted units.
    Assert 129 scheduled functions, 128 ordered two-argument call instructions
    in the flat SSA schedule, one `Scheduled` item claim naming its root slot,
    a compilable schedule crate under the default source limit, and no
    `LoweringByteLimitExceeded` or `SourceTooLarge`. Permute request order
    with a second healthy item and compare source, claims and location bytes.
    Keep step 10's actual Scalar/Equality and call-item lowering failures as
    negative controls; keep step 2's malformed source edges and AC-27 typed
    refusals as separate negative controls. This is a public strict-reader
    case, not step 1's lowered-graph seam or step 11's inspection.

## Verification Status

AC-7 is unverified. The source-bound V2 fixture exceeds the default 1 MiB
reader budget at depth 128 and 129; no admission, lowering or generation was
attempted for those samples. The byte measurements and the smaller-valid-
encoding blocker assigned to IR-768 are recorded in [FR-021](../functional/FR-021-function-application-oracles.md#status).
The AC-7 acceptance criterion and verification steps above remain unchanged.

## Expected Results

Every admitted body has a deterministic flat schedule, compact location map
and one item disposition. Deep generation succeeds under raised work and
source-byte limits on a small stack, compiles as a schedule crate and has no
depth refusal. The first denied work or byte charge refuses before action or
partial publication. Existing duplicate, unknown-name, lowering and blocker
precedence remains testable. Concrete QSL-owner-path compilation and execution
is outside CG's boundary (IR-845); no Contract Runtime `exact` path is required.

Direct formula, fairness and case requests have one typed unsupported claim
per affected call item, retaining the call request's identity separately from
the causative node. They add no schedule or location and do not alter the
healthy function's bytes; the temporal-clause control keeps its QSL-121
blocker.
