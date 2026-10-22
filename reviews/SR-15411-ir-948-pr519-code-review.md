---
id: SR-15411
title: "IR-948 PR 519 V2 invariant code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@c9b9fd3e2d4e29e9701cf9c1968c857418fe4cf7; src/kani/generate/clause/v2.rs; src/kani/generate/negotiate.rs; src/kani/identity.rs; tests/it/kani_obligations_state_frame.rs; tests/state_frame_support/native_twin.rs; FR-015-AC-38,39,40,46,49; TC-025"
review_set: subset
---

## Summary

Ticket: IR-948. Diff-scoped code and Rust review of CG PR 519 at exact head c9b9fd3e2d4e29e9701cf9c1968c857418fe4cf7 on base 5c19d08a. The V2 invariant branch draws the oracle's bounded inputs, asserts its completed true result, and emits its final cover without a customer subject call. No CI, manifest, spec, or plan file changed.

## Verdict

**CONDITIONAL** — the new criterion tests use only `TC-025` trace tags, so the declared FR-015 invariant criteria remain unbound in the computed matrix. Add the specific FR-015 criterion ids to each new test's existing `#[trace]` attribute and recheck the matrix. The Rust implementation otherwise has no finding in the examined diff; final CI and real Kani remain with the author.

## Coverage

- FR-015-AC-38 (examined): the admitted clause node chooses invariant kind; the emitted harness has a distinct invariant symbol.
- FR-015-AC-39 (examined): each selected model field read by the clause gets one inclusive symbolic domain assumption.
- FR-015-AC-40 (examined): the source embeds the clause oracle returned for the same package and node.
- FR-015-AC-46 (examined): the renderer puts one cover after the oracle assertion.
- FR-015-AC-49 (examined): invariant asserts the clause without calling the customer subject.
- TC-025 (examined): two new default tests and one new ignored real Kani test exercise the invariant path; the existing precondition Kani test is retained through a shared runner.
- Rust checklist: checked i64 domain conversions, exhaustive `ObligationKind` matching, generated identifier and source limits, no added production panic or unsafe, and no changed CI lane were examined.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | New invariant tests cite only `TC-025`; no test binds FR-015-AC-49 or the other criterion claims in the computed matrix. A criterion-level gate can therefore report the invariant untested despite the new behavioral checks. | tests/it/kani_obligations_state_frame.rs:447 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Exact head bee0f85c binds the new invariant tests to FR-015-AC-38, 39, 40, 46, and 49; an independent computed matrix lists one or more binders for each. |

## Disposition verdict

**PASS** at exact head bee0f85c4c2de810a7571a13f50e6b0bab344b49 for the former trace finding. The changed source is confined to test traces, a negative test, and its QSL twin.
