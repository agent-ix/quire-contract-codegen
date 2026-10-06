---
id: SR-1824
title: "IR-631 follow-up failure domain: occurrence selection, report-to-context tie and operand domain provenance"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8; spec/assurance/AD-003-evidence-chain.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: reviews
---

# SR-1824: IR-631 follow-up failure domain

## Summary

Ticket: IR-631. PR: quire-contract-codegen, not yet opened. It is spec only. Reviewer: claude-opus-5-5, session 8950150c-5937-457d-9408-c19aa15729b8, run f92b6bc1-4101-4d05-affe-ea238bac36af. Three medium findings.

## Method

I looked for identity confusion and unstated failure modes in the scalar identity, the converter binding and the AD-003 E-1 boundary. I checked them against merged QSL `30d7beb7483b721bcb1ec98926e8b717e9a76fb7`:

- `operator_parity.rs::settle`: the occurrence must be one of the node's occurrences.
- `scalar_site.rs::check_operands`: a `GraphChild` is checked by reference only, with no range check.
- `scalar.rs::OperatorIdentity`: it has no `package_id`, `selected_function` or source digests.

I also checked FR-022, which generates one item per distinct node.

The following were examined and are clean:

- Adverse witness: in-domain operands with an exact result out of range (AC-8).
- Refused digest: Encoding versus Obligation (AC-10).
- Native-outcome-only mutation leaving O-09 unchanged (TC-047 step 6, FR-357-AC-16).
- The 8 MiB cap.
- Operand cardinality (`OperandCount`).
- The outcome projection for the four operators.

## Verdict

**PASS with three medium findings.** The AD-003 E-1 boundary separates the O-09 claim digest from the full report claim and from the driver-authenticated content tie, and that split holds. The findings concern what each layer leaves unbound.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-032 requires "the occurrence selected for the proved item" and forbids a "chosen occurrence ordinal". FR-022 generates one Kani item per distinct node, and merged QSL accepts any one of a node's occurrences. A scalar application node with more than one occurrence therefore has no authentic selected occurrence. Generation selects none, and "ambiguous occurrence" refuses but is never defined. Two outcomes follow: either every multi-occurrence node is silently unreplayable, or an implementer picks one, and the O-09 identity then depends on the choice. No criterion or TC-047 step covers a node with several occurrences. Define "ambiguous", say whether multi-occurrence nodes refuse or which occurrence is authentic, and add a check for it. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:217-219, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:256-258 |
| FND-002 | medium | AC-6 treats the report's full claim plus "another result/run" as the binding. But merged `OperatorIdentity` carries none of the original `call_site` context: `package_id`, `selected_function` and source digests are absent. The O-09 digest does not cover the package either. Suppose a driver retains the right claim but replays it against another wire whose package still contains the same node (a stale or rebuilt package, or another enclosing function). QSL recompiles that wire consistently and returns a report whose `claim()` equals the sent claim. The converter then settles the original binding on evidence from a context AD-002 R-7 forbids. FR-032 should state that the report claim does not bind the wire, and require the converter to compare the retained original wire or call-site identity for the run, not the claim alone. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:297, spec/assurance/AD-003-evidence-chain.md:251 |
| FND-003 | medium | FR-032 requires CG to "validate the actual harness ranges against the checked/emitted proving schema" because QSL's `GraphChild` check ignores range. It never says where each operand class's authoritative domain comes from. A parameter has a declared type range. A literal node has a singleton. A subterm such as `x + 1` inside `(x + 1) * 2` has no declared domain, only a derived result range, and today CG takes it from IR's lowered claim bounds. Without that provenance rule, one implementer validates a subterm against its result range and another rejects it as having no declared domain. A widened harness range on a graph child then passes QSL either way. State the domain source for each of parameter, subterm and literal-node children. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:263-266 |
