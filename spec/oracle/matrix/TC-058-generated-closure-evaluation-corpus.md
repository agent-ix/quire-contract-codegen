---
id: TC-058
title: "Load CG generated-closure cases into QSL evaluator comparison"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-044
    type: verifies
---
# TC-058: Load CG generated-closure cases into QSL evaluator comparison

## Description

Verify that CG publishes QSL-admitted generated-closure source applications
through the FR-402 `quire-eval-corpus` case format. QSL's harness runs both
evaluators and decides agreement; this case verifies CG's export boundary and
its later `quire-eval` expectation path.

## Test Procedure

1. Admit a scalar, an equality and a nested generated-to-generated-to-
   interpreted source application through QSL. Enumerate their case ids from
   the live source applications before loading the bundle. Run
   `make evaluator-corpus EVALUATOR_CORPUS_DIR=<dir>` and load `cases.jsonl`
   through `quire_eval_corpus::load` and QSL's FR-402 admission. Assert exact
   `quire.evaluator-case/v1` fields, ascending case ids, four source labels,
   dependency units, `package_id` from compilation, callable function,
   arguments, decimal-string limits, `object_environment: null`, sorted
   `generated_bodies: []` and `expected: null` while `quire-eval`
   cannot yet execute registered bodies. Change authority, identity and
   valid source text one at a time without recompiling `package_id` and assert
   `package_mismatch`; change only revision labels and assert admission with
   the same `package_id` (FR-044-AC-1).
2. Export twice from the same admitted cases and compare exact bytes. Compare
   the independently enumerated case ids with the loaded bundle, then remove
   one record and assert a census failure. Perturb the callable, argument,
   dependency and limit independently; assert the loaded
   change or FR-402's stated load/admission refusal. Fail one producer-side
   compile, case-construction, case-encoding or load check and assert no partial replacement:
   the prior complete bundle is byte-identical, or the target remains absent
   (FR-044-AC-2).
3. Inspect CG's writer and dependencies for `quire-eval-corpus` use, and the
   replay module for `qsl_replay::compile_package`. Confirm no CG-local case
   or compared-record codec, loader refusal, `qsl-eval` dependency, direct QSL
   evaluator call outside replay, or expectation computed from emitted source
   or Contract Runtime `exact` (FR-044-AC-3).
4. Once `quire-eval` runs registered generated bodies, use one real
   `quire-eval` run per selected case to set its non-null `expected` record.
   Assert `generated_bodies` contains the sorted unique names of exactly the
   bodies registered for that run; perturb one name and assert FR-402's
   resulting loaded change or refusal.
   Run QSL FR-402's expected/std check over the bundle; assert exact result,
   refusal, location, losses and ten consumed counters. Check Boolean and
   Integer completed values, a non-Boolean/Integer `unencodable_value`, and
   `charges` populated only with non-truncated FR-402 `charge-log` test support
   and `null` otherwise (FR-044-AC-4).

## Expected Results

All intact cases load and admit as their authored applications. Case bytes,
case census and adverse controls have the stated outcomes; a failed export
never publishes a partial bundle. Before registered-body execution exists,
`expected` is `null`; after it exists, non-null expectations pass QSL's
expected/std check under the FR-402 encoding rules. QSL FR-402-AC-10 owns
its differential harness run over CG's bundle.
