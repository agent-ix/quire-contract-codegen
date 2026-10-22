---
id: TC-060
title: "Verify finite rational whole-function predicates"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-046
    type: verifies
---
# TC-060: Verify finite rational whole-function predicates

## Description

Verify [FR-046](../functional/FR-046-rational-function-predicate-harnesses.md)
through genuine QSL source, strict owner admission and lowering, public CG
whole-function generation, installed proof execution and owner replay. This case
is planned; historical source admission or a native arithmetic check does not
establish proof acceptance. Execute generated proofs only with the authorized
runner and budget; a parked CG-self-proof gate is not feature evidence.

## Test Procedure

1. Preserve the original healthy and wrong-addend source cases from the
   QSL-owned `rational_mvp_source` intake. Admit both unchanged through the public
   source route and strict reader, retaining original source association and
   owner-issued identities. The healthy case is:

   ```text
   language "ix:native" edition "1-draft";
   profile v = "quire.value.complete/v1";
   function rational_commutative using v(a: Rational[-9, 9; 1, 9], b: Rational[-9, 9; 1, 9]): Boolean pure {
       a + b = b + a
   }
   ```

   The only source mutation replaces `a + b = b + a` with `a + b = a + a`.
   Select the entire function declaration, inspect its lowered Boolean body and
   request its predicate harness. Retain an honest typed capability refusal if
   that owner route is absent; do not build a synthetic substitute (AC-1, AC-8).
2. Independently enumerate the semantic input domain using exact host rational
   arithmetic and compare it with the constructor/accessor roundtrip encoding.
   Check 111 unique canonical values per argument and all 12,321 tuples, including
   zero `0/1`, `-9/1`, `9/1`, `1/9`, negative fractions and noninteger values.
   Check soundness, injectivity and completeness rather than only these examples.
   Reject duplicate encodings such as `0/2` and `2/2`; include mutations accepting
   unrestricted normalization, dropping an endpoint or drawing only denominator
   one. Ensure these changes fail the correspondence check (AC-2).
3. Generate the actual oracle and compare its full native observations with an
   independently constructed reference using public owner operations on the same
   admitted function. Preserve all typed outcomes and ordered Meter charges,
   counters and stops at every occurrence-denial prefix and zero/exact/short
   budgets. Include `a = b = 9` to exercise a sum outside the input domain and
   owner continuation beyond a machine-sized intermediate in a separately
   admitted finite-domain case. Exercise reachable undefined/refused/incomplete
   outcomes without input filtering. Mark an unavailable authentic fixture or
   owner operation explicitly rather than reporting that control as passing.
   Inspect generated assumptions for dependence on body outcomes (AC-3, AC-4).
4. Run the healthy harness through the approved bounded production path with its
   actual complete ordered options and no stubbing or fixture edits. Verify all
   required checks and exactly one final completion cover. Run the wrong-addend
   mutation over the same entire domain. Retain its actual failed predicate check
   and solver playback; validate and replay its arguments through the owner route
   for that exact declaration and assert completed Boolean false. For comparison,
   `a = 1/2, b = 1/3` mathematically gives `5/6 != 1`, but this example is not a
   solver counterexample until actually produced and replayed (AC-5, AC-9).
5. Check separate adversarial controls for a missing/unsatisfied cover, failed
   unwind, missing verdict and execution-limit failure. Inspect the healthy and
   mutated harnesses for one cover after assertions and no outcome-based assumes.
   Preserve raw backend outcomes separately from semantic replay settlement;
   noncompletion must not be relabelled Boolean false (AC-4, AC-6).
6. Request an otherwise valid unbounded Rational argument and assert named
   `requires-bound` with no harness. Request an owner-admitted finite domain with
   an endpoint beyond signed-i64 capacity and assert named `unsupported` with
   the exact endpoint and no clipping. Exercise absent lowering, an unsupported
   body operation and unavailable correspondence, alongside a supported sibling;
   assert total per-item accounting and unchanged sibling output. An upstream
   source/reader refusal is recorded at that boundary, not counted as the
   corresponding CG refusal test (AC-7, AC-8).
7. Perturb a retained witness to be out of domain, noncanonical, duplicate zero,
   stale identity or bound to another declaration. Assert responsible-boundary
   rejection without minted keys or changing the application into scalar parity.
   Record absent public replay capabilities as open dependencies (AC-9).
8. Inspect the run record for complete function/source association, input-domain
   correspondence, actual tool versions, target, dependency cohort, limits,
   harness selection and full ordered argv including unwind/loop options. Match
   checks, single cover, actual playback, exit and terminal classification to the
   raw run. Verify retained source is replayed unchanged and that all no-run or
   refused cases report their actual disposition (AC-10).

## Expected Results

The supported encoding is a bijection onto the original mathematical domain;
intermediate arithmetic and owner observations retain their semantics. The
healthy predicate verifies with a satisfied completion cover; its source-authored
wrong-addend mutation fails with an actual in-domain false replay. Every refused,
noncomplete, inconclusive or unrun control retains its own classification and
cannot receive proof or replay credit. Until the complete generated-proof and
replay path runs successfully, IR-258 remains unaccepted regardless of historical
source-admission or native agreement results.
