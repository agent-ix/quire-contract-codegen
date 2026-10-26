---
id: SR-590005
title: 'spec-architecture-evaluation review of IR-1022 CG PR #590'
type: SpecReview
analysis: architecture-evaluation
scope: agent-ix/quire-contract-codegen; spec/assurance/AD-007-v2-symbolic-width-tractability.md
review_set: subset
---

## Summary

Independent Claude Opus 5.5 review of CG PR #590 (Ticket: IR-1022) at 1e3c2936b997e047198791674400f7224e096295, base 4788b46723d7454af86fdbe9c7b4f4ad1154cbb6. AD-007 was evaluated against the stated quality goals: full-domain soundness, unchanged oracle, subject and native expectation, sealed identity, typed witness refusal, and the 16 GiB ceiling. It was also evaluated against the current generator (`clause.rs` `symbolic_arguments`), identity (`ObligationBinding.primitive_type`) and V2 decoder (`witness.rs` `byte_width`/`decode_falsification`).

The proposal is internally sound. The verification plan can fail: tractability fails as MemoryExhausted and is recorded as inconclusive, and the representation, witness-refusal and exact-999 controls each name a failing form.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AD-007's Decisions table weighs no alternative, and a narrower sound option exists. The harness can keep the existing i64 `kani::any()` draw and its [0,1000] assumption, then rebind `let amount: i64 = i64::from(raw as u16);`. Under that assumption the conversion is the identity, so all 1001 values reach the same subject. Downstream arithmetic sees constant-zero high bits, which is the claimed tractability mechanism. The witness stays 8 bytes, the decoder and persisted schema stay the same, and the exact-999 playback is unchanged. Only the harness source and its record change, as AC-47 already allows. This may avoid the FR-015-AC-125/AD-006 draw conflict entirely. It should be evaluated and recorded before the owner is asked to amend two normative artifacts. | spec/assurance/AD-007-v2-symbolic-width-tractability.md:111 |
| FND-002 | medium | AD-007 asks the owner to amend FR-015-AC-125 and AD-006 before any measurement shows that width reduction lowers peak RSS. Its own risk notes that the oracle and the independent evaluator both run the real native kernel and Meter. AD-006's risks name Meter expansion as a likely cost too. An earlier, uncredited diagnostic is available and changes no contract: a labelled no-credit width-reduced or rebound harness at 16 GiB, or a formula-size comparison. Without it, the owner decision rests on an unmeasured benefit. | spec/assurance/AD-007-v2-symbolic-width-tractability.md:112 |
| FND-003 | low | AD-007 says to "record the selected physical draw primitive", but `ObligationBinding.primitive_type` already records the draw type. `symbolic_arguments` uses that same field as the Rust type of the binding passed to the subject, and the decoder uses it for byte width. AD-007 should say whether `primitive_type` becomes the physical carrier and a separate semantic argument type is added, or the reverse. Otherwise the subject binding type or the decoder width could silently follow the wrong field. | spec/assurance/AD-007-v2-symbolic-width-tractability.md:75 |

## Verdict

Changes requested before the owner decision. FND-001 and FND-002 change what the owner is asked to decide. FND-003 is a grounding clarification. No owner approval exists, and no 16 GiB fit is claimed or implied.
