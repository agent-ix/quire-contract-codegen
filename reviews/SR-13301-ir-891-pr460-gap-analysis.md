---
id: SR-13301
title: "IR-891 PR 460 gap analysis: NFR-002-AC-1/AC-2 header coverage against TC-001"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@619b84e2e959b68fc3f6f1a81624ab6930447c98 vs origin/main 58f44780; spec/core/non-functional/NFR-002-provenance-boundary.md, spec/core/matrix/TC-001-reproducibility.md, spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md (AC-12), src/** emitters and tests/it/** changed by the PR"
---

# SR-13301: IR-891 PR 460 gap analysis

## Summary

Ticket: IR-891 (item 1) and IR-897. Planless gap analysis of PR #460 against NFR-002-AC-1,
NFR-002-AC-2 and the TC-001 procedure on main (merged PR #458, 58f44780). Plan completion: not
assessed.

`quire matrix --scope .` at the head binds NFR-002-AC-1 and AC-2 to 20 tests (status `tagged`).
The reviewer mapped each NFR-002 Scope family to a test that reads real emitter output:

- Strategy harness: harness_generation tc_004 (requirement@revision and both clauses).
- Integer and enum strategies: strategy_generation tc_004 and enum_strategy_header_retains_input_identity.
- Bound strategy: bound_strategy_header_retains_its_clause_identity.
- Bound population: bound_populations tc_018 (population only, see SR-13300 FND-005).
- Boolean and bound oracles: oracle_generation tc_001 and bound_generation (requirement, revision and clause).
- Exact-scalar package `src/lib.rs`: exact_scalar_generation tc_024 (checked-package path only; FR-330 contract path untested, SR-13300 FND-003).
- Composite equality `src/lib.rs`: tc_029_ac10 fresh-package test (mis-tagged, SR-13300 FND-006). Extra modules: unit test on a synthetic call only (SR-13300 FND-002).
- Function package `src/lib.rs` and schedule flat source, split root and parts: exact_function_generation tc_031_ac34, ac7 and ac13 (root claim nodes and lib declaration nodes untested, SR-13300 FND-004).
- Kani generic render: render.rs unit test. V2 contract: kani_obligations tc_025 per clause. V1 bundle: kani_generation. Exact-scalar Kani: kani_obligations tc_025 scalar. Collection: symbolic_domains negotiation test. State frame: state_frame tc_025.
- Bounded corpus `.oracle.rs`, `.strategy.rs` and `.kani.rs`: bounded_kani_corpus tc_023.

Every family has at least one real-output header test. AC-2 (SPDX) holds on every emitter, and
the generic-render SPDX mutant is killed. For AC-1, five reviewer mutants on emitter subject
paths survive (SR-13300).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | NFR-002-AC-1 is not fully tested for several emitter paths that TC-001 names. "For every generated `.rs` artifact of each family... Include oracle-package `src/lib.rs` files, equality extra modules, and function `src/schedule_*.rs` part files; do not accept a single passing file as coverage for a family." Untested are the FR-330 contract exact-scalar `src/lib.rs`, equality `src/composites_*.rs` from the real emitter path, the function split-root claim subjects, the function package `src/lib.rs` declaration subjects, and bound-population relation and domain. Each was shown by a surviving mutant (SR-13300 FND-002 to FND-005). Fix: add the assertions listed in those findings. | tests/it/generated_headers.rs:1-39, src/oracle/scalar/mod.rs:1213-1222, src/oracle/equality/mod.rs:1715-1738 |
| FND-002 | medium | FR-031-AC-12 states: "The oracle generated for an expression with no arithmetic node is byte-identical to the oracle the generator produced for it before this requirement." NFR-002-AC-1 now requires a new generator line in that oracle, so the two criteria conflict. The PR edits the test still tagged `FR-031-AC-12` (expected bytes plus the generator line, and the doc now says "apart from the required NFR-002 header change"), but leaves the AC text unchanged. The test therefore no longer asserts the criterion it is bound to. The fixture change is the one NFR-002 requires; the spec text is what is stale. Fix: amend FR-031-AC-12 to exclude the NFR-002 header lines, in this PR or a linked spec PR before merge. | tests/it/oracle_arithmetic.rs:825-877, spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:514 |

## Verdict

**FAIL for full NFR-002-AC-1 coverage.** No family is entirely missing a test, and AC-2 is covered.
AC-1 still has the five untested emitter subject paths above, which TC-001's "every artifact"
procedure requires to be closed. FR-031-AC-12 needs a spec amendment so it is consistent with
NFR-002.
