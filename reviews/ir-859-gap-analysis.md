---
id: SR-12211
title: "AC-to-test gap analysis of IR-859 FR-015-AC-96..102 status refresh (CG #424)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #424 at 86d63d3 (IR-859): FR-015-AC-96, -97, -99..-102 against tests/it/kani_obligations_symbolic_domains.rs and src/kani/generate/collection.rs tests"
review_set: subset
date: 2026-10-17
---

## Summary

Ticket: IR-859. Plan completion: not assessed. The review was planless and read-only, and no cargo or kani command was run. Each test cited in the AC-96, -97 and -99..-102 cells was read on 86d63d3. Each exists at its cited line, carries a `/// Trace:` tag that `quire matrix --format tsv` binds, and is a plain `#[test]` except :2080. That test is `#[ignore = "kani lane: run serially through make kani-gate"]`, and `KANI_TEST` runs it with `--ignored` (Makefile:86). The test sources are unchanged since merge fe20ecc. Per IR-268's close-out, the full suite passed on that tree (it 594/0/34), and the ignored Kani test passed (1 passed, exit 0). Hosted CI runs only the CLA check (IR-826). The test bodies assert the AC clauses they are cited for, with three trace gaps, which are all low severity.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-99 requires that the harness "ends with the FR-015-AC-7 cover after its assertions" and draws symbolic inputs rather than a concrete `Vec<i128>`. Only `negotiation_renders_the_selected_sequence_claims_deterministically` (:975) asserts either clause: `find("assert!(claim") < find("kani::cover!")` and `[i64; 3] = kani::any()`. That test is tagged AC-96 and AC-100, and the AC-99 cell does not cite it. The AC-99 binders (:1250, :2064, :2080) do not assert the ordering; a Kani `Verified` outcome implies the cover was satisfied, not that it comes last. Cite :975 for AC-99, or add the AC-99 tag in a code follow-up. | FR-015-AC-99 (tests/it/kani_obligations_symbolic_domains.rs:975) |
| FND-002 | low | The AC-96 cell cites `ordered_set_shape_is_refused_without_erasing_its_uniqueness_semantics` (:1461) for the declared-uniqueness clause. That test is tagged AC-97 and AC-102, not AC-96, so the matrix does not bind it to AC-96. The refusal itself is honestly disclosed ("refused, not modelled"). Either say that AC-96's uniqueness clause is met only through AC-97's unsupported-form refusal, or tag :1461 with AC-96. | FR-015-AC-96 (tests/it/kani_obligations_symbolic_domains.rs:1461) |
| FND-003 | low | The AC-101 cell cites `canonical_model_wire_rejects_cardinality_above_i64` (:1675) for the representational ceiling. That test asserts that `quire_canonical::to_vec` rejects the model document, which happens before any request reaches the generator, so it does not show a request that "settles unsupported with a typed reason". The generator-level control `domain_representability_failures_keep_the_typed_reason` (src/kani/generate/collection.rs:1070) asserts `DomainNotRepresentableInI64` but is untagged. The cited :1137 also asserts only the helper's `CollectionDomainRefusal::FieldNotAdmitted`; it never asserts the request disposition or that no harness is emitted. Cite or tag the generator-level control, and qualify :1675 as upstream evidence. | FR-015-AC-101 (tests/it/kani_obligations_symbolic_domains.rs:1675; src/kani/generate/collection.rs:1070, :1137) |

## Verdict

The verification is sound for AC-96, AC-97 and the collection families of AC-100, -101 and -102. AC-99 is backed, including native replay and the Kani lane. The three low findings are citation and tag gaps; none is unbacked behaviour. These findings alone would not block merge. See SR-12210 for the two medium findings.
