<!-- reviewer-dispositions repo=agent-ix/quire-contract-codegen visibility=public quoin=0.29.1 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-2026110283832 round=1 model=gpt-6 run=1900249b-3122-4032-bebd-a4d068ff72b1 -->

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Verified in fix diff; after_excerpt below. |
| FND-002 | fixed | Verified in fix diff; after_excerpt below. |

+++ [reviewer data]

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: p.model_operations(c, 0)
- fnd: FND-002
  outcome: fixed
  after_excerpt: assert_eq!(domain_assertions, 1, "the produced domain is an assertion");
```

+++
