---
id: SR-12821
title: spec-review-ears-conformance review of IR-796 standalone historical disposition
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-contract-codegen; spec/oracle/functional/FR-035-caller-text-admission.md,
  spec/oracle/matrix/TC-050-caller-text-admission.md, spec/oracle/matrix/tests.md,
  spec/kani/matrix/tests.md
review_set: subset
---

## Summary

Ticket: IR-796. This is a standalone historical measured disposition, not implementation acceptance for draft #381. No caller_text.rs exists on main. The six FR-035 criteria, symbolic length/two alphabet-index encoding, TC-050 procedure and expected results are unchanged from main. All six criteria compute untagged; this pre-existing planned implementation gap is owned by IR-494/#381 and is not new coverage or proof credit. No Cargo, Kani or historical measurement was run. No applicable AssuranceProfile exists in spec/.

## Scope

- {excerpt: 'When a caller requests runtime Text admission, the code generator shall emit an

    oracle and a separately bounded Kani obligation for a caller-owned

    `TextPayload` and declared `TextType`. This is a caller-ingress claim, not a

    claim about an expression node in an admitted `CheckedPackageV2`. The oracle

    calls the exact kernel''s `quire_exact::admit_text` API and returns its

    outcome and metering unchanged. Its evidence identifies the caller-declared

    profile, inclusive length bounds, payload class and proof configuration,

    without `ir_confirmed` provenance, a checked node id or a QSL term identity.


    I', id: FR-035/Description, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: "- A caller-supplied `TextType::new(min, max, profile)` with nonempty inclusive\n  bounds and one of the six `TextProfile` values.\n- A runtime `TextPayload` made by the public `TextPayload::from_utf8(&[u8])`\n  constructor, and a caller-supplied `Meter`. Invalid UTF-8 bytes are refused\n  by that constructor before a `TextPayload` exists or admission is called.\n- For proof, a finite symbolic source class: zero, one or two Unicode scalars\n  chosen independently from `e`, U+0301 COMBINING ACUTE ACCENT, U+00E9 LATIN\n  SMALL LETTER E WITH ACUTE, and U+1F600 GRINNING FACE. The harness encodes\n  the sel", id: FR-035/Inputs, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: "- When given a valid `TextPayload` and `TextType`, the emitted oracle shall\n  call `rt::admit_text(&payload, &text_type, meter)` and return its `Outcome<Text>`\n  unchanged. It shall neither pre-normalize the payload nor substitute an\n  unmetered local bound check.\n- When `TextPayload::from_utf8` rejects invalid bytes, the caller-ingress\n  route shall expose that constructor refusal before it can call admission;\n  the meter shall not be charged by admission. The generated oracle takes a\n  `TextPayload`, so this refusal is not an `admit_text` outcome.\n- When an admitted payload violates the decl", id: FR-035/Behavior, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: "- **Upstream**: quire-exact, the exact kernel this repository locks, which\n  defines and exports `quire_exact::admit_text`, `quire_exact::TextPayload`,\n  `quire_exact::TextType` and `quire_exact::Meter`.\n- **Execution**: [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md)\n  classifies and retains evidence for the generated caller harness.\n- **Related**: [FR-014](./FR-014-exact-scalar-oracles.md) for checked-package\n  text comparison. QSL-694 explores text-to-number parsing inside a term; it\n  does not gate this caller-ingress operation.\n- **Proof cost**: With the current kernel a", id: FR-035/Dependencies, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'A generated caller-ingress oracle accepts `&TextPayload`, the declared `TextType` and `&mut Meter`, calls `rt::admit_text` once, and returns the runtime outcome, retained text, original runtime provenance, charge sequence and consumed counters unchanged for all six profiles over representative empty, ASCII, combining, composed and supplementary payloads.', id: FR-035-AC-1, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8` before admission and leave the admission meter unchanged; `admit_text` itself is never claimed to accept those bytes.', id: FR-035-AC-2, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'For each profile, `Text[1,1]` admits or refuses according to the profile length of the retained sequence, including `e` plus U+0301 under NFC and NFD; a length refusal occurs before `text.result-retain`, and denial of each reached charge point yields the direct runtime stop, charges and consumed counters.', id: FR-035-AC-3, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness symbolically ranges over all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime call including outcome and metering, and ends with one satisfied cover after its assertions. The proof identity names this finite class and its type.', id: FR-035-AC-4, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'In a copy of the generated oracle with its admission call or declared `TextType` altered so the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement assertion with concrete playback; restoring the call verifies over the same symbolic class.', id: FR-035-AC-5, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}
- {excerpt: 'A request outside the declared finite proof class has a typed unsupported or requires-bound disposition and no harness, and no caller-ingress claim is marked `ir_confirmed` or assigned a checked-package node id. The six attempted text-operand `numeric.convert` nodes remain Contract IR refusals and do not count as passing caller-admission coverage.', id: FR-035-AC-6, path: spec/oracle/functional/FR-035-caller-text-admission.md, role: examined}

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the requirement grammar and semantic trigger in FR-035. The event is a caller requesting admission, the actor is the code generator and the response is concrete oracle/obligation emission. The edited API attribution and dependency paragraph introduce no new shall statement. Existing Behavior is examined as context without requesting unrelated grammar rewrites. Scoped engine validation reports no FR-035 grammar warnings; its three warnings belong to unchanged FR-017 and FR-014. No new vague or ambiguous normative response was introduced.
