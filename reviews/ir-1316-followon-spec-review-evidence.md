---
id: SR-2026110283866
title: spec-review/evidence review of IR-1316 follow-on
type: SpecReview
analysis: evidence
scope: agent-ix/quire-contract-codegen; PR745 changed tests/ and spec/; selected IR-1316
  criteria
review_set: subset
---

## Summary

This is a PR/ticket subset review, not whole-repository assurance. Ticket: IR-1316; PR: quire-contract-codegen#745. Historical PR #743 findings and artifacts are separate and were not reused. No applicable AssuranceProfile is declared under spec/. The protected files are unchanged. No code, specification, lockfile, or branch mutation was performed by the reviewer.

## Verdict

**PASS** — No new defect in the scoped evidence changes. This is not a repository-wide assurance verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Ran quoin advise --json against the installed catalog. For the two new executable bindings, the catalog recommends Test methods (including property-based testing for the universal mirror statement and stable-output approval for determinism), consistent with authored Test. FR-008-AC-10 and FR-022-AC-20 receive appropriate source/API inspection or analysis treatment; no fabricated test tag is added.

The advisor flags some existing method mismatches from broad example/universal property shapes: FR-021-AC-18/31 and FR-022-AC-1 in particular. Reviewer judgement confirms their authored methods: they constrain documentary claims or compiler exhaustiveness rather than backend execution. Other flagged retired rows receive no new evidence credit. FR-044-AC-3 requires positive shared-codec integration and stays Planned; absence of a local codec alone is correctly rejected. FR-004-AC-17 and FR-002-AC-9 remain unimplemented architecture inspection obligations. These are disclosed advisory mismatches, not evidence that the requirements should be retagged as runtime tests.

Manually inspected strategy production modules: selected comparison and point use public typed accessors, population consumes the resulting relation, numeric map serializes its typed comparison; the rust_body helper handles emitted Rust text rather than semantic-node JSON. Routed GenerationContexts::has and dispatch use explicit Kani/Process arms; routed item's backend/kind fields are private, constructor checks descriptor identity, and no public mutable backend/kind accessor bypasses it. These are the exact method-only claims newly recorded. WorkCounter::new rejects MAX as InvalidWorkLimit, leaving FR-021-AC-30's different public error and complete near-MAX control undisclosed as proof and truthfully Planned.

## Examined scope

```yaml
scope:
- id: FR-004
  path: spec/evidence/matrix/tests.md
  role: examined
  excerpt: '| FR-004 | FR-004-AC-17 | Inspection | 🚧 Planned (IR-1316; IR-815): V2
    population scan reads package.graph directly outside core/ir |'
- id: FR-044
  path: spec/oracle/matrix/tests.md
  role: examined
  excerpt: '| FR-044 | FR-044-AC-1, FR-044-AC-2, FR-044-AC-3, FR-044-AC-4 | TC-058
    | 🚧 PLANNED (IR-1316; IR-591, IR-976): QSL FR-402 defines the quire-eval-corpus
    encoding and loader; CG has no loadable bundle yet, and non-null expectations
    wait for quire-eval''s registered-body implementation |'
- id: FR-022
  path: spec/routed/matrix/tests.md
  role: examined
  excerpt: '| FR-022 | FR-022-AC-1 | Analysis | 🚧 Planned |'
- id: FR-026
  path: spec/routed/matrix/tests.md
  role: examined
  excerpt: '| FR-026 | FR-026-AC-5 | Analysis | 🚧 Planned |'
- id: FR-002
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: '| FR-002 | FR-002-AC-9 | Inspection | 🚧 Planned (IR-1316; IR-815): harness
    generator still uses V1 typed bodies outside the V2 access path |'
- id: FR-008
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: '| FR-008 | FR-008-AC-8 | Inspection | 🚧 Planned (IR-1316; IR-364): FR-014
    does not call the shared lowerer |'
- id: TC-058
  path: spec/oracle/matrix/tests.md
  role: examined
  excerpt: '| TC-058 | Load CG generated-closure cases into QSL evaluator comparison
    | Integration | P0 | FR-044-AC-1, FR-044-AC-2, FR-044-AC-3, FR-044-AC-4 | 🚧 Planned
    (IR-1316; IR-591, IR-976): use QSL FR-402''s quire-eval-corpus loader; non-null
    expected control waits for quire-eval''s registered-body implementation |'
- id: TC-037
  path: spec/routed/matrix/tests.md
  role: examined
  excerpt: '| TC-037 | Verify the backend adapter trait and its closed-enum dispatch
    | Integration | P0 | FR-026-AC-1, FR-026-AC-4 | 🚧 Planned |'
- id: FR-004-AC-10
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. CG binds coverage inside the same live campaign-runner
    invocation, using the original `/2` wire bytes and their QSL strict-reader typed
    result directly, the producer execution record, and the same generated source/map
    buffers used by that campaign. The runner starts the campaign with fresh private
    profile output, performs any required LLVM merge over only that output, starts
    LLVM export over only that profile, and captures exact export bytes and tool/version
    from that LLVM child's direct output stream while retaining the linked child handles.
    Binding verifies this live campaig
- id: FR-004-AC-11
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. The consuming report retains vacuity and native outcome
    as evidence for IR FR-045; an oracle-success run with an unobserved consequent
    denies the coverage obligation, and LLVM probe observations alone produce no completed-proof
    credit or separate eligible denominator.
- id: FR-004-AC-12
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED / UNRUN. CG consumes QSL''s typed strict-reader clause-run or
    command-error variant for exact `native-run-result/2`: a decisive basis retains
    its witness with assigned members, while `closed-scope` or `unavailable` retains
    none. Missing or other versions, missing/unknown basis, malformed witness, contradictory
    basis/witness and mixed/invalid variant shape are located wire refusals before
    binding; no `/1` fallback or CG copy of the wire reader exists.'
- id: FR-004-AC-13
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. QSL `run_clause` success and a decisive witness remain
    source-clause facts and do not count as generated-campaign execution or LLVM coverage;
    QSL violation, refusal, undefined, incomplete, cancelled, unsupported and internal
    failure remain distinct typed adverse/non-success facts. A bound live run does
    not upgrade any QSL disposition or itself grant campaign success, measured classification,
    proof credit or coverage discharge. The same reader's command-error variant retains
    stage/code/optional cause/message/details and unavailable basis as non-success,
    with no campaign qualifi
- id: FR-004-AC-14
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. Only CG's campaign runner constructs the private, non-cloneable,
    non-serializable live run context; callers cannot construct or restore it from
    a `/2` document, paths, QSL `package_id`, metadata, or a forwarded report. Absent,
    foreign or expired context, or an omitted association yields structured binding
    non-success retaining the typed QSL outcome and available identities, with no
    qualification, classification or discharge. A complete same-invocation context
    admits binding without changing the QSL outcome. The caller supplies maximum encoded-input
    and decoded-working-byte lim
- id: FR-004-AC-15
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. A controlled attempt to replace the strict-reader typed
    result, execution record, generated source, source map, LLVM export stream or
    LLVM tool/version with a foreign-run value cannot construct a valid same-invocation
    context, even when paths, QSL `package_id`, source/map and tool/version declarations
    match. In particular, foreign export bytes offered through a detached buffer or
    a different producer stream/child handle refuse as `producer_run_mismatch` before
    classification because neither is the current runner-owned campaign/profile/export
    chain. A malformed association obse
- id: FR-004-AC-17
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN (IR-815; pending IR-1013). Source inspection shows the
    V2 analyzer opens a `CheckedPackageV2` body only through the `core/ir/` layer
    of AD-004, and names no `BoundPackage` or `ClauseRef`, both of which today's `analyze_bound_coverage`
    names.
- id: FR-004-AC-25
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'SUPERSEDED by FR-004-AC-26 under QSpec #449 / IR-1070; no implementation
    credit. Original planned text: For any executable clause in the V2 analyzer''s
    expected population selected for strategy generation, the generator uses the accepted
    QSL point returned by Contract IR FR-058''s typed accessor for the admitted package,
    checked clause id and authentic `claim` `CheckedOccurrence`. Two authentic occurrences
    on one node with different accepted points retain their distinct rendered points
    when selected separately. A wrong-role, wrong-ordinal or other-node occurrence,
    an authentic tuple with no acce'
- id: FR-004-AC-6
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Campaign counts and test outcome remain complete facts independent of coverage
    classification.
- id: FR-004-AC-8
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. Adverse coverage and non-success native execution cannot
    discharge the coverage obligation merely because a report serialized successfully.
- id: FR-021-AC-18
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of the function-application
    corpus, no TC-031 step and no oracle-matrix row claims, or presents as evidence,
    agreement in outcome, refusal, charge order or consumed counters with the QSL-owned
    application authority; text that states this boundary is permitted. That agreement
    is not claimed by CG and is owned by quire-integration IR-842 (FR-014-AC-47).
- id: FR-021-AC-30
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A requested generation work limit of `u64::MAX` returns `InvalidGenerationWorkLimit`
    before lowering or charging; at `u64::MAX - 1` the first denied count is representable
    in the `u64` refusal and never wraps or panics. PLANNED (IR-511).
- id: FR-021-AC-31
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of the 100,000-node
    case, no TC-031 step and no oracle-matrix row claims, or presents as evidence,
    that a generated oracle compiled, ran on a 512 KiB stack or agreed with a shallow
    equivalent through FR-485 without an actual owner-path run; schedule-source compilation
    (AC-7) is not execution evidence. The planned FR-485 corpus remains distinct from
    QSL value-authority agreement (FR-014-AC-47).
- id: FR-031-AC-11
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): IMPLEMENTED IN CG (IR-596); THE QSL EXEMPLAR RUN IS PENDING. The clause
    that runs QSL''s IT-011 and IT-010-SC-05 against this branch is pending the quire-integration
    run; CG''s in-repo run of the same bundle shape under real Kani is the closest
    equivalent and does not stand for it. `generate_kani_bundle` over the postcondition
    `amount < 1000` implies `amount + 1 <= 1000`, with `amount` an integer of `0..=1000`
    under `reject`, generates a bundle whose postcondition oracle returns `bool`,
    holds exactly one synt'
- id: FR-031-AC-19
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): IMPLEMENTED (IR-596). The Kani bundle oracle of an expression whose
    arithmetic nodes are add, subtract and multiply renders them as the infix `+`,
    `-` and `*` on `i64` with one binary operator per arithmetic node, contains no
    `exact::` call and no `checked_`, `wrapping_`, `saturating_` or `overflowing_`
    method (a remainder node''s `wrapping_rem` is FR-031-AC-23''s, not this criterion''s),
    takes no meter, and its file states that its arithmetic is checked by Kani and
    is not a native evaluator.'
- id: FR-031-AC-20
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): IMPLEMENTED (IR-596). Under real Kani, a harness that calls a bundle
    oracle with unconstrained operands at which IR''s discharge does not hold fails
    with Kani''s arithmetic-overflow check and a counterexample: the oracle of `x
    < 5 && x * 2 <= 10` over `0..=10` under `reject`, called with `x` unconstrained,
    fails the "attempt to multiply with overflow" check, and the counterexample''s
    playback value satisfies `x < 5` and overflows `x * 2` (any such value is valid;
    the assertion does not name one); a hand-writte'
- id: FR-031-AC-21
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): IMPLEMENTED (IR-596). For each of add, subtract and multiply, the bool
    the Kani bundle oracle returns equals the `bool` inside `Completed` of the native
    oracle of the same typed expression, on every vector of a grid inside the declared
    domain at which the expression''s guards pass and IR''s discharge holds, including
    the domain''s edges, with the bundle oracle compiled and run natively in a generated
    crate against `quire-contract-runtime` in plain `cargo test`, with no Kani. This
    is the differential that holds'
- id: FR-031-AC-23
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): PLANNED (IR-602). The Kani bundle oracle renders Divide over a `reject`
    type as the infix `/` and Remainder as `wrapping_rem` on `i64`, one operator or
    call per arithmetic node, contains no `exact::` call and no `modulo`, `div_euclid`,
    `rem_euclid`, `checked_`, `saturating_` or `overflowing_` method, no `wrapping_`
    method other than `wrapping_rem` on a remainder node, takes no meter, returns
    `bool`, and its file states that its arithmetic is checked by Kani. `generate_kani_bundle`
    over a divide or a remaind'
- id: FR-031-AC-24
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): PLANNED (IR-602). Under real Kani, a harness that calls the bundle
    oracle of the construction Z-1 (a divide) with unconstrained operands fails the
    checks "attempt to divide by zero" and "attempt to divide with overflow", each
    with a counterexample, and one that calls the bundle oracle of the construction
    Z-3 (a remainder) with unconstrained operands fails the check "attempt to calculate
    the remainder with a divisor of zero" and no other arithmetic check; a probe harness
    that assumes the declared domain and '
- id: FR-031-AC-25
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion
    retained): PLANNED (IR-602) (its native side calls the kernel). For divide and
    for remainder, the bool the Kani bundle oracle returns equals the `bool` inside
    `Completed` of the native oracle of the same typed expression, on every vector
    of a grid inside the declared domain at which the expression''s guards pass and
    the divisor is non-zero and IR''s discharge holds, including the domain''s edges
    and R-1 and R-2, and including the remainder `y != 0 && x % y <= 0` over `i64::MIN..=i64::MAX`
    at `x = i64::MIN`, `y = -1` (nat'
- id: FR-031-AC-27
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-1316; IR-348) for obsolete error-code cleanup after IR-1045
    native replacement. Original criterion retained: PLANNED (IR-602) (it cannot land
    before the native division of AC-2 exists). The code `UnsupportedIntegerDivision`
    does not exist: the crate''s `GenerationErrorCode` has no such variant, nothing
    under `src/`, `tests/` or `schemas/` spells it, no stable code string of that
    name is emitted, and no diagnostic message that generation can emit names IR-601.
    On the tree before the change the variant exists and a divide or remainder over
    `reject` is refused with it.'
- id: FR-044-AC-1
  path: spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md
  role: examined
  excerpt: 'Given an admitted scalar, an equality and a nested generated-to-generated-to-interpreted
    source application, `make evaluator-corpus` writes one stable unique case id per
    application in `cases.jsonl` at `EVALUATOR_CORPUS_DIR`. Every line is a `quire-eval-corpus`
    `quire.evaluator-case/v1` record with FR-402''s exact members: four source-identity
    labels, display path and text, sorted dependency units, QSL-compiled `package_id`,
    callable function name, FR-071 arguments, decimal-string limits, `object_environment:
    null`, `generated_bodies: []` and `expected: null` while `quire-eval` has no registere'
- id: FR-044-AC-2
  path: spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md
  role: examined
  excerpt: Repeating export of the same cases is byte-identical. Loaded case ids equal
    an independent enumeration of the live QSL-admitted source applications; deleting
    a record fails that census. Perturbing a callable, argument, dependency or limit
    changes the loaded case or causes FR-402's stated load/admission refusal. A producer-side
    compile, case-construction, case-encoding or load failure leaves the previous
    complete bundle byte-identical, or the target absent when none existed.
- id: FR-044-AC-3
  path: spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md
  role: examined
  excerpt: Inspection finds CG uses `quire-eval-corpus` for case writing and loading,
    obtains `package_id` via `qsl_replay::compile_package` inside CG's replay module,
    defines no local case/compared-record codec or loader refusal, and computes no
    expectation from emitted source or Contract Runtime `exact`. CG uses no `qsl-eval`
    dependency or direct QSL evaluator call outside the existing replay facade.
- id: FR-044-AC-4
  path: spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md
  role: examined
  excerpt: Once `quire-eval` implements registered generated-body execution, each
    selected case records a non-null `ComparedRecord` from one run with its sorted
    unique generated-body function names and fresh meter; FR-402's expected/std check
    passes. Perturbing a generated-body name changes the loaded case or causes FR-402's
    stated refusal. Boolean and Integer completed values encode; a completed value
    of another kind refuses `unencodable_value`. The compared record includes the
    exact result arm, refusal, location, losses and all ten consumed counters, with
    `charges` present only under FR-402's non-trunc
- id: FR-024-AC-1
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-47; the former CG-minted O-09 digest scheme
    is obsolete and earns no implementation credit.
- id: FR-024-AC-14
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-53; the former document-digest equality
    check is obsolete and earns no implementation credit.
- id: FR-024-AC-20
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-48; the former four-member O-09 preimage
    and encoder are obsolete and earn no implementation credit.
- id: FR-024-AC-21
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-49; the former kind/occurrence-inside-digest
    control is obsolete and earns no implementation credit.
- id: FR-024-AC-24
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: RETIRED, replaced by FR-024-AC-51; the former frame identity minting rule
    is obsolete and earns no implementation credit.
- id: FR-024-AC-35
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A falsified frame or postcondition harness draws a present model field
    whose accessor type has no `i64` range (for example `IntRange` with a lower endpoint
    below `i64::MIN` and an upper endpoint of -1), and playback binds an `i64` value
    outside that model range. QSL refuses the pre state and settlement is `Inconclusive`
    with `ReplayRefused` (FR-029-AC-16), naming the field and its persisted `TypeNotRange`
    reason; it never reports `Verified` or a violation. Playback inside the model
    range is unaffected, a
- id: FR-024-AC-36
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'Two distinct test backend adapters each produce a typed outcome: success
    carries equivalent assignments, each adapter''s backend identity, the same opaque
    obligation identity, declared domains, source/provenance and replay packet members;
    decode refusal carries only backend identity, obligation identity and typed cause.
    Native text is observed only within its own adapter; the common intake neither
    parses native text nor remints the obligation identity, and QSL receives each
    successful adapter''s original backend and source identity in a complete packet.'
- id: FR-024-AC-37
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The common intake maps an adapter's decode-refusal arm to a typed decode
    evidence failure retaining its cause, with no envelope or replay. Before native
    replay, an out-of-domain assignment yields a distinct typed domain evidence failure;
    an incomplete or QSL-refused witness/packet yields a distinct typed QSL admission
    evidence failure retaining its cause. A replay verdict disagreement remains a
    distinct typed evidence failure. None settles as contract success or contract
    failure, and none of the pre-replay failures invokes replay. Kani's exact decode
    and refusal partition remain governed by FR
- id: FR-024-AC-38
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'Starting from one admitted original failure, one valid reduced candidate
    that preserves the failure becomes a new CG lineage revision referring to its
    exact admitted QSL envelope, its exact parent revision and the unchanged opaque
    obligation identity. After retention, the original QSL envelope compares equal
    to its pre-reduction clone by `WitnessEnvelope<P>: Eq`, and the original CG record''s
    revision identity, parent, obligation and typed evidence fields read back equal
    to their captured values. No lineage member is added to QSL''s envelope or packet.'
- id: FR-024-AC-39
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: From a retained failure revision, an out-of-domain reduction and a domain-valid
    reduction that changes the native verdict each create no revision. After each
    rejection, every prior QSL envelope compares equal to its pre-proposal clone and
    every prior CG record's revision identity, parent, obligation and typed evidence
    fields equal their captured values; a later valid reduction names the actual retained
    parent, with no skipped or substituted parent.
- id: FR-024-AC-40
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness` envelope holds a transcript from its own backend
    re-run, bound to its own reduced assignments; it cannot reuse any ancestor's transcript.
- id: FR-024-AC-49
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-21). Structural identity changes for a different
    frame node, owning model or assigned ordinal and remains equal for unchanged components
    across comments, blank lines, source-span, symbols, solver, options and state-field
    display changes. A change of frame grant that rekeys its producer node changes
    identity; if the node remains unchanged, full grant and field comparison detects
    a changed body. A changed kind or occurrence under held identity refuses before
    replay. Distinct postcondition claims of one operation retain their own complete
    occurrences even if they sha
- id: FR-024-AC-51
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-24). `FrameReplayInputs` has no `obligation_identity`
    member. The frame request and envelope carry the same producer-issued typed identity
    from the selected frame claim; no caller stand-in, CG mint or hash reaches either.
    A changed grant that rekeys the producer frame node changes both identities; a
    grant/body mismatch under a held node is refused by the full record comparison.
    The state-clause path remains outside this criterion.
- id: FR-024-AC-7
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A reduced `Witness`-arm candidate is retained only when a backend re-run
    and native replay both preserve domain validity and the failure; a candidate that
    changes domain validity or the verdict is discarded.
- id: FR-024-AC-8
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness`-arm envelope carries its own backend re-run's
    transcript, never the original counterexample's.
- id: FR-022-AC-1
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Generation dispatches one arm per variant of the closed `BackendKind` through
    an exhaustive `match` with no catch-all, and `GenerationContexts::has` handles
    each variant; adding a variant without either arm does not compile. `Process`
    requires no context.
- id: FR-022-AC-20
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: '`RoutedGenerationItem.backend` and `.kind` are private while `request_index`
    and `node_id` remain public; external callers cannot create an item with a hand-built
    `Process(id)` or reassign its backend after `from_descriptor` construction.'
- id: FR-022-AC-26
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Given the checked QSL RR-16 package with two `x + 1` occurrences of one
    application node, directly narrowed to `Int[0, 10]` and `Int[0, 20]`, the driver
    supplies two distinct request indexes and occurrence keys. CG matches each to
    its own requirement record and selected narrow; both supported scalar outputs
    use the shared application operation and their respective result bounds, and their
    obligation and harness identities differ by application occurrence. Reversing
    routed-item order leaves these pairings unchanged.
- id: FR-022-AC-27
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'For `if p then x + 1 else x + 1`, each occurrence''s generated scalar
    claim is conditional on its own QSL path guard: the same guard occurrence with
    `holds=true` for the then item and `holds=false` for the else item. CG preserves
    the full outermost-first guard sequence and uses it when forming the proof obligation;
    a result under `p=false` cannot prove the then claim and one under `p=true` cannot
    prove the else claim. An empty path condition remains unguarded.'
- id: FR-022-AC-28
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: If a routed scalar item's occurrence has no entry in the supplied requirement
    records, its key's node disagrees with the authenticated application selection,
    or the keyed record disagrees with the item's result bound, direct conversion
    occurrence or ordered path condition, CG returns a typed refusal identifying the
    request index and occurrence before emitting an oracle, harness or obligation
    identity for that request; it does not use another record with the same node or
    reconstruct missing values from the IR graph or source text.
- id: FR-026-AC-1
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: PLANNED (IR-1316; IR-629). The Kani adapter implements the adapter trait,
    and a function generic over that trait reaches the Kani adapter's generation arm,
    execution, transcript parser and witness renderer through it.
- id: FR-026-AC-4
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: PLANNED (IR-1316; IR-629). The execution evidence type is an associated
    type of the adapter trait, and the Kani adapter's is `KaniExecutionEvidence`.
- id: FR-026-AC-5
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: PLANNED (IR-629). `Process(id)` has no CG adapter trait implementation
    or CG execution path; an inspection of all adapter implementations and execution
    dispatch arms finds no process arm.
- id: FR-002-AC-10
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause identity that is unknown, of the wrong
    clause kind, or the same for both selections yields a typed, located diagnostic
    naming the selection and no artifact.
- id: FR-002-AC-11
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause body outside the FR-014-AC-38 shape
    yields a structured unsupported diagnostic and no artifact, and the generator
    emits no filtering strategy for it.
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Repeated V2 harness generation for the same admitted
    package, selected checked clause ids, authentic `claim` occurrences, and campaign
    policy is byte-identical. The repeated selections include the same FR-011 identity
    inputs; the generated header, runtime identities, source map, symbols, and remaining
    bytes all agree across repetitions.
- id: FR-002-AC-13
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which both oracles complete true
    runs the subject once and is a pass.
- id: FR-002-AC-14
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    false is a rejected precondition and the subject is not invoked.
- id: FR-002-AC-15
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    true and the postcondition oracle completes false is a failed postcondition.
- id: FR-002-AC-16
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: 'PLANNED (IR-364). For each selected precondition and postcondition oracle,
    a seeded `Ok(Outcome::Undefined)`, `Ok(Outcome::Refused)`, `Ok(Outcome::Incomplete)`,
    `Err(ClauseStop::Oracle(OracleStop))`, and `Err(ClauseStop::InvalidConstant)`
    returns CG-owned `V2HarnessCaseResult::NonCompleted` retaining the exact typed
    stop cause, selected claim identity, and selected accepted typed `ExecutionPoint`.
    The generated campaign returns `V2HarnessCampaignResult::NonCompleted` carrying
    that case and a summary whose `attempted` includes this stopped case once and
    whose `non_completed` is one. No runtime '
- id: FR-002-AC-17
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not complete does not invoke the subject.
- id: FR-002-AC-18
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not complete does not evaluate the postcondition oracle.
- id: FR-002-AC-19
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Each V2 selection preserves the accepted QSL `ClauseBinding.execution_point`
    for its exact admitted package, checked clause id, and authentic `claim` occurrence
    through the injective runtime rendering; precondition and postcondition points
    may differ. Two requests selecting different authentic claims on the same precondition
    node each retain that node's accepted `Pre` operation point while selecting the
    correct occurrence. A rejection carries the precondition point; a failed or passed
    verdict after completed postcondition evaluation carries the postcondition point;
    a non-comp
- id: FR-002-AC-20
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). After FR-002-AC-10's clause id, kind, and distinctness
    checks, a wrong-role, wrong-ordinal, or other-node `claim` occurrence for either
    selected V2 clause yields `HarnessErrorCode::MissingClaimOccurrence` and no artifact
    even if a sibling occurrence has a point; an authentic selected tuple with no
    accepted point yields `HarnessErrorCode::MissingExecutionPoint` and no artifact
    even if another authentic occurrence of that node has a point; a render mutant
    that changes only that selection's generated harness or verdict point yields `HarnessErrorCode::ExecutionPointMismatch`
    befo
- id: FR-002-AC-21
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: RETIRED, replaced by FR-002-AC-25; the former digest-shaped identity rendering
    is obsolete and earns no implementation credit.
- id: FR-002-AC-22
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For either selection, an out-of-driver missing or non-claim
    occurrence returns `HarnessErrorCode::MissingClaimOccurrence`, a referenced model
    element absent from the admitted V2 relation returns `HarnessErrorCode::UnboundModelElement`
    naming it, and a process FR-331 owner differing from the admitted package returns
    `HarnessErrorCode::OwnerIdentityMismatch`. Each produces no artifact. A linked
    request uses one driver-exported/E4-emitted/E5-admitted package and a selected
    claim checked against that package; a foreign claim absent from it returns `HarnessErrorCode::MissingClaimO
- id: FR-002-AC-23
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: RETIRED, replaced by FR-002-AC-26; the former digest-shaped text control
    is obsolete and earns no implementation credit.
- id: FR-002-AC-24
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-364). With a supplied runtime report whose snapshot is `at_limit`
    before a seeded non-completed V2 oracle case, `V2HarnessCampaignResult::NonCompleted`
    retains that case and a summary with `non_completed == 1` and exact `attempted
    == None`; it does not infer an exact prior count from the saturated report. The
    report's existing counters and `at_limit` signal remain unchanged, and no runtime
    verdict is constructed or recorded for the stopped case.
- id: FR-002-AC-25
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815; replaces AC-21). The V2 harness header, `RequirementId`,
    `RevisionId`, source map and symbols carry FR-011's canonical structural `qocc2_`/`qpkg2_`
    renderings of each driver's selected claim's complete node, role and ordinal and
    the same admitted package's name and complete lock. Changing any one structural
    identity component changes its rendered text. An unchanged node and lock identity
    never excuses a changed admitted body from the existing body comparison.
- id: FR-002-AC-26
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815; replaces AC-23). At a V2 harness identity text comparison
    boundary, malformed, truncated, case-folded or otherwise noncanonical structural
    `qocc2_`/`qpkg2_` text, including an invalid byte count or malformed nested path
    or lock encoding, returns `HarnessErrorCode::InvalidIdentityRendering` before
    publication and without normalizing or mutating a report. A different but canonical
    occurrence returns `MissingClaimOccurrence`; a different but canonical package
    name or lock returns `OwnerIdentityMismatch`, likewise before publication. Exact
    typed and canonical-text comparisons agre
- id: FR-002-AC-7
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For one precondition and one postcondition selected from
    an admitted `CheckedPackageV2`, the generated harness embeds each selected clause's
    FR-014 V2 clause oracle.
- id: FR-002-AC-8
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). In the V2 harness, an instrumented snapshot, precondition
    oracle, subject and postcondition oracle record that the snapshot precedes precondition
    evaluation, which precedes subject invocation, which precedes postcondition evaluation,
    for every case.
- id: FR-002-AC-9
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 harness generator opens
    a `CheckedPackageV2` body only through the `core/ir/` layer of AD-004, and names
    no `BoundPackage`, `ClauseRef`, `quire_contract_model::TypedExpression` or `oracle::boolean_v1`
    item, each of which today's V1 harness names or uses.
- id: FR-008-AC-10
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: Source inspection shows the generator reads no `CheckedSemanticNodeV2.body`
    field and decodes no body JSON (the verification of FR-008-CON-1).
- id: FR-008-AC-7
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Beyond the per-operator agreement stated in FR-008-AC-3, which
    this criterion does not repeat, for each of integer.eq, ne, lt, le, gt and ge
    the selected claim's V2 comparison oracle returns, for every valuation of a 1-
    to 4-member domain (read-versus-read and read-versus-literal, the literal at each
    member and in either authored position), the result of an independently evaluated
    ordered relation. For every such valuation, the FR-014 clause oracle's result
    for the same direct comparison is identical to the selected oracle's result and
    to that independent relation.
- id: FR-008-AC-8
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. The V2 comparison oracle takes only the accessor's typed result
    for the selected claim. Source inspection finds exactly one `lower_typed_integer_comparison`
    function under `src/oracle` used by both the selected strategy oracle and FR-014's
    clause oracle for all six operators and the ordered direct read/read and read/literal
    forms; neither path has a second CG comparison lowerer or evaluator module, and
    no V1 bound oracle, `BoundPackage` or V1-to-V2 adapter is an input to the V2 path.
- id: FR-008-AC-9
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. A CG integration test calls IR's public clause-context accessor
    through its public API only, and the generator's census, ordered relation and
    diagnostics equal the accessor's operator, ordinals, provenance, observation,
    bounds and refusal fields for each closed operator, provenance and observation
    variant.
- id: FR-009-AC-7
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: For each noncommutative operator, a comparison written with the literal
    in the first position produces the same satisfying and violating sets as its mirrored
    read-first form, enumerated independently.
- id: FR-011-AC-10
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED. The QSL driver forwards only FR-331 items accepted by `export_bindings`;
    its `UnknownItem`, `Unbound`, and `Fault` results stay driver-side. An out-of-driver
    non-claim or absent occurrence returns CG `StrategyErrorCode::MissingClaimOccurrence`,
    and a selected claim with an unbound referenced model element returns `StrategyErrorCode::UnboundModelElement`
    naming it. Neither produces an artifact or verdict, and a matched sibling remains
    unaffected.
- id: FR-011-AC-11
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: RETIRED, replaced by FR-011-AC-21; the former digest-shaped text control
    is obsolete and earns no implementation credit.
- id: FR-011-AC-12
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: Repeated numeric runner generation with the same admitted package, selected
    driver-bound FR-331 claim, checked clause, and campaign policy yields byte-identical
    source and map.
- id: FR-011-AC-18
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1124; FR-004 live binding remains separate). A campaign run
    from the emitted V2 runner retains the selected claim and package identity with
    its own source/map and result for same-invocation producer binding. Published
    bytes or a detached/copied result alone yield no authenticated campaign, measured
    coverage or proof credit. A refused selection yields no campaign evidence, including
    when another claim of the same package succeeds.
- id: FR-011-AC-19
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-8). The numeric runner's `RequirementId`,
    `RevisionId`, generated header, source map and symbol suffix retain the selected
    driver-bound FR-331 claim's complete structural node, role and ordinal and the
    same-owner admitted package's complete name and lock through the canonical `qocc2_`/`qpkg2_`
    renderings. Changing any one admitted identity component changes the corresponding
    text; a body-only edit retaining the same structural node and lock leaves that
    identity text unchanged and is still compared through the admitted body.
- id: FR-011-AC-20
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-9). For a process request, a FR-331 request
    whose package name or complete lock differs from the separately admitted package
    returns `StrategyErrorCode::OwnerIdentityMismatch` before any bundle, verdict
    or report mutation, while a matched sibling retains its disposition. A package-byte/lock
    mismatch stays the FR-331 reader's typed refusal. For a linked request, E4/E5
    package disagreement stays the driver's handoff refusal; CG checks the selected
    claim against the one held E5-admitted package without a second owner field.
- id: FR-011-AC-21
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-11). Malformed, truncated, case-folded or
    otherwise noncanonical structural `qocc2_`/`qpkg2_` text, including a wrong byte
    count or malformed nested path or lock encoding, yields `StrategyErrorCode::InvalidIdentityRendering`
    before bundle publication or verdict/report mutation, without normalization. A
    different but canonical node, role or ordinal yields `MissingClaimOccurrence`;
    a different but canonical package name or lock yields `OwnerIdentityMismatch`,
    at the same no-publication boundary. Exact typed and canonical-text comparisons
    agree on the accepted pair.
- id: FR-011-AC-6
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: 'SUPERSEDED by FR-011-AC-14 under QSpec #449 / IR-1070; no implementation
    credit. Original planned text: For a selected checked clause id and authentic
    `claim` occurrence in one admitted V2 package, every constructed verdict carries
    the runtime point rendered from the accepted QSL `ClauseBinding.execution_point`
    of that exact tuple, with variant and anchor name preserved. Invariant fixtures
    at both initialization and handler anchors, and two distinct handler anchors,
    produce distinct runtime point strings; `Precondition` and `Postcondition` fixtures
    preserve their own binding points. Two authen'
- id: FR-011-AC-7
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: 'SUPERSEDED by FR-011-AC-15 under QSpec #449 / IR-1070; no implementation
    credit. Original planned text: An absent checked clause maps Contract IR FR-058
    `UnknownClause` to `StrategyErrorCode::UnknownClause`; a non-state-clause maps
    `NotStateClause` to `StrategyErrorCode::UnsupportedClauseKind`, before occurrence
    checks and with no artifact. A wrong-role, wrong-ordinal, or other-node `claim`
    occurrence returns `StrategyErrorCode::MissingClaimOccurrence` and no artifact,
    even when another authentic occurrence of the selected node has a point. An authentic
    selected tuple lacking an accepted bindi'
- id: FR-011-AC-8
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: RETIRED, replaced by FR-011-AC-19; the former digest-shaped rendering is
    obsolete and earns no implementation credit.
- id: FR-011-AC-9
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: RETIRED, replaced by FR-011-AC-20; the former domain/algorithm/digest owner
    comparison is obsolete and earns no implementation credit.
```

## Method selection

Code-review includes Rust; separate gap-analysis, spec base, integrity and evidence artifacts were produced. EARS is skipped because no behavioral requirement statement changed; dependency because no relationship edge changed; object, scope-boundary, risk-complexity and failure-domain as standalone methods because no object, allocation, architecture or failure behavior was introduced. Integrity included the failure-domain probes. Criterion-strength/Jev is not needed for unchanged AC semantics; no calibrated strength claim is made. No sub-reviewer was dispatched.
