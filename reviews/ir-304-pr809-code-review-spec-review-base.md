---
id: SR-17940486709564847047753
title: "IR-304 PR #809 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #809 at 3c7845ca; IR-304 second PR; spec/routed/functional/FR-022-routed-generation.md, spec/routed/matrix/TC-033-routed-generation.md, spec/routed/matrix/tests.md, docs/qualification/ir304-kani-guard-native-harness.md, docs/qualification/ir304-kani-guard-cover-attempt.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: reviews
---

## Summary

Ticket: IR-304. The spec edits flip FR-022-AC-35 from PLANNED to an unqualified criterion,
which reads as implemented. They add a harness-shape sentence and the measured verdicts to the
AC text. TC-033 step 28 and the "Scoped evidence status" paragraphs now say AC-35 is covered by
the qualification record. The routed `tests.md` row reads `✅ Implemented`. `quire validate`
passes on the three spec files. Sub-analyses: only base review ran. The edits are status and
criterion-text edits to one AC; no new FR, relationship edge or domain object, so
ears/integrity/object/dependency analyses do not apply.

Key question, whether FR-022-AC-35 may be marked Implemented: not as worded. The AC claims "FR-017
execution under installed Kani reports `CoverUnsatisfied`", with verification method Test
(TC-033). TC-033 says to "run both harnesses through FR-017 under installed Kani and read the
evidence outcome". The evidence is raw `cargo kani` on the generated text in a scratch crate. The
record itself says CG's runner, the FR-017-AC-63 two-phase run and classification never ran, and
the FR-017 verdict names were mapped by hand. The raw runs are genuine: the reviewer matched the
executed crates to this head's generated text, and the Kani logs confirm each verdict. So the
evidence is real but of a different kind. This repo's precedent for that kind is FR-015-AC-147
and AC-148 (IR-1022). Those rows read "Implemented (...). Partial installed-Kani evidence ...
run with raw `cargo kani` ...", use method Demonstration, and hand the runner leg to a separate
AC (FR-015-AC-151, done in IR-1509 with `tc_025_real_kani_runner_*` tests). Either option below
is acceptable:

1. Preferred, and cheap here: add the kani-lane runner test (gap-analysis SR FND-001), which
   makes the current wording true. Keep Test (TC-033) and drop the measured numbers from the AC
   text, leaving them in the record.
2. Keep the raw-run evidence only. Mark AC-35 PARTIAL (owner IR-304 or a named follow-up) and
   say how it was shown: "raw `cargo kani` (Kani 0.68.0) on the `generate_routed_native` harness
   text copied to a scratch crate, not CG's runner; the verdict names are mapped by the FR-017
   rule". Leave the FR-017 execution clause Planned, set the method to Demonstration for the raw
   part, and match TC-033 step 28 and the `tests.md` row (no ✅).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | FR-022-AC-35 now reads as implemented, and its evidence status reads covered (FR-022 "Scoped evidence status", TC-033 step 28 and "Scoped evidence status", `tests.md` `✅ Implemented`). Yet its own verdict clause ("FR-017 execution under installed Kani reports `CoverUnsatisfied`") and the TC-033 procedure ("run both harnesses through FR-017") were not met. The cited record states the runs bypassed CG's runner and classification. Status and wording must either be backed by a runner test or say how the result was shown (Partial, raw `cargo kani`, Demonstration), as FR-015-AC-147/148 do | spec/routed/functional/FR-022-routed-generation.md:711, spec/routed/functional/FR-022-routed-generation.md:845-847, spec/routed/matrix/TC-033-routed-generation.md:313-317, spec/routed/matrix/TC-033-routed-generation.md:328-329, spec/routed/matrix/tests.md:27 |
| FND-002 | medium | The new AC-35 sentence "The routed integer-arithmetic harness computes on native integers, never drops its meter or outcome ..." is wrong for ordinary `generate_routed`. That entry point keeps the general `rt::Integer` route (AD-006: "ordinary generate_routed keeps the general route"), and the record of the cover attempt shows its harness for this same scenario gives no verdict in 30 minutes. "CG emits a harness" names no entry point either. Name `generate_routed_native` (the native integer policy) as the route this AC covers, and state that the general route still has no verdict for this scenario | spec/routed/functional/FR-022-routed-generation.md:711, spec/assurance/AD-006-admitted-native-integer-oracles.md:32 |
| FND-003 | low | AC-35 now packs four obligations into one row: the verdict pair, the harness text shape, the mutant outcomes, and measured step count, wall time and memory (95,300 steps, 8.0 s, 0.60 GB). Measured numbers are evidence, not requirement. In the AC text they go stale on any toolchain or quire-exact change, a churn source. Keep the measurements in the qualification record and TC-033, and split the text-shape clause (already tested) from the verdict clause so each can carry its own status | spec/routed/functional/FR-022-routed-generation.md:711 |
| FND-004 | low | The routed `tests.md` row uses `✅ Implemented`, while sibling rows use `✅ Covered` / `⚠️ Partially covered` / `🚧 Planned`. The cover-attempt record now ends "FR-022-AC-35 was then delivered on the native harness", which claims more than the native record's "What this does not establish" section allows | spec/routed/matrix/tests.md:27, docs/qualification/ir304-kani-guard-cover-attempt.md:37 |

## Verdict

Not mergeable at 3c7845ca until FND-001 and FND-002 are fixed in this PR; FND-003 and FND-004
are small wording fixes for the same edit. The qualification record itself is accurate and
appropriately qualified. The defect is that the AC, TC-033 and the matrix drop those
qualifications.
