---
id: SR-8662
title: "IR-550 PR #372 code review with Rust lane"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@a877339ebc41b5e37c0c449d50fbefa285273fb4; src/oracle/{mod,scalar/mod,equality/mod,function/mod}.rs; tests/it/{exact_scalar_generation,exact_function_generation}.rs; .github/workflows/ci.yml; Makefile; FR-014-AC-44, FR-018-AC-32, FR-021-AC-33"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-018", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
---

## Summary

Reviewed PR #372 at exact head `a877339` against its parent `2ac7cf64`, including the Rust and test changes, related criteria, and the default CI invocation. The typed refusal implementation is consistent on the covered direct roots, but new fixture tests panic in the repository's default test/CI environment unless an undocumented `QSPEC_DIR` is supplied.

## Verdict

**FAIL** — the default `cargo test` gate deterministically reaches a new `expect` with `QSPEC_DIR` unset; the equality criterion also lacks a trace-bound, complete test.

## Reviewed Units

- **FR-014-AC-44 (examined):** "For each IR-admitted direct request naming a `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node, the scalar claim map has exactly one entry with that requested node id and `ExactScalarRefusal::Unsupported`."
- **FR-018-AC-32 (examined):** "Each IR-admitted direct `temporal`/`formula`, `temporal`/`fairness` and `expression`/`case` request has one claim-map entry keyed by its requested node id and refused as `CompositeEqualityRefusal::Unsupported`."
- **FR-021-AC-33 (examined):** Direct temporal/formula, temporal/fairness, and expression/case call-node requests retain their call id and typed `Unsupported`; the declared-body-root case retains the separate call id and body refusal, while a healthy sibling remains byte-identical.
- **Changed implementation (examined):** `src/oracle/mod.rs`, `src/oracle/scalar/mod.rs`, `src/oracle/equality/mod.rs`, and `src/oracle/function/mod.rs`.
- **Changed tests (examined):** `tests/it/exact_scalar_generation.rs` and `tests/it/exact_function_generation.rs`.
- **Default test entry points (examined):** `.github/workflows/ci.yml` and `Makefile`; workflow checks out CG only and invokes `cargo test` without setting `QSPEC_DIR`.
- **Fixture authority (context only):** QSpec `origin/main` `47fd6030c352428b566fc2136453510fe5c55d32` in `/home/peter/dev/quire-specification`; both cited published fixtures are present there.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | New QSpec-backed tests panic in default CI because `QSPEC_DIR` is required but never configured | tests/it/exact_scalar_generation.rs:2380 |
| FND-002 | medium | The direct equality case test is untagged and does not establish FR-018-AC-32's identity, no-generation, or healthy-sibling guarantees | tests/it/exact_scalar_generation.rs:2458 |

## Review Notes

The direct-case classifier is narrow to `CheckedNodeTag::Expression` with `case` form/operator. Scalar direct temporal refusals use the lowering record's unsupported cause; the separate declared-body function test preserves the call-node claim and verifies a generated healthy sibling. The PR explicitly marks the missing admitted fairness case and AC-33 completion as open; I did not treat that disclosed gap as a completed acceptance claim.

No full CI, Cargo test, or Kani run was started in this reviewer pass because the shared gate slot was reserved elsewhere. The PR body reports focused runs, but those receipts were not independently re-run.

## Finding Detail

### FND-001 — high

`qspec_package()` unconditionally calls `std::env::var_os("QSPEC_DIR").expect(...)` at line 2380. The new direct formula and case tests call this helper. The checked-in GitHub workflow (`.github/workflows/ci.yml:31`) runs `cargo test` after a plain checkout and declares no QSpec checkout or `QSPEC_DIR`; `Makefile`'s `test` target also invokes `cargo test` without configuring it. Consequently the default suite panics at this line before it can pass. Make the authoritative QSpec dependency available to the normal test gate or make this opt-in test skip cleanly when it is unavailable.

### FND-002 — medium

`tc_029_ac32_direct_control_case_is_unsupported` has no `Trace: FR-018-AC-32` line or Rust `#[trace]` attribute, and `quire matrix` reports FR-018-AC-32 as `untagged`. Its single assertion checks only the refusal variant and causative node id; it does not assert the request-keyed claim identity, absence of generated source/symbol, or that a healthy sibling claim/source remains byte-identical. Add the trace binding and assertions that discriminate the rest of AC-32.

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e44aba8aa331ade926397a1ec1de86992c0fa949 |
| FND-002 | fixed | e44aba8aa331ade926397a1ec1de86992c0fa949 |

The Makefile now provides an overridable QSPEC_DIR default from SIBLINGS and passes it to `cargo test`; the GitHub workflow also checks out QSpec and sets QSPEC_DIR. The equality test is trace-bound and verifies identity, typed refusal, no generated case output, and unchanged healthy sibling output.
