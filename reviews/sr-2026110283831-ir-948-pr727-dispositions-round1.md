<!-- reviewer-dispositions repo=agent-ix/quire-contract-codegen visibility=public quoin=0.29.1 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-2026110283831 round=1 model=gpt-6 run=1900249b-3122-4032-bebd-a4d068ff72b1 -->

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Verified in fix diff; after_excerpt below. |
| FND-002 | still-open | The no-result postcondition emits a typed unit call but checks only the oracle; the subject can be arbitrarily mutated without affecting the proof assertion. |

New finding in this fix round:

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-002 | high | soundness | The no-result scalar proof drops the prior subject-versus-expected assertion and observes no subject result or state; a compiled subject mutant cannot falsify it. |

+++ [reviewer data]

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: "let result_postcondition = if kind == ObligationKind::Postcondition\
    \ {\n        Some(read_postcondition_signature(view, clause, census)?)\n    }\
    \ else {\n        None\n    };"
- fnd: FND-002
  outcome: still-open
  reason: The no-result postcondition emits a typed unit call but checks only the
    oracle; the subject can be arbitrarily mutated without affecting the proof assertion.
new_findings:
- fnd: FND-002
  severity: high
  confidence: high
  method: code-review
  check: soundness
  artifact_id: FR-015-AC-126
  related_ids:
  - FR-025-AC-12
  - FR-015-AC-119
  path: src/kani/generate/clause/v2.rs
  lines: 1195-1208
  excerpt: Genuine TC-027 healthy and bound mutant, TC-025 arithmetic mutant, IR-917
    clause implication and IR-948 V2 pre/post/invariant profiles regenerate through
    public CG with the reviewed admitted kernel API. Original semantic domains, selected
    propositions/clauses, independent expected values, subject controls, assertions,
    final completion covers and existing solver/unwind/time settings remain. Static
    plan-admission checks establish that the original TC-027, IR-917 and IR-948 fixture
    operand/intermediate intervals remain admitted under this policy, including their
    original 0..1000 domains; those ch
  finding: The no-result scalar proof drops the prior subject-versus-expected assertion
    and observes no subject result or state; a compiled subject mutant cannot falsify
    it.
```

+++
