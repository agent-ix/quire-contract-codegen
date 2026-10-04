---
id: "SR-1293"
title: "CG PR 250 code grounding: NFR-005-AC-1 to AC-5 and the site table against CG main"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@2995b8d4d22fe0fc508c3a1bb22f5365d932651e; spec/core/non-functional/NFR-005-no-generation-panics.md (Behavior table, NFR-005-AC-1 to AC-5), spec/core/matrix/TC-042-no-generation-panics.md, against src/kani/generate/scalar.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/routed/capability.rs, src/evidence/bound_coverage.rs, src/routed/generate.rs, src/kani/generate/corpus/bounded_kani_corpus.rs, src/oracle/function/mod.rs, src/oracle/boolean_v1.rs, tests/common/panic_scan.rs at origin/main f970692 (read only)"
---

# SR-1293: CG PR 250 code grounding

## Summary

Ticket: IR-543. PR: agent-ix/quire-contract-codegen#250 at 2995b8d. The diff is spec only. This
lane reads each AC and each site-table row against the code it pins, read only, at `main`
f970692 (the PR's base).

Grounded and clean:

- AC-1. `classify` in `panic_scan.rs` already has `Kind::Literal`, so
  `non_test_code_outside_literals` is a one-line variant of `non_test_code` that also drops
  `Kind::Literal` bytes. It is plausible and cheap. The literal classifier handles `"..."`,
  `b"..."`, raw strings and character literals, and tells a character literal from a lifetime.
- AC-5. `RoutedGenerationError` (`routed/generate.rs:126-158`) has no `KaniRecordCountMismatch`
  today, so the variant must be added. It is a public enum without `#[non_exhaustive]`, which is a
  breaking change, accepted before release. FR-015-AC-23 ("Every requested item receives exactly
  one `ObligationDisposition`") is the basis for the count. To test the pairing on hand-built
  inputs, the pairing has to be pulled out of `generate_kani` into its own function, and the AC
  implies that.
- The `TypeEnvironment` row: `rt::TypeEnvironment` derives `Default` in quire-contract-runtime,
  and FR-021:196-197 already requires `TypeEnvironment::default()` for the emitted package.
- The `own_shape` and Stage 2 rows: the Stage 1 loop and Stage 2 iterate the same
  `ordered_functions`, so carrying Stage 1's resolved kinds forward is sound.
- The `boolean_v1` row: every `expression_diagnostic` caller passes one of the four codes the
  `debug_assert!` checks, and a release build already skips the check.
- The corpus rows: `KaniOutcomeKind::Refused` exists in quire-contract-ir ("The selected profile
  or construct refuses the request"). For an internal serialization failure it is a better fit
  than `InvalidInput` ("The offered finite input is malformed"), which the other corpus refusals
  use. The code string follows the `kani_corpus_*` naming. SR-1291 FND-002 covers the conflict
  with PR #222.

## Verdict

AC-1, AC-3 and AC-5 match the code. AC-2 and AC-4 each need one change before they can be
implemented without ambiguity. The two medium findings are below.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-4 does not say the LLVM coverage export is supplied. In `observe_clause` the `observe` closure first runs `coverage.ok_or_else(.. UnavailableObservation, "LLVM export was not supplied")` and only then reads `region.probe`. Without coverage, a probe-less map yields `UnavailableObservation`, not `MapMismatch` "missing semantic probe". Whether AC-4 holds therefore depends on an unstated precondition and on which check the fix puts first. Add "with a coverage export supplied" (or fix the check order in the Behavior row). The message and code match the preflight's existing `diag(CoverageErrorCode::MapMismatch, "missing semantic probe")` at line 394, so that convention is right | spec/core/non-functional/NFR-005-no-generation-panics.md:93, src/evidence/bound_coverage.rs:437-447 |
| FND-002 | medium | AC-2 has `classify_claim` report a claim of a renderable family (`quire.op.integer.add`) with no checked bound as `UnsupportedObligation::OperationNotRendered`. The documented meaning of that public variant is "this generator has no Kani harness renderer for its family yet (today: every family but `IntegerArithmetic`)" (`outcome.rs:220-221`), and TC-025 says "Every other confirmed family is refused as `OperationNotRendered`, an unbuilt renderer". `scalar.rs:132-144` says these two causes are deliberately not represented. There is a precedent: `scalar.rs:217-225` already maps an operand bound missing from `checked_bounds` ("a claim map this generator did not produce") to `NoRenderer`, so reusing `NoRenderer` is defensible and adds no public variant. Recommendation: keep `NoRenderer`, but have NFR-005 require the widened meaning in writing. The `NoRenderer` and `OperationNotRendered` doc comments and TC-025's sentence should say "or a claim map this generator did not produce", and the `scalar.rs:132-144` comment should be replaced. If a separate cause is wanted, use a private `ScalarLoweringRefusal` variant that still maps to `OperationNotRendered` | spec/core/non-functional/NFR-005-no-generation-panics.md:71, spec/core/non-functional/NFR-005-no-generation-panics.md:91, src/kani/generate/outcome.rs:220-227, spec/kani/matrix/TC-025-bounded-kani-obligations.md:164 |
| FND-003 | low | The `ItemSettlement::warning` row classes the site as "invariant of settlement". But `ItemSettlement`, `Disposition` and `Cause` are `pub` with `pub` fields, so any caller can build `Disposition::Unsupported { cause: Cause::AbsentKind }`, and `warning()` panics on it today. This is a panic a caller can reach through the public API, not an internal invariant. That makes AC-3 a real regression test. Returning `None`, like the `InvalidRequest` arm, does not hide a CG defect, because CG never builds that value. FR-019:73 does say an `unsupported` settlement carries a warning, so document the `None` case in `warning()`'s doc comment and correct the row's class | spec/core/non-functional/NFR-005-no-generation-panics.md:72, spec/core/non-functional/NFR-005-no-generation-panics.md:92, src/routed/capability.rs:356-403 |
| FND-004 | low | For the `unreachable!` in `ScalarOperation::reachable` (operand count), the site table prescribes `Err(NoRenderer)`. The operand list is built in the same function from `0..operation.operand_names().len()` (`scalar.rs:213`), so its length is fixed by the operation. Statement bullet 2's own preference, using the guarantee by pattern matching, fits better: pass fixed-arity operands per operation, so no arm is left over, rather than threading a refusal through `reachable` that nothing can trigger | spec/core/non-functional/NFR-005-no-generation-panics.md:71, src/kani/generate/scalar.rs:97-122 |

## Dispositions

Round 1, reviewed at e7f149d9f4ed23d75053a7fcd54e6754c095aff8.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed e7f149d | e7f149d9f4ed23d75053a7fcd54e6754c095aff8 |
| FND-002 | fixed e7f149d | e7f149d9f4ed23d75053a7fcd54e6754c095aff8 |
| FND-003 | fixed e7f149d | e7f149d9f4ed23d75053a7fcd54e6754c095aff8 |
| FND-004 | fixed e7f149d | e7f149d9f4ed23d75053a7fcd54e6754c095aff8 |
