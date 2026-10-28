---
id: SR-619011
title: "IR-1086 AD-004 frame boundary spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen#622; IR-1086; spec/assurance/AD-004-cg-crate-layout.md; FR-024-AC-30"
review_set: subset
---

## Summary

Base review of the AD-004 L-6 edits in CG #622 at 5bd6d62df149057dac0a3ef99a048deaf80aa018. The boundary accounting is directionally correct, but one edited sentence is malformed and the related FR-024 current-state text still names the typed frame slice as pending.

## Examined units

- AD-004 L-6 (examined): Frame, function, and state-clause replay intake boundaries and interim exceptions.
- FR-024-AC-30 (examined): Planned typed intake and QSL-emitted-harness acceptance condition.
- FR-024 Current state (examined): The paragraph naming remaining work under IR-1086 and IR-624.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The edited sentence reads “Frame replay retains the frame replay boundary also takes typed Playback,” so the AD no longer states the boundary coherently. | spec/assurance/AD-004-cg-crate-layout.md:816 |
| FND-002 | low | The AD marks typed frame intake complete, but FR-024 Current state still says that typed intake remains planned under IR-1086; distinguish the delivered boundary from the IR-624 emitted-harness work. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:525 |

## Verdict

CONDITIONAL until the two documentation inconsistencies are fixed. The changed AD file passes structural Quire validation per the coder's focused evidence; that gate cannot detect these semantic defects.
