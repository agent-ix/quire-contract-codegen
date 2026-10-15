---
id: SR-11241
title: "IR-469 gap-analysis of typed inverted-range refusal and criterion bindings"
type: SpecReview
analysis: gap-analysis
scope: "quire-contract-codegen; tests/it/kani_obligations.rs; tests/it/strategy_generation.rs; FR-015-AC-5; FR-002-AC-4; FR-002-AC-6; supporting context listed below"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: references
---

## Summary

Reviewed the IR-469 test-only repair against its reader diagnostic contract and the changed criterion bindings. No defect was found in the scoped diff. At the initial review, required full native and Terra proof gates were pending; this record does not independently reassess their later qualification.

## Verdict

**PASS** — no findings within the examined IR-469 repair scope. Gate qualification was incomplete at the initial review, as detailed below; this is the scoped diff verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope and method

Independent reviewer-only inspection of the complete two-file IR-469 diff: tests/it/kani_obligations.rs and tests/it/strategy_generation.rs (9 insertions, 4 deletions). Repository AGENTS.md, CLAUDE.md, CONTRIBUTING.md, Cargo.toml, Cargo.lock, clippy.toml, deny.toml and rustfmt.toml supplied conventions/context. No CI workflow, production, dependency, lock, spec or plan change exists in this diff. The code-review Rust lane applied the Rust idiom, test/seam, completeness, panic, numeric, async/lifecycle, wire and resource checklists before analysis and again before handoff. Unchanged architecture surfaces are context, not a repository-wide clean claim.

Context inspected: spec/strategy/functional/FR-002-tristate-proptest.md (AC-4/AC-6), spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-5), src/strategy/campaign.rs, src/kani/generate/negotiate.rs, and the Cargo-selected contract-model expression.rs, wire.rs and binding.rs reader path. Negative-inventory context: tests/it/kani_obligations_state_frame.rs, tests/it/bound_strategy_generation.rs, tests/it/oracle_arithmetic.rs and tests/it/oracle_generation.rs. No applicable committed AssuranceProfile was found under spec/.

## Assertion and binding assessment

The valid projection(1000) control binds before projection(-1) must fail. The reader's IntegerType constructor returns InvalidNumericBounds at type.integer.bounds for inverted limits, and the binding reader propagates that diagnostic. Requiring one diagnostic before indexing makes count, typed code and path unconditional. The earlier exact CG UnsatisfiableBound payload assertion remains intact; the added reader assertion verifies its upstream admission prerequisite.

The supported-populations fixture compiles and executes generated Rust; its deterministic range/correlation value-tree traversal checks constraints through shrinking. That supports FR-002-AC-4. Its integer cases execute expected-domain verification on measured cases, supporting the integer subclause of FR-002-AC-6. The enum fixture executes an all-admitted membership expectation, validates the customer-enum values and checks rejection-verdict mismatch, supporting the enum subclause of AC-6. Each binding is correct for that contribution. Neither test establishes all Boolean campaign behavior or complete AC-6 family coverage. The residual population loop checks both reported populations, but does not traverse residual shrinking or independently rederive classification for every residual primary value; no stronger claim is made.

The ten remaining explicit is_err sites in the six-module inventory are intentional schema invalidity checks (seven), generated verdict mismatch checks with successful same-case controls (two), or a negative following the same construction's exact typed arithmetic refusal (one). The mismatch verifiers have a single mismatch failure path; requiring a new message-derived discriminator would not strengthen these checks. The state-frame/schema cases exercise deliberately changed wire shapes after a valid original. No demonstrated analogous unrelated-error hole was found in those unchanged sites.

The existing ignored real-Kani control still requires CoverUnsatisfied { satisfied: 0, total: 1 } and typed Inconclusive { reason: TimedOut } under its configured deadline. Its deadline-check mutation comment and exact CG UnsatisfiableBound assertion are unchanged. There is no new elapsed-time oracle. At the initial review, runtime qualification of the real-Kani control was pending.

## Evidence and limits

The reviewer read root-owned focused receipt.json and stage-0/1/2.log: exact list/build exit 0 selects three named tests; bounds runtime exit 0 reports 1 passed, 0 failed, 0 ignored; touched integration Clippy exit 0. The two generated-strategy scratch runtimes were listed/compiled in the integration binary but were not executed by this focused run.

The reviewer read the root-owned unrelated-mutation receipt and focused.log: freshly built/listed unrelated-format mutant fails exit 101 at InvalidWireFormat versus expected InvalidNumericBounds; the original bare-Err comparator passes exit 0 on that same malformed input; restored candidate passes exit 0. These observed results substantiate the repaired weakness. The reviewer did not create or execute a mutation.

Private evidence remains under /tmp/ix-handoff/ir469-focused-3513317/ and /tmp/ix-handoff/ir469-unrelated-mutation-3513317/. The computed matrix JSON and its diagnostics are retained alongside this review. Its examined rows compute tagged and resolve the exact named tests for FR-002-AC-4/AC-6 and the retained FR-015-AC-5 binding. The matrix command was invoked with --strict, but the shell wrapper did not preserve that subcommand's exit status; no whole-repository strict success is claimed. No handwritten matrix was produced.

At the initial review, full pre-PR native gates had not qualified; an earlier busy attempt ran zero stages. Terra's allocated lane was the normal make kani-gate, including the existing cover/timeout control. No separately executed new deadline-check mutant was allocated or claimed by this review. Full pre-merge gates were separately owed. This review PASS means no defect in the scoped diff; later gate qualification belongs to the owners' evidence and was not independently reassessed during record completion. No Cargo, test, Clippy, solver, probe, new worktree, build target, source/index/lock edit, commit or external message was performed by the reviewer.

## Coverage

Plan completion: not assessed

This is the explicitly requested scoped gap/acceptance check of the IR-469 repair, not an end-to-end repository assurance audit. The computed matrix was read for the changed bindings; tag existence is separated from semantic sufficiency above. Existing related fixture bodies and source contracts were inspected for stubs, vacuous assertions and reverse ownership. No new production behavior requires specification; the test change has FR-015 ownership and the Trace additions resolve to FR-002. No broad optional semantic audit or delegation was run. No whole-family or repository acceptance verdict is asserted.
