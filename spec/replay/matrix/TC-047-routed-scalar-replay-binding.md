---
id: TC-047
title: "Check public routed scalar lowering replay and staged-content binding"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: references
---
# TC-047: Check public routed scalar lowering replay and staged-content binding

## Description

Planned public-consumer coverage of
[FR-032](../functional/FR-032-routed-scalar-replay-binding.md), distinguishing lowering parity
failure from source-property refutation. Partial native evidence exists (IR-1184: #702, #741,
#757, #481): `scalar_playback.rs`, `scalar_literal_operands.rs`, `scalar_replay_binding.rs` and
`scalar_replay_residuals.rs` drive the real routed generation, the real QSL operator facade and
a compiled generated artifact, without a Kani run. `scalar_settlement.rs` implements the
settlement steps 10 to 14 (IR-631), and `kani_obligations.rs` implements the descriptor-level
bounded-addition check of step 2. `scalar_replay_acceptance.rs` implements the setup-precedence, single-member binding, stale-dependency and invented-membership checks (steps 3, 6 and 8, FR-032-AC-2, AC-6, AC-12, AC-17, AC-18). Steps 1, 7 and 9 and the real-Kani run of step 2 remain
Planned, as does any real falsified Kani run through the route. No
passing proof-driven replay is claimed.

## Test Procedure

1. Obtain a real QSL-emitted checked package for `x + 1` with input `Int[0,9]` and result
   `Int[0,10]`; check those emitted ranges, then call public
   `generate_routed` for its scalar node and retain the returned identity and generated harness.
   Compile a public scalar builder call using those types directly, without a function identity.
2. In the named real-Kani lane, run the generated harness and an arithmetic mutation control.
   Retain its actual selected assertion playback and proved generated content. This increment
   has no admitted out-of-result-range exact value; do not invent one. Separately, on the
   descriptor `kani_obligations::scalar_package` accepts, with both operands and result in
   `[-1000,1000]`, assert the operand bounds and the harness's checked result domain, decode a
   playback of an in-domain pair (for example `(600,600)` giving `1200`), require the adjacent
   out-of-domain operand to be refused, and derive the pair's exact sum and assert it is outside
   the result range. QSL's checker refuses a source with those checked ranges (`Unproved(Range)`),
   so no QSL-emitted package exists for this case and its native observation and
   harness-assertion mutation are not part of this step. An arithmetic mutation of the
   increment fixture supplies the divergence control. The observed increment ranges must be
   asserted; a hand-built package never replaces QSL admission.
3. Build the scalar plan from the actual identity, decoded playback, proving package and
   retained staged path and byte set of that same proved generated oracle and the retained original generation
   limits context. Check setup precedence with one defect at a time: context/identity/schema,
   selected playback bytes above 8 MiB, adapter block/harness/arity/width, then operand domain.
   Those distinct refusals occur before an absent-capability check. Only an otherwise valid
   request reaches the unsupported-capability refusals of FR-032-AC-24; conversion of any of them returns no settlement.
   Try missing limits context and replay limits altered from the original retained context;
   no current `ScalarObligationIdentity.limits` member or default is assumed. The original
   context is recorded by the scalar generation authority alongside the proved source.
4. For the admitted upstream route, record the exact QSL request's operator, operands, operand
   ranges, result range and retained original limits, plus its QSL obligation identity and
   original `call_site` package/source tie required by AD-002 R-6/R-7. Check package/source and
   scalar-node membership mismatches through the upstream typed refusal. Operand correspondence
   remains governed by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-16.
   Execute the same proved oracle artifact natively over those operands and limits, following the
   driver authentication contract, and retain the typed outcome and its checked completion.
   CG supplies the original generation/proving record; the driver authenticates the observation
   by comparing the receipt's staged bytes with its original artifacts, alongside the decoded vector and retained limits. Attempt
   replay with a bare echoed outcome, without authenticated observation, with another
   artifact's observation and with a regenerated correct oracle replacing the proved arithmetic
   mutation. Each must refuse before QSL evaluation; QSL receives no artifact and cannot be
   credited with authenticating the driver's execution.
5. Call `replay_operator_parity(wire, claim, replay_limits)` in the public consumer, with
   retained `ReplayLimits` last; keep that byte-admission guard distinct from the claim's
   retained `ScalarLimits` and do not invent a report identity member for it. Then pass the
   result to the CG converter.
   Compare generated/exact divergence, agreement despite retained Kani falsification, a QSL
   non-fault refusal and an executor fault. Distinguish native `Incomplete`/execution fault
   (`GeneratedFault`, `Failed`) from exact limit exhaustion (`Incomplete(ResourceExhausted)`);
   verify `RefusedInput` keeps `invalid_runtime_input` and other QSL refusals follow the typed map.
   Check all four supported integer operators and
   refuse an undefined/incomplete generated outcome as agreement; those outcomes do not
   satisfy [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37. Change the
   actual generated arithmetic, not the
   claimed replay verdict. Error-path seam observations may use a recording/failing executor;
   the positive evaluation must use QSL and the actual generated artifact.
6. For every `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete` and
   other `Refused` report produced by the actual public QSL facade, pass that immutable report
   with CG's retained expected sent `OperatorIdentity` and checked run/context completion to
   the converter. Compare its full `claim()` before settlement. Hold the real report fixed and
   change only one retained expected member at a time: obligation, node,
   occurrence, kind, operator, operand identity/value/range, result range, limits or native
   outcome. Require binding refusal with no settlement. Keep the obligation digest equal for
   the other member mutations; a native-outcome-only difference cannot be found by the digest.
   Do not mutate or fabricate QSL's private report representation. A faithful report of a
   mutated request still equals that request's sent claim. Separately bind a report from
   another run and require refusal. Retain the actual wire used by checked driver completion
   and compare it against the original proving context. Substitute a consistently compiled
   package, another enclosing function, source bytes, dependencies or limits while keeping
   the claim equal; require no settlement. An echoed expected context cannot establish the
   actual sent context. Pass valid settlements, and no settlement, to the scalar pair functions of steps 11 and 12.
   The converter-refusal half of these tests is exercised by `scalar_replay_binding.rs`; the
   settled half is step 10.

7. Using the positional rule measured in merged QSL change #650, after
   IR-648's authoritative typed `CheckedPackageV2` operand access (Done), independently
   inspect the scalar preimage passed to the public shared `parity_obligation` mint function. Use the claimed operator application node and its actual selected
   expression occurrence. With exactly one such occurrence, retain it; with zero or multiple
   expression occurrences require typed unavailable/ambiguous refusal and no identity, never
   select the first ordinal. Multi-occurrence replay awaits owning generation selection.
   Use the sole scalar-family allocation in FR-032; its typed generation source/retention
   remains CODE-gated, with no free caller string or clause-role substitution.
   Check domain provenance by position: own bounded-domain parameters/subterms use their
   checked derived interval; inline or referenced literal nodes use the actual singleton;
   an untyped nonliteral subterm uses the enclosing claim's first checked result bound as
   current generation does. Missing provenance or widening any retained assumed range refuses.
   Retain one argument per operand/harness draw position, each with its
   position ordinal, typed `OperandIdentity` and actual range. An actual graph reference uses
   `GraphChild { node_id }`, including parameters, subterms or referenced existing literal nodes.
   An inline literal uses `InlineLiteral { node_id, occurrence_key, position }` from the authentic
   application node/selected occurrence and position ordinal; its singleton range carries the
   literal value. Assert no extra literal node is required or fabricated. Draw one parameter
   independently at two positions and assert two distinct
   position entries; identifier sorting and node-id deduplication must fail. Exclude unrelated
   enclosing-function parameters. Change each included member and require an identity change;
   changing source span or native outcome leaves it unchanged.
   Missing selected occurrence, authentic operand identity/range or scalar kind whose typed retention is gated refuses with
   no identity. IR graph/source-map access supplies node occurrence data, but raw JSON or
   an invented literal node is not the typed scalar accessor; IR-648 (Done) exposes
   the same tagged operand identity.

8. Using merged QSL O-09 and the public shared `qsl_replay::parity_obligation` mint function,
   check CG calls that owner through its existing dependency and implements no second encoder.
   Verify independently from current-run authoritative metadata that the canonical scalar
   preimage has the exact closed members,
   tags, lowercase node hex, numeric ordinals/positions and decimal-string bounds specified
   in FR-032: arguments `{domain, operand, position}`, scalar domain
   `{lower, tag: "range", upper}`. Send the CG-minted identity to the actual operator entry
   and require acceptance; mutate the digest alone and require `ScalarIdentity::Obligation`
   with claimed/recomputed digests and `stale_dependency`/`revision-mismatch` before comparison.
   Repeat with an otherwise membership-valid changed range/kind or occurrence and retained
   digest; invalid identity/occurrence mutations instead reach the prior membership refusal.
   A carry-only report or stand-in verifier cannot pass. Independently mint a self-consistent
   claim with invented occurrence/graph-child metadata or wrong inline literal value/singleton
   range; require the merged typed membership refusal before identity comparison. Use the public owning mint with an
   unencodable preimage and require `IdentityEncodeError`, no identity and typed CG setup
   refusal before replay. An invented occurrence sent to the facade instead reaches its
   earlier membership refusal; do not fabricate a private Encoding report. Keep original function/clause carry-only behavior and existing
   function/frame encodings.
   These are planned, unrun CG conformance checks.
9. For FR-032-AC-19, decode synthetic Kani playback against the identity of a guarded
   FR-022-AC-33 harness with one integer guard input and of an FR-022-AC-27 harness with one
   Boolean guard input: valid entries, one entry short, an integer guard entry of four bytes, a
   Boolean byte `2`, and an integer guard value one outside its narrowed domain. Decode the
   unguarded `x + 1` identity unchanged.

10. Settlement (FR-032-AC-20, AC-21). With the `x + x` over `Int[0,9]` fixture of
    `scalar_replay_binding.rs` (the routed generation path), build a plan, complete it with the
    genuine observation and call the real `replay_operator_parity` for each of: the unchanged
    observation (`Agrees`); the same artifact bytes with the driver-asserted outcome
    `Completed(7)` (`Diverged`); the outcome `Incomplete` and then `ExecutionFault`
    (`GeneratedFault`). Pass each report and the actual sent request to
    `CompletedScalarReplay::settlement` and assert `ScalarSettlement::Parity` over that
    report's result, and that `scalar_run_terminal_value` equals `report.terminal_value()` and the
    FR-032 table value. `RefusedInput`, exact `Incomplete` and non-fault `Refused` are unreachable
    through a completed plan: CG refuses out-of-domain operands first, the original limits are the
    renderer's `u64::MAX`, and `plan` itself decodes the request under the context's replay limits
    (a too-small `input_bytes` fails there as `ScalarBuildError::Context`, before any QSL report).
    Pass genuine facade reports for a claim with an out-of-range operand, with
    `limits.work_units = 0` and with a changed occurrence ordinal (the claim of member 5 in
    `scalar_replay_binding.rs`) straight to `ScalarSettlement::Parity` and assert
    `Inconclusive(ReplayRefused(invalid_runtime_input))`, `Incomplete(ResourceExhausted)` and
    `Inconclusive(ReplayRefused)` carrying the report's catalog code. Inspect the mapping function
    for the `#[deny(clippy::wildcard_enum_match_arm)]` attribute and for arms that return
    `result.terminal_value()`.
11. Pairing (FR-032-AC-22). For each report of step 10, call `scalar_run_terminal_value` on a
    falsified evidence value and `scalar_ir_outcome_terminal_value` on a `Counterexample`
    outcome and require equal values. Collect every value and assert none is `Refuted`,
    `Proved`, `Tested` or `Inconclusive(ReplayParity)`. Pass a settlement with a `Verified`
    run and with a non-counterexample IR outcome and require `UnexpectedSettlement`. Pass `None`
    for non-falsified outcomes and require the ordinary values.
12. Missing settlement (FR-032-AC-23). Pair falsified evidence and a `Counterexample` outcome with
    `None` and require `TerminalPairError::MissingSettlement` from both pair functions; the pair
    functions read only the evidence or outcome and the settlement, so no routed fixture
    changes this result. Then, with the completed plan of step 10, call `settlement` with an
    absent report and with a report of another run, and pass each
    `settlement(...).ok()` itself, never a literal `None`, to both pair functions; require
    `MissingSettlement` again.

13. Setup defects (FR-032-AC-24). Produce a real `ScalarBuildError` of each reachable kind: a
    four-byte operand (`Playback`), a changed body (`BodyMismatch`), changed artifacts
    (`Artifacts`), a mutated observation (`Observation`), a changed `harness.rust` so that
    `scalar_replay_body` returns `GeneratedArtifactChanged` (`Authority`, tamper), a routed
    Boolean `Implies` scalar harness (the `boolean_implies_scalar` fixture of
    `kani_obligations.rs`), whose `scalar_replay_body` returns `ScalarAuthorityError::Unavailable`
    (`Authority`, unavailable capability), an `OriginalScalarContext::new` call with an unknown
    function name and a plan under a too-small `input_bytes` (`Context`), and through
    `settlement` a changed budget (`SentContext`) and another claim's report (`Report`). Assert
    `setup_settlement` returns `None` for the tamper, unavailable-capability and binding
    variants and `Some(CgDefect)` for `Playback`, and that the pair functions give
    `MissingSettlement` and `Failed` respectively, so a falsified `Implies` run yields no
    terminal record. A test-local exhaustive `match` over `ScalarBuildError` and over
    `ScalarAuthorityError` asserts the classification for the causes no public input reaches
    (`Staging`, `Identity`, `ScalarBuildError::Operation` and the `Occurrence`, `Operands`,
    `Operation`, `Ranges` and `Record` authority causes; `Staging`, `Identity`, and the `Operation`, `Ranges` and `Record` authority
    causes are `CgDefect`, the rest `None`), so a new variant fails the build. Neither cause site
    of `Authority(Operation)` (a non-application generated node, or a harness operation that
    differs from the admitted node's) can be driven through the public API, because the harness
    fields are sealed after generation; it is tested only by this local match, which pins the
    `CgDefect` reading, and by Inspection of the two sites.
14. Typed seam (FR-032-AC-25). Compile the full flow of steps 1 and 10 to 12 using only
    `ScalarObligationIdentity`. Scan `src/replay/scalar_builder.rs` and `src/lib.rs` re-exports
    for `KaniObligationIdentity`, CG's `ReplayPackage` (not `qsl_replay::ReplayPackage`) and a
    `settle` method on `CompletedScalarReplay`; require none.

## Expected Results

| Criterion | Planned observation | Mutation that must fail the check |
|---|---|---|
| FR-032-AC-1 | Public typed consumer uses scalar identity; unavailable named capability refuses after setup; all named capabilities available yields an admitted plan | Require function identity, admit a synthetic predicate, or continue refusing after every named capability is available |
| FR-032-AC-2 | Adapter-decoded playback uses persisted order; builder rejects above 8 MiB before parsing | Decode in alphabetic order, ignore another harness name or discard extra bytes |
| FR-032-AC-3 | Operand endpoints and request operator/ranges agree with the harness/package; limits match original generation context, including AD-002 identities | Skip a bound check, replace operator/node, use result bounds for a literal or swap operands |
| FR-032-AC-4 | Driver authenticates same-proved-artifact observation by direct staged-byte comparison and checked completion ties operands/limits/content | Accept an unchecked outcome echo, treat playback as a generated result or accept fresh regeneration that removes the mutation |
| FR-032-AC-5 | Driver executes the proved artifact and calls QSL between builder and converter; QSL measures the current four-operator completed-value/refused-outside-range projection | Execute inside the CG builder, substitute a local evaluator, stub the positive verdict, accept `Undefined`/`Incomplete` as agreement or add unproved refusal-cause/accounting-counter equality |
| FR-032-AC-6 | Immutable full report claim and actual sent context match the retained binding/original proving context on every outcome, or no settlement | Check only the obligation digest or agreement; omit node/operator/operand range/result range/limits/native outcome on a refusal, exact Incomplete, generated fault or divergence |
| FR-032-AC-7 | Divergence/generated fault is `Failed`, agreement is `Inconclusive(ScalarAgrees)`, exact exhaustion is `Incomplete(ResourceExhausted)` and typed QSL refusal keeps its code; never `Refuted`/`Verified` | Convert every Kani falsification to `Reproduced`, fabricate a predicate disagreement or turn a fault into a data refusal |
| FR-032-AC-8 | Increment emission/result range is observed and its reachable exact sums are derived from the generated identity and shown inside the result range; the descriptor-level bounded-addition check asserts the `[-1000,1000]` bounds and checked result domain, refuses the adjacent operand and derives a chosen pair's exact sum outside the result range, with no claim of native observation for that case | Use a hand-built function harness, substitute increment as the out-of-range case, compare two hard-coded intervals instead of deriving them, relabel an expected refusal as source falsification, accept wrong emitted fixture ranges or claim native observation of the bounded-addition case |
| FR-032-AC-9 | Scalar positional shape measured in merged QSL change #650: actual application/expression occurrence/kind and one ordinal/tagged-identity/range entry per draw; `GraphChild` for graph references, `InlineLiteral` for application/occurrence/position, literal value in singleton range, repeated parameter draws distinct | Invent occurrence/operand ids or extra literal nodes, omit or swap identity tags, sort by identifier, deduplicate repeated nodes, omit a literal position, add unrelated function parameters/content/outcome/tracking fields or narrow a harness range |
| FR-032-AC-10 | Shared mint conforms to independently checked O-09 scalar bytes and actual QSL accepts the identity once kind/metadata authority is available | Use a copied encoder, stand-in verifier or synthetic acceptance |
| FR-032-AC-11 | Digest mismatch yields typed Obligation refusal with both digests and owning codes | Retain old digest after positional/occurrence/kind change and accept |
| FR-032-AC-12 | Correctly hashed invented occurrence/child or false literal value/range yields typed membership refusal | Accept self-consistent hashing as membership authority |
| FR-032-AC-13 | Public mint refuses an unencodable preimage; CG setup returns no identity or replay | Return a usable sentinel identity or accept an unencodable claim |
| FR-032-AC-14 | Scalar addition preserves function/clause carry-only and function/frame identity rules | Redirect other paths through scalar positional parity |
| FR-032-AC-19 | Guard entries follow the operands in `guard_inputs` order; a short playback is `kani_witness_arity_mismatch`, a wrong width `kani_witness_width_mismatch`, a Boolean byte `2` or an out-of-domain guard value `cg_scalar_witness_guard_out_of_domain`; guard values never reach QSL as operands; an identity without guard inputs decodes as before | Count only operands, decode a Boolean as eight bytes, skip the guard domain check or append guard values to the QSL operand vector |
| FR-032-AC-20 | Step 10: `settlement` returns `Parity` over the genuine report's result after the AC-6 checks and no `ScalarSettlement` otherwise; `settle` is gone | Skip the sent-context comparison (the changed-`input_bytes` report then settles); return `ScalarSettlement::CgDefect` for an absent report instead of `Err` (the `settlement(...).ok()` input then yields `Failed`, not `MissingSettlement`); keep `settle` alongside (step 14 finds it) |
| FR-032-AC-21 | Step 10: the six result classes read as the table; equal to `report.terminal_value()` | Map `Diverged` to `Refuted` (fails the `Completed(7)` input); map `Agrees` to `Failed` (fails the unchanged observation); map `RefusedInput` to `Failed` (fails the out-of-range operand report); map `Incomplete` to `Incomplete(TimedOut)` (fails the `work_units = 0` report); read every `Refused` as `Failed` (fails the changed-occurrence report, which must carry a catalog code); replace a `result.terminal_value()` arm with a wildcard (fails the lint and the inspection) |
| FR-032-AC-22 | Step 11: settlement pairs only with falsified or counterexample outcomes | Return `Refuted` for any `Parity` (fails the `Agrees` report); return a value for `Verified` plus a settlement (fails the `UnexpectedSettlement` input); ignore the settlement for a `Counterexample` IR outcome (fails equality with the run entry point) |
| FR-032-AC-23 | Step 12: omitted settlement gives `MissingSettlement` from both entries, also after a converter refusal | Map falsified-with-`None` to `Failed` (fails the omitted-settlement input); drop the report-claim comparison in `settlement` (fails the other-run report, whose `.ok()` result is then `Some` and pairs to a value) |
| FR-032-AC-24 | Step 13: CG-raised causes on a supported family yield `CgDefect`; unavailable capabilities and binding mismatches yield `None` | Return `None` for `Playback` (fails the four-byte operand input, whose pair result becomes `MissingSettlement`); return `Some(CgDefect)` for `Observation` or for `GeneratedArtifactChanged` (fails the mutated-observation and changed-`harness.rust` inputs); return `Some(CgDefect)` for `Unavailable` (fails the routed `Implies` input, which would write `Failed` for a family with no operator-parity route); read `Authority(Operation)` as `None` (fails the local exhaustive match, the only input that reaches it, and leaves a supported-family run with no terminal record) |
| FR-032-AC-25 | Step 14: the flow compiles and the scan finds nothing | Add a `KaniObligationIdentity` parameter to `plan` (fails the compile of the step 1 call); reintroduce `settle` (fails the scan) |

Checks other than the Partial criteria listed in the replay matrix are planned; completed-route observations have the remaining whole-route gates (the driver's E9 run, IR-610, and a real falsified Kani run, the IR-631 code slice) stated in
[FR-032](../functional/FR-032-routed-scalar-replay-binding.md). Scalar lowering replay never
produces a source-property `Refuted` or `Verified` terminal. The scalar absent-settlement check is FR-032-AC-23; ordinary
[FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15 is unchanged and
still governs `run_terminal_value`. Missing upstream types/causes
must be reported as a gate, not implemented as local QSL lookalikes.
