---
id: TC-048
title: "Check composite node parity replay, canonical values and shadow settlement"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: verifies
---
# TC-048: Check composite node parity replay, canonical values and shadow settlement

## Description

Planned public-consumer scenarios for
[FR-033](../functional/FR-033-composite-parity-replay-binding.md),
[FR-025](../../kani/functional/FR-025-generated-subject-abi.md) AC-9 and
[FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-17 and AC-19 to AC-28. All
completed-route checks are GATED until CG consumes the QSL-640 node-parity/value/settlement API
delivered in QSL #645; a spec-only merge does not satisfy that gate. This scenario artifact claims no executable coverage. FR-028 AC-17/24 retain the
strength/ceiling evidence checked here; native execution and backend controls use their named lanes
and never replace positive QSL evaluation with a verdict double.

## Test Procedure

1. Obtain real QSL-emitted composite equality and inequality claim nodes with original source and
   package identity and actual recompiled occurrence membership. Exercise parameter/parameter,
   repeated/self parameter, parameter/literal and literal/literal forms, retaining positional
   graph-child nodes and actual literal singleton values.
   Retain the actual generated harness, native refinement, original limits and canonical identity of
   the same proved artifact. Assert the original nodes, operand declarations and literal singleton
   values. Record public builder inputs and the QSL request; change source, package, selected node,
   operation and operand declaration individually and require refusal. No hand-built source/package
   replacement or enclosing Boolean function satisfies selection.
2. Assert FR-025 AC-9's persisted bindings against actual draw order and reconstruct the selected
   backend playback through them. Swap or remove a leaf, repeat a draw, change a typed path or
   domain, alter primitive width and select another harness. Check that each refuses rather than
   silently binding a different operand, without retained Disagreed. A mismatched persisted
   binding schema refuses the claim; malformed playback values follow same-valid-claim
   disagreement precedence in step 10. Keep absent, null and present slots distinct, including
   nested option controls and parameter/literal inputs; literal-only inputs remain pinned.
3. Through the authoritative canonical reader/encoder, round-trip every FR-033 listed scalar and
   composite family. Include ExactInteger leaves beyond i64 inside composites while top-level
   Integer stays i64, exact IEEE NaN/signed-zero bits, rational zero and sign normalization,
   retained decimal scale, Unicode text containing `% ; < >`, enum identity/member, exact
   quantity/unit and reference identity. Include mixed nested option, record, tuple and union,
   sequence order, sorted set/bag encoding with bag duplicates, and ordered-set order/distinctness.
   Mutate one canonical/typed property per refusal: unknown tag, non-JCS spelling, malformed percent
   escape, wrong declaration/member/unit/reference identity, wrong slot presence/arity, leaf domain,
   collection cardinality, duplicate set/ordered-set member and canonical sorted order. Check
   authoritative equality duplicate rules, not only byte-identical values. No copied upstream
   schema/fixture or executable is used. Exercise these operand refusals without retained
   Disagreed; step 10 checks the separate disagreement-first case. Union text decode is delivered,
   but union replay conversion remains unsupported until
   checker union admission is delivered; do not count a decode as successful union replay.
4. Submit arbitrary public input at the configured encoded-byte limit and one byte over. Observe the
   byte guard before parsing/unescaping. Exhaust checked occurrence and work counters separately.
   Exercise a legal 100,000-link recursive value on a 512 KiB native stack through decode, clone,
   equality, redacted Debug and drop; a one-byte-too-small budget refuses and no depth cap is
   invented. FR-017 launcher capture alone cannot establish the public guard.
5. In the named real-Kani lane for each lawful generated family, retain an unmutated verified
   control and actual falsification playback from independent equality-result and pair-count
   mutations, plus an assertion-only mutation leaving the comparison correct. Execute that same
   proved artifact natively and record actual result/count or typed refusal/incomplete/fault. Call
   QSL's exact parity arm with this observation and retained shadow result/count. Missing
   observation refuses without retained Disagreed; step 10 checks that its absence cannot erase
   same-valid-claim disagreement. Another artifact/context or fresh regeneration that erases the
   mutation refuses claim binding. Check no
   early count exit after an unequal pair. Canonical-only families with no lawful generated shadow
   remain explicitly gated/unsupported, never a fake end-to-end passing fixture.
6. For the same claim exercise QSL-declared coverage: exact and wider legal bounds, tightened
   range/cardinality/depth, omitted key, empty list with declared keys, unbounded/recursive key,
   unknown key, duplicate and bound-kind mismatch. Use literal singleton domains. Record QSL's
   authoritative declared-key derivation; no caller-supplied declaration list establishes proof.
   Test an enum harness covering every source variant and one omitting a variant. Add an
   undeclared variant to Variants, then submit a DeclaredDomain over an enum position: each must
   produce QSL's typed refusal naming that key, not covered/Proved. Other listed
   non-Boolean leaf families stay uncovered/Tested while no authoritative whole-domain bound kind
   exists; a maximum text length or decimal range alone cannot count as complete coverage.
7. Enumerate every FR-028 AC-17 strength through FR-029 AC-17 and AC-19 to AC-28, retaining
   independent refinement evidence. Verify the same four-state Refinement is supplied on both paths.
   Test disagreement for verified, falsified (including agreeing replay), inconclusive and
   cover-unsatisfied outcomes, including zero checks. Test verified CeilingReached with covered
   bounds; ordinary vacuous/cover-unsatisfied zero checks retain their backend class and record.
   Exercise every shadow-inconclusive reason with no disagreement, exhausted covered nonzero checks
   and exhausted uncovered bounds, completed sampled and not-run cases. Verified settlement takes no
   native-observation field: record not_run as explicit absence, without an extra probe or
   fabricated observation. Use the typed execution entry to cause actual native wall-clock and
   memory stops in the named ceiling lane; assert FR-028 AC-24's class, recorded ceiling and final
   Incomplete(ResourceExhausted), never Tested. Exercise a QSL settlement resource refusal
   separately; it cannot enter the completed sampled path.
8. Feed each QSL result/record through the public converter and terminal map. Read actual record
   category/cause for Proved0. Cross-bind another run/result and alter node, operation, operand,
   domain, limits or proof-content identity; each CG precheck or failed report binding returns no
   report terminal value. Keep QSL refusal codes, executor faults and CG defect causes typed. Until
   CG consumes the delivered QSL parity facade, also exercise inconclusive
   and cover-unsatisfied shadows with retained refinement_failed: each takes typed interim
   NonProductionProof carrying that strength and returns no terminal value. IR-241 owns the refusal
   variant and verified-strength input; IR-635 owns the widened interim case. Cases without
   disagreement take ordinary inconclusive rows. After CG consumer delivery, the disagreement-first
   Failed/CgDefect rule applies. While the CG consumer gate holds, an otherwise valid request reaches
   the unavailable-capability refusal; malformed earlier setup reaches its own refusal. Executable
   tests trace the exact criteria they assert; scenario prose is not coverage.

   Retain the actual
   sent CompositeIdentity::new over request obligation, claim and complete evidence. Before ANY
   outcome projection, including refusal/generated fault/incomplete, require decoded report.claim()
   to equal it. Independently corrupt decoded node, occurrence, operation/kind, operand/domain,
   harness bounds, ScalarLimits, content identity, native cause/result/count, shadow result/pair
   count, refinement and verified SUCCESS count with the sent identity held fixed; each returns
   no settlement. As a separate control, change a request BEFORE send and require its legitimate
   changed echo, without claiming consumer corruption or artifact authentication. Public ReplayLimits
   is the last entry argument and separate from ScalarLimits/full claim. Read actual record

9. For O-09 composite identity, retain the exact claimed application node, original recompiled
   occurrence and kind. Use the owning public typed parity_obligation with one argument per operand
   position in operand order. Compare a parameter with itself (`a == a`) and require TWO positional
   graph_child arguments for the same actual parameter node, each retaining the drawn bounds;
   compare two distinct parameters and require their two positional arguments. Preserve repeated
   positions rather than deduplicating nodes. Composite literal operands each contribute their own
   graph_child node with empty Bounds; literal-only comparison therefore still has two arguments,
   with no free parameter position. Exercise inline integer singleton Range identities at their
   application/occurrence/position and require typed refusal for an unsupported inline literal.
   Sort each argument's BoundEntries by canonical encoded key bytes: paths `[0,0]` before `[0]`
   and `[10]` before `[2]`. Duplicate keys within one argument refuse, while repeated parameter
   bounds across two positional arguments remain legal. Retain actual drawn cardinality/depth
   entries even for unbounded/recursive positions and declared-domain substitution. Independently
   change node, occurrence, operand position, literal graph node/value or drawn bound; require changed
   identity or original membership refusal. Invoke the typed encoder refusal without manufacturing
   an identity. Change only CG abstractions, size budget, static closure pair-node count or
   unexercised behaviours and require the same O-09 while full FR-015 AC-76 evidence retains them.
   Keep the static record count distinct from the runtime count checked in step 5/F-7. Verify the
   four-member preimage gains no operator/native/content/refinement/counter top-level member.
   Replace the actual proved artifact or retained original context with O-09 and echoed claim held
   equal; require the independent canonical content/R-6/R-7 binding refusal, not a new tracking
   digest or echo-authentication claim. Existing function/frame vectors remain unchanged.
10. Submit a changed source/package/node/occurrence/bound/O-09 request with Disagreed. A CG
    precheck rejection returns no report/terminal value. For a reachable QSL `prepare` refusal,
    send a valid wire request through the public facade with a replay input-byte limit below its
    encoded size, then pass the actual report to CG's public converter with the same sent identity.
    Its bound non-fault `Refused` result returns Inconclusive(ReplayRefused) with QSL's code before
    F-1, even with Disagreed. Then use
    a valid `prepare` and an operand that would fail admission with Disagreed; F-1 wins because
    admission follows it. Inspect QSL's public `terminal_value` mapping and CG's pass-through for
    any actual `Fault` or `Admission(Fault)` report: each maps to Failed, but no invariant-fault
    report or public triggering input is fabricated. Change each full
    `CompositeIdentity` member independently in a returned report: obligation, node, occurrence,
    operator, obligation kind, harness bounds, limits, content identity, falsified operands,
    shadow verdict and pair count, native observation/cause and refinement, then verified SUCCESS
    count and refinement. Every mismatched or missing report returns a typed CG refusal with no
    terminal value; the matching report binds. On the same valid claim, combine Disagreed with
    operand refusal (a composite field `x` declared `Int[0, 9]` but supplied as `x: 12`), missing
    native observation,
    native fault, exact-limit/refinement-ceiling and agreeing replay:
    Failed/CgDefect wins before any early operand/native setup refusal; do not fabricate absent
    native evidence. Without disagreement, missing required native observation refuses the replay
    continuing beyond F-1. Combine an out-of-domain operand with native Incomplete and with native
    ExecutionFault, each also under a request admission limit, an exact-evaluation limit and
    CeilingReached: F-2 is GeneratedFault/Failed retaining its NativeCause. QSL FR-358 owns the
    internal no-admission/no-exact-evaluation rule; assert CG's typed report and terminal value.
    With Completed native, make one operand fail admission and set CeilingReached: F-3 is
    RefusedInput/ReplayRefused with operand index and QSL code. Exhaust the request's accounting
    limit while admitting an otherwise valid operand and set CeilingReached: F-4 is
    Incomplete/Admission with counter, configured limit and count reached. QSL FR-358 owns the
    internal no-exact-evaluation rule; assert CG's Admission report stage and terminal value.
    Exhaust QSL's exact limit after admission and set CeilingReached: F-5 is
    Incomplete/ExactEvaluation; with sufficient limits, CeilingReached reaches F-6
    Incomplete/RefinementCeiling. Assert F-4, F-5 and F-6 have the same ResourceExhausted terminal
    value but distinct retained stages. Completed/Refused native evidence cannot supply
    F-7's verdict; mutate shadow equality/inequality result and pair count independently. Backend
    Kani timeout/memory outcomes stay outside replay and retain ordinary final Incomplete mapping
    under FR-028 AC-2/3; independent refinement ceiling stays AC-24's class. No actual backend stop
    becomes Tested.

All claim-shape and complete-identity checks added here are PLANNED/UNRUN. Real Text-leaf profile
selection, nested set/bag single-encoding performance and source-unbounded coverage limitations
remain explicit open checks; rational substitution or derivation/coverage-seam success cannot count
as lawful source-function/generated-family acceptance. Use actual driver/native same-artifact
verify/falsify controls, never a verdict double or invented source predicate.

## Expected Results

| Authority | Planned observation | Mutation that must fail |
|---|---|---|
| FR-033-AC-1 | Original-package node-selected parity and actual literal singleton domains | Replace the claim with a source predicate or manufacture a Boolean function |
| FR-025-AC-9; FR-033-AC-5 | Original-node/path/domain bindings reconstruct the actual ordered draws | Swap operands/paths, omit a required leaf or unpin a literal |
| FR-033-AC-2/3 | All canonical families preserve exact values and typed shape; malformed cases refuse | Round IEEE or quantity, collapse absence/null, lose order/multiplicity or use byte equality for set admission |
| FR-033-AC-4 | Public encoded-byte guard and checked counts/work; iterative admitted lifecycle | Apply capture-only guard, add a depth cap, recurse during drop or allow arithmetic overflow |
| FR-033-AC-6/7/10 | Actual same-artifact QSL parity: divergence Failed/CgDefect, assertion-only agreement Inconclusive; real generated verify/falsify controls | Supply a canned native verdict, regenerate away the mutation, ignore pair count or label parity Refuted |
| FR-033-AC-8 | QSL derives complete declared-key coverage and literal singleton bounds | Treat empty/omitted/unbounded keys as covered, accept duplicate/unknown key or enum undeclared variant/DeclaredDomain |
| FR-029-AC-17 and AC-19 to AC-27; FR-028-AC-17/24 | Closed strengths; cross-outcome disagreement wins, then verified refinement ceiling, ordinary vacuous record, exhausted+covered, completed Tested; all shadow-inconclusive rows and actual record category preserved | Delete a strength row, let zero defeat disagreement/ceiling, promote Proved0 or label a stopped refinement sampled/Tested |
| FR-033-AC-11 | Positional O-09 includes repeated/self operands and literal graph nodes with empty Bounds; canonical encoded-key byte ordering and typed encoder refusal; full record extras and independent same-artifact content tie remain separate | Deduplicate self-comparison, omit literal arguments, sort DomainKey Ord, admit duplicate key within an argument, add a top-level operator/native/content member or substitute echo equality for artifact authentication |
| FR-033-AC-12/13 | CG precheck yields no report value; QSL `prepare` Refused report precedes Disagreed, while later admission refusal follows it; shared Refinement and ordered F rows preserve actual native cause and separate resource stages | Settle another claim as Failed, hide disagreement behind admission/agreement, compare native instead of shadow, collapse limit stages or send backend ceilings through replay |
| FR-033-AC-9; FR-029-AC-28 | Full `CompositeIdentity` binding and truthful typed capability/refusal/fault readings; QSL public terminal mapping and CG pass-through inspected for invariant faults | Accept one changed observation member; drop QSL Refused report's terminal value; label an actual bound Fault report ReplayRefused; invent a fault report, QSL code/cause or predicate replay route |

Unimplemented canonical conversions, family harnesses or legal cause representations remain reported
gaps. The future terminal payload names have the semantic meaning in FR-029, and the owning
delivered API must establish their actual Rust representation before positive tests run.
