---
id: "SR-11620"
title: "IR-635 Ne slice code review: structural.ne in the original composite builder"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen branch code/ir-635-ne-builder (no PR yet; reviewed head recorded in the IR-635 Linear review marker, not here), diff against origin/main; src/replay/composite_builder.rs, tests/it/composite_original_eq.rs"
---

# SR-11620: IR-635 Ne slice code review

## Summary

Ticket: IR-635. Code review with the Rust lane (rust-review idiom and smell checklist) folded
in, scoped to the branch diff only (4 files, about +215/-31). The reviewer re-measured: the
`composite_original_eq` module passes 16/16 on the reviewed head (QSPEC_DIR set to a fresh
QSpec checkout), `cargo fmt --check` is clean and `cargo clippy --locked --all-targets -D
warnings` is clean. The full `make ci` gate was not run by the reviewer (the leader runs it).

Examined:

- `OriginalCompositeEqContext::request` (new `operator: EqualityOperator` parameter, the node
  operator read, the `OperatorMismatch` refusal, the claim's operator) and
  `CompositeBuildError::OperatorMismatch` with its `Display` arm.
- The new tests `original_ne_parameter_operands_keep_order_and_operator`,
  `original_ne_literal_operand_keeps_empty_bounds`,
  `original_operator_mismatch_refuses_with_typed_operators`, and every updated `request` call.
- Upstream context, read only: the locked IR `composite_application_operands` (accepts
  `quire.op.structural.eq` and `quire.op.structural.ne`, refuses anything else with
  `IneligibleOperator`, returns no operator); the locked QSL `EqualityOperator { Equal, NotEqual }`
  and `ParityPreimage { node, occurrence, obligation_kind, arguments }`; QSL ADR-013 O-09 parity
  preimage (members `arguments`, `node`, `obligation_kind`, `occurrence_key`, no operator); QSL
  FR-358 step 2 and step 6, and rows V-1 to V-5.

What is sound:

- The operator is bound only through the application node, as O-09 defines it: the preimage
  type has no operator member, and FR-358 step 6 hashes node, occurrence, kind and positional
  arguments. The claim's `operator` is the request's value only after it equals the node's, so
  the settle path never carries an operator the node does not hold.
- Argument order is the accessor's positional order for Ne as for Eq; nothing reorders it.
- One error variant per condition; the match over the two identities has an error arm, not a
  value arm. No `unwrap`, `expect`, `panic!` or indexing on caller input in library code.
- Every `request` caller (all in `tests/it/composite_original_eq.rs`) is updated; no other caller
  exists in `src/` or `tests/`. `src/lib.rs`, `Cargo.lock` and `Cargo.toml` are unchanged, and
  no file of the overlapping PR #381 is touched.
- Mutants run in a throwaway worktree: bypassing the operator check, deriving the operator from
  the request, reverting the claim to a hard-coded `Equal`, reversing O-09 arguments, mapping Ne
  to Equal, swapping operands for Ne only and swapping the error fields are all killed. Only the
  "accept the request operator for a node carrying neither" mutant survives, and it is
  equivalent (FND-001).

## Verdict

PASS with low findings. The change is correct and its tests kill every non-equivalent mutant
tried. FND-001 and FND-002 are design notes that may be fixed or dispositioned without blocking.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The node operator is re-derived in CG by reading the raw JSON pointer `/operation/identity` with a linear, unmetered `find` over all graph nodes, duplicating the eq/ne string check the IR accessor already made. The `_ => OperandShape` arm is unreachable (the accessor refuses any other identity first with `IneligibleOperator`), so the mutant replacing it with `_ => operator` survives as an equivalent mutant, and its variant names a shape fault for an operation fault. A typed operator on IR's `CheckedCompositeOperands` (upstream) would remove the string parse and the dead arm. | src/replay/composite_builder.rs:252-264 |
| FND-002 | low | `request` takes the operator from the caller only to refuse it when it differs from the node, although the node alone determines it and QSL FR-358 step 2 already refuses a claim whose operator the node does not apply (`ScalarIdentity`, binding-checked report). The source-breaking parameter adds a second CG-local copy of a QSL-owned check rather than a capability; deriving the operator from the node (and exposing it on the request) would remove the mismatch case. Judgement call: keep it if an explicit caller intent check is wanted, and then state it in an AC. | src/replay/composite_builder.rs:240-270 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred: IR-803 | Round 1 (fix commits 1ec4b90, 04b4e79). Verified in the diff: the unreachable arm now returns the distinct typed `UnsupportedNodeOperation` through an exhaustive `Some(_) \| None` arm instead of the mislabelled `OperandShape`, and a comment cites IR-803. IR-803 exists in the IR team's backlog (the owner of `composite_application_operands`) and asks for a typed operator on `CheckedCompositeOperands` with no raw JSON re-read left in CG. The raw `/operation/identity` read and the unmetered `find` remain until then, and mutant M5 remains an equivalent survivor. Filing it with the upstream owner is an acceptable disposition. |
| FND-002 | accepted-no-change | Round 1. The caller-supplied operator and the pre-invocation `OperatorMismatch` stay by design, and FR-033-AC-1 now states them: the request carries the node's operator, and a different requested operator refuses with typed `OperatorMismatch` before any QSL invocation, with no report or terminal value. That meets the condition this finding named. The overlap with QSL FR-358 step 2 is intended defence in depth and is not a divergent rule. |
