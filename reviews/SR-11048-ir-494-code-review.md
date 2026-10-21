---
id: SR-11048
title: "IR-494 PR 381 current-head code-review"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen; IR-494; PR #381 head fd46534c04b697c53f3b282fe6cc0435507d5e35; changed caller oracle, Kani generation/execution, scalar mapping, tests and gate script"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-035
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-050
    type: references
---

## Summary

Ticket: IR-494. Review of PR #381 at fd46534c04b697c53f3b282fe6cc0435507d5e35 against current origin/main b88fa7fd47ac337ae4c8438337a0182938d5ccf1. The Rust and code-review lanes checked the full changed production/test diff, the prior SR-9002 findings, and the main merge delta. SR-9002 FND-001 remains fixed: the proof source embeds the public generated oracle. SR-9002 FND-002 remains fixed: the current test asserts no-cover inconclusive evidence and separate mixed-batch caller claims, paths, and playback. No new code defect was found.

## Examined Units

- FR-035-AC-1 (examined) — spec/oracle/functional/FR-035-caller-text-admission.md: A generated caller-ingress oracle accepts `&TextPayload`, the declared `TextType` and `&mut Meter`, calls `rt::admit_text` once, and returns the runtime outcome, retained text, original runtime provenance, charge sequence and consumed counters unchanged for all six profiles over representative empty, ASCII, combining, composed and supplementary payloads.
- FR-035-AC-2 (examined) — spec/oracle/functional/FR-035-caller-text-admission.md: Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8` before admission and leave the admission meter unchanged; `admit_text` itself is never claimed to accept those bytes.
- FR-035-AC-3 (examined) — spec/oracle/functional/FR-035-caller-text-admission.md: For each profile, `Text[1,1]` admits or refuses according to the profile length of the retained sequence, including `e` plus U+0301 under NFC and NFD; a length refusal occurs before `text.result-retain`, and denial of each reached charge point yields the direct runtime stop, charges and consumed counters.
- FR-035-AC-4 (examined) — spec/oracle/functional/FR-035-caller-text-admission.md: For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness symbolically ranges over all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime call including outcome and metering, and ends with one satisfied cover after its assertions. The proof identity names this finite class and its type.
- FR-035-AC-5 (examined) — spec/oracle/functional/FR-035-caller-text-admission.md: In a copy of the generated oracle with its admission call or declared `TextType` altered so the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement assertion with concrete playback; restoring the call verifies over the same symbolic class.
- FR-035-AC-6 (examined) — spec/oracle/functional/FR-035-caller-text-admission.md: A request outside the declared finite proof class has a typed unsupported or requires-bound disposition and no harness, and no caller-ingress claim is marked `ir_confirmed` or assigned a checked-package node id. The six attempted text-operand `numeric.convert` nodes remain Contract IR refusals and do not count as passing caller-admission coverage.
- FR-017-AC-26 (examined) — spec/kani/functional/FR-017-kani-execution-evidence.md: A generated FR-035 caller Text-admission harness enters the same production execution path in single and compatible batch runs; its evidence carries the exact typed caller identity (profile, bounds and finite payload class) supplied with that harness and `None` for contract obligation kind. A matching successful report with a satisfied cover and successful agreement assertion classifies `Verified`; an unsatisfied cover classifies `CoverUnsatisfied` with counts; a failed agreement assertion with its own concrete playback classifies `Falsified` with that playback. A source-mismatched harness, mi
- TC-050 (examined) — spec/oracle/matrix/TC-050-caller-text-admission.md: Verify the caller Text admission oracle, its constructor boundary, runtime accounting agreement and finite real-Kani claim separately from checked-package text comparison and the six refused text-operand conversion nodes.

- TC-024 (context_only) — spec/oracle/matrix/TC-024-exact-scalar-oracles.md: Verify that scalar oracles generated from an admitted CheckedPackage V2 cover every scalar family, refuse every non-generated item with a typed reason, are byte-deterministic, and agree with the runtime and the QSL value authority.

## Verdict

**PASS** for the changed code-review scope. The generated oracle feeds the proof harness; the caller executor keeps typed identity through classification. The added tests cover the formerly missing negative outcomes. The real Kani proof remains unrun and this review makes no proof-success claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Diff-scoped Rust idiom, panic/resource, test-intent, code-test alignment, vendoring/duplication, and gate-scope inspection. CI workflow files are unchanged. `cargo test --locked --lib caller_text` passed 10/10; `cargo test --locked --test it tc_050_caller_executor` passed 2/2; `cargo fmt --check` passed; `git diff --check` passed. No heavy Kani or aggregate gate was run.
