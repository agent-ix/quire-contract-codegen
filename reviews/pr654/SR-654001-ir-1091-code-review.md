---
id: SR-654001
title: "IR-1091 CG654 QSL clause equivalence independent review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-1091; PR654; exact 85ad5088c2c40b8531138f0365464e1f38b293b2; Cargo.lock and tests/it/kani_obligations_symbolic_domains.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-015", type: references }
---

## Summary

Ticket: IR-1091 (PARTIAL). Independent PR-diff review of CG #654 at exact `85ad5088c2c40b8531138f0365464e1f38b293b2`, merge base `6227850874b5891516be6c9fcc56a81a900705c4`. Current `main` is `d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae`, one commit ahead of that base; it touches only `reviews/ir-1116-code-review.md`, `tests/it/composite_equality_agreement.rs` and `tests/it/exact_scalar_agreement.rs`, none of which this PR changes. The code-review, rust-review (with rust-style) and gap-analysis lanes run in this one artifact, as the dispatching brief asks for a single artifact and comment. No Cargo build, test, Kani or gate was run by the reviewer, and no branch edit was made.

## Verdict

CONDITIONAL. One medium and four low findings; none is high. The core claim holds for the three compared valuations. The QSL pin family, fixture authenticity and absence of false proof credit are clean. FND-001 should be fixed before merge because the comparison set omits the one valuation class that separates the generated array draw from the native truncated collection.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No compared valuation places the witness in an inactive slot, so ignoring the drawn length survives this equivalence test. | tests/it/kani_obligations_symbolic_domains.rs:2009 |
| FND-002 | low | The comparison uses the SmallInt 0..=3 fixture, so no i64-width element valuation reaches both evaluators. | tests/it/kani_obligations_symbolic_domains.rs:1962 |
| FND-003 | low | FR-015-AC-99 still says the precondition independent-oracle control awaits QSL-1302 after this PR adds that control. | spec/kani/functional/FR-015-bounded-kani-obligations.md:901 |
| FND-004 | low | The duplicate-member control accepts any Coded refusal instead of the specific code and cause. | tests/it/kani_obligations_symbolic_domains.rs:2084 |
| FND-005 | low | The helper hand-rolls digest hex although the file already uses DigestRecord::hex. | tests/it/kani_obligations_symbolic_domains.rs:1929 |

## Coverage

**QSL pin.** The `Cargo.lock` diff changes exactly seven `source` lines, all to `93fcdc171125a70d3fee4943bbb0110424900dcf`: qsl-cst, qsl-eval, qsl-forms, qsl-foundation, qsl-package, qsl-replay and qsl-semantics. These are all seven QSL entries in the lock, so one copy holds. No dependency list or other source changes, and `Cargo.toml` is untouched. `gh` compare shows the old pin `9e74a126` is 28 commits behind the new pin, with none diverged, and the new pin is identical to QSL `main` at review time. No file other than `Cargo.lock` names either SHA.

**Authenticity.** `precondition_collection_fixture` goes through `call_site` (QSL emission), then `CheckedPackageV2::read` (IR admission), then `negotiate_kani_obligations` (CG V2 generation). The test takes `oracle_clause` and `ClauseEvaluation` from the generated harness by `syn`, compiles them with `rustc` and runs the binary. `evaluate_clause` recompiles the same source bytes under the same four source labels and model bytes, then evaluates `CollectionReady` at `PreCall` with the same receiver object and no parameters. `lower_collection_precondition` refuses any extra parameter, so receiver-only is a generator property. The compiled unit is the oracle and enum, not the `kani::proof` body, whose assumptions are source-pinned by existing tests. That is the correct seam for this claim.

**Independence.** `expected` is a test literal. The generated value comes from a separately compiled process and the native value from QSL's evaluator, and each is asserted against that literal. Neither value derives from the other, so a real disagreement fails the test. An `exists`-to-`forall` swap fails at `[2, 3]`; an off-by-one upper bound fails at `[2, 3]` and `[0, 1, 3]`; a wrong literal fails at the true cases. A non-Completed generated value panics, and a native `Undefined` fails the `matches!`.

**Outcome separation.** `Completed(true)`, `Completed(false)`, `Incomplete(Accounting)` and `Refused(Coded)` are each asserted by variant. `Incomplete` and `Refused` are native-only controls: the generated oracle has no meter and returns `Completed` by construction, and `[3, 3]` is excluded from the generated domain by `kani::assume`. That is honest, because the PR text claims agreement only for true and false completions. Source tracing at the pin shows `[3, 3]` reaches `invalid_runtime_input`/`duplicate-member` at `qsl-semantics/src/model/observation/document.rs:2289`.

**False-credit check.** The test changes no proof state, identity, readiness or status, and no spec row is promoted. `#[trace("FR-015-AC-99", "TC-025")]` is a correct binding for the precondition independent-oracle clause only. The PR does not credit the postcondition leg, all AC-99 rows, or QSL FR-491-AC-7 S6a live stops. Unsupported shapes are untouched by this diff.

**Logs.** The focused log shows the worktree path, QSL at `93fcdc17`, and `1 passed; 0 failed; ... 803 filtered out` for the named test. The clippy log ends in `Finished` with no warning or error. Both timestamps (06:33 and 06:35 local) postdate head commit 06:09. Neither log records the SHA or the command line, so `-D warnings`, `--locked` and the exact head are inferred from the PR text and timing, not shown by the logs.

**Gap analysis.** `quire trace --id FR-015-AC-99` resolves 12 claims, including the new test at line 1961. The matrix was not otherwise changed. Plan completion: not assessed.

**Rust style.** Reviewed against rust-style and its testing reference. Trace attribute, placement in the `it` binary, `Command::new("rustc")` usage and assertion context all match repository practice. `TemporaryDirectory` scopes the scratch build. FND-004 and FND-005 are the only smells.

## Examined scope and bindings

```yaml
head: 85ad5088c2c40b8531138f0365464e1f38b293b2
base: 6227850874b5891516be6c9fcc56a81a900705c4
clean: false
scope:
  - {id: Cargo.lock, path: Cargo.lock, role: examined, excerpt: "seven QSL source entries at 93fcdc171125a70d3fee4943bbb0110424900dcf"}
  - {id: FR-015-AC-99, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "The precondition totality/satisfiability control compares the same authored clause through a QSL public clause-evaluation facade"}
  - {id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, path: tests/it/kani_obligations_symbolic_domains.rs, role: examined, excerpt: "generated and native clauses agree at {items:?}"}
  - {id: native_collection_precondition, path: tests/it/kani_obligations_symbolic_domains.rs, role: examined, excerpt: "ClauseObservation::PreCall { snapshot, self_object: Some(object), parameters: BTreeMap::new() }"}
  - {id: render_collection, path: src/kani/generate/render.rs, role: context_only, excerpt: "let claim = (0..{length_name}).{collection_call}(|index| {"}
  - {id: evaluate_clause, path: qsl-replay/src/clause_evaluation.rs, role: context_only, excerpt: "FR-491: evaluate request.clause at request.observation."}
bindings:
  - {test_id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, ac_id: FR-015-AC-99, trace: correct}
```
