---
id: "SR-12181"
title: "CG PR 421 spec review (integrity): AD-004 step 4b HarnessSpec and render"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@77f5178102658bdac3d77b4024d262f029e5da46 (PR #421, IR-858); spec/assurance/AD-004-cg-crate-layout.md (The HarnessSpec and render contract (step 4b), L-4, step 4b, import rule, Not verified), spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-112 to FR-015-AC-117), spec/kani/matrix/TC-025-bounded-kani-obligations.md (steps 42 to 47), spec/kani/matrix/tests.md; checked against src/ and tests/it/ at origin/main 0e0f92e"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: references
---

# SR-12181: CG PR 421 spec review (integrity): AD-004 step 4b HarnessSpec and render

## Summary

Ticket: IR-858. Integrity of the step 4b subsection, against itself, the rest of AD-004, FR-015, and the
code at origin/main 0e0f92e (src/kani/generate/scalar.rs render_scalar, record.rs, negotiate.rs:135,
core/identity.rs, kani/abi.rs, outcome.rs, tests/it/cover_last.rs, tests/it/layout.rs).

Checked and consistent: the order of refusals in R-1 item 5 (harness_path's InvalidGeneratedSyntax, then
ResourceLimitExceeded at MAX_GENERATED_SOURCE_BYTES, then syn::parse_file's InvalidGeneratedSyntax, then
record's RenderFailed) matches render_scalar; options are adapter_options(path, unwind, Cadical, false) as
today; the cover is the last statement as FR-015-AC-7 and FR-015-AC-58 require; L-4's "exactly one cover"
agrees with FR-015-AC-7 ("exactly one non-vacuity cover"); `pub(super)` matches the sibling families; the
step order outcome, record, spec, render, census_validation fits the rank rule in tests/it/layout.rs.

Units examined: FR-015-AC-112, FR-015-AC-113, FR-015-AC-114, FR-015-AC-115, FR-015-AC-116, FR-015-AC-117, AD-004-H-1, AD-004-H-2, AD-004-H-3, AD-004-H-4, AD-004-R-1, AD-004-P-1, AD-004-P-2, AD-004-P-3, AD-004-L-4, AD-004-import-rule, AD-004-step-4b, TC-025-step-42, TC-025-step-43, TC-025-step-44, TC-025-step-45, TC-025-step-46, TC-025-step-47, tests.md-FR-015-AC-112-117, tests.md-TC-025-row. Context only: FR-015-AC-7, FR-015-AC-58, AD-004-L-3.

## Verdict

FND-001 is a contradiction between H-1 and P-2: the statement shape cannot render today's scalar template
byte for byte. FND-002 to FND-004 and FND-008 are statements that disagree with the code or with L-3. Not
mergeable until FND-001 to FND-004 and FND-008 are fixed; the lows are one-line edits.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Statement::Let has fields name, mutable, init and no type, but the scalar template emits `let exact: i128 = {exact};` (src/kani/generate/scalar.rs:432). The H-1 shape cannot render that line, so a port built to H-1 cannot be byte-identical as P-2 and FR-015-AC-115 require. Fix: give Let an optional typed annotation (for example a small primitive enum that includes i128), and say so in H-1. | spec/assurance/AD-004-cg-crate-layout.md:463-465, spec/assurance/AD-004-cg-crate-layout.md:550-557, src/kani/generate/scalar.rs:432 |
| FND-002 | medium | HarnessPath is defined in src/core/identity.rs:104, not record.rs; record.rs defines harness_path, its validating builder. The H-1 path row says 'the validated HarnessPath that record.rs defines today', and the import rule says 'spec imports record (for HarnessPath) and kani::abi'. spec also needs outcome for MAX_OBLIGATION_UNWIND (outcome.rs:28, invariant H-3 item 5). render needs core::artifact and has no use for record (R-1 item 6: it builds no record). Fix the row and list the imports as the code needs them. | spec/assurance/AD-004-cg-crate-layout.md:444, spec/assurance/AD-004-cg-crate-layout.md:366, src/core/identity.rs:104, src/kani/generate/record.rs:15-27 |
| FND-003 | medium | H-3 item 6 justifies refusing a literal `true` cover by 'FR-015-AC-7 already forbids kani::cover!(true, ...) for the scalar harness'. FR-015-AC-7 does not say that, and no FR-015 text forbids a true cover. The invariant is new. Say so; FR-015-AC-112 carries it already, so drop the false citation. | spec/assurance/AD-004-cg-crate-layout.md:493-496, spec/kani/functional/FR-015-bounded-kani-obligations.md:639 |
| FND-004 | medium | R-1 item 1 says render.rs 'is the only non-test code that spells' all seven L-3 items. At 4b, ProofKind has one variant, so render.rs has no emitter for proof_for_contract, requires or ensures. precondition.rs, contract.rs, frame.rs, collection.rs, v1_bundle.rs and the corpus still spell proof attributes (tests/it/cover_last.rs PROOF_TEMPLATES). That contradicts L-3, which lands only after 4f, 4g and 4h. Fix: state it as the end state at L-3's landing, and at 4b as 'the only spelling site for the ported family'. | spec/assurance/AD-004-cg-crate-layout.md:515-518, spec/assurance/AD-004-cg-crate-layout.md:799-806, tests/it/cover_last.rs:191-199 |
| FND-005 | low | Step 4b says 'after 4b the interim exceptions of L-3 lose the scalar entry only', but L-3's interim list names v1_bundle.rs, the corpus and collection.rs only, and has no scalar entry. Remove the clause, or say that the FR-015-AC-58 guard's list loses scalar.rs. | spec/assurance/AD-004-cg-crate-layout.md:948, spec/assurance/AD-004-cg-crate-layout.md:799-806 |
| FND-006 | low | H-1 Input says ty is 'a KaniPrimitive from kani/abi.rs'; the type is KaniPrimitiveType (src/lib.rs:127). | spec/assurance/AD-004-cg-crate-layout.md:454-456 |
| FND-007 | low | The label row says the header is 'the one header comment line, // Obligation: <label>'. The scalar source opens with two lines, `// SPDX-License-Identifier: MIT OR Apache-2.0` and the Obligation line, then a blank line, and a newline follows the oracle source (scalar.rs:463-470). R-1 item 2's 'the header' does not define it. Byte identity needs render's fixed header spelled out. | spec/assurance/AD-004-cg-crate-layout.md:443, spec/assurance/AD-004-cg-crate-layout.md:519-521, src/kani/generate/scalar.rs:463-470 |
| FND-008 | medium | H-3 item 6 requires every message to be 'valid under H-2', but H-2 defines validity for ExprText only (it parses as syn::Expr). Messages go into `assert!(c, "<message>")` and `kani::cover!(c, "<message>")` format strings, and the scalar assert message interpolates the operation identity. A message with `"` or `\` breaks the literal. A message with `{` or `}` becomes a format placeholder, which syn::parse_file accepts and rustc rejects only under Kani. Fix: define the message rule (refuse or escape `"`, `\`, `{` and `}`) and add it to FR-015-AC-112. | spec/assurance/AD-004-cg-crate-layout.md:493, spec/assurance/AD-004-cg-crate-layout.md:470-480, src/kani/generate/scalar.rs:439 |

## Dispositions

Round 1, checked at 15d7cad (fix 77f5178..15d7cad).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Let carries an optional spec-local LetType {I64, I128}. That is the right call: KaniPrimitiveType is public API (lib.rs:127) with Boolean and I64, and widening it to i128 would change both the public surface and the binding ABI that H-4 says stays unchanged. |
| FND-002 | fixed | The path row names core/identity.rs, and the import rule lists outcome, core::identity, kani::abi for spec and drops record from render. |
| FND-003 | fixed | The false AC-7 citation is gone, and the true-cover rule is stated as new in 4b. |
| FND-004 | fixed | R-1.1 now says render is the spelling site for the families it renders, and that L-3 lands after 4f, 4g and 4h. |
| FND-005 | fixed | Step 4b now says the FR-015-AC-58 guard list loses scalar.rs and gains render.rs. |
| FND-006 | fixed | The type is now KaniPrimitiveType. |
| FND-007 | fixed | The fixed header is spelled out: the SPDX line, the Obligation line, a blank line, the oracle source, then one newline. |
| FND-008 | fixed | A message must not contain a double quote, backslash, brace or line break, and one that does is refused rather than escaped; label follows the same rule. AC-112 and step 42 now test it. |
