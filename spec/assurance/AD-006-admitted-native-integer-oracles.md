---
id: AD-006
title: "Admitted native integer oracles preserve the genuine proof boundary"
type: ArchitectureDescription
status: proposed
owner: quire-contract-codegen-maintainer
system: CG scalar and V2 clause oracle generation over checked finite integer domains
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: realizes
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-exact/FR-384
    type: references
  - target: ix://agent-ix/quire-exact/AD-003
    type: references
---
# AD-006: Admitted native integer oracles preserve the genuine proof boundary

## System Boundary

Peter approved IR-705 option 2: a kernel-owned admitted bounded-native API with a proven refinement to BigInt, followed by genuine IR-705, IR-917 and IR-948 proof qualification. CG selects and renders that production API from authentic IR-admitted CheckedPackageV2 data. quire-exact owns native value/operation admission, arithmetic and Meter semantics. This decision does not authorize a private CG evaluator or replace existing general Integer and as_big contracts.

## Views

The views below separate authentic graph admission, production native execution and independent proof qualification. Each retains its own owner and refusal boundary.

### Admission and generation view

CG reads operand intervals and every reachable arithmetic operation from the admitted graph through the existing IR-owned seams. It derives conservative interval bounds in expression order, preserving source identities and parameter relationships. The native capacity request includes intermediate results, not only the final declared domain. The kernel checks Add/Subtract/Multiply/Negate interval capacity and creates a checked operation plan; per-call membership creates an opaque admitted operation. A plan and a value from unrelated domains cannot be combined without that check.

Host admission derives canonical native endpoints from the genuine IR domains. Generated production/proof code reconstructs the selected interval plan from those native constant endpoints through the same checked kernel constructor, then admits actual payloads before evaluation. Both constructors and operand admission remain bounded-native on their successful paths. The host does not serialize an unchecked capability and the emitted proof does not repeat general Integer/BigInt admission or cache construction. This boundary must be evidenced in the actual generated call graph.

CG records the selected kernel capability and its input/result interval decisions in the generated artifacts and identity inputs using the repository's existing canonical identity mechanism. It does not invent checked node ids or equate a caller descriptor with package admission. Any public struct/serialized identity change follows the owning schema's compatibility rules rather than silently changing a digest interpretation.

### Execution and refusal view

Generated production native and Kani clients call the same kernel-owned admitted operations with the real public Meter. Chained values remain native i128 carriers, without promotion into general Integer/cache on the successful arithmetic path. Comparison and Boolean operations remain kernel-owned; subjects and independent expected-value expressions preserve their existing distinct roles.

A kernel capacity refusal is a typed generation/capability disposition carrying the offending source and capacity reason, before artifact success. It is distinct from IntegerOutOfDomain for a completed mathematical result outside a declared semantic domain. The general exact route remains available under its existing capability rules; selection failure is not proof success. No added assumption may shrink the authored population to make a native plan admissible.

### Refinement and genuine proof view

Kernel qualification supplies machine-checked full-capacity refinement against an independent mathematical abstraction of actual BigInt semantics; sampled differential tests alone are not a universal proof. Kernel qualification proves native admission and every operation against BigInt semantics over the full accepted capacity, including real Meter/refusal refinement. CG qualification separately obtains authentic QSL-emitted, IR-admitted inputs through the public generator and installed backend. No generated fixture is hand-edited into a replacement evaluator, and no healthy proof becomes ignored or waived.

The route can change generated oracle source bytes and native ABI carriers. Those changes are recorded and reviewed. It preserves the original proposition, selected clause, input/domain bounds, compiled subject intent, independent oracle/expectation, assertions, final completion cover and substantive mutants. The general-only route's source bytes are not an acceptance oracle for this architecture change.

## Decisions

| Decision | Choice | Alternatives and trade-off |
| --- | --- | --- |
| Value ownership | Kernel Copy i128 carrier, checked per-operation interval plan and opaque per-call admission. | General Integer/cache and private Copy charge tuning preserve more call sites but have not established 16 GiB fit. A new capability creates an explicit admission/refinement obligation while removing successful native execution's BigInt branch. |
| Selection source | Existing admitted graph and IR-owned finite bounds; every intermediate checked by the kernel. | A caller-provided range or hardcoded proof range could omit legal valuations and is rejected. Conservative intervals can refuse correlated expressions whose exact range would fit; no truncation or silent input narrowing repairs that refusal. |
| Accounting | Existing public Meter, cancellation/injected-denial precedence and normative charge schedule. | A simplified proof-only meter would prove a different implementation and is rejected. General Meter branches may remain a resource cost; the real profiles determine fit. |
| Refinement evidence | Kernel full-capacity refinement plus independent BigInt differential/adverse controls, then genuine consumer verification and mutant playback. | Small fixed tests or a healthy-only proof cannot establish the full contract. This decision makes no prediction that the new route fits before actual results. |
| Acceptance budget | Healthy genuine TC-027, IR-917 and IR-948 profiles must complete at 16 GiB with existing per-profile solver/time/unwind/cover rules. | Authorized larger-memory diagnostics may identify costs but earn no 16 GiB acceptance. MemoryExhausted, timeout, unknown or missing cover is inconclusive. |

## Risks

- Existing real Meter and other helper expansion can still exceed the budget after the native carrier removes BigInt from arithmetic. Owner: IR-705 code lead. Record actual terminal outcome and peak for each profile before any fit claim.
- Conservative endpoint arithmetic can reject a representable correlated expression. Owner: kernel admission and CG lowering reviewers. Keep a typed refusal and original population; any stronger range analysis requires a separately reviewed sound derivation.
- Generated native ABI or identity changes can break callers or reuse stale evidence. Owner: CG implementation reviewer. Record exact selection/source changes, retain historical proof inputs, and regenerate through the public route rather than editing archived proof artifacts.
- A refinement test restricted to the demonstration range would leave accepted API values unproved. Owner: kernel reviewer. Require explicit full-capacity symbolic/mechanical obligation coverage and disclose remaining gaps.
