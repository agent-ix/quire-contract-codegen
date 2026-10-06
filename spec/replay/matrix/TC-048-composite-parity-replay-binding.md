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
[FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-17 and AC-19 to AC-27. All
completed-route checks are GATED on actual QSL-640 node-parity/value/settlement API delivery, not a
spec-only merge. This scenario artifact claims no executable coverage. FR-028 AC-17/24 retain the
strength/ceiling evidence checked here; native execution and backend controls use their named lanes
and never replace positive QSL evaluation with a verdict double.

## Test Procedure

1. Obtain real QSL-emitted composite equality and inequality claim nodes with original source and
   package identity. Exercise parameter/parameter, parameter/literal and literal/literal forms.
   Retain the actual generated harness, native refinement, original limits and canonical identity of
   the same proved artifact. Assert the original nodes, operand declarations and literal singleton
   values. Record public builder inputs and the QSL request; change source, package, selected node,
   operation and operand declaration individually and require refusal. No hand-built source/package
   replacement or enclosing Boolean function satisfies selection.
2. Assert FR-025 AC-9's persisted bindings against actual draw order and reconstruct the selected
   backend playback through them. Swap or remove a leaf, repeat a draw, change a typed path or
   domain, alter primitive width and select another harness. Check that each refuses rather than
   silently binding a different operand. Keep absent, null and present slots distinct, including
   nested option controls and parameter/literal inputs; literal-only inputs remain pinned.
3. Through the authoritative canonical reader/encoder, round-trip every FR-033 listed scalar and
   composite family. Include integer beyond i64, exact IEEE NaN/signed-zero bits, rational zero and
   sign normalization, retained decimal scale, Unicode text containing `% ; < >`, enum
   identity/member, exact quantity/unit and reference identity. Include mixed nested option, record,
   tuple and union, sequence order, sorted set/bag encoding with bag duplicates, and ordered-set
   order/distinctness. Mutate one canonical/typed property per refusal: unknown tag, non-JCS
   spelling, malformed percent escape, wrong declaration/member/unit/reference identity, wrong slot
   presence/arity, leaf domain, collection cardinality, duplicate set/ordered-set member and
   canonical sorted order. Check authoritative equality duplicate rules, not only byte-identical
   values. No copied upstream schema/fixture or executable is used.
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
   observation, another artifact or fresh regeneration that erases the mutation refuses. Check no
   early count exit after an unequal pair. Canonical-only families with no lawful generated shadow
   remain explicitly gated/unsupported, never a fake end-to-end passing fixture.
6. For the same claim exercise QSL-declared coverage: exact and wider legal bounds, tightened
   range/cardinality/depth, omitted key, empty list with declared keys, unbounded/recursive key,
   unknown key, duplicate and bound-kind mismatch. Use literal singleton domains. Record QSL's
   authoritative declared-key derivation; no caller-supplied declaration list establishes proof.
7. Enumerate every FR-028 AC-17 strength through FR-029 AC-17 and AC-19 to AC-27, retaining
   independent refinement evidence. Test disagreement for verified, falsified (including agreeing
   replay), inconclusive and cover-unsatisfied outcomes, including zero checks. Test verified
   CeilingReached with covered bounds; ordinary vacuous/cover-unsatisfied zero checks retain their
   backend class and record. Exercise every shadow-inconclusive reason with no disagreement,
   exhausted covered nonzero checks and exhausted uncovered bounds, completed sampled and not-run
   cases. Verified settlement takes no native-observation field: record not_run as explicit absence,
   without an extra probe or fabricated observation. Use the typed execution entry to cause actual
   native wall-clock and memory stops in the named ceiling lane; assert FR-028 AC-24's class,
   recorded ceiling and resource-inconclusive terminal, never Tested. Exercise a QSL settlement
   resource refusal separately; it cannot enter the completed sampled path.
8. Feed each QSL result/record through the public converter and terminal map. Read actual record
   category/cause for Proved0. Cross-bind another run/result and alter node, operation, operand,
   domain, limits or proof-content identity; each returns no settlement. Keep QSL refusal codes,
   executor faults and CG defect causes typed. Before QSL-640 delivery, also exercise inconclusive
   and cover-unsatisfied shadows with retained refinement_failed: each takes typed interim
   NonProductionProof carrying that strength and returns no terminal value, while cases without
   disagreement take ordinary inconclusive rows. After delivery, the disagreement-first
   Failed/CgDefect rule applies. While the upstream gate holds, an otherwise valid request reaches
   the unavailable-capability refusal; malformed earlier setup reaches its own refusal. Executable
   tests trace the exact criteria they assert; scenario prose is not coverage.

## Expected Results

| Authority | Planned observation | Mutation that must fail |
|---|---|---|
| FR-033-AC-1 | Original-package node-selected parity and actual literal singleton domains | Replace the claim with a source predicate or manufacture a Boolean function |
| FR-025-AC-9; FR-033-AC-5 | Original-node/path/domain bindings reconstruct the actual ordered draws | Swap operands/paths, omit a required leaf or unpin a literal |
| FR-033-AC-2/3 | All canonical families preserve exact values and typed shape; malformed cases refuse | Round IEEE or quantity, collapse absence/null, lose order/multiplicity or use byte equality for set admission |
| FR-033-AC-4 | Public encoded-byte guard and checked counts/work; iterative admitted lifecycle | Apply capture-only guard, add a depth cap, recurse during drop or allow arithmetic overflow |
| FR-033-AC-6/7/10 | Actual same-artifact QSL parity: divergence Failed/CgDefect, assertion-only agreement Inconclusive; real generated verify/falsify controls | Supply a canned native verdict, regenerate away the mutation, ignore pair count or label parity Refuted |
| FR-033-AC-8 | QSL derives complete declared-key coverage and literal singleton bounds | Treat empty/omitted/unbounded keys as covered or accept duplicate/unknown key |
| FR-029-AC-17 and AC-19 to AC-27; FR-028-AC-17/24 | Closed strengths; cross-outcome disagreement wins, then verified refinement ceiling, ordinary vacuous record, exhausted+covered, completed Tested; all shadow-inconclusive rows and actual record category preserved | Delete a strength row, let zero defeat disagreement/ceiling, promote Proved0 or label a stopped refinement sampled/Tested |
| FR-033-AC-9 | Same-binding converter and truthful typed capability/refusal/fault readings | Accept another result, invent QSL code/cause or bridge through predicate replay |

Unimplemented canonical conversions, family harnesses or legal cause representations remain reported
gaps. The future terminal payload names have the semantic meaning in FR-029, and the owning
delivered API must establish their actual Rust representation before positive tests run.