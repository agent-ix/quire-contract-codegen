---
id: SR-8660
title: "IR-597 typed operator catalog code and Rust review"
type: SpecReview
analysis: code-review
scope: "quire-contract-codegen@39c4df68d9eb8626d1927c502772d76360c34a65 vs f1e7ec610ec4450b9eb77b2ff08626dc4a905c53; src/operator_catalog.rs, src/lib.rs, src/oracle/scalar/mod.rs, src/kani/generate/scalar.rs, src/kani/generate/frame.rs, tests/it/exact_scalar_generation.rs, tests/it/kani_obligations.rs; FR-038, FR-014, FR-015, TC-024, TC-025"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-038", type: references }
---

## Summary

Reviewed IR-597 commit `39c4df68d9eb8626d1927c502772d76360c34a65` against its parent, including the changed Rust production and test code, repository conventions, and FR-038/FR-014/FR-015 alignment. The catalog consolidates the selected V2 operator metadata, but the original commit had an ineffective identity assertion and two broken integration controls.

## Verdict

**FAIL at the originally reviewed commit** — the new test target had a type mismatch and the catalog identity check was self-referential; a separate fixture-ID mismatch also made the wrong conversion-target control assert against provisional identities.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Catalog invariant test compares `entry.identity` to itself instead of an expected identity, so an operator mapped to another operator's identity still passes this identity check | src/operator_catalog.rs:301-319, FR-038-AC-1 |
| FND-002 | high | The new wrong-arity assertion supplies `&str` where `FormMismatch.found` is `String`, so the integration test target fails to compile | tests/it/exact_scalar_generation.rs:1092-1097 |
| FND-003 | high | Wrong-conversion-target control uses a provisional item ID and expects the unrelated `DEC` bound (-1,000..1,000), while fixture node 1086 carries a distinct decimal bound (-100,000..100,000), so the negative control does not assert the admitted node and actual bound identities | tests/it/exact_scalar_generation.rs:1059-1135 |

## Dispositions

| ID | Outcome | Evidence |
| --- | --- | --- |
| FND-002 | fixed fd8f9510a5b8e6a6f42559c1f7eb296391140928 | `found: "binary".to_owned()` now matches the `String` field type at `tests/it/exact_scalar_generation.rs:1096`. No Cargo command was run by this reviewer. |
| FND-001 | fixed 311718ab03f370b4e3ffb1253ac443dd8167af1f | Each catalog expectation now includes its literal identity and the assertion compares `entry.identity` to `expected.0` at `src/operator_catalog.rs:309-320`. |
| FND-003 | fixed 43af85923ec459bc56530f6907f8131b5ae08b7d | Correction to the prior disposition: the 311718 change resolved IDs but still expected unrelated `DEC` (-1,000..1,000). The current test resolves the actual node-1086 bound `Bound::Decimal(-100_000, 100_000, 0, 2, "nearest-even")` and the node identity (`tests/exact_scalar_support/package.rs:1896-1909`, `tests/it/exact_scalar_generation.rs:1110-1141`). |

## Delta Review

Rechecked exact head `43af85923ec459bc56530f6907f8131b5ae08b7d` against `311718ab03f370b4e3ffb1253ac443dd8167af1f`, with the earlier `311718ab` delta checked against `fd8f9510a5b8e6a6f42559c1f7eb296391140928`. FND-001 is fixed by the explicit identity expectation; FND-002 remains fixed from `fd8f9510a5b8e6a6f42559c1f7eb296391140928`. FND-003 was not fully fixed at `311718ab`: its expected bound resolved `DEC` (-1,000..1,000), not the distinct bound attached to node 1086. At this head the expected ID resolves the actual `Bound::Decimal(-100_000, 100_000, 0, 2, "nearest-even")` fixture bound, so the requested negative control now checks the actual mismatch. The additional exact-head delta `1ed03534dda396928e79e45b97169775a8b0327d` changes only four needless borrows in calls to `function_body(&str, &str)` (`tests/it/exact_scalar_generation.rs:1029, 1052, 1076, 1083`). The arguments are already `&str`; removing the extra `&` is type-correct and does not change assertions or finding dispositions. The delta leaves no open code-review findings. No Cargo or Kani command was run.

## Review Notes

The Rust lane found no additional production correctness, panic-surface, unsafe, boundary-conversion, or test-seam issue in the changed code. The typed catalog and closed consumer matches preserve family, arity, result, operand order, runtime call selection, and existing caller-declared refusal behavior. At the original commit, the local identity test's other metadata projections were checked against expected values but its identity round trip was self-referential; the delta now checks identity against a separate literal expectation.

No Cargo, Kani, or full repository gates were run. The compile failure in FND-002 was identified by the test's expected field type and expression type.
