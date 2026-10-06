---
id: SR-1694
title: "IR-635 follow-up spec review (failure domain): composite O-09 argument identity edge"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@04608d2942dc27f47d0b820a8a58a1b82b1669c3; spec/assurance/AD-003-evidence-chain.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: reviews
---

# SR-1694: IR-635 follow-up spec review, failure domain

## Summary

Ticket: IR-635. PR: quire-contract-codegen#303 (spec only). Reviewer: claude-opus-5-5. I looked for identity confusion, unstated edge cases and collapsed resource stages in the new O-09 split, the content tie and the F-1 to F-6 ordering. One medium finding. The setup-versus-`Disagreed` conflict appears once, in SR-1692 FND-001, and is not repeated here.

## Method

I enumerated operand shapes for the O-09 preimage against the measured QSL #645 FR-358 step 6 (`7e508c4`):

- parameter/parameter
- parameter/literal
- literal/literal
- the same parameter on both sides
- a Boolean leaf with no derived position
- an enum position

I checked that an artifact or context replacement under an equal O-09 still refuses through the separate content tie, and that FR-033's binding/converter bullets name a CG owner for that check. I checked that the three resource stages and the backend Kani ceiling stay distinct. I checked that a native Completed or Refused observation never becomes the settlement oracle. I checked that FR-028-AC-2/3 keep their exact ordinary terminal categories.

## Verdict

**PASS with one medium finding.** The following are clean:

- **Content tie:** the converter validates it (FR-033 Behavior, binding and converter bullets), and it is independent of O-09.
- **Resource stages:** a native stop is F-3, a QSL exact limit is F-4, a refinement ceiling is F-5, and a backend ceiling stays outside replay.
- **Literal-only claims:** these derive no parameter id.
- **Boolean leaves:** these derive no position.
- **`Tested`:** no stage collapses into `Tested`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The composite O-09 preimage is stated as "the parameter-operand node IDs with their actual harness bounds" (AD-003 E-1 extension, FR-033 Inputs, AC-11, TC-048 step 9). That leaves undefined how a claim that compares one parameter with itself (`f(a: Q): Boolean { a == a }`) contributes its arguments. One reading makes one argument per operand and lists `a` twice. Another makes one argument per distinct parameter, which matches FR-358 step 6's "each argument is a parameter node id and its declared domain, where the declared domain is the harness bounds keyed under that parameter". The two readings produce different digests. If CG mints the first and QSL recomputes the second, an otherwise valid claim refuses as an O-09 mismatch. Duplicate DomainKeys could also trip QSL's duplicate-harness-bound refusal. State that `arguments` holds each distinct parameter once, or say how a repeated parameter operand is handled, and give TC-048 step 9 a self-comparison case. | spec/assurance/AD-003-evidence-chain.md:189, spec/replay/functional/FR-033-composite-parity-replay-binding.md:77, spec/replay/functional/FR-033-composite-parity-replay-binding.md:230, spec/replay/matrix/TC-048-composite-parity-replay-binding.md:97 |
