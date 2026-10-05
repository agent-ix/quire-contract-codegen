---
id: "SR-1526"
title: "CG PR 281 criterion strength: FR-031-AC-2, AC-6, AC-16 to AC-18, AC-22 to AC-25"
type: SpecReview
analysis: criterion-strength
review_set: subset
scope: "agent-ix/quire-contract-codegen@607a975f0ab00aab0730539b75ac506a1380dbaa; spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md AC-2, AC-6, AC-16, AC-17, AC-18, AC-22, AC-23, AC-24, AC-25; spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md steps 1, 4, 5, 7, 10, 11, 14-17"
---

# SR-1526: CG PR 281 criterion strength

## Summary

Ticket: IR-602. The skill's Jev client does not exist (its own SKILL.md says no client is
implemented), so these judgements are made by hand over the spec text, with the vectors computed
from the kernel and IR code.

Each AC can fail directly:

- AC-2 is a source inspection that pins `DivisionProfile::Truncating`, the member and `Bounded`.
- AC-6, Z-1 to Z-4: each vector reaches a zero divisor and must give `Undefined`, not
  `Refused`. For Z-3 and Z-4, `y + 11` at `y = -11` completes with 0 inside -10..=11, so the
  divide is reached.
- AC-16, O-3 and O-5: a pair-admitting or overflow-reporting division fails one of the two.
- AC-17, R-1 and R-2: q 2 in 1..=10 with r 0 outside it, and r 0 in -10..=5 with q 10 outside
  it. A pair division gives `Refused` on both, so the test fails. IR admits both: R-1's quotient
  interval is [1,10] and R-2's remainder interval is [0,0].
- AC-18 asserts the variant is absent.
- AC-22 asserts the alias, the type paths and the one-copy lock.
- AC-23 asserts the syntactic shape (`BinOp::Div`, `wrapping_rem`, no `BinOp::Rem`).
- AC-24: the Z-1 bundle with unconstrained operands reaches both y=0 and (MIN,-1) through
  `y <= 1`. For the Z-3 bundle, the guard `y <= 0` rules out add overflow, so only the zero-divisor
  check can fail. A raw `%` probe at (MIN,-1) fails where `wrapping_rem` verifies.
- AC-25 includes (MIN,-1) under a full-range type, where a raw `%` panics in plain `cargo test`,
  so a regression to `%` fails it.

## Verdict

The criteria are strong. One wording gap remains in AC-17.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-17's model says "the selected member in the type's interval: `Completed` with it". The oracle under test returns `Outcome<bool>`, so the member value cannot be seen. The comparison is what can be compared, as the AC-8 test's `Expect::Completed(bool)` does. Also, R-1 and R-2 complete `true` under every law, so the law is pinned behaviourally only if the grid's comparison bounds separate the laws (for example `x / y <= -4` at x=-7, y=2: false under truncating, true under floor). Say "`Completed` with the comparison over that member", and name one vector that separates the laws. AC-2 already pins the law syntactically. | FR-031:472; TC-044 step 7 |

## Dispositions

Round 1, reviewed at f27977083e292d1334aa7a92c63f570af305b532 (fix commit f279770 plus a merge of main 735e704).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f279770: AC-17 now says "`Completed` with the comparison's Boolean over that member" and adds the law vector `y != 0 && x / y <= -4` over `reject` -10..=10 at x=-7, y=2. By hand: IR admits it, because each divisor subinterval gives a quotient interval of [-10,10], inside the type. Truncating gives -3, so `-3 <= -4` is false. Floor gives -4 and Euclidean gives q=-4, r=1, so both are true. The vector separates the laws, and TC-044 step 7 carries it too. |
