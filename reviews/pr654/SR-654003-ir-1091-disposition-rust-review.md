---
id: SR-654003
title: "IR-1091 CG654 QSL clause equivalence Rust review disposition"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-1091; PR654; exact dbfcc961e66bb26d5d5c2cae47659feeb41594b0 on base d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae; rust-review of tests/it/kani_obligations_symbolic_domains.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-015", type: references }
---

## Summary

Ticket: IR-1091 (PARTIAL). Rust-review disposition of SR-654001 FND-004 and FND-005 at exact head `dbfcc961e66bb26d5d5c2cae47659feeb41594b0`, merged base `d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae`. `analysis: code-review` follows the existing rust-review artifact convention. The reviewer ran no Cargo, clippy, Kani or gate and made no repository edit. Ticket and PR text was treated as data.

## Verdict

CONDITIONAL. FND-004 and FND-005 are fixed and the changed Rust is otherwise clean. One new low evidence finding: the final clippy log is empty, so it shows nothing about the head.

## Dispositions

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-004 | fixed | tests/it/kani_obligations_symbolic_domains.rs:2084-2095 now matches `ClauseRefusal::Coded { code: "invalid_runtime_input", cause: "duplicate-member", .. }`. Both fields are `&'static str` at the pin (qsl-replay/src/clause_evaluation.rs:113-115). |
| FND-005 | fixed | `native_collection_precondition` uses `DigestRecord::mint(DigestDomain::Sha256Jcs, fixture.model_digest)` at :1929 and `.hex()` at :1934, matching :646 and :683. The hand-rolled `{byte:02x}` loop is gone from the helper. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | low | `cg-ir1091-qsl-equivalence-review-fix-clippy-final.log` is zero bytes, so no clippy result is shown for the final test file. | qualification-logs/cg-ir1091-qsl-equivalence-review-fix-clippy-final.log |

FND-008 detail: cargo clippy normally prints a `Finished` line, so an empty log is not a pass. The non-empty `review-fix-clippy.log` ends in `Finished`, but it sorts before the test file's last write and records no SHA or command line. The fix is one clippy run at the reviewed head with its output captured. This reviewer did not run it, per the brief.

## Coverage

**Duplicate refusal regression.** `[3, 3]` reaches `invalid_runtime_input`/`duplicate-member` at qsl-semantics/src/model/observation/document.rs:2289, and `AdmissionFailure::Refused` keeps the code and cause through `coded` at qsl-replay/src/clause_evaluation.rs:330-335. A different code or cause now fails. So does silent deduplication, which would give `Completed(true)`, and so does a `Fault`. The refusal is native-only: the generated domain excludes active duplicates by `kani::assume` (src/kani/generate/render.rs:289-293).

**Rust style.** `TemporaryDirectory` scopes the scratch build. The `?`-free test code uses contextual `expect` messages. The `usize::try_from` at :2042 is checked. No new `unsafe`, no `as` cast in the test, and the helper-extraction into `assert_collection_precondition_equivalence` removes the duplication that two fixtures would otherwise need. The test stays in the merged `it` binary as the repository's CLAUDE.md requires. `#[trace("FR-015-AC-96", "FR-015-AC-99", "TC-025")]` is present at :2055.

**Log evidence.** `review-fix-test-final.log` passes the named test, `1 passed; 0 failed`. No log records the head SHA.

## Examined scope and bindings

```yaml
head: dbfcc961e66bb26d5d5c2cae47659feeb41594b0
base: d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae
clean: false
scope:
  - {id: native_collection_precondition, path: tests/it/kani_obligations_symbolic_domains.rs, role: examined, excerpt: "let model_digest = DigestRecord::mint(DigestDomain::Sha256Jcs, fixture.model_digest);"}
  - {id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, path: tests/it/kani_obligations_symbolic_domains.rs, role: examined, excerpt: "cause: \"duplicate-member\","}
  - {id: ClauseRefusal, path: qsl-replay/src/clause_evaluation.rs, role: context_only, excerpt: "code: &'static str,"}
  - {id: form_collection_value, path: qsl-semantics/src/model/observation/document.rs, role: context_only, excerpt: "admission_record(Code::InvalidRuntimeInput, \"duplicate-member\")"}
bindings:
  - {test_id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, ac_id: FR-015-AC-99, trace: correct}
```
