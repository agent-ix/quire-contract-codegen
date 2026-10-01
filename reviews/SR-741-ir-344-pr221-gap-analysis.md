---
id: "SR-741"
title: "CG PR 221 gap analysis: do the AD-004 amendments close SR-731's three gaps"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@5f257db12acc16dac22fece8c0a9e93aa3955183; spec/assurance/AD-004-cg-crate-layout.md (steps 2d-0, 2f, 2g-0, module map, L-2), src/ at base 66a67c8"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-741: CG PR 221 gap analysis

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#221 at 5f257db. This is a focused, planless
check of whether the amendment closes the three gaps that SR-731 found (FND-001 to FND-003), each
checked against the code at base 66a67c8.

1. Naming helpers (SR-731 FND-001): closed. 2d-0 creates `core/naming.rs` before 2d moves
   `oracle.rs` whole, and the constant gets a home in `core/artifact.rs`. `dependency_parameters`
   is not naming: it calls `typed_dependency_parameters` and then `analyze_supported_expression`,
   and only `src/harness.rs:14` uses it. The `MAX_GENERATED_SOURCE_BYTES` users are oracle, kani,
   harness, state_frame, strategy, kani_obligations, exact_scalar, exact_function,
   composite_equality and bound_strategy/{census,generation,population}, plus the `lib.rs`
   re-export. All are above `core`. One residue is FND-002.
2. 2f split wording (SR-731 FND-002): closed. 2f now calls the split a verbatim item move, and
   Risks agrees.
3. Root-import rewrite (SR-731 FND-003): partly closed. Each of 2d to 2g owns the root imports in
   the files it moves, and 2g-0 sweeps what is left. The 12-file count is right for `use` lines.
   But two non-`use` root paths fall outside both the sweep and the L-2 test (FND-001).

Stubs: none, the PR adds no code. Matrix: unchanged, spec-only PR.

## Verdict

CONDITIONAL: one medium gap in the root-import ownership, plus two low notes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Two files name a root re-export as an inline type path, not in a `use` line: `crate::GenerationDiagnostic` at `src/kani.rs:889` and at `src/kani_obligations.rs:961`. The dependency rule (line 294-295) says `lib.rs` is the only file that names a root re-export. But 2g-0 is scoped to "`use crate::{..., Item}` and `use crate::Item`", the 12-file count covers `use` lines only, and the L-2 layout test "reads `use crate::` lines", so neither the sweep nor the test catches these two. Risks lists macro and `super::` paths as the layout test's blind spots but not inline `crate::Item` paths. Widen 2g-0 to inline `crate::Item` paths in code (string literals such as `"crate::State"` in generated templates excluded), and either add inline paths to L-2's test or name them in Risks | spec/assurance/AD-004-cg-crate-layout.md:622-626, spec/assurance/AD-004-cg-crate-layout.md:531-533, spec/assurance/AD-004-cg-crate-layout.md:294-295, spec/assurance/AD-004-cg-crate-layout.md:747-749, src/kani.rs:889, src/kani_obligations.rs:961 |
| FND-002 | low | Row 231 puts `typed_dependency_parameters` with `dependency_parameters` in `oracle/boolean_v1.rs`, giving "they run the V1 expression analysis" as the reason. That holds for `dependency_parameters` (harness only), but `typed_dependency_parameters`, `DependencyParameter` and `RustValueType` are also imported by `src/kani.rs:23` and by the V2 `src/kani_obligations.rs:77-78,968`. `boolean_v1.rs` is "deleted with V1" (step 6, line 725-726). The V2 Kani path already depends on the same file through `generate_boolean_oracle` (root import, `kani_obligations.rs:59`), so the dependency predates this PR. Still, the new row states a reason that does not match the code. Either name the V2 Kani users in the row, or leave the home of `typed_dependency_parameters` to the oracle split and do not assert it here | spec/assurance/AD-004-cg-crate-layout.md:231, src/kani_obligations.rs:77, src/kani.rs:23 |
| FND-003 | low | 2d-0 does not say where the three `#[cfg(test)]` tests of `unique_names` and `oracle_symbol` go (`src/oracle.rs:1068-1134`, traced to FR-022-AC-9 and TC-033). They test the moved items, so they should move to `core/naming.rs` with them. If they stay in `oracle`, `use super::{oracle_symbol, unique_names}` breaks, and the TC-033 trace ends up living away from the code it covers. Say that the tests move with the items | spec/assurance/AD-004-cg-crate-layout.md:607-614, src/oracle.rs:1068-1134 |

## Coverage

- Rows: unchanged (spec-only PR; no test or tag touched).
- Semantic review: skipped (an AD amendment adds no criteria).
- Plan completion: not assessed
