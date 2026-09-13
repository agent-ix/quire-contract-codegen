---
id: REV-022
title: "Scope-boundary review of the numeric/state strategy slice"
type: SpecReview
analysis: scope-boundary
scope: "FR-008..FR-013, NFR-004, TC-017..TC-022, interface-001 bound_strategy_slice"
review_set: subset
---
# Scope-boundary review of the numeric/state strategy slice

## Summary

Reviewed `agent-e/codegen-3-numeric-strategies-spec` at `6f73e2d` against base `240fad8`. The
assignment comes from agent-ix/quire-contract-codegen#3 (the 2026-09-13 numeric/state comment) under
epic agent-ix/quire-spec-language#83. The slice was also compared with Agent A's in-flight codegen#4
spec (`task/4-numeric-state-oracles` at `393c6a1`: FR-001, interface-001 `oracle_slice`, test
matrix). It was checked against the generated harness and strategy code at base (`src/harness.rs`,
`src/strategy.rs`) and against `quire-contract-runtime` at `8a4d02b`.

What the slice gets right about scope:

- It covers every #3 deliverable and acceptance item: constructive correlated pre/post populations,
  a reported discard rate, constraint-preserving shrinking, boundary campaigns inside and outside
  each domain edge, and IT-010 consumption with no local wire schema.
- It reuses Quoin's `ProofAttestationV1` and adds no `schemas/` file.
- It does not specify Kani (#2), the SL pin bump, IT-010 execution (#84), SL#68, or monitoring.

Where the justification holds:

- FR-010 excludes checked-arithmetic overflow edges. That is correct. codegen#4's FR-001 revision
  refuses numeric arithmetic and negation, and it refuses every definedness obligation until an
  IR/runtime invalid-result API exists. No #4-admitted clause has an arithmetic overflow edge.
- #3 itself never asked for overflow edges. The `checked_*` helpers are #4's deliverable.
- Domain edges at `i64::MIN`/`i64::MAX` are still covered by FR-009-AC-5 and FR-010-AC-3.

The boundary problems fall into four groups:

1. **Admission is copied, not coupled.** FR-008 restates a grammar of its own instead of deriving
   it from #4's admission. It refuses forms #4 admits and adds a refusal #4 does not have.
2. **FR-011 goes past #3 into runtime-owned accounting.** It adds a new `OutOfDomain` tag, an
   `out_of_domain` counter, consumer domain-admission, and two rate accessors. The counter sits
   outside `quire-contract-runtime`'s four-counter `CampaignCounts`. The tag contradicts the existing
   integer `Boundary` semantics, where out-of-domain means `Rejected`.
3. **FR-013 promises a runtime observation type that does not exist.** It also binds package
   identity through argv rather than FR-001's input-digest field.
4. **Housekeeping edits outside #3 collide with Agent A's branch.** These are the test-matrix header
   and paragraph edits and the index plan to relocate #4/#2-owned files.

No high-severity finding.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-22001 | medium | **FR-008 copies the admission grammar instead of reusing #4's.** It restates the grammar and only asserts "never looser" through FR-008-AC-5, which is a corpus sample. #4's FR-001 revision admits Boolean `not`/connectives over the six integer comparisons; FR-008 refuses them. FR-008 also adds an `UnsupportedOverflowPolicy` refusal for `Saturate` that #4 lacks. That refusal is irrelevant to compare-only clauses and unreachable from SL, which lowers only `OverflowPolicy::Reject`. Two hand-maintained grammars will drift. **Fix:** make admission call #4's bound-oracle admission first, then apply a named, enumerated strategy restriction (single `Compare` root). List each #4-admitted form refused here, with the reason, in `bound_strategy_slice.out_of_scope`. Drop the `Saturate` refusal, or justify it. Pin the FR-001 revision that FR-008 depends on (#4 is unmerged). | FR-008 Behavior, FR-008-AC-5, interface-001 `bound_strategy_slice.admission`, task/4 FR-001 / `oracle_slice.supported_expression_grammar` | wrong-requirement |
| FND-22002 | medium | **FR-011 invents out-of-domain semantics that conflict with the existing harness and runtime.** It adds an `OutOfDomain` tag, a consumer-supplied domain-admission result, and a separate `out_of_domain` counter. At base, integer `Boundary` cases just outside the domain are tagged `Rejected` and verified as `RejectedPrecondition` (`src/strategy.rs` `ExpectedDomain`, FR-002 "accepted or rejected domain check"). `quire-contract-runtime` `CampaignCounts` has exactly accepted, rejected, failed and discarded, and its snapshot carries a counter-semantics version. A fifth counter is either a runtime/Quoin accounting change owned elsewhere or a local shadow counter outside shared accounting. #3 only asks for cases just outside the domain edge. **Fix:** reuse the existing `Rejected`/`RejectedPrecondition` mapping for out-of-domain values and drop the new counter. If a distinct out-of-domain disposition is truly required, record it as a runtime API request under the qcir/runtime owner, not a codegen-local counter. | FR-011 Behavior and AC-2, FR-010 Outputs, interface-001 `bound_strategy_slice.verdict_mapping` and `rates`, FR-002 Behavior, `src/strategy.rs` | wrong-requirement |
| FND-22003 | medium | **FR-011 assumes a numeric harness that no requirement specifies.** It maps numeric cases onto the "existing generated campaign runner", but the base harness is Boolean-only: `bool` inputs, `&mut bool` state, and a precondition/postcondition pair built from `generate_boolean_oracle`. Running `Pre`/`Post`-anchored `i64` clauses needs the harness generalized to `i64` input/state bindings and a single anchored clause. It also needs a stated dependency on consuming #4's generated oracle signature. No FR states the binding shape, the subject/snapshot contract, or that the oracle is consumed rather than produced here. **Fix:** add behavior (in FR-011 or a sibling FR) for `i64` harness bindings and the single-clause anchor. State that the verdict adapter calls #4's generated bound oracle and never lowers the clause itself. | FR-011, FR-002 Behavior, interface-001 `harness_strategy_slice`, `src/harness.rs` | missing-requirement |
| FND-22004 | low | **The rate accessors are an unforced addition to the shared summary.** #3 requires only that the discard rate be reported. interface-001 `accounting_unit` already retains exact numerators and the `attempted` denominator. FR-011 adds `discard_rate()` and `rejection_rate()` to the shared generated summary, which also changes PR #22's Boolean/integer runner surface. **Fix:** satisfy "discard rate is reported" through the existing `discarded`/`attempted` fields plus an AC that asserts them. Otherwise, state that the accessors apply only to bound-strategy summaries and confirm the Boolean runner output stays byte-identical. | FR-011 Outputs and AC-3/AC-4, interface-001 `accounting_unit`, #3 acceptance | wrong-requirement |
| FND-22005 | medium | **FR-013 needs a runtime type that does not exist, and binds identity the wrong way.** It requires generated case accessors returning "the runtime observation kind", but `quire-contract-runtime@8a4d02b` exports no pre/post/current observation enum (only `ClauseKind`, `ClauseOutcome`, `VerdictKind`, `FailureKind`). Meeting FR-013 would need a runtime change or a codegen-local type, and a local type is the mapping table the requirement says consumers must not need. FR-013 also puts the `BoundPackage` digest and `ClauseRef` in attestation argv. FR-001 binds the bound input through the existing input-digest field and says the command stays a library-call description. **Fix:** either name an existing runtime type or declare a generated `&'static str` observation label. Bind package identity the way FR-001 does, not through argv. | FR-013 Outputs, Behavior, AC-4, interface-001 `bound_strategy_slice.consumer`, FR-001 Behavior (bound batch attestation binding) | wrong-requirement |
| FND-22006 | low | **FR-013-AC-5 gates on Agent A's ticket.** It gates merge on linking the interface on agent-ix/quire-spec-language#84, which Agent A owns. FR-013's description also narrates IT-010's procedure ("feed each case into `runtime::execute`, and compare verdicts"), which is #84's scope. **Fix:** make AC-5 a codegen-side inspection (interface-001 records the consumed surface at a pinned revision). Reword the description as a consumer assumption, not a statement of what #84 does. | FR-013 Description and AC-5, test-matrix FR-013-AC-5 row | wrong-requirement |
| FND-22007 | low | **State-pinned and no-event campaigns are silently missing.** #3's deliverable asks for "broad, boundary, state-pinned, and no-event campaigns where applicable", and base `StrategyCampaign` already has `StatePinned` and `NoEvent`. The bound slice offers only `Satisfying`, `Violating`, `Broad` and `Boundary`, and neither covers nor defers the other two. `VersionUnchanged` is itself a no-event relation. **Fix:** add `StatePinned` and `NoEvent` populations for admitted state-scalar clauses, or list them in `bound_strategy_slice.out_of_scope` with the reason. | interface-001 `generate_bound_strategy` inputs and `out_of_scope`, FR-009 Inputs, #3 Deliverables, `src/strategy.rs` `StrategyCampaign` | missing-requirement |
| FND-22008 | low | **The overflow-edge exclusion does not name who owns the gap.** FR-010 excludes checked-arithmetic overflow edges "until the arithmetic slice is specified". The justification holds (see Summary), but the text names no owner or trigger. **Fix:** cite codegen#4's arithmetic/negation refusal and the qcir/runtime undefined-result decision under agent-ix/quire-spec-language#83 as the gating owner. State that admitting arithmetic in #4 reopens FR-008, FR-009 and FR-010. | FR-010 Behavior (last bullet), interface-001 `bound_strategy_slice.out_of_scope` | wrong-requirement |
| FND-22009 | low | **FR-008 treats Boolean declarations inconsistently.** It refuses "not an integer and not a Boolean" declarations with `UnsupportedDeclarationType`, which implies Boolean reads are admitted. Its admission rules, though, require every operand to be an integer `ValueReference` or `IntegerLiteral`, and `bound_strategy_slice.admission` says integer only. Which code a Boolean read gets is unspecified. **Fix:** state that a Boolean-typed read is refused, and with which code. Keep Boolean clauses routed to the existing Boolean harness path, and record that in `out_of_scope`. | FR-008 Behavior, interface-001 `bound_strategy_slice.admission` and `refusals` | wrong-requirement |
| FND-22010 | low | **The new refusal fields are not added to the shared diagnostic contract.** FR-008 requires every `StrategyDiagnostic` to carry the full `ClauseRef` and a source span, but base `StrategyDiagnostic` has neither field, and interface-001 `diagnostics.fields` is not updated here. #4's branch separately adds "optional exact IR source span" to the oracle diagnostic fields. **Fix:** add the `ClauseRef` and span fields to the strategy diagnostic contract in interface-001, reusing #4's span field definition rather than a parallel one. | FR-008 Behavior, interface-001 `diagnostics.fields`, `src/strategy.rs` `StrategyDiagnostic`, task/4 interface-001 | missing-requirement |
| FND-22011 | low | **Housekeeping edits outside #3 overlap Agent A's branch.** The slice renames the test-matrix `Coverage Status` columns and rewrites the upstream-conflict paragraph. `task/4-numeric-state-oracles` makes the same rename with different paragraph text and restructures the NFR table, so the two will merge-conflict. The slice also adds an index plan to move #4/#2-owned FR-001, FR-003 and TC files into subsystem directories. **Fix:** limit this slice's matrix change to the FR-008..FR-013, NFR-004 and TC-017..TC-022 rows. Leave the column rename and paragraph to whichever branch lands first, and drop the relocation plan for files other owners are editing. | spec/test-matrix.md, spec/index.md "Subsystem layout" | wrong-requirement |
| FND-22012 | low | **The matrix does not trace shrinking evidence back to FR-002-AC-4.** FR-012 says it closes FR-002-AC-4 for the numeric/state slice, and TC-021 declares `verifies FR-002`. The FR-002 matrix row still maps AC-4 only to TC-004. **Fix:** add TC-021 against FR-002-AC-4 (numeric/state slice) in the matrix, or drop the "closes FR-002-AC-4" claim. | FR-012 Description, TC-021 relationships, spec/test-matrix.md FR-002 row | correct-requirement-no-evidence |

## Resolution

Verified against the uncommitted rework on top of `6f73e2d`. "partially resolved" rows name the
remaining gap and stay open until the spec is amended.

| Finding | Disposition | Evidence |
|---|---|---|
| FND-22001 | resolved | FR-008 Upstream and interface-001 `depends_on` pin `task/4-numeric-state-oracles` at 393c6a1; interface-001 `out_of_scope` lists Boolean literal/reference/negation roots, same-read Compares, and Current mixed with Pre/Post |
| FND-22002 | resolved | The `OutOfDomain` tag and counter are removed; out-of-domain cases are an untagged generated array for consumer domain-admission tests and the codegen runner never evaluates them (FR-010:29, FR-011:61, interface-001:133); runtime `CampaignCounts` is unchanged |
| FND-22003 | resolved | The numeric subject harness is out of scope (FR-011:24-26, interface-001:139); the runner calls the generated oracle embedded from bound oracle generation (FR-011:32) and never lowers the clause itself |
| FND-22004 | resolved | FR-011:66-67 and interface-001:136 emit the accessors on the bound-strategy summary only and leave the PR #22 harness summary unchanged |
| FND-22005 | resolved | FR-013:36-37 uses a `&'static str` observation name; FR-013:56-58 binds through `--input-digest` |
| FND-22006 | resolved | FR-013-AC-5 (FR-013:69) is a codegen-side inspection of interface-001 `consumer`; FR-013:19-23 describes the consumer as an assumption |
| FND-22007 | resolved | interface-001:139 lists `StatePinned` and `NoEvent` as out of scope, with the reason |
| FND-22008 | resolved | see FND-19005; operands are restricted to integer reads and `IntegerLiteral` |
| FND-22009 | resolved | see FND-19005 |
| FND-22010 | resolved | FR-008 and interface-001 name the span field codegen#4 adds to `diagnostics.fields` |
| FND-22011 | partially resolved | Declined in part: the spec/index.md:87-96 relocation plan is kept on the owner's instruction to organize specs by subsystem. Matrix conflict: the header, paragraph and NFR table now copy 393c6a1 verbatim (test-matrix:11,45-47,64-70,76). But `git merge-file` of this matrix against 393c6a1 over base 240fad8 still reports 3 conflict hunks, at the FR-008..FR-013 rows (test-matrix:36-43), the paragraph at test-matrix:59-60, and the NFR-004 rows (test-matrix:71-72), each an addition against an empty side. Fix: rebase onto codegen#4 after it merges, or resolve those 3 hunks as unions |
| FND-22012 | resolved | FR-012:19-20 refines FR-002-AC-4 and leaves it verified by TC-004; TC-021:5-7 drops `verifies FR-002` |
