---
id: "SR-2233"
title: "CG IR-666 spec-ears-analysis: composite converter delivery and status amendments"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@code/ir-666-composite-converter (second frozen head, fix commit 'Bind composite reports and cover public parity rows'; reviewed revision recorded in the IR-666 Linear marker only, per this repository's no-SHA rule); spec/core/functional/interface-001-codegen-api.md (composite_parity_terminal_value, verified_shadow_terminal_value, Features), spec/kani/functional/FR-029-run-outcome-terminal-record.md (Description, Implementation Gate, AC-20, AC-21, AC-28), spec/kani/functional/FR-030-ir-outcome-terminal-map.md (Inputs, Outputs, Behavior, AC-15 to AC-17), spec/replay/functional/FR-033-composite-parity-replay-binding.md (Description, Outputs, Behavior binding-operations bullet, Falsified Settlement, Setup Refusal step 3, AC-7, AC-9, AC-12, AC-13, Dependencies), spec/kani/matrix/TC-041-ir-outcome-terminal-map.md (Status), spec/replay/matrix/TC-048-composite-parity-replay-binding.md (Description, step 8, step 8a, closing text), spec/kani/matrix/tests.md, spec/replay/matrix/tests.md, spec/tests.md"
---

# SR-2233: CG IR-666 EARS analysis

## Summary

Ticket: IR-666. No FR title or opening requirement statement changed. Quire's advisory lint
(`quire lint --module spec-artifacts-process`) reports no rule finding on FR-029, FR-030,
FR-033, interface-001, TC-041 or TC-048, before or after. The FR-030 Inputs/Outputs edits and
the FR-029 and TC prose edits are status text, not requirement statements. One new Behavior
bullet is a requirement statement.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new FR-033 Behavior bullet ("The public CG binding operations are `composite_parity_terminal_value` ... Both errors carry no terminal value. The separate IR-635 builder owns ...") is the only bullet in that list without a "shall". It packs five claims into one bullet: names, inputs, the settlement's accessors, the error variants and the ownership boundary. Its siblings use the ubiquitous form ("The converter shall ..."). As written it reads as description, not obligation, so the matching interface-001 entry is the only normative source for the names. Fix: split it into "The converter shall expose ..." and "When the report is absent or its claim differs, the converter shall return ... with no terminal value" | spec/replay/functional/FR-033-composite-parity-replay-binding.md:196-203 |

## Verdict

EARS conformance is unchanged apart from the one descriptive Behavior bullet.
