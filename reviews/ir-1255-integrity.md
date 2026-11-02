---
id: SR-202610310169
title: "IR-1255 integrity review"
type: SpecReview
analysis: integrity
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
| FND-001 | medium | AC-54 says all four values, including request_index and package_id, equal the QSL call-site result. At the locked QSL revision, CallSite<OperationSite> exposes package bytes and OperationSite exposes frame_operation and frame_occurrence, but neither carries request_index. State separately that the driver association matches the independently selected CG item and the target value/frame occurrence match QSL; name the source of each comparable value. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:526 |

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
