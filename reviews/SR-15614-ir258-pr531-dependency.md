---
id: SR-15614
title: IR-258 PR531 dependency review
type: SpecReview
analysis: dependency
scope: agent-ix/quire-contract-codegen; IR-258; spec/kani/functional/FR-046-rational-function-predicate-harnesses.md;
  spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md; spec/spec.md
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-046
  type: references
---

## Summary

Exact frozen head 73f1782c8c0482a7c9ec07462280a282fa94d961, base 3c045f06e412b20bd5c1d4048691a2f0ffa37f19, ticket IR-258, spec-only PR531. Two new artifacts and one registry row only. Author worktree is read-only; no Cargo, engine, namespace, aggregate or merge. #521 merge hold retained. FR-046 is a user-visible feature. Owner admission/lowering, exact kernel operations and correspondence, ABI, harness/run reporting and public replay are enablement prerequisites. Local edges FR-015/039/017 plus QSpec FR-196 resolve and form no cycle returning to FR-046. FR-025/ADR-004 ABI amendment is required before support (the contradiction is recorded by base/integrity). Absent whole-function/replay capabilities must remain named unsupported; accepted specification never asserts those capabilities have shipped.

## Verdict

PASS. Scoped review only; no whole-row implementation or proof acceptance.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Independent fractions.Fraction enumeration and constructor/accessor canonical-pair reasoning found 171 raw pairs, 111 accepted mathematical values, 12,321 argument tuples, zero healthy false tuples and 12,210 mutant false tuples. Owner RationalDomain bounds reduced numerator and positive denominator, so roundtrip restriction is a bijection for this exact domain; zero is 0/1. Owner public Rational::new/numerator/denominator exist; this is mathematical/static evidence, not current-cohort compilation or solver credit. Both retained original source fixtures were compared byte-for-byte against QSL db729fe811e3927d2ed45fe7a111dfe895cd939f, with only b+a to a+a changed.

Plan completion: not assessed

Only the changed planned feature is examined. Full repository reverse audit and optional whole-tree semantic audit are not claimed. No applicable AssuranceProfile exists in this spec tree. Object review is N/A: no new shared domain-object schema is introduced; existing Rational ownership stays in quire-exact. Criterion-strength/Jev is not invoked. Explicitly selected base/integrity/EARS/dependency/failure-domain methods ran; Rust review is N/A.

## Examined scope

```yaml
scope:
- id: FR-046-AC-1
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'The genuine source function `rational_commutative(a: Rational[-9,9;1,9],
    b: Rational[-9,9;1,9]): Boolean` with body `a + b = b + a` selects its whole declaration
    and complete body through the public QSL admission, strict IR reader/lowering
    and CG route. Its artifact proves that predicate, not only validity or parity
    of either addition. Replacing the body by `a + b = a + a` selects the same authored
    mutation with unchanged parameter domains.'
- id: FR-046-AC-2
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Every accepted symbolic argument has canonical constructor/accessor roundtrip
    equality and lies in its original inclusive component domains. Every semantic
    value in those domains has exactly one accepted representation. Zero is `0/1`;
    negative, fractional and boundary values remain reachable. For the original domain
    there are 111 canonical values per argument and 12,321 argument tuples; admitting
    all 171 raw component pairs, restricting denominators to one, or clipping an endpoint
    fails correspondence validation under QSpec FR-196-AC-5.
- id: FR-046-AC-3
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Generated evaluation preserves the authored full tree and declaration identity,
    exact owner arithmetic, intermediate values outside the input domains, arbitrary-precision
    continuation, full typed outcomes and Meter charge/stop order. Native comparison
    against an independent owner-operation reference checks those observations, including
    each denied charge prefix and exact/short budgets; it does not calculate expectations
    by running the generated oracle twice.
- id: FR-046-AC-4
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Generation introduces no assumption on completion, truth or an absence
    of undefined/refused/incomplete results. Controls producing each reachable noncompletion
    retain that typed outcome and cannot become a successful predicate proof; an outcome-based
    assume mutation is rejected. A completion assertion failure without a replayed
    false value is not labelled a confirmed semantic false counterexample.
- id: FR-046-AC-5
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: The healthy original function receives `Verified` only from an actual successful
    proof with all required checks and its completion cover satisfied. The wrong-addend
    mutation produces a substantive failed predicate assertion with actual solver
    playback of canonical in-domain arguments, and replay of that same authored function
    completes with Boolean false. A hypothetical witness or cover witness is not substituted
    for that playback.
- id: FR-046-AC-6
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Each emitted predicate harness has exactly one final completion cover after
    all predicate assertions, including harnesses with representation constraints.
    Unsatisfied or unreported completion cover, failed unwind, timeout, memory exhaustion
    or missing verdict cannot yield `Verified`; classifications retain FR-017 and
    FR-029 distinctions.
- id: FR-046-AC-7
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: A missing or unbounded Rational parameter domain yields source-associated
    `requires-bound` naming the selected function, argument and missing domain, with
    no harness. A finite component endpoint outside this encoding's signed-i64 capacity
    yields named `unsupported`, retaining the original endpoint and no harness; generation
    never clips, casts away precision or substitutes a smaller domain.
- id: FR-046-AC-8
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: An unavailable owner-lowered whole-function representation, absent one-to-one
    correspondence or unsupported body operation yields named `unsupported` with its
    source-associated cause and no harness. Every requested item has exactly one disposition
    under FR-039; a refused item does not remove or alter a supported sibling. Malformed
    source or owner-refused packages remain upstream refusals, not fabricated CG claims.
- id: FR-046-AC-9
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Counterexample decoding validates canonical components, original domains
    and the selected declaration/application binding through the public owner replay
    route. Out-of-domain, duplicate-zero, noncanonical, wrong-declaration and stale-identity
    witnesses are rejected by the responsible boundary; CG neither mints keys nor
    substitutes scalar application validity for the authored Boolean result. An unavailable
    replay capability is reported without claiming replay acceptance.
- id: FR-046-AC-10
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: Run evidence records the selected function and source association, inclusive
    component domains and correspondence, actual subject and full ordered backend
    arguments, every unwind/loop bound and option, Kani/Rust/CBMC/solver versions,
    target, actual dependency cohort and execution limits, raw checks/cover/playback,
    exit and typed terminal outcome. Replay uses the retained generated artifact without
    hand editing. No-run, compile refusal, cover failure, resource exhaustion and
    semantic falsification remain distinguishable; preparation alone earns no proof
    result.
- id: FR-046-statement-1
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'When an admitted request selects a pure Boolean function whose parameters
    have

    finite Rational component domains and whose complete body consists of those

    parameters, exact rational addition and rational equality, CG SHALL generate a

    bounded proof of that authored Boolean predicate over every value in its declared

    domain. This is the IR-258 whole-function slice, distinct from proving an

    individual scalar operation valid or comparing two implementations.'
- id: FR-046-statement-2
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'When the function is representable, CG SHALL draw canonical numerator
    and

    positive-denominator components independently for each argument from the exact

    inclusive domains in the admitted input. The first encoding represents finite

    component endpoints that fit signed `i64`; it does not change the mathematical

    Rational domain to a machine arithmetic domain.'
- id: FR-046-statement-3
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'When constructing each drawn argument, the harness SHALL call the public
    owner

    `Rational::new(Integer::from(n), Integer::from(d))` and admit that representation

    only when the returned Rational''s `numerator()` and `denominator()` equal the

    original drawn components. This constructor/accessor roundtrip is the canonical

    representation constraint: it excludes duplicate encodings, including zero with

    denominator other than one, without excluding any declared mathematical value.

    It introduces no kernel normalization API or copied GCD implementation.'
- id: FR-046-statement-4
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'When evaluating the body, the generated oracle SHALL use the actual owner''s

    exact operations and Meter with the original expression order, typed outcomes,

    charge ordering, stops and laziness. Input-domain restrictions do not constrain

    intermediate sums: for example, two permitted inputs equal to nine can sum to

    eighteen. Owner arbitrary-precision continuation remains intact.'
- id: FR-046-statement-5
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'When asserting the predicate, the harness SHALL require actual completion
    with

    Boolean true over every admitted argument tuple. A false, undefined, refused or

    incomplete body is never an assumed-away input or a successful predicate result.

    Representation constraints apply before body evaluation and do not depend on its

    outcome. Raw backend failure is not by itself evidence of a semantic false

    counterexample: native replay retains the actual typed outcome under FR-029.'
- id: FR-046-statement-6
  path: spec/kani/functional/FR-046-rational-function-predicate-harnesses.md
  role: examined
  excerpt: 'When the request selects a function, CG SHALL preserve that declaration
    occurrence

    through generation and playback. A successful operation application, a different

    function with an equal body, or a scalar child claim cannot replace this predicate.

    The owner-issued callable and replay identities remain the owner''s contract;

    CG does not mint an occurrence key or introduce an alternative replay API.'
- id: TC-060-step-1
  path: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  role: examined
  excerpt: "Preserve the original healthy and wrong-addend source cases from the\n\
    \   QSL-owned `rational_mvp_source` intake. Admit both unchanged through the public\n\
    \   source route and strict reader, retaining original source association and\n\
    \   owner-issued identities. The healthy case is:\n\n   ```text\n   language \"\
    ix:native\" edition \"1-draft\";\n   profile v = \"quire.value.complete/v1\";\n\
    \   function rational_commutative using v(a: Rational[-9, 9; 1, 9], b: Rational[-9,\
    \ 9; 1, 9]): Boolean pure {\n       a + b = b + a\n   }\n   ```\n\n   The only\
    \ source mutation replaces `a + b = b + a` with `a + b = a + a`.\n   Sele"
- id: TC-060-step-2
  path: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  role: examined
  excerpt: "Independently enumerate the semantic input domain using exact host rational\n\
    \   arithmetic and compare it with the constructor/accessor roundtrip encoding.\n\
    \   Check 111 unique canonical values per argument and all 12,321 tuples, including\n\
    \   zero `0/1`, `-9/1`, `9/1`, `1/9`, negative fractions and noninteger values.\n\
    \   Check soundness, injectivity and completeness rather than only these examples.\n\
    \   Reject duplicate encodings such as `0/2` and `2/2`; include mutations accepting\n\
    \   unrestricted normalization, dropping an endpoint or drawing only denominator\n\
    \   one. Ensure these changes fail the "
- id: TC-060-step-3
  path: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  role: examined
  excerpt: "Generate the actual oracle and compare its full native observations with\
    \ an\n   independently constructed reference using public owner operations on\
    \ the same\n   admitted function. Preserve all typed outcomes and ordered Meter\
    \ charges,\n   counters and stops at every occurrence-denial prefix and zero/exact/short\n\
    \   budgets. Include `a = b = 9` to exercise a sum outside the input domain and\n\
    \   owner continuation beyond a machine-sized intermediate in a separately\n \
    \  admitted finite-domain case. Exercise reachable undefined/refused/incomplete\n\
    \   outcomes without input filtering. Mark an unavailable"
- id: TC-060-step-4
  path: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  role: examined
  excerpt: "Run the healthy harness through the approved bounded production path with\
    \ its\n   actual complete ordered options and no stubbing or fixture edits. Verify\
    \ all\n   required checks and exactly one final completion cover. Run the wrong-addend\n\
    \   mutation over the same entire domain. Retain its actual failed predicate check\n\
    \   and solver playback; validate and replay its arguments through the owner route\n\
    \   for that exact declaration and assert completed Boolean false. For comparison,\n\
    \   `a = 1/2, b = 1/3` mathematically gives `5/6 != 1`, but this example is not\
    \ a\n   solver counterexample until actu"
- id: TC-060-step-5
  path: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  role: examined
  excerpt: "Check separate adversarial controls for a missing/unsatisfied cover, failed\n\
    \   unwind, missing verdict and execution-limit failure. Inspect the healthy and\n\
    \   mutated harnesses for one cover after assertions and no outcome-based assumes.\n\
    \   Preserve raw backend outcomes separately from semantic replay settlement;\n\
    \   noncompletion must not be relabelled Boolean false (AC-4, AC-6)."
- id: TC-060-step-6
  path: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  role: examined
  excerpt: "Request an otherwise valid unbounded Rational argument and assert named\n\
    \   `requires-bound` with no harness. Request an owner-admitted finite domain\
    \ with\n   an endpoint beyond signed-i64 capacity and assert named `unsupported`\
    \ with\n   the exact endpoint and no clipping. Exercise absent lowering, an unsupported\n\
    \   body operation and unavailable correspondence, alongside a supported sibling;\n\
    \   assert total per-item accounting and unchanged sibling output. An upstream\n\
    \   source/reader refusal is recorded at that boundary, not counted as the\n \
    \  corresponding CG refusal test (AC-7, AC-8)."
- id: TC-060-step-7
  path: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  role: examined
  excerpt: "Perturb a retained witness to be out of domain, noncanonical, duplicate\
    \ zero,\n   stale identity or bound to another declaration. Assert responsible-boundary\n\
    \   rejection without minted keys or changing the application into scalar parity.\n\
    \   Record absent public replay capabilities as open dependencies (AC-9)."
- id: TC-060-step-8
  path: spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md
  role: examined
  excerpt: "Inspect the run record for complete function/source association, input-domain\n\
    \   correspondence, actual tool versions, target, dependency cohort, limits,\n\
    \   harness selection and full ordered argv including unwind/loop options. Match\n\
    \   checks, single cover, actual playback, exit and terminal classification to\
    \ the\n   raw run. Verify retained source is replayed unchanged and that all no-run\
    \ or\n   refused cases report their actual disposition (AC-10).\n"
- id: FR-025-AC-7
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: context_only
  excerpt: An argument of a family with no row in the Rust-type table (rational, decimal,
    IEEE, text, enum, composite, collection or function) is accounted `unsupported`
    with a typed reason naming the family, and no harness is emitted.
- id: FR-196-AC-5
  path: /home/peter/dev/quire-specification/spec/functional/ir-proof/FR-196-generate-kani-contracts.md
  role: context_only
  excerpt: A finite encoding records its one-to-one semantic correspondence; absent
    correspondence emits unsupported and no Rust item.
```

## Dependency order

1. Owner source/admission and complete lowering; exact kernel Rational/Meter and bijective domain representation.
2. Coordinated Rational symbolic ABI and lossless witness decoding under FR-025/ADR-004.
3. Whole-function oracle and harness generation, with existing runner/terminal basis.
4. Owner replay and full generated-proof acceptance, including falsifying control.

No schedule, owner delivery date, public constructor or capability availability is invented.
