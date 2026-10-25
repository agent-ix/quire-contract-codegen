---
id: SR-17017
title: "IR-705 CG #514 spec review (base): native integer route vs AD-006 and FR-015"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen#514@27d2cfd086fa40a74d50af769f2542a908961bdf vs origin/main 4eccd682; spec/assurance/AD-006-admitted-native-integer-oracles.md, spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-119, FR-015-AC-125, FR-015-AC-126, FR-015-AC-127), spec/kani/matrix/TC-027-kani-execution-evidence.md, docs/qualification/ir705-native-map.md"
review_set: subset
---
# SR-17017: IR-705 CG #514 spec review (base)

## Summary

Ticket: IR-705. The PR changes no `spec/**` or `plan/**` file. It adds one qualification document, `docs/qualification/ir705-native-map.md`. This base review checks two things:

- whether the implemented route conforms to its governing spec units: AD-006 System Boundary / Execution / Refinement views, FR-015-AC-119/125/126/127, and the TC-027 IR-705 regeneration section;
- whether those units are clear enough to decide the PR's design choices.

Conformance results:

- **Typed capacity refusal: conforms.** It is kept separate from semantic-domain refusal (AD-006 Execution view).
- **Identity binding: conforms.** Policy and revision are bound into artifact identity (FR-015-AC-125).
- **Original domains: conform.** The original i64 draws, solver, unwind and ceilings are retained (FR-015-AC-126, TC-027).
- **IR-917 postcondition fixture: conforms.** It matches FR-015-AC-119's authored `amount < 1000 implies amount + 1 <= 1000` over `Int[0,1000]`.

The defects are a spec ambiguity that let the scalar route diverge from AD-006, and a qualification document that misdescribes the implemented control.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-125 and AD-006 do not say what selects the native route for routed generation. It could be automatic when the plan admits, or an explicit caller opt-in, and nothing says which production artifact the selection applies to. The AC says "the existing policy returns an honest general route or typed … native refusal", and AD-006 says production and Kani clients "call the same kernel-owned admitted operations". The PR reads this as a separate public opt-in, `negotiate_kani_native_obligations`, that only the proof harness takes. Routed production generation stays general. That reading produces SR-17015 FND-001 without contradicting any single sentence. Two implementers would diverge here. The AC should name the selection trigger and require the routed oracle artifact and the harness-embedded oracle to be the same bytes on the native route. | spec/kani/functional/FR-015-bounded-kani-obligations.md:872, spec/assurance/AD-006-admitted-native-integer-oracles.md:40 |
| FND-002 | low | `docs/qualification/ir705-native-map.md` says the TC-027 bound mutant is "the same public harness; only its independent result bound changes" and that it retains "the original native oracle". The implemented control regenerates the package with a changed semantic domain, which changes the oracle's plan domain and selection record too (SR-17015 FND-002). The qualification map therefore describes evidence the code does not produce. | docs/qualification/ir705-native-map.md:11 |

## Verdict

**PASS WITH FINDINGS (spec level).** No spec text changed, and the governing units are internally consistent. FND-001 is the gap that let the scalar native route bind its proof to a proof-only oracle. Resolve it in the code (SR-17015 FND-001) or tighten it in the spec before AC3 qualification. Fix FND-002 together with SR-17015 FND-002.
