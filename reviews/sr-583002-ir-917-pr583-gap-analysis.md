---
id: SR-583002
title: Gap analysis of IR-917 CG PR 583 (V2 clause harness oracle and arithmetic proof controls)
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-contract-codegen; PR #583 head cc02657484bbd99fdba3ec26326968434ee5a7b8 vs base
  9196cbe40a9341c32b0f85afda22d342dddf193c; src/kani/generate/clause/v2.rs, src/kani/generate/negotiate.rs,
  src/oracle/clause.rs, tests/it/kani_obligations_state_frame.rs; criteria FR-014-AC-38, FR-015-AC-40,
  FR-015-AC-41, FR-015-AC-119, TC-025'
review_set: subset
date: '2026-10-26'
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-014
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
- target: ix://agent-ix/quire-contract-codegen/TC-025
  type: references
---

## Summary

Ticket: IR-917. PR: quire-contract-codegen#583, frozen head cc02657484bbd99fdba3ec26326968434ee5a7b8
over base 9196cbe40a9341c32b0f85afda22d342dddf193c. Reviewer model: Claude Opus 5.5
(`claude-opus-5-5`). Method: quoin gap-analysis, planless. The matrix was computed with
`quire matrix --scope . --format json` at the head (quire 0.36.2, engine 0.50.2; quoin 0.29.1). The
semantic review step was not offered as a separate opt-in. The dispatcher asked for specific intent
checks, and those are recorded per criterion under Coverage.

## Verdict

FAIL for repository-wide traceability by the verdict rule: 315 criteria are untagged at this head and
16 are tagged only by ignored tests. All of these are inherited. The PR adds no criteria, edits no spec
file and removes no trace tag, so it introduces none of them. Scoped to this PR the result is
CONDITIONAL, with one medium and three low findings. No acceptance credit is given: no installed-Kani
verdict exists at this head, and FR-015-AC-40, AC-41 and AC-119 correctly remain PLANNED.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-41's installed-backend clause rests only on the `#[ignore]`d, unrun `tc_025_real_kani_falsifies_both_same_clause_addition_mutants`. The matrix's `tagged` status reflects generation-time assertions only: Verified with cover, and Falsified with playback 999, have no run evidence. | tests/it/kani_obligations_state_frame.rs:767-910 |
| FND-002 | low | FR-015-AC-119 is untagged at this head. Its CG-side generation clauses (one draw, same value to the subject, independent `!left \|\| right`, no helper in oracle or expectation) are asserted by a test tagged AC-38/40/41 rather than AC-119. The installed healthy and helper-body mutant runs belong to quire-integration #24 and have not run. | spec/kani/functional/FR-015-bounded-kani-obligations.md:866 |
| FND-003 | low | `native_clause_expectation_accepts_only_in_range_literal_narrows` is tagged FR-015-AC-40, which is oracle byte-identical embedding. The test asserts the native expectation's literal-narrow admission, which belongs to the AC-41 independent-assertion clause. | src/kani/generate/clause/v2.rs:857-869 |
| FND-004 | low | The literal-narrow elision in clause lowering and in the native expectation is behaviour no criterion states. FR-014-AC-38 lists literals, references and two field terms as operands and does not mention a `quire.op.numeric.narrow` wrapper, and FR-014-AC-45 covers narrowing only for scalar outer operations. The refusal of out-of-range or non-literal narrows is therefore unspecified. | src/oracle/clause.rs:510-525 |

## Coverage

Per criterion, at the head:

- FR-014-AC-38 (Test, TC-024): tagged by 15 tests. The new unit test `a_literal_narrow_is_elided_only_when_its_value_is_inside_the_target`
  asserts the exact retained literal step and four refusals (out-of-range, no bound, non-i128, Boolean
  argument). Its assertions can fail.
- FR-015-AC-40 (Test, TC-025): tagged by 4 tests. The 12-condition loop asserts that the public
  `generate_v2_clause_oracle` source appears exactly once, contiguously, in each general harness:
  connectives, `not`, `implies`, Boolean `=`/`!=`, and the six comparisons. The implication test does
  the same for the native oracle. Same-clause oracle bytes are verified at generation time. PLANNED,
  unchanged.
- FR-015-AC-41 (Test, TC-025): see FND-001. At generation time, the native probe has exactly one
  `evaluate_native_integer_arithmetic` site, and the probe assertion compares the kernel result with an
  independent i128 expression. The mutant test's reachability was checked: `make kani` runs
  `--ignored` with filter `kani_obligations`, which matches the module path. Its assertion strength is
  recorded in SR-583001 FND-001.
- FR-015-AC-119: see FND-002. PLANNED, owned with quire-integration #24 SC-02.
- TC-025 step 11: the generation half is backed. The installed-backend half is unrun.
- Checked literal narrow semantics: the value is preserved and refusals are typed. I did not inspect
  parity of QSL runtime narrow metering, so that is an evidence limit, not a pass.
- General postcondition RT equality: `amount = 0` binds `rt::Integer::from`, calls
  `environment.check_equality`, and typechecks offline in a scratch crate (author-run). The other
  general shapes are not compile-checked (SR-583001 FND-006).
- Reverse gaps: Boolean `eq`/`ne` in the native expectation is owned by FR-015-AC-40. The general-route
  arithmetic refusal is near FR-015-AC-42 scope, which is untagged repository-wide, and is untested
  (SR-583001 FND-002). No stub, `todo!` or placeholder return was introduced.

Evidence limits: this reviewer ran no cargo build, test or Kani, by dispatcher instruction. Focused
test results are author-reported. Plan completion: not assessed.

## New findings (disposition pass 1)

Disposition pass 1 reviewed fix head 738ca6faa267e2966456a2bb26066a70ae19cca4 (rebased onto main
1175b995bc4314798cafe839a85a1ca2a745408c). The matrix was recomputed at that head: 565 tagged, 313
untagged, 16 tagged only by ignored tests, 53 method-without-symbol.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | `general_v2_arithmetic_postcondition_refuses_without_a_harness` is tagged FR-015-AC-42 and is that criterion's only binder. AC-42 states that an unsupported operator (division, modulo, absolute value) is refused with a typed reason naming the node and the operator. The test asserts a general-route render refusal of the supported `+`, and its reason `V2ClauseConditionNotRendered { node_id }` names no operator. AC-42 changes from untagged to tagged without its claim being tested. | tests/it/kani_obligations_state_frame.rs:638-640 |
| FND-006 | low | FR-014-AC-54's two tagged tests cover the unchanged value and the refusal shapes (missing target, out of range, nonliteral, unparseable). They do not cover parity of ordered charges, consumed counters and outcome with direct-owner evaluation, independence from a generated sibling, or refusal of another `convert` identity. AC-54 correctly remains PLANNED. | src/oracle/clause.rs:743-784 |

## New findings (disposition pass 2)

Disposition pass 2 reviewed fix head 0afa590bca21eb2ebcb0afb185911ac82892d9a9. The matrix was
recomputed at that head: 564 tagged, 314 untagged, 16 tagged only by ignored tests, 53
method-without-symbol. FR-015-AC-42 is untagged again.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | medium | FR-015-AC-41 requires each owner-`+` mutant run to be falsified by the independent native assertion with concrete playback `amount_current = 999` for the first. Under that mutant the native assertion fails for every admitted amount 0..=999, so no backend witness can be required to be 999. The test substitutes a separate concrete replay. AC-41 as written is not attainable from backend evidence and needs reconciliation by the spec owner; AC-41 stays PLANNED. | spec/kani/functional/FR-015-bounded-kani-obligations.md:789 |

## New findings (disposition pass 3)

Disposition pass 3 reviewed fix head be9328efa7d0ad383377b55e381a70a19f480813 on main
aa55d2256bebea3d06a583d5febf7c77f77801a2. The matrix was recomputed at that head: 564 tagged, 314
untagged, 16 tagged only by ignored tests, 53 method-without-symbol. FR-015-AC-41 now has four binders,
one of them the ignored installed test. FR-015-AC-40, AC-41 and AC-119 and FR-014-AC-54 remain PLANNED
in spec.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | The new public API (`execute_kani_concrete_playback`, `KaniConcretePlayback`, `KaniConcretePlaybackOutcome`, `KaniExecutionRefusal::ConcretePlaybackSourceMismatch`) has no owning requirement and no interface-001 entry. Its bounds and teardown unit test is tagged FR-015-AC-41, which says "the playback mechanism is not prescribed" and states no bounds, refusals or outcomes. Behaviour such as the source check, the bounded launcher and the outcome variants is therefore unspecified (reverse gap). | src/lib.rs:166-170 |

## New findings (disposition pass 5)

Disposition pass 5 reviewed fix head d571e110df94360c23d8ddad854c5cb1c663d652 on main
4788b46723d7454af86fdbe9c7b4f4ad1154cbb6 (spec PR #586 merged). The matrix was recomputed at that
head: 567 tagged, 314 untagged, 17 tagged only by ignored tests, 53 method-without-symbol.
FR-017-AC-57 and AC-58 are bound to `concrete_playback_uses_the_proof_process_bounds`; AC-59 adds
`concrete_playback_parser_pairs_the_exact_test_and_summary`. FR-017-AC-60 is tagged only by the
ignored installed test, which is red on its healthy proof (MemoryExhausted, IR-1022). FR-015-AC-41,
AC-119, FR-017-AC-57 to AC-60 and FR-014-AC-54 remain PLANNED in spec.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | low | Two FR-017 clauses that TC-027 lists have no playback-entry test. AC-57's absent or unreadable `src/lib.rs` refusal is correct by inspection (`read_file` error maps to `ConcretePlaybackSourceMismatch`) but untested. AC-58's stdout/stderr over-limit and unread typed refusals (TC-027 step 4) are exercised for proof runs only, not through `execute_kani_concrete_playback`. | src/kani/run/execute.rs:2110-2449 |

## New findings (disposition pass 7)

Disposition pass 7 reviewed head 98f7c46c9d14a9150b690fbbd28353f225a184b3 on main
8db3554e0426ece64840e36cb73d1016de99960b. AD-007 is now status `accepted` and still states that
installed healthy and mutant evidence remains required for FR-015 credit. FR-015-AC-41 and AC-119,
FR-017-AC-57 to AC-60, FR-014-AC-54 and TC-025/TC-027 remain PLANNED. FND-001 remains deferred (its
latest outcome): no installed run exists at this head.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | low | The new public `V2SymbolicEncodingPolicy` and the required `V2ClauseObligationIdentity.symbolic_encoding_policy` member are owned only by AD-007, an architecture description. No FR-015 criterion or interface-001 entry states the record member, its two values or the [0, u16::MAX] selection rule. The selection unit test borrows FR-015-AC-39 and AC-47, which state neither. The parent identity struct already lacked an interface-001 entry before this change. | src/kani/identity.rs:158-174 |

## New findings (disposition pass 8)

Disposition pass 8 reviewed head 92efc9c705735a7c765154cb54d93cdc678a8775 (one commit over 98f7c46c,
main 8db3554e). The spec delta only adds text: FR-015 prose, FR-015-AC-128 to AC-131, a TC-025 step 11
paragraph and interface-001 `public_types`. No existing criterion is narrowed. The new criteria are
unmarked and carry no PLANNED/IMPLEMENTED status; they are generation-time controls, and none grants
installed proof credit. FR-015-AC-41 and AC-119, FR-017-AC-57 to AC-60, FR-014-AC-54 and TC-025/TC-027
remain PLANNED.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-011 | low | FR-015-AC-131's "grants no proof credit" is tested only through `execute_kani_concrete_playback` (`concrete_playback_uses_the_proof_process_bounds`, a policy-changed V2 identity refused as `HarnessProvenanceMismatch`). The proof-run entry `execute_kani_obligation` reaches the same `has_valid_provenance` check through `require_in_crate` (execute.rs:451, :612-618), but no test changes a V2 identity's policy and asserts that entry's refusal. | src/kani/run/execute.rs:451-453 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | The installed-backend clause of FR-015-AC-41 still has no run evidence. `tc_025_real_kani_falsifies_both_same_clause_addition_mutants` is ignored and was explicitly not run this round, because CG #569 holds the host. This PR claims no AC-41 acceptance, AC-41 and TC-025 remain PLANNED, and IR-917 stays open until a recorded `make kani` run exists. |
| FND-002 | fixed | `qsl_v2_implication_harness_embeds_its_clause_oracle_and_checks_native_arithmetic` now carries FR-015-AC-119 for its generation clauses. The installed healthy and helper-body runs remain owed by quire-integration #24 SC-02, so AC-119 remains PLANNED. |
| FND-003 | fixed | The narrow unit test is retagged `FR-014-AC-54`, `FR-015-AC-41`, `TC-025`. |
| FND-004 | fixed | FR-014-AC-54 (CG #584, merged to main and present at this head) now specifies checked literal narrowing for V2 clause oracles, including its refusals. |
| FND-005 | fixed | Round 2: `general_v2_arithmetic_postcondition_refuses_without_a_harness` is now tagged `TC-025` only, and FR-015-AC-42 is back to untagged in the computed matrix. |
| FND-006 | fixed | Round 2. Owner parity: the public agreement corpus adds the QSL-authored `self.balance = 0` case, asserts that the package holds a `quire.op.numeric.narrow` node, and compares outcome, ordered admitted charges and every consumed counter with direct-owner evaluation, including denial prefixes. Sibling: the defensive seam asserts every refused narrow, and a refused `quire.op.quantity.convert`, leave a lowered sibling plan unchanged; a QSL twin asserts that the narrow oracle is unchanged across sibling generation. Convert: `quantity.convert` is refused. AC-54 remains PLANNED. |
| FND-007 | fixed | Round 3: merged CG #585 rewrites FR-015-AC-41 and TC-025 step 11. The unconstrained installed-Kani mutants may select any in-domain witness, and the exact 999 control is a separate deterministic playback that must execute the actual generated arithmetic assertion and attribute failure by message or location. The test at this head matches that wording. Acceptance remains PLANNED: no installed run exists. |
| FND-008 | deferred | Round 4: still unspecified at this head. Draft, unmerged CG #586 (head 322f78313264326b4e83918e7d81e876522334e5) adds FR-017-AC-57/58/59, TC-027 "Concrete playback execution" steps 1-4 and an interface-001 operation entry. Read at that head, they own the public entry, its source and provenance refusals, the bounded launch, the outcome variants and `ConcretePlaybackTestNotObserved`. Closes when #586 merges and the playback unit test is retagged from FR-015-AC-41 to FR-017-AC-57-59. |
| FND-008 | fixed | Round 5: CG #586 is merged into this PR's base (4788b467). FR-017-AC-57 to AC-60, TC-027 "Concrete playback execution" and the interface-001 `execute_kani_concrete_playback` entry own the public API. Implementation matches: V2-only `ConcretePlaybackUnsupportedHarness`, full sealed-artifact restoration including header and oracle, `kani playback --lib -Z function-contracts -Z concrete-playback -- <name> --exact --nocapture` (V2 harness options carry no `-Z stubbing`), and the AC-59 parser. The unit tests are retagged FR-017-AC-57/58/59 with TC-027, and the installed test adds FR-017-AC-60 and TC-027. |
| FND-009 | fixed | Round 6 (head 3fa8bce4fa8d61ca7c0b59f03952b47b5de2e62d, a test-only delta in `src/kani/run/execute.rs`): `concrete_playback_uses_the_proof_process_bounds` now drives the public `execute_kani_concrete_playback` entry through stand-ins for each case. An absent `src/lib.rs` and a directory in its place each refuse `ConcretePlaybackSourceMismatch` with zero launcher calls. More than `CAPTURE_LIMIT` bytes on stdout, then on stderr, each refuse `OutputOverLimit { stream, limit: CAPTURE_LIMIT, harnesses: 1 }` after one launch. An injected poll failure on each stream refuses `OutputUnread { stream }` after one launch, with the gate's failure flag observed. Every case is an `Err`, never a `Completed` outcome, so none confers playback or proof credit. Production source, sealed-artifact checks and gate scope are unchanged. Verified by inspection only; I did not run the test. |
| FND-010 | fixed | Round 8: FR-015 now owns the policy in SHALL prose and FR-015-AC-128 (endpoint selection), AC-129 (sealed record member), AC-130 (one i64 draw, exact assumption, 8-byte witness, rebind only under the u16 policy) and AC-131 (byte-identical regeneration, policy-change provenance refusal). TC-025 step 11 adds the matching inspection. interface-001 adds `public_types` entries for `V2SymbolicEncodingPolicy` and the required `V2ClauseObligationIdentity.symbolic_encoding_policy` / `identity.symbolicEncodingPolicy` contract. The selection unit test is retagged FR-015-AC-128; the generated-source tests carry AC-128/129/130; the decoder test carries AC-130; the determinism and playback-provenance tests carry AC-131. |
