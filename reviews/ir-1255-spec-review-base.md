---
id: SR-202610310168
title: "IR-1255 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/replay/functional/FR-024-counterexample-envelope-intake.md; spec/core/functional/interface-001-codegen-api.md"
review_set: subset
---

## Summary

Ticket: IR-1255. PR: quire-contract-codegen#704. Frozen head 70ed73bf40cfacd5b8a53bc92155c62123a38336. Reviewed AC-54 through AC-56 and the changed public API inventory against the locked QSL call-site type, driver FR-009-AC-17/19, and current CG frame replay source. No applicable AssuranceProfile was found under spec/.

## Verdict

FINDINGS: address the rows below before spec handoff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The unchanged FR-024 description says CG obtains frame_operation from its own QSL call-site lookup, while AC-54 and the new Behavior require propagation of the driver-supplied target. Amend the owning description so it states the driver target is copied after comparison; the internal lookup only validates it. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:60-63,526 |
| FND-002 | medium | AC-54 through AC-56 claim Test verification but TC-035 has no procedure or expected results for the selected-target public input, exact propagation, missing/foreign/unknown association, or same-node different-ordinal controls. Add focused TC-035 cases and link these ACs to TC-035. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:526-528 |

## Scope and bindings

```yaml
{
  "clean": false,
  "scope": [
    {
      "id": "FR-024-AC-54",
      "path": "spec/replay/functional/FR-024-counterexample-envelope-intake.md",
      "role": "examined",
      "excerpt": "Test |"
    },
    {
      "id": "FR-024-AC-55",
      "path": "spec/replay/functional/FR-024-counterexample-envelope-intake.md",
      "role": "examined",
      "excerpt": "Test |"
    },
    {
      "id": "FR-024-AC-56",
      "path": "spec/replay/functional/FR-024-counterexample-envelope-intake.md",
      "role": "examined",
      "excerpt": "Test |"
    },
    {
      "id": "interface-001:FrameReplay::new",
      "path": "spec/core/functional/interface-001-codegen-api.md",
      "role": "examined",
      "excerpt": "FrameReplay::new inputs, output and semantics inventory changed for selected target and association."
    }
  ],
  "bindings": []
}
```
