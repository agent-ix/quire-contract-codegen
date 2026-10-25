---
id: SR-514010
title: "Base spec review of IR-364 PR 573 shared V2 comparison core"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/strategy/functional/FR-008-bound-domain-strategy-admission.md, spec/core/functional/interface-001-codegen-api.md (PR #573 head abe97829, base 1944268f)"
review_set: subset
---

## Summary

Ticket: IR-364. Base checklist review of draft PR #573, scoped to its diff, at frozen head
abe978294776e27ab9aef8822e29af5a3f252731 against base 1944268fc680d151105ab21d6ce2229809aab4b5
(current `main`). The PR encodes the planner ruling: the V2 comparison oracle shares FR-014's
executable comparison core, the IR-703 typed accessor stays the admission seam, and the V1
`generate_bound_strategy` entry is a declared interim whose expiry is the same change that ships
its V2 replacement and deletes the V1 reader. No obligation was narrowed. Four medium findings
remain: the added shared-core evidence in AC-7 has no pass condition and cannot distinguish a
shared core from a duplicate; the "executable comparison evaluation core" is undefined, so AC-8's
inspection cannot fail for two parallel renderers; TC-017 was not updated for the new AC-7/AC-8
obligations; and interface-001's new execution-point text has no owning requirement on `main`
until #572 lands.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-008-AC-7's added shared-core sentence is not test-observable and has no pass condition. "Use the same executable comparison core" is a structural claim a Test cannot verify, and "the test exercises both public paths for at least one valuation of each operator" states no expected result. Two independent evaluators that agree would pass. State the observable: both public paths return identical results, equal to the independent relation, for the stated valuations; leave sharing to AC-8's inspection. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:129 | wrong-requirement |
| FND-002 | medium | "Executable comparison evaluation core" and "the operators and operand forms they share" are undefined. FR-014's oracles call Contract Runtime owner operators, so two separate CG derivation/rendering paths that both emit the same runtime call would satisfy "one core" under AC-8's inspection, which is the duplicate-module state the ruling forbids. Name the shared unit (the CG descriptor/derivation or rendering function both oracles use) and the shared form set so inspection is falsifiable. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:29-33; spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:94-98; spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:130 | wrong-requirement |
| FND-003 | medium | TC-017 step 6 and its Expected Results are unchanged: they still verify only the per-valuation agreement and the "only input is the accessor's typed result" inspection. Nothing exercises FR-014's clause-oracle path, checks cross-path agreement, or inspects for one shared core, a second evaluator or a V1-to-V2 adapter. The changed AC-7/AC-8 obligations have no test procedure. | spec/strategy/matrix/TC-017-bound-domain-admission.md:42-46; spec/strategy/matrix/TC-017-bound-domain-admission.md:66-67 | correct-requirement-no-evidence |
| FND-004 | medium | interface-001's V2 inputs and `v2_bound_strategy_slice.execution_point` add an explicit per-clause typed execution point, but on this PR's base FR-011 OQ-1 still says the point has no V2 source and no criterion depends on it, and FR-008's Inputs do not list it. The owning text arrives only in open #572 (FR-011-AC-6/AC-7, FR-002-AC-19/AC-20). Merged first, #573 leaves interface text that contradicts main and that `verification:` attributes to criteria that do not yet exist. Merge #572 first, or drop the execution-point text from this PR. | spec/core/functional/interface-001-codegen-api.md:45; spec/core/functional/interface-001-codegen-api.md:335; spec/strategy/functional/FR-011-numeric-harness-campaigns.md (base) | missing-requirement |
| FND-005 | low | interface-001 `generate_bound_strategy.inputs` puts V1 and V2 inputs in one list separated by a semicolon, so whether "population ... campaign policy" belongs to both entries is left to inference. `execution_point` also carries a multi-clause harness rule ("a harness selecting multiple clauses ...") inside a single-selected-claim slice; that rule belongs to FR-002's harness. Split the inputs per version and move the harness clause out. | spec/core/functional/interface-001-codegen-api.md:45; spec/core/functional/interface-001-codegen-api.md:335 | wrong-requirement |

## Retention check (main versus head)

- FR-008-AC-7: every word of the `main` text is retained verbatim. The PR appends two sentences:
  the shared-core statement and the both-public-paths exercise. Nothing was narrowed (but see FND-001).
- FR-008-AC-8: "The V2 comparison oracle takes only the accessor's typed result for the selected
  claim" is verbatim. "and no V1 bound oracle or `BoundPackage` is an input to it" becomes "and no
  V1 bound oracle, `BoundPackage` or V1-to-V2 adapter as its input", inside a new "Source
  inspection finds ..." sentence. That is equivalent wording that adds an item. Nothing was
  removed; verification stays Inspection.
- FR-008 Description: the open-decision text ("Open decision (not decided here) ... The planner
  owns the question") is replaced by the decision. The removed parenthetical "(planned; today's
  code has no integer eq/ne descriptor)" survives as "planned". Code confirms that `main` still
  refuses integer eq/ne as `OperationNotDerivable` (oracle matrix row for FR-014-AC-35).
- FR-008 Behavior: a new bullet is added; no existing bullet is changed.

## Ruling conformance

- Shared core: FR-008 Description and Behavior both require one executable comparison core with
  FR-014's clause oracle, with no second evaluator and no V1-to-V2 adapter (definition gap: FND-002).
- Seam: the IR accessor's typed result remains the only admission input (FR-008-CON-1,
  AC-8, AC-9 unchanged). Sharing explicitly does not widen FR-008's relation grammar.
- BoundPackage: AC-8 and interface-001's `comparison_oracle` exclude `BoundPackage`, the V1 bound
  oracle and any adapter, consistent with AD-004 L-12.
- V1 interim: interface-001 `status` names the V1 slice as implemented, then states the planned
  replacement "of this entry and deletion of its V1 reader in the same change under AD-004; no
  parallel public strategy entry". `v2_bound_strategy_slice.status` says "not a simultaneous
  supported V1 and V2 strategy path". The expiry condition is stated. No compatibility layer,
  shim or second public entry is created.

## Consistency

- FR-021 / QSL FR-485: no conflict. FR-021 names FR-485 as the function-application owner path
  and forbids `exact` aliases or compatibility layers. This PR touches comparison oracles only
  and introduces no application route.
- #572: the execution-point vocabulary matches ("accepted QSL binding", "caller invocation
  context", "never inferred from clause kind"). The ordering dependency is FND-004.
- #574: FR-004 uses the same "authenticated against" wording and "refused before runner
  publication". Consistent.
- #571: it touches a different interface-001 region (`coverage_analysis_slice`); there is no
  textual overlap.

## Checklist and coverage

Changed IDs keep valid unique formats (FR-008-AC-7, FR-008-AC-8). Each keeps its verification
method, and the strategy matrix maps them unchanged (AC-7 to TC-017; AC-8 to Inspection). The
diff introduces no hash, digest, pin or tracking record. No AssuranceProfile with
`review_selection` applies in the scoped tree.

## Validation

The Quoin SpecReview skeleton and schema were fetched with `quoin write --types SpecReview`.
`quire validate --scope .` on both changed artifacts at the head returned no document errors;
only the existing module advisories (duplicate archetypes, inverse edge, inline schema) were
emitted. Tools: quoin 0.29.1, quire 0.36.2 (engine 0.50.2).

## Verdict

Not mergeable as is: four medium findings need a fix round. The ruling itself is encoded
faithfully and no existing obligation was narrowed.

## New findings (disposition pass 1)

Re-checked at fix-round head 79fa86745f (base d6f09905 now carries merged #572).

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-006 | low | `lower_typed_integer_comparison` does not exist in `src/` at this head, but FR-014's Description says "The V2 clause comparison path shares CG's internal `lower_typed_integer_comparison` unit", and FR-008's Description says "The common CG unit is one internal typed integer-comparison lowerer", both in the present tense. FR-014 Dependencies and interface-001 call the same unit planned. Mark the unit as planned wherever the Description names it, so the specs do not state a current fact the code contradicts. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:35-38; spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:31-36 | wrong-requirement |
| FND-007 | low | interface-001 keys the IR-1013 execution-point lookup by "this selected checked clause and authentic claim occurrence" on the V2 node, but merged FR-011 (lines 49-53 and 77-79) and FR-002 (line 85) still speak of "the selected V2 node's accepted QSL `ClauseBinding`", with no occurrence key. The two are compatible, but a node-only reading of FR-011 is the lookup the interface rules out. Align FR-011 and FR-002 to the clause-and-occurrence key in a follow-up, because those files are outside this PR's diff. | spec/core/functional/interface-001-codegen-api.md:48; spec/core/functional/interface-001-codegen-api.md:336; spec/strategy/functional/FR-011-numeric-harness-campaigns.md:49-53 | wrong-requirement |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-008-AC-7 now states the pass condition. The FR-014 clause-oracle result is identical to the selected result and to the independent relation for every valuation. Main's AC-7 text is retained verbatim. |
| FND-002 | fixed | The shared unit is named (`lower_typed_integer_comparison` in `src/oracle`), and the form set is defined (six operators, direct read/read and read/literal, either literal position, Current/Pre/Post). AC-8 requires exactly one such function and no second lowerer or evaluator module, so inspection can now fail. |
| FND-003 | fixed | TC-017 step 6 now evaluates both public paths, compares them with each other and with the independent relation, and inspects for the one function and for no second lowerer, adapter, `BoundPackage` or V1 oracle. Expected Results are updated to match. |
| FND-004 | fixed | #572 merged as d6f09905, which is this PR's base, so FR-011 now owns the single accepted `ClauseBinding.execution_point` source. interface-001 now takes the point only through IR-1013's planned accessor, with no caller field. The point is gone from `planned_v2_inputs`, which matches FR-008's Inputs. |
| FND-005 | fixed | V1 `inputs` and `planned_v2_inputs` are now separate lists, each with its own population and campaign policy. The multi-clause harness rule is removed from `v2_bound_strategy_slice`. |
| FND-006 | fixed | Round 2, head 4a2a95ad. The FR-014 Description now reads "The planned V2 clause comparison path shares CG's planned internal `lower_typed_integer_comparison` unit". The FR-008 Description now reads "The planned common CG unit ... It will emit ... will use that unit". The diff from 79fa8674 is only these two Description edits. |
| FND-007 | deferred | Deferred to CG #575 (open, head 212ca3e1), which keys FR-011 and FR-002 by the admitted package, checked clause id and authentic `claim` occurrence tuple, adds `MissingClaimOccurrence`, forbids sibling-occurrence substitution, and updates TC-004 and TC-020. These files are outside PR #573's diff. |
