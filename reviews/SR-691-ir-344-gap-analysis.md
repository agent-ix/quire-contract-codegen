---
id: "SR-691"
title: "CG PR 217 gap analysis: AD-004 steps 1b-1d against spec and matrix"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@757efc45d63ec97dddb83bbada83dacaeb908a76; src/identity.rs, src/profile.rs, src/routed_generation.rs, spec/core/functional/interface-001-codegen-api.md, spec/routed/functional/FR-022-routed-generation.md, spec/routed/matrix/TC-033-routed-generation.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/assurance/AD-004-cg-crate-layout.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: references
---

# SR-691: CG PR 217 gap analysis

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#217 at 757efc4. Planless gap analysis
scoped to the diff. It checks the new code's owning requirements, the traces of the new tests,
and AD-004's step 1b to 1d decisions (L-10, L-11, the identity and profile bullets) against the
code. Plan completion: not assessed.

Checked and found as stated:

- 1b: the public and persisted identity records (`KaniObligationIdentity`,
  `ScalarObligationIdentity`, `StateFrameIdentity`, `ObligationDisposition::Supported`) and
  `KaniExecutionEvidence` carry `ModuleSymbol`, `HarnessSymbol` and `KaniSolver`, not `String`.
  The private lowering structs still hold `String` stems. That matches the coder's stated
  reading of L-10 (public and persisted types only), and they become typed at render.
- 1c: the oracle manifest is written once (`src/profile.rs`) and the three emitters call it.
  `RUNTIME_REVISION` is deleted from src, tests and the lib re-export. No spec text outside
  TC-029 and TC-031 named the runtime revision (grep of spec/).
- 1d: as AD-004 Shared core states, and `make deny` passes with one copy.
- The stated "not done" items (1a, typed bounds, `decode_falsification` `&str`, flat layout) are
  outside steps 1b to 1d or deferred by AD-004 itself.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `RoutedGenerationError::DuplicateHarness { harness: HarnessPath }` is a new variant of a public error with no owning requirement. interface-001 lists the variants `RoutedGenerationError` has and does not include it. FR-022 has no criterion for it (AC-5 covers `DuplicateRequestIndex` and `MissingKindContext` only). Its test `tc_033_two_harnesses_sharing_a_symbol_are_a_typed_error_not_a_silent_overwrite` traces TC-033, whose procedure has no step for a duplicate harness symbol. A reader of the API catalog cannot learn the refusal exists, and the trace counts toward TC-033 coverage that TC-033 does not describe. Fix: add the variant to interface-001:139. Then either add an FR-022 criterion and TC-033 step, or trace the test to AD-004 L-10 alone and say it is an internal-invariant guard, with no TC claim | spec/core/functional/interface-001-codegen-api.md:139, src/routed_generation.rs:140-145, src/routed_generation.rs:427 |
| FND-002 | low | Deviation from AD-004 not recorded in AD-004. The identity bullet says "`generate_routed` keys by `HarnessPath`", and L-10's test is "a duplicate-key test on `generate_routed`". The code keys by `HarnessSymbol`: `index_harnesses` returns `BTreeMap<HarnessSymbol, _>`, and the test calls the private helper, not `generate_routed`. Keying by symbol is forced, because a record's `Supported` disposition names only `harness_symbol`, so this is a defensible choice that the coder disclosed. But the merged AD now states a design the code does not follow, and the next step (2b, the identity collapse) will read the AD. Amend the bullet and L-10 to "keys by `HarnessSymbol`, the name a record carries", or key by path once records carry it | spec/assurance/AD-004-cg-crate-layout.md:461-465, :549-550, src/routed_generation.rs:155-171 |
| FND-003 | low | The identity-validation tests (`tc_025_a_symbol_is_a_non_keyword_ascii_identifier`, `tc_025_a_harness_path_displays_module_then_harness_and_serializes_symbols_bare`) trace TC-025, whose procedure says nothing about symbol validation or bare serialization. The bare-serialization half does back TC-025's persisted-identity claim. The keyword and ASCII validation half has no TC step | src/identity.rs:120-152, spec/kani/matrix/TC-025-bounded-kani-obligations.md |

## Verdict

1b to 1d are implemented as AD-004 orders them, with no compatibility layer and no alias left
for `RUNTIME_REVISION`. The gaps are spec catalog and trace gaps around the one new public
behaviour (DuplicateHarness). FND-001 should be fixed in this PR, at least the interface-001
line. FND-002 and FND-003 are documentation of choices that are already sound.
