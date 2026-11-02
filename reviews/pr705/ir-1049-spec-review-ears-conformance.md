---
id: SR-202610310176
title: spec-review/ears-conformance review of IR-1049 PR705
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-041-claimed-module-proof-gate.md;
  spec/core/functional/interface-001-codegen-api.md; spec/kani/matrix/TC-056-claimed-module-proof-gate.md
review_set: subset
---

## Summary

Frozen source 4b72270e503147cb2854ee2ea0cacd93d838e917 against base 8cfe80927fb9a2f00ae8a335aef450d55b039387; PR quire-contract-codegen#705; ticket IR-1049. Spec-only review: no implementation, proof, mutation or ticket-completion credit. No source edits, Cargo or Kani runs. No applicable AssuranceProfile exists in spec/; direct scoped review applies. The dispatch explicitly selected applicable analyses. All six protected rows match the supplied baseline verbatim.

## Analysis

Engine validation of the three explicit changed paths reports 3/3 documents grammar-clean and zero grammar findings. Semantic inspection of the new SHALL statements finds named actors and observable responses: borrowed canonical input, exact Boolean profile, typed refusal, actual subject call, independent expectation, semantic identity and qualification controls. The word support is followed by an exact finite profile rather than an unmeasurable capability. No event/state trigger changes, missing actors or ambiguous failure responses were found.

## Examined scope

```yaml
scope:
- id: FR-041-AC-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A nonempty set of current-run generated module claims is checked against the current compiled-module
    census. An absent compiled module or empty set names the claim or condition and fails. Open pending a genuine
    current-build claim source. The former merge-base declaration comparison is retired by IR-1105.
- id: FR-041-AC-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside only the first and an UNREACHABLE
    check inside the second reports the second `unreached` and fails. A SUCCESS check inside each passes this census;
    a SATISFIED cover, unknown location or non-SUCCESS status cannot supply the missing check.
- id: FR-041-AC-3
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The current build's transitive expectation/subject reachability yields the shared-helper intersection;
    the gate fails when the checked-in generated evidence omits or adds a helper, and records an explicit empty
    set when none is shared. For an exact operation, the expectation uses QSpec vectors or a model with no `quire-exact`
    call.
- id: FR-041-AC-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Each claimed module and computed shared helper has a positive unmodified real-Kani proof and a source
    mutation that compiles, executes, and reports a failed proposition check. Deleting a control or making its mutant
    verify, fail to compile, time out, skip execution, or merely trip the dirty-tree guard fails the gate; evidence
    names the exact diff, command and failed check from this run.
- id: FR-041-AC-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A stubbed executor, predetermined outcome or build-only run produces no qualifying module evidence and
    cannot pass the gate.
- id: FR-041-AC-10
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A production V2 module implements the Boolean state-transition obligation originally described by FR-003@4,
    with precondition `input_current \|\| state_pre`, postcondition `state_post`, the actual transition subject,
    authentic checked-package/source provenance and a separately named independent expectation.
- id: FR-041-AC-10
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The new claim has its own stable V2 module and obligation identities and is registered in the current
    compiled census without restoring a V1 row or asserting V1 identity continuity.
- id: FR-041-AC-10
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Installed Kani gives that module an in-module SUCCESS proposition and satisfied cover; its in-module
    source mutant compiles, executes and reports an in-module FAILURE check with in-domain playback, with per-helper
    controls under AC-3 and AC-4.
- id: FR-041-AC-10
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A different obligation, V2 implication proof, absent current V2 claim or saved verdict does not discharge
    it.
- id: FR-041-AC-10
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: PLANNED (IR-1049), dependent on the IR-1047 machinery.
- id: FR-041:profile-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The `StateTransition` item of the existing `KaniObligationRequest` SHALL borrow one admitted `CheckedPackageV2`,
    a selected `state`/`transition` node identity, and the Rust path of its actual transition subject.
- id: FR-041:profile-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: '`negotiate_kani_obligations` SHALL remain the single item-settlement entry.'
- id: FR-041:profile-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The producer SHALL consume the public typed term and operation views of the admitted package, including
    the canonical `anchor`, `subject`, ordered `inputs`, optional `pre`, and `post` bindings specified by QSpec
    FR-341 and IR FR-067.
- id: FR-041:profile-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: It SHALL introduce no new wire carrier, mirrored transition schema, or independently inferred model signature.
- id: FR-041:profile-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: 'This delivery SHALL support the original Boolean contract profile: exactly one ordered Boolean operation
    input, the selected required Boolean model-field subject, a precondition equivalent by operand order to `input_current
    || state_pre`, and a postcondition reading that subject as `state_post`.'
- id: FR-041:profile-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Field comparisons SHALL use the canonical declaration identity, field name and receiver binding, rather
    than a field name alone.
- id: FR-041:profile-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A different transition profile SHALL receive a located typed unsupported disposition with no harness.
- id: FR-041:profile-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: An invalid request SHALL follow FR-015's existing no-partial-output rule.
- id: FR-041:profile-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Neither an integer frame nor an implication scalar claim supplies this profile.
- id: FR-041:profile-3
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: 'The caller''s subject ABI SHALL be `fn(input_current: bool, state: &mut bool)`: the mutable slot is
    the transition body''s selected Boolean field.'
- id: FR-041:profile-3
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The generated named `subject_boolean_state_transition` root SHALL snapshot that field's pre-state, call
    the actual nominated subject with the admitted input binding, and return its post-state Boolean value.
- id: FR-041:profile-3
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The proof SHALL draw the input and pre-state as two independent symbolic Booleans, assume the admitted
    precondition, call that root, and assert the admitted postcondition.
- id: FR-041:profile-3
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Its execution identity and evidence SHALL describe Boolean domains, never integer ranges substituted
    for Boolean types.
- id: FR-041:profile-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A separately named `expected_boolean_state_transition(input_current, state_pre)` root SHALL compute `input_current
    || state_pre` directly as the checked-in specification model.
- id: FR-041:profile-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The proof SHALL also compare the actual post-state with this model and retain one final non-vacuity cover.
- id: FR-041:profile-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Under the required precondition the model's result is true; the claim makes no assertion about behavior
    outside that precondition.
- id: FR-041:profile-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The expectation SHALL call no subject, oracle or `quire-exact` operation.
- id: FR-041:profile-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Both roots and the actual subject call SHALL be visible in the healthy current compiled call graph, with
    the ordinary exact shared-helper census and controls of AC-3 and AC-4.
- id: FR-041:profile-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: '`KaniObligationOutcome::Emitted` SHALL carry the generated `KaniStateTransitionHarness` values in request
    order in `state_transition_harnesses`.'
- id: FR-041:profile-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: '`KaniExecutableHarness::StateTransition` SHALL execute that same generated artifact through the existing
    bounded executor.'
- id: FR-041:profile-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The new `StateTransitionIdentity` SHALL retain the selected transition and actual subject binding, the
    canonical `CheckedOccurrence` and `CheckedSourceMapEntry` values for the selected transition, and identity material
    covering the complete selected semantic body, its pre/post dependency meaning, ordered input/subject bindings
    and selected profile.
- id: FR-041:profile-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A valid body edit retaining a structural node key SHALL change the identity material and generated module/obligation
    identities.
- id: FR-041:profile-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Identical admitted inputs and configuration SHALL reproduce the same identities and bytes.
- id: FR-041:profile-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: These are new V2 identities; no V1 row is restored or replaced.
- id: FR-041:profile-6
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: 'Qualification SHALL run four small installed-Kani controls over genuinely QSL-emitted, IR-admitted inputs:
    healthy subject, an in-module forced-false post-state, an in-module suppressed transition call, and an in-module
    removed precondition assumption.'
- id: FR-041:profile-6
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Healthy code SHALL produce a located in-module SUCCESS proposition and satisfied final cover.
- id: FR-041:profile-6
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Each mutant SHALL compile, execute, and produce a located FAILURE of the claimed postcondition with an
    in-domain Boolean playback.
- id: FR-041:profile-6
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: In particular, suppression distinguishes a real state update from a pre-state read, and removing the
    assumption exposes the false-input/false-pre-state case.
- id: FR-041:profile-6
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: These controls SHALL retain their exact source diffs, invocations and checks.
- id: FR-041:profile-6
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The required existing `make kani-gate` claim control SHALL include this new claim only with the full
    current compiled census and all its live module/helper controls; no optional lane or saved verdict supplies
    qualification.
- id: interface-001:108
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'planned_state_transition_item:'
- id: interface-001:109
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'kind: StateTransition'
- id: interface-001:110
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'inputs: [borrowed admitted CheckedPackageV2, selected state/transition CheckedNodeId, actual Rust subject
    path]'
- id: interface-001:111
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'subject_abi: "fn(input_current: bool, state: &mut bool)"'
- id: interface-001:112
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'profile: one ordered Boolean input; canonical selected Boolean field; precondition input_current OR
    state_pre; postcondition state_post; unsupported profiles have a located typed disposition without a harness'
- id: interface-001:113
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'output: KaniStateTransitionHarness in KaniObligationOutcome::Emitted.state_transition_harnesses, with
    StateTransitionIdentity'
- id: interface-001:114
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'identity: complete admitted transition/pre/post semantic material, selected profile, actual subject
    binding and canonical claim/source provenance; new stable V2 module and obligation identities'
- id: interface-001:115
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'execution: KaniExecutableHarness::StateTransition through the existing bounded executor, with Boolean
    symbolic domains'
- id: interface-001:116
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'semantics: planned IR-1049 producer under FR-041-AC-10; typed canonical carrier only; one existing negotiation
    entry; independent named specification model and current compiled census/module/helper controls; no V1 identity
    continuity'
- id: interface-001:117
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'semantics: settles every item in request order and, only when no item is invalid, emits one harness
    per supported item; an invalid item returns no harness bytes (FR-015); planned: an item is also a V2 clause
    claim over a precondition, postcondition or invariant node of an admitted CheckedPackageV2, with an optional
    declared census (FR-015-AC-38 to FR-015-AC-49); an item is also a StateFrame item (IR-461; FR-015-AC-60 and
    FR-015-AC-68 stay planned: the comparison against generate_state_frame_role is not testable from tests/it (crate-private)
    and no V2 package builder exists under'
- id: interface-001:117
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: src/; byte identity holds by construction; the tests compare against the public entry for a clause whose
    roles both succeed; one state_clause postcondition node, a contract or frame role, the state struct path and
    fields and the item's own subject path), recorded with the existing ObligationDisposition and reasons plus StateFrameRefused,
    InvalidStatePath, InvalidStateField, InvalidStateUnwind and MixedStatePackages, by one total mapping from StateFrameRefusal,
    the two roles settling independently, and its harnesses returned in the state_frame_harnesses field of
- id: interface-001:117
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: KaniObligationOutcome::Emitted (FR-015-AC-59 to FR-015-AC-68); planned IR-268 collection and graph items
    take cardinality, integer and Boolean element, and reference-field multiplicity domains from admitted V2 model
    declarations and typed IR bounds; graph reachability requires a positive-length path even when start equals
    target, and finite model-object population still needs a typed Contract IR input; absent or unbounded domains
    settle by typed disposition with no harness, and FR-017 execution supplies proof verdicts (FR-015-AC-96 to FR-015-AC-102)
- id: interface-001:118
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: '- name: generate_state_frame_obligations'
- id: interface-001:127
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'inputs: [KaniExecutionRequest, whose harness is a KaniExecutableHarness (planned StateTransition arm
    under FR-041-AC-10); KaniExecutionRequest::home_directory, the optional home directory of the launcher''s environment,
    used only to redact NoVerdict diagnostics and as a grouping key (FR-017-AC-35, FR-017-AC-36)]'
- id: TC-056:transition-step-1
  path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md
  role: examined
  excerpt: 1. Compile the QSL-owned Switch model/source inputs by reference, adding an authored unpointed precondition
    `flag or self.on` to the current source. Read the fresh package through public IR admission and inspect its
    canonical five transition bindings, Boolean field/input/result and all claim/source regions. Select that actual
    transition in the production Kani request.
- id: TC-056:transition-step-2
  path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md
  role: examined
  excerpt: 2. Require the new state-transition harness/identity arm, Boolean argument domains and a real nominated
    `fn(bool, &mut bool)` subject. Check repeatability, source provenance retention, and a valid semantic-body edit
    that retains a structural key but changes the generated identity. Refuse foreign/missing selections and unsupported
    profiles by typed disposition without artifacts.
- id: TC-056:transition-step-3
  path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md
  role: examined
  excerpt: 3. Compile the generated module and actual subject. Inspect the named subject and independent `input_current
    || state_pre` expectation roots, exact compiled module census and transitive helper intersection. The field-transition
    claim is distinct from the V2 implication claim and has its own identity.
- id: TC-056:transition-step-4
  path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md
  role: examined
  excerpt: '4. Run the healthy subject and three distinct in-module source mutants: forced false post-state, suppressed
    transition call, and removed precondition assumption. Require healthy SUCCESS plus a satisfied cover, and a
    compiling, executed postcondition FAILURE with Boolean playback for every mutant. Check the false-input/false-pre-state
    witness for the missing assumption and the true-input/false-pre-state witness for suppression. Retain exact
    diffs, invocations and check locations. Run every computed shared helper''s controls.'
- id: TC-056:transition-step-5
  path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md
  role: examined
  excerpt: 5. Register the newly qualified module in the mandatory current-build claim control. Run the ordinary
    missing/uncompiled/unreached/empty/missing-control adverse cases, retaining FR-041-AC-1 through AC-5 and NFR-006-AC-19
    verbatim. An unsupported source, absent source module or saved verdict supplies no state-transition proof credit.
- id: NFR-006-AC-19
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: On a clean current candidate, `make kani-gate` runs FR-041's current-build module census and real mutation
    controls in its one required lane, and a failure of any claim or shared-helper control makes the gate exit non-zero.
    An optional mutation target, retired fixed inventory or saved earlier result cannot substitute. Open until a
    genuine current-build claim source exists.
- id: FR-041:relationship
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: ix://agent-ix/quire-specification/FR-341
- id: FR-041:relationship
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: ix://agent-ix/quire-contract-ir/FR-067
- id: FR-041:relationship
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: ix://agent-ix/quire-spec-language/FR-105
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the scoped specification change. Engine validation of the three explicit changed paths reports 3/3 documents grammar-clean and zero grammar findings. Semantic inspection of the new SHALL statements finds named actors and observable responses: borrowed canonical input, exact Boolean profile, typed refusal, actual subject call, independent expectation, semantic identity and qualification controls. The word support is followed by an exact finite profile rather than an unmeasurable capability. No event/state trigger changes, missing actors or ambiguous failure responses were found.

No Rust, React or implementation gap-analysis applies to the docs-only diff. Risk/complexity, security, architecture, correctness/fuzz and criterion-strength analyses were not selected: no changed criticality, security boundary, architecture quality target, executable test implementation or AC text. Object review is not separately applicable: no domain-object schema is introduced; canonical carrier ownership is covered by scope/dependency analysis.
