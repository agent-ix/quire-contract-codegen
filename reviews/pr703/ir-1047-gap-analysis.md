---
id: SR-202610310166
title: IR-1047 PR703 gap-analysis
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-codegen; scripts/cg_gates.rs, src/kani/generate/render.rs,
  tests/it/kani_gate.rs, tests/it/kani_generation.rs, tests/it/kani_generation/claims.rs,
  tests/it/kani_generation/claims/census.rs, tests/it/kani_obligations.rs
review_set: subset
---

## Summary

Reviewed frozen head 53a457260259ef9b29312d9772bfd87957782f4c against base 1b28e456bf641e04358d4e35c6126df8517e15a0 for IR-1047 / CG #703. Scope is FR-041 AC1–9 and NFR-006 AC19; AC10 is the separately planned IR-1049 transition obligation. No V1 identity continuity or FR-014 relabel is credited.

## Verdict

PASS — no defects found in the scoped review. Merge remains subject to the outstanding ordinary integration suite and parent-owned aggregate gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Supplied current-run logs inspected: /tmp/titan-ir1047-focused5.log (19 passed, 2 explicitly ignored lane tests), /tmp/titan-ir1047-clippy-final6.log and /tmp/titan-ir1047-fmt-final6.log (passed), /tmp/titan-ir1047-real-controls4.log (installed two-module control passed). Inspected qualified-current-claims receipts under /home/peter/dev/qualification-logs/titan-ir1047/: compiled metadata, graph, symbol census, exact source and manifest/lock, real invocations, located checks, both subject-only unified diffs and Boolean playback. Both positives verified and both separate mutants falsified the independent assertion with false-left playback. The helper intersection is explicitly empty; synthetic transitive-graph controls check nonempty set mismatch/missing-control rejection. No Cargo command was run by this reviewer, as dispatched. Full ordinary serial integration tests and the final aggregate gate remain the parent lead’s pre-merge responsibility; this review does not attest those unrun gates.

Plan completion: not assessed

Computed the repository-wide matrix using `quire matrix --scope . --format json`; examined scoped binders and the source they exercise. All nine FR-041 delivery criteria and NFR-006 AC19 have trace binders. The required real proof binder is ignored in the default lane by design and explicitly selected and guarded in the installed-Kani lane. The repository matrix also contains existing unrelated gaps (328 untagged criteria, including explicitly deferred FR-041 AC10; 12 tagged only by ignored tests); this is a subset verdict, not a repository-wide completeness claim. Reverse-gap and stub checks on all seven changed files found no unowned behavior, placeholder implementation or hollow positive proof. Optional standalone semantic gap analysis was not run; production/test faithfulness was checked under the mandatory code-review method.

## Scope checked

```yaml
scope:
- id: FR-041-AC-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A nonempty set of current-run generated module claims is checked against
    the current compiled-module census. An absent compiled module or empty set names
    the claim or condition and fails. Open pending a genuine current-build claim source.
    The former merge-base declaration comparison is retired by IR-1105.
- id: FR-041-AC-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside
    only the first and an UNREACHABLE check inside the second reports the second `unreached`
    and fails. A SUCCESS check inside each passes this census; a SATISFIED cover,
    unknown location or non-SUCCESS status cannot supply the missing check.
- id: FR-041-AC-3
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The current build's transitive expectation/subject reachability yields
    the shared-helper intersection; the gate fails when the checked-in generated evidence
    omits or adds a helper, and records an explicit empty set when none is shared.
    For an exact operation, the expectation uses QSpec vectors or a model with no
    `quire-exact` call.
- id: FR-041-AC-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Each claimed module and computed shared helper has a positive unmodified
    real-Kani proof and a source mutation that compiles, executes, and reports a failed
    proposition check. Deleting a control or making its mutant verify, fail to compile,
    time out, skip execution, or merely trip the dirty-tree guard fails the gate;
    evidence names the exact diff, command and failed check from this run.
- id: FR-041-AC-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A stubbed executor, predetermined outcome or build-only run produces no
    qualifying module evidence and cannot pass the gate.
- id: FR-041-AC-6
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The V2 claim is produced by the admitted V2 Boolean implication request
    through the production scalar renderer; its generated module and harness identities
    match the current-build claim source and compiled census. A V1 source, hand-authored
    module, missing module, changed identity or uncompiled claim is rejected by name.
    Open pending a genuine current-build claim source.
- id: FR-041-AC-7
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The V2 module exposes separate named subject and expectation roots in the
    current compiled call graph. The subject reaches the runtime implication operation;
    the expectation evaluates `!left
- id: FR-041-AC-8
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The unmodified generated V2 module has a location-attributed SUCCESS proposition
    and satisfied final cover with installed Kani. Replacing only its parsed subject-call
    expression with `false` compiles and executes the same harness, then yields an
    in-module FAILURE at the subject-versus-expectation assertion with in-domain playback
    where `left` is false; each computed shared helper has its own positive and falsifying
    control.
- id: FR-041-AC-9
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: After AC-6 through AC-8 pass in the current lane, the V2 implication module
    is included in the current-build claim census. The required `make kani-gate` run
    executes its real-Kani control and fails on any missing, skipped or failing claim
    or helper control. This completes the IR-1047 machinery delivery independently
    of the new Boolean state-transition claim in AC-10; implication proof credit does
    not discharge that transition obligation. Open pending a genuine current-build
    claim source.
- id: NFR-006-AC-19
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: On a clean current candidate, `make kani-gate` runs FR-041's current-build
    module census and real mutation controls in its one required lane, and a failure
    of any claim or shared-helper control makes the gate exit non-zero. An optional
    mutation target, retired fixed inventory or saved earlier result cannot substitute.
    Open until a genuine current-build claim source exists.
- id: scripts/cg_gates.rs
  path: scripts/cg_gates.rs
  role: examined
  excerpt: //! Host controls for the Kani lane and QSpec fixture preflight.
- id: src/kani/generate/render.rs
  path: src/kani/generate/render.rs
  role: examined
  excerpt: //! Pure ordered emission of validated harness descriptions (FR-015).
- id: tests/it/kani_gate.rs
  path: tests/it/kani_gate.rs
  role: examined
  excerpt: //! Contract controls for the real-Kani lane gate (NFR-006, TC-045).
- id: tests/it/kani_generation.rs
  path: tests/it/kani_generation.rs
  role: examined
  excerpt: //! Surviving admitted V2 generation controls in the real Kani lane.
- id: tests/it/kani_generation/claims.rs
  path: tests/it/kani_generation/claims.rs
  role: examined
  excerpt: //! Current generated claims are qualified from compiled artifacts, never
    a saved verdict.
- id: tests/it/kani_generation/claims/census.rs
  path: tests/it/kani_generation/claims/census.rs
  role: examined
  excerpt: //! Fail-closed attribution and reachability for current compiled claims.
- id: tests/it/kani_obligations.rs
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: //! FR-015 separate bounded Kani obligations, FR-017 Kani execution, and
    Contract IR FR-036
- id: TC-056
  path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md
  role: examined
  excerpt: Verify that the existing real-Kani lane accepts a declared module claim
    only when the current build and transcript locate a discharged proposition there
    and an injected mutation actually falsifies it.
bindings:
- test_id: a_generated_source_or_saved_verdict_without_this_build_does_not_qualify
  ac_id: FR-041-AC-1
  trace: correct
- test_id: current_generated_claim_has_fresh_compiled_roots_and_falsifying_control
  ac_id: FR-041-AC-1
  trace: correct
- test_id: assembled_current_modules_preserve_production_items_and_mutate_only_one_subject
  ac_id: FR-041-AC-2
  trace: correct
- test_id: tests::two_modules_in_one_file_need_separate_located_successes
  ac_id: FR-041-AC-2
  trace: correct
- test_id: current_generated_claim_has_fresh_compiled_roots_and_falsifying_control
  ac_id: FR-041-AC-3
  trace: correct
- test_id: tests::transitive_intersection_requires_exact_evidence_and_each_helper_control
  ac_id: FR-041-AC-3
  trace: correct
- test_id: current_generated_claim_has_fresh_compiled_roots_and_falsifying_control
  ac_id: FR-041-AC-4
  trace: correct
- test_id: tests::transitive_intersection_requires_exact_evidence_and_each_helper_control
  ac_id: FR-041-AC-4
  trace: correct
- test_id: a_generated_source_or_saved_verdict_without_this_build_does_not_qualify
  ac_id: FR-041-AC-5
  trace: correct
- test_id: current_generated_claim_has_fresh_compiled_roots_and_falsifying_control
  ac_id: FR-041-AC-5
  trace: correct
- test_id: current_claim_comes_from_authored_admitted_v2_implication
  ac_id: FR-041-AC-6
  trace: correct
- test_id: a_generated_source_or_saved_verdict_without_this_build_does_not_qualify
  ac_id: FR-041-AC-6
  trace: correct
- test_id: assembled_current_modules_preserve_production_items_and_mutate_only_one_subject
  ac_id: FR-041-AC-6
  trace: correct
- test_id: current_generated_claim_has_fresh_compiled_roots_and_falsifying_control
  ac_id: FR-041-AC-6
  trace: correct
- test_id: current_claim_comes_from_authored_admitted_v2_implication
  ac_id: FR-041-AC-7
  trace: correct
- test_id: current_generated_claim_has_fresh_compiled_roots_and_falsifying_control
  ac_id: FR-041-AC-7
  trace: correct
- test_id: assembled_current_modules_preserve_production_items_and_mutate_only_one_subject
  ac_id: FR-041-AC-8
  trace: correct
- test_id: current_generated_claim_has_fresh_compiled_roots_and_falsifying_control
  ac_id: FR-041-AC-8
  trace: correct
- test_id: kani_gate_refuses_a_deleted_or_unselected_current_claim_control
  ac_id: FR-041-AC-9
  trace: correct
- test_id: current_generated_claim_has_fresh_compiled_roots_and_falsifying_control
  ac_id: FR-041-AC-9
  trace: correct
- test_id: kani_gate_refuses_a_deleted_or_unselected_current_claim_control
  ac_id: NFR-006-AC-19
  trace: correct
- test_id: current_generated_claim_has_fresh_compiled_roots_and_falsifying_control
  ac_id: NFR-006-AC-19
  trace: correct
```
