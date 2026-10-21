---
id: SR-15276
title: IR-17 tuple stage gap-analysis review
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-codegen; IR-17; spec/oracle/functional/FR-018-composite-equality-oracles.md;
  spec/oracle/matrix/TC-029-composite-equality-oracles.md
review_set: subset
---

## Summary

Independent reviewer-only PR499, exact head ebb26afd36236bda990224c48a2b69fe4ce80a00; base 6b1ba8b4e4906c2f2e5e2b398fd59e7f2fb6c7f8. Source-stage contract review; no product qualification.

## Verdict

FAIL: the full repository computed matrix retains 313 untagged/ignored-only criteria, plus three manually identified stage-evidence gaps despite existing tags. These remain explicitly open; this specification amendment does not implement them.

## Examined scope

scope:
- id: FR-005-AC-1
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Failed generation leaves no partial published bundle or edited developer
    region.
- id: FR-005-AC-2
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Supported platforms reproduce byte-identical files.
- id: FR-005-AC-3
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: For every shared-corpus case on which two backends' normalized outcomes
    differ, the TC-007 Differential Disposition Register holds exactly one row naming
    the ca
- id: FR-005-AC-4
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Executable, proptest, Kani, and coverage semantics agree or retain an explicit
    difference.
- id: FR-005-AC-5
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: A bundle naming more than 4096 artifacts, an artifact whose contents exceed
    16,777,216 bytes, or a complete artifact byte total exceeding 134,217,728 bytes
    is r
- id: FR-005-AC-6
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: 'When an artifact path is absolute, or contains a `..` segment, a `.` segment,
    an empty segment, a trailing separator, a backslash or a control character, or
    is '
- id: FR-005-AC-7
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When publication fails before the destination is replaced, or fails after
    replacement began and the prior destination is restored, the publisher reports
    destina
- id: FR-005-AC-8
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When removal of the replaced bundle's backup fails after the staged bundle
    has replaced the destination, the publisher reports destination state `published`,
    th
- id: FR-005-AC-9
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When replacement fails and restoring the prior destination also fails,
    the publisher reports destination state `unknown`, the destination path is absent,
    and th
- id: FR-005-AC-10
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When two artifacts of one bundle share the same path, bundle construction
    is refused with `DuplicateArtifactPath` naming the later `bundle.artifacts[i].path`,
    b
- id: FR-037-AC-1
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: Both `tests/exact_scalar_support/package.rs` and `tests/composite_equality_support/package.rs`
    derive every candidate application node identity through `derive_
- id: FR-037-AC-2
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: A fixture containing an application in a recursion group supplies the complete
    ordered draft graph to `ApplicationIdentityContext`; its candidate agrees with
    th
- id: FR-037-AC-3
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: 'A canonical enum-member fixture identity is derived through `NominalIdentityPreimage::EnumMember(...).digest`,
    agrees with the checked reader, and changes when '
- id: FR-037-AC-4
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: Positive exact-scalar and composite-equality packages remain admitted.
    A stale candidate key or a body mutation without rekeying is refused by the checked
    reade
- id: FR-037-AC-5
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: The consumer change leaves deterministic fixture-only keys distinct from
    application identities and uses `quire-canonical` for ordinary canonical-value
    hashes w
- id: FR-038-AC-1
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: A source inventory compares every identity in [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)'s
    specified derivable set with the implemented sc
- id: FR-038-AC-2
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The typed source distinguishes implemented integer, rational, decimal,
    IEEE, text, enum and quantity operations and their applicable result and mode
    forms. When
- id: FR-038-AC-3
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: Focused V2 oracle, scalar Kani and state/frame tests distinguish the wrong
    identity, family, operand arity or order, comparison symbol and applicable runtime
    ca
- id: FR-038-AC-4
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The four scalar Kani harnesses still compare their embedded oracle result
    with a separately rendered native `i128` arithmetic expression. A mutation of
    the prod
- id: FR-038-AC-5
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The V1 `strategy/bound` relation evaluator remains outside this V2 catalog
    migration until the V2 strategy replacement specified by [AD-004](../../assurance/AD-
- id: FR-038-AC-6
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: 'While the V1 `strategy/bound` relation evaluator remains live, focused
    tests discriminate all six comparison operators and a primary read on the left
    versus on '
- id: FR-039-AC-1
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Each of two genuine QSL-admitted source graphs with a different clause
    or pure-function operation lowers through IR to its own admitted FR-330 package.
    CG's pro
- id: FR-039-AC-2
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A mixed request with a representable clause or function and an independent
    `unsupported`, `requires_bound` or `invalid_input` sibling retains one typed decision
- id: FR-039-AC-3
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A selected dependency function reference resolves only through the reader-validated
    selected package identity and target key. A missing, wrong-identity or stale
- id: FR-039-AC-4
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: For a genuinely admitted FR-330 package, CG binds a `protocol`/`protocol_reference`
    carrier's exact `body.target` to the local `protocol` node, retaining its ty
- id: FR-039-AC-5
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A malformed or noncanonical FR-330 wire, wrong version, invalid accounting/source
    map, stale semantic identity or incomplete reader result reaches no CG consume
- id: FR-039-AC-6
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: 'An item whose admitted package accounting is `failed`, `invalid_body`
    or `body_incomplete`, or whose `lowered` form is unrepresentable by the selected
    backend, '
- id: FR-039-AC-7
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: 'Generated scalar, composite equality and exact function claims each serialize
    `contract_package_identity` equal to the admitted FR-330 `identity` and `node_id` '
- id: FR-039-AC-8
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: 'For each admitted `lowered` bound driver item, Contract IR''s authoritative
    traversal is total over the required root, bounds and selected dependencies, and
    the '
- id: FR-039-AC-9
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: In an admitted package with a bound driver item, an excluded accounting-only
    position and another bound driver item, CG matches all three positions and exact
    `n
- id: FR-039-AC-10
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Given a bound driver item followed by an excluded accounting-only position,
    a selected conversion context root outside the preceding item's contiguous span
    yiel
- id: NFR-001-AC-1
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Identical inputs and configuration produce byte-identical bundles.
- id: NFR-001-AC-2
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: An injected generation failure leaves no partially published bundle.
- id: NFR-001-AC-3
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Generation does not modify developer-owned regions or files.
- id: NFR-001-M-1
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Byte differences between repeated generations
- id: NFR-001-M-2
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Partial published bundles after injected failure
- id: NFR-001-M-3
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Developer-owned files modified
- id: NFR-002-AC-1
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Every generated Rust file in a family named in Scope has a header naming
    `quire-contract-codegen`, every input-carried subject identity it renders as defined
    in
- id: NFR-002-AC-2
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Every generated Rust file carries the `MIT OR Apache-2.0` SPDX identity.
- id: NFR-002-AC-3
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Unsupported and inconclusive states remain explicit rather than silently
    succeeding.
- id: NFR-002-AC-4
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Generated output makes no project-specific validation, accreditation, certification,
    or release-approval claim.
- id: NFR-002-M-1
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Generated Rust files missing generator or input-carried subject identity
- id: NFR-002-M-2
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Generated files missing dual-license SPDX header
- id: NFR-002-M-3
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Silent unsupported/inconclusive states
- id: NFR-005-AC-1
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Every `.rs` file under `src/`, except a file its parent module declares
    under `#[cfg(test)]`, contains zero panic tokens in its non-test code with comments,
    eve
- id: NFR-005-AC-2
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`lower_scalar_claim` returns `Err(ScalarLoweringRefusal::NoRenderer)`
    for a claim whose `checked_bounds` is empty and for a claim whose first checked
    bound has '
- id: NFR-005-AC-3
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`ItemSettlement::warning` returns `None` for a `Disposition::Unsupported`
    carrying each of `Cause::AbsentKind`, `UnknownKind`, `AbsentExtent`, `UnknownBackend`,'
- id: NFR-005-AC-4
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: With a coverage export supplied, `observe_clause` given a map whose oracle-evaluation
    region has no probe returns a row with a `MapMismatch` diagnostic with the
- id: NFR-005-AC-5
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: 'The step of `generate_kani` that pairs FR-015''s records with the Kani
    group, given hand-built inputs in a `#[cfg(test)]` module, returns `RoutedGenerationError:'
- id: NFR-005-AC-6
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`rewrite_duplicate_position`, a function of `src/routed/generate.rs` that
    `route_records` (which `generate_kani` calls to pair, rewrite and join FR-015''s
    record'
- id: NFR-005-AC-7
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`observe_clause`, given a map of zero regions and a map of one region,
    each with and without a coverage export, returns a row with a `MapMismatch` diagnostic
    wi'
- id: NFR-005-AC-8
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: The scan locates the body of `generate_kani`, `route_records`, `rewrite_duplicate_position`,
    `observe_clause` and `generate_boolean_oracle_inner` in the literal
- id: NFR-005-M-1
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Panic tokens in the non-test, literal-free code of `src/`
- id: NFR-005-M-2
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Measured sites a test can reach that abort instead of returning a typed
    value
- id: NFR-005-M-3
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Index, slice and unchecked subtraction expressions in the bodies of the
    five IR-577 functions
- id: FR-004-AC-1
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: An evaluated implication whose consequent is unobserved yields a vacuity
    finding even when every oracle return was true; the consuming coverage obligation
    denie
- id: FR-004-AC-2
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: A clause whose oracle-evaluation region was not observed is unexecuted,
    not vacuous.
- id: FR-004-AC-3
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: The four measured clause classifications form a total partition; an implication-bearing
    clause is exercised only when every expected consequent entry probe is o
- id: FR-004-AC-4
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Successful analysis verifies the bound population, source/map, and native
    execution bindings.
- id: FR-004-AC-5
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Malformed, summary-only, identity-mismatched, path-ambiguous, unsupported,
    or missing observation inputs retain structured non-success outcomes without invented
- id: FR-004-AC-6
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Campaign counts and test outcome remain complete facts independent of coverage
    classification.
- id: FR-004-AC-7
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Removing an expected clause, consequent, evaluation probe, or all clauses
    prevents successful analysis; the expected census comes from bound typed IR.
- id: FR-004-AC-8
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Adverse coverage and non-success native execution cannot discharge the
    coverage obligation merely because a report serialized successfully.
- id: FR-004-AC-9
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Complete bound observations without a native campaign run are reported
    as observations only and never as a passed coverage result. Valid informational-only
    popu
- id: FR-004-AC-10
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. CG binds coverage inside the same live campaign-runner invocation,
    using the original `/2` wire bytes and their QSL strict-reader typed result directly
- id: FR-004-AC-11
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. The consuming report retains vacuity and native outcome as evidence
    for IR FR-045; an oracle-success run with an unobserved consequent denies the
    cover
- id: FR-004-AC-12
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED. CG consumes QSL''s typed strict-reader clause-run or command-error
    variant for exact `native-run-result/2`: a decisive basis retains its witness
    with as'
- id: FR-004-AC-13
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. QSL `run_clause` success and a decisive witness remain source-clause
    facts and do not count as generated-campaign execution or LLVM coverage; QSL viola
- id: FR-004-AC-14
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED. Only CG''s campaign runner constructs the private, non-cloneable,
    non-serializable live run context; callers cannot construct or restore it from
    a `/2` '
- id: FR-004-AC-15
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. A controlled attempt to replace the strict-reader typed result,
    execution record, generated source, source map, LLVM export stream or LLVM tool/version
- id: FR-004-AC-16
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). The expected clause population and implication counts
    of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2
- id: FR-004-AC-17
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 analyzer opens a `CheckedPackageV2`
    body only through the `core/ir/` layer of AD-004, and names no `BoundPackag
- id: FR-004-AC-18
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). A V2 clause whose generated source map declares an implication
    count unequal to the count derived from its body yields a structured binding no
- id: FR-004-AC-19
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, a clause whose oracle evaluation
    was observed, that has at least one implication and for which no expected conseq
- id: FR-004-AC-20
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, a mapped clause whose oracle-evaluation
    region has no positive execution count is `unexecuted`.
- id: FR-004-AC-21
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause
    for which some but not all expected consequent probes were observed is `partially_e
- id: FR-004-AC-22
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED (IR-815). Over a V2 clause population, an implication-bearing
    clause with every expected consequent probe observed, and an implication-free
    clause with '
- id: FR-004-AC-23
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, removing one clause, one
    consequent probe, one evaluation probe, or every clause from the generated artifacts,
    ea
- id: FR-004-AC-24
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, each malformed input class
    of FR-004-AC-5 yields a non-success outcome with no invented classification.
- id: FR-036-AC-1
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: A genuine typed fixture with criteria C1/C2, obligations O1/O2/O3, source
    identities S1/S2, domain D and two production functions in one module exports
    three ob
- id: FR-036-AC-2
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: For the unchanged AC-1 fixture, omitting a result, reporting unsupported
    or pre-run refusal, retrying and changing method/profile/subject/artifact/run
    leave the
- id: FR-036-AC-3
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Capability facts preserve unsupported capability and reason without artifact/run/outcome;
    pre-run refusal facts preserve producer/stage/cause separately from ac
- id: FR-036-AC-4
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: A matching executed production fact retains its actual typed artifact and
    run identities separately from attempt identity, exact source/profile/subject/domain/b
- id: FR-036-AC-5
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Shadow-only evidence, a narrower effective bound and separately executed
    sampled native refinement retain distinct subject/scope/assumption observations.
    Missin
- id: FR-036-AC-6
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: A downstream consumer above CG and IR borrows the public census/fact types
    over the reviewed neutral correspondence contract without source/transcript/name
    pars
- id: FR-036-AC-7
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Missing/incomplete authoritative inputs, a relation targeting a foreign
    selection and a dangling relation endpoint produce distinct typed census refusals
    with t
- id: FR-015-AC-1
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Pre, post, invariant and frame obligations of one contract produce separate
    harnesses with distinct identities.
- id: FR-015-AC-2
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every harness identity records its model-domain bounds, taken only from
    IR `bounded_domain` nodes, and its solver and options and unwind bound.
- id: FR-015-AC-3
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An unbounded or non-finite obligation is refused with a typed reason and
    no harness.
- id: FR-015-AC-4
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: No harness assumption excludes an undefined, refused or incomplete runtime
    outcome.
- id: FR-015-AC-5
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An obligation whose bounds are unsatisfiable is refused with a typed reason
    and no harness.
- id: FR-015-AC-6
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An obligation over an oracle whose operation identity is `caller_declared`
    is refused with a typed reason and no harness.
- id: FR-015-AC-7
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every generated harness contains exactly one non-vacuity cover; a precondition
    harness covers that the precondition holds within the IR bounds, and a contract
    h
- id: FR-015-AC-8
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A postcondition or invariant harness emits every package precondition sharing
    its anchor operation as a `requires` on its generated contract and records each
    in
- id: FR-015-AC-9
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: "Every harness identity records solver `cadical` and the complete ordered\
    \ option vector \u2014 function contracts, concrete playback, the exact fully\
    \ qualified harnes"
- id: FR-015-AC-10
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Regeneration from equal inputs is byte-identical, and changing the unwind
    bound or the customer subject changes the harness identity.
- id: FR-015-AC-11
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every symbolic argument carries an inclusive assumption equal to its IR
    `bounded_domain` or a literal operand's own value (FR-015-AC-16); for a V2 scalar
    claim,
- id: FR-015-AC-12
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request naming no items, more than 256 items, an unparsable subject path,
    or an unwind bound outside `1..=1024` is refused whole, with no item accounted
    and n
- id: FR-015-AC-13
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An otherwise-supported obligation whose generated harness source exceeds
    the bounded-resource ceiling is refused with a distinct resource-limit reason
    naming th
- id: FR-015-AC-14
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar-claim item whose graph node carries a tag/form pair this generator
    does not model as a contract role is refused with a typed reason naming that tag
    and
- id: FR-015-AC-15
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar-claim item whose claim is a `NoDerivableClaim` refusal yields
    `unsupported` with the `no_derivable_claim` reason naming the node and the derivation
    ref
- id: FR-015-AC-16
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar harness constrains a literal operand, inline or a reference to
    a `value` node whose body is a literal, to exactly its own value: `x + 1` over
    `x: Int[0'
- id: FR-015-AC-17
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar claim whose literal operand does not fit `i64` (`x + 10^23`) is
    `unsupported` as `domain_not_representable_in_i64` naming the literal as both
    endpoints
- id: FR-015-AC-18
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar claim whose operation over its operand ranges yields no result
    inside the result range (`x + 100` over `x: Int[0, 9]` into `Int[0, 10]`, reachable
    `[10'
- id: FR-015-AC-19
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A zero-input transition over one bounded-integer state value verifies for
    an identity subject and is falsified with a concrete counterexample for a subject
    that
- id: FR-015-AC-20
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A harness for a Boolean-connective or bounded-integer comparison claim
    (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons) embeds
    its FR
- id: FR-015-AC-21
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A precondition that reads a post-state value is refused with a typed reason
    naming the node, and no harness is emitted.
- id: FR-015-AC-22
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A declared proof-dependency census with an empty or duplicate identity,
    an inconsistent kind, state and path combination, or a non-`Required` kind is
    refused as
- id: FR-015-AC-23
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every requested item receives exactly one `ObligationDisposition` (`supported`,
    `requires-bound`, `unsupported` or `invalid-request`), and an item that is not
    `
- id: FR-015-AC-24
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A falsifying in-domain assignment of a plain bounded-integer comparison
    claim is reported through Kani's concrete playback.
- id: FR-015-AC-25
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A valid declared census is folded into the harness identity, readiness
    is `ready` only when every dependency passed and `incomplete` while any is missing
    or fai
- id: FR-015-AC-26
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: One `postcondition` `state_clause` of an admitted package, whose condition
    is one integer comparison of the pre and post reads of one field through its first
    pa
- id: FR-015-AC-27
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The operation-contract harness assumes each ranged state field within the
    inclusive `i64` endpoints returned by `model_object_fields` for the QSL-emitted
    select
- id: FR-015-AC-28
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-effect harness takes the granted fields from the frame node's
    `modifies` entries and asserts every other state field the caller names unchanged.
- id: FR-015-AC-29
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A state clause that is not a postcondition, whose condition is not one
    comparison of the pre and post reads of one field through `self` (a literal operand,
    a ne
- id: FR-015-AC-30
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, the operation contract verifies for a healthy
    subject and is falsified, naming the postcondition, for a subject mutated to debit.
- id: FR-015-AC-31
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, a frame-allowed effect verifies, a frame-forbidden
    effect is falsified naming the forbidden field, and regenerating the frame from
    a
- id: FR-015-AC-32
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The forbidden frame counterexample, executed natively, reproduces as a
    frame violation of that field through QSL's `replay_frame`, and the allowed run
    replays a
- id: FR-015-AC-33
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-replay payload's anchor, frame and frame occurrence are the values
    `qsl_replay::call_site` names for the operation's name.
- id: FR-015-AC-34
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-replay envelope's `clause_node` is the payload's frame node and
    its `occurrence_key` is the payload's frame occurrence.
- id: FR-015-AC-35
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An operation the unit names no frame for, whether the domain package declares
    it or not, is refused by the call site when the frame-replay request is built.
- id: FR-015-AC-36
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`FrameReplay::replay` returns QSL''s `replay_frame` result: a forbidden
    write settles a reproduced violation naming the written field, and a write the
    frame gran'
- id: FR-015-AC-37
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar function-application harness asserts the oracle''s outcome against
    the clause''s operation evaluated natively in `i128` over the same symbolic operands, '
- id: FR-015-AC-38
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 clause claim naming a `state`/`state_clause` node of an admitted `CheckedPackageV2`
    whose `operation.member.clause` is `precondition`, `postcondition` or `
- id: FR-015-AC-39
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A V2 clause harness draws one nondeterministic value per drawn input:
    the pre-state of each state field of `self` the body reads, whether bare or under
    `pre(...'
- id: FR-015-AC-40
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A harness for a V2 clause whose condition, inline or referenced from an
    admitted Boolean expression node, contains Boolean connectives and bounded-integer
    compa
- id: FR-015-AC-41
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 clause whose body holds integer arithmetic (add, subtract, multiply,
    negate) as an operand of a comparison lowers, and its harness keeps the independent
    na
- id: FR-015-AC-42
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 clause body holding an operator the V2 arm does not support (division,
    modulo, absolute value, and every operator outside connectives, the six integer
    comp
- id: FR-015-AC-43
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 postcondition harness emits every `precondition` clause node sharing
    its anchor node as a `requires` and records each in its identity's embedded oracles,
    e
- id: FR-015-AC-44
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A census declared on a V2 clause claim with an empty or duplicate dependency
    identity, an inconsistent kind, state and path combination, or a non-`Required`
    kin
- id: FR-015-AC-45
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A valid census declared on a V2 clause claim is folded into that harness's
    identity record, readiness is `ready` only when every dependency passed and `incomple
- id: FR-015-AC-46
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every V2 clause harness contains exactly one non-vacuity cover, placed
    after every assumption, after the subject call and after every assertion the harness
    carr
- id: FR-015-AC-47
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Regeneration of a V2 clause harness from equal inputs is byte-identical,
    and changing the unwind bound or the customer subject changes the harness identity
    reco
- id: FR-015-AC-48
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The obligation identity of a V2 clause claim is formed by AD-003 E-1 from
    the clause node id, its occurrence key, the obligation kind, and the arguments
    each as
- id: FR-015-AC-49
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 invariant claim yields a harness that draws its state and parameters
    within their declared domains (FR-015-AC-39) and asserts the invariant clause;
    its sub
- id: FR-015-AC-50
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`negotiate` reports an obligation whose scalar refusal is `ExactScalarRefusal::LoweringByteLimitExceeded`
    or `LoweringLimitUnrecognised` as `Outcome::Unsupporte'
- id: FR-015-AC-51
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, a finite input validated under a profile selection '
- id: FR-015-AC-52
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, the context of every outcome and refusal that
    path '
- id: FR-015-AC-53
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Each of the seven harness kinds the generator emits (precondition, V1 contract,
    scalar, state-clause, frame-effect, V1 bundle and corpus) is subject to the cove
- id: FR-015-AC-54
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V1 bundle harness (`generate_kani_bundle`, the `proof_for_contract` harness
    it emits for a requires clause, an ensures clause and the bundle's bounded argumen
- id: FR-015-AC-55
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: While a family's corpus arm remains (deleted by FR-015-AC-109 for arithmetic,
    FR-015-AC-110 for collection, FR-015-AC-111 for graph), a bounded-corpus harness
    o
- id: FR-015-AC-56
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, a V1 bundle harness whose requires clause some
    bounded argument satisfies and whose `ensures` holds for every such argument classifi
- id: FR-015-AC-57
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: While its arm remains (FR-015-AC-109 to FR-015-AC-111), with the installed
    backend, a bounded-corpus harness of each of the arithmetic, graph and collection
    fam
- id: FR-015-AC-58
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A test generates a harness through every emitting entry point (the precondition,
    V1 contract, scalar, state-clause and frame-effect families, `generate_kani_bun
- id: FR-015-AC-59
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request of `StateFrame` items in which the first item is refused and
    later items are supported, requires-bound, unsupported or invalid yields exactly
    one `Obl
- id: FR-015-AC-60
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item that yields is recorded `supported` carrying its harness
    symbol, and its harness, returned in the `state_frame_harnesses` field of `KaniObli
- id: FR-015-AC-61
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A `StateFrame` item whose lowering is a requires-bound record is recorded
    `requires_bound` with that record''s `unbounded_type` and no harness. A selected
    model '
- id: FR-015-AC-62
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item whose lowering is an unsupported-family record is recorded
    `unsupported` with reason `NoFiniteEncoding` naming the record's node and family,
- id: FR-015-AC-63
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item whose condition is a negation, compares to a literal,
    uses an operator other than the six integer comparisons, reads through a parameter
    oth
- id: FR-015-AC-64
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A `StateFrame` item with an unparsable state path or an unparsable item
    subject path, an invalid or lacking state field, an absent node, a repeat of an
    earlier '
- id: FR-015-AC-65
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request holding `StateFrame` items and no items, more than `MAX_OBLIGATION_ITEMS`,
    an unparsable request subject path or an unwind bound outside `1..=MAX_OBLI
- id: FR-015-AC-66
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A test builds every retained `StateFrameRefusal` variant (including `ModelFieldsUnavailable`,
    `MemberAbsent`, `ModelMemberNotI64Range` with each cause, and each
- id: FR-015-AC-67
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`generate_state_frame_obligations` refuses with `MalformedClause` at the
    node, not `InvalidField` or `ConditionNotSupported`, for a clause whose graph
    field nam'
- id: FR-015-AC-68
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` request whose frame grants every state field records its
    `frame` item `unsupported` (`StateFrameRefused`) and its `contract` item `supported`
    wit
- id: FR-015-AC-69
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A composite-equality claim over operands that are parameters or literals
    of one record, tuple or option type whose leaves are Boolean or bounded integers,
    with '
- id: FR-015-AC-70
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'The harness draws one nondeterministic value per leaf of each parameter
    operand and no other value: a `bool` per Boolean leaf, an `i64` per bounded-integer
    leaf'
- id: FR-015-AC-71
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Over every drawn pair the harness asserts that the shadow's verdict equals
    the expectation's and that the shadow's pair count equals the expectation's, where
    th
- id: FR-015-AC-72
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The emitted shadow and harness, parsed, contain no loop (`while`, `loop`,
    `for`), no recursive call, no `Box`, `Vec`, `Rc`, `String` or map, and no integer
    type
- id: FR-015-AC-73
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Each row of the unsupported-shape table above is produced by an item reaching
    that shape and yields its disposition and reason with no harness: an unbounded
    int'
- id: FR-015-AC-74
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, the harness of the corpus case (two record-typed
    and two tuple-typed values compared with `=` and `!=`, a record with an optional
    fi
- id: FR-015-AC-75
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'With the installed backend each seeded mutant of the shadow is `Falsified`
    with a concrete playback: an `absent` slot read equal to a `null` slot; one field
    of '
- id: FR-015-AC-76
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The identity of a composite-equality harness records family `composite_equality`,
    proof subject `bounded_shadow`, the closure's declaration keys each with its l
- id: FR-015-AC-77
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. For the QSL-emitted twin package, `field_range` calls `model_object_f
- id: FR-015-AC-78
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG implementation pending. `field_range`
    returns `MemberAbsent` for a requested name absent from the selected model de
- id: FR-015-AC-79
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. From the package QSL emits for the twin's unit, both roles of each cl
- id: FR-015-AC-80
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. With the installed backend, the cases of FR-015-AC-30 and FR-015-AC-3
- id: FR-015-AC-81
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A present listed model field with `member_type()` of `None`, a non-`I
- id: FR-015-AC-82
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, a request for
    distinct named constructs returns one `CapabilityEntry` per requested construct
    in req
- id: FR-015-AC-83
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, an empty, duplicate
    or absent construct is a typed `KaniOutcome` refusal rather than a partial dispo
- id: FR-015-AC-84
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, a profile/dispatch-admitted add, subtract, multiply, divide or remainder
- id: FR-015-AC-85
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, an inverted result range is `InvalidInput`; zero division, checked overf
- id: FR-015-AC-86
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer,
    a profile/dispatch-admitted request follows only the selected reference field,
    finds a p
- id: FR-015-AC-87
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer,
    a zero expansion bound or unknown endpoint is `InvalidInput`, and exhausting the
    bound i
- id: FR-015-AC-88
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection
    lowerer, a profile/dispatch-admitted `ForAllNonNegative` or `ExistsEqual` query
    retains
- id: FR-015-AC-89
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Until the collection slice (FR-015-AC-110) deletes the finite-collection
    lowerer, an input longer than `max_items` is `ResourceExhausted`, with no query
    result '
- id: FR-015-AC-90
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), one admitted arithmetic,
    graph or
- id: FR-015-AC-91
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, that path returns a generation-time case classifica
- id: FR-015-AC-92
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), a declared dependency
    census with
- id: FR-015-AC-93
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, a profile/input mismatch, family-lowering
    refusal, '
- id: FR-015-AC-94
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), equal case inputs produce
    equal a
- id: FR-015-AC-95
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: AD-004 step 4g retires the hand-built bounded-corpus path in the three
    per-family slices FR-015-AC-109 (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-11
- id: FR-015-AC-96
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'An admitted QSL V2 clause/function body that contains a `quire.op.collection.forall`
    or `quire.op.collection.exists` operation over a selected model collection '
- id: FR-015-AC-97
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'For an admitted collection claim whose selected field has multiplicity
    lower bound `0` and no finite upper bound, QSL admits an unbounded collection
    type; when '
- id: FR-015-AC-98
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim (one reachability application of either form above, at its
    `state_clause` node id and argument path) generates a harness when the admitted
    package
- id: FR-015-AC-99
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The collection harness asserts the admitted clause/function body over symbolic
    inputs drawn from the selected model field's typed domain, ends with the FR-015-A
- id: FR-015-AC-100
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For the collection family, changing the selected field's authored cardinality
    or element bound, admitted body claim, customer subject or unwind changes the
    harn
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request whose symbolic collection domain exceeds the generator's representational
    or resource ceiling settles `unsupported` with a typed reason naming the fai
- id: FR-015-AC-102
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An admitted collection body claim is settled through the FR-015 V2 obligation
    path, one disposition per requested item; supported items alone emit harnesses.
    Th
- id: FR-015-AC-103
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Through `generate_routed`, `(x + 1) + 1` over `x: Int[0, 9]` into `Int[0,
    20]`, `x * -1` over `x: Int[1, 9]` into `Int[-9, -1]`, and `-(x + 1)` over `x:
    Int[0, '
- id: FR-015-AC-104
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A plain-Integer parameter as any nested leaf keeps `requires_bound` with
    the Integer `unbounded_type` and no harness; a direct reference to a raw application
    ne
- id: FR-015-AC-105
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For a graph claim the harness draws an object count in `[0, N]` of the
    one covering population and symbolic objects within it; for every object, the
    selected ed
- id: FR-015-AC-106
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The independent native oracle replays the same authored clause over each
    original-domain concrete graph through QSL's replay (`qsl_replay`), and does not
    substi
- id: FR-015-AC-107
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Graph claims are located by `state_clause` node id and argument path:
    one reachability written twice in one condition yields two items at positions
    0 and 1; one'
- id: FR-015-AC-108
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim settles by the disposition sequence in the Description (the
    paragraph "Disposition sequence for a graph claim"), with no harness, domain reduction
- id: FR-015-AC-109
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-598 arithmetic slice only; merged IR-858 satisfies AD-004
    step 4b). After this slice no source under `src/` contains the corpus `Arithmetic`
    req
- id: FR-015-AC-110
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and
    on AD-004 step 4h, because the collection family renders through `HarnessSpec`
    and `rende
- id: FR-015-AC-111
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 graph and final slice; gated on IR-858, AD-004 step 4b,
    on FR-015-AC-109 and FR-015-AC-110 having landed, and on IR-805, whose graph claims
    need
- id: FR-015-AC-112
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858, AD-004 step 4b). `HarnessSpec::new` is the only constructor
    and refuses with a typed `HarnessSpecError`, never a panic and never a harness:'
- id: FR-015-AC-113
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). For every spec that constructs, `render` emits the
    header, the oracle source, the module wrapper and the proof attribute, then the
    spec''s '
- id: FR-015-AC-114
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). `render` of one spec returns equal bytes on repeated
    calls and in a second process, and a spec whose `Input` list is reordered renders
    the
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). After the scalar port, for each claim of a fixed
    set (each of `quire.op.integer.add`, `sub`, `mul` and `negate`; a claim with a
    literal op
- id: FR-015-AC-116
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). `render` holds no family-specific template text:
    for hand-built specs whose label, expressions and messages are not the scalar
    family''s, t'
- id: FR-015-AC-117
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). With the installed backend, after the scalar port,
    the unmutated scalar harness classifies `Verified` with its cover satisfied and
    the exi
- id: FR-015-AC-118
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For an admitted V2 scalar `implies` claim over two Boolean operands, the
    generated harness embeds its FR-014 oracle as FR-015-AC-20 requires and draws
    both oper
- id: FR-015-AC-119
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-917; depends on FR-015-AC-40 and FR-015-AC-41). QSL source
    authors an operation with one parameter `amount: Int[0, 1000]` and its V2 postcondition
    `'
- id: FR-015-AC-120
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-942; depends on QSL ADR-013 O-07/O-09 and QSV FR-123). After
    the selected owner and occurrence validate under AC-122, CG forms the per-step
    record c
- id: FR-015-AC-121
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-942). A per-step `KaniObligationIdentity` uses the QSV-owned
    `StepSimulationRecord` as a typed subject, carries its content identity and the
    owning '
- id: FR-015-AC-122
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-942). A selected proof-eligible refinement under QSpec FR-322-AC-88
    through AC-90 with an `Initial` record and `Step { row: i }` records for its adm'
- id: FR-015-AC-123
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: External public-consumer calls to `prepare_bounded_collection_query`, `prepare_finite_graph_reaches`
    and `generate_bounded_kani_corpus_case` have the direct res
- id: FR-015-AC-124
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: After infallible-constructor adoption, each retained family/corpus adverse
    control returns the original exact non-success kind, typed cause, source identity
    and
- id: FR-015-CON-1
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Clause ABI construction SHALL check both endpoints of a bounded integer
    dependency before conversion to i64 and refuse one outside i64 as `UnsupportedDependency
- id: FR-015-CON-2
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Subject binding SHALL check both endpoints of a bounded integer dependency
    before conversion to i64 and refuse one outside i64 as `UnsupportedBinding`, without '
- id: FR-017-AC-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: An absent launcher is refused with a typed reason naming its path before
    anything runs.
- id: FR-017-AC-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run is verified only when the process exited successfully, the report
    states success, and every cover property is reported satisfied; a successful run
    with an
- id: FR-017-AC-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A failed non-unwinding check with a concrete playback is falsified carrying
    that playback verbatim and never the playback of a satisfied cover; a failure
    with n
- id: FR-017-AC-6
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The execution evidence of a single run carries the obligation kind, the
    harness path, the launcher path, the complete argument vector, the unwind bound,
    the sol
- id: FR-017-AC-7
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A crate whose library source does not contain the harness source byte for
    byte is refused, and no backend runs.
- id: FR-017-AC-11
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A routed FR-022/FR-014 exact-scalar harness (`KaniScalarObligationHarness`)
    runs through `execute_kani_obligation` and `kani_launch_command` the same way
    an FR-
- id: FR-017-AC-12
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Real Kani captures of a verified run, a falsified run with a playback,
    an exhausted unwind bound, a run whose only check is unreachable, a partly satisfied
    cove
- id: FR-017-AC-13
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run whose process exited successfully and whose backend reported success
    with zero successful checks is inconclusive with the vacuous-proof reason, never
    veri
- id: FR-017-AC-14
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A launcher whose stdout or stderr carries more than 8 MiB times its harness
    count is stopped, its process group is killed, and the run is refused with `kani_out
- id: FR-017-AC-15
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A timeout too large to add to the current instant never elapses and does
    not panic.
- id: FR-017-AC-16
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The launcher''s capture threads are stopped and joined on every outcome:
    they return what the launcher wrote before it ended, stop while a write end is
    still hel'
- id: FR-017-AC-17
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run that times out has the whole group the launcher leads killed, a real
    grandchild included.
- id: FR-017-AC-18
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A report that is malformed, of an unknown check or harness status, of another
    schema version, that holds other than one harness result in a single run, or whose
- id: FR-017-AC-19
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The generator appends `-Z unstable-options --export-json /proc/self/fd/N`
    after unchanged harness options, with actual child-only anonymous pipe writer
    N >= 5, '
- id: FR-017-AC-20
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The evidence and the classified run list every check of a real run with
    its id, class, source file and line and status, a line stated as unknown is absent,
    and '
- id: FR-017-AC-21
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: N harnesses with equal option vectors (the harness selection removed) and
    equal identity `ProofCeilings` (`wall_clock` T and `memory_bytes` M) start exactly
    one
- id: FR-017-AC-22
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over a real or captured batch report, members are matched by `module::harness`
    path (two members with the bare symbol `check` stay distinct), and each member''s '
- id: FR-017-AC-23
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A group's report that lacks a requested harness, holds one twice or holds
    an unrequested one refuses the group with a typed cause and classifies no member;
    a me
- id: FR-017-AC-24
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A launcher that exits on its own after leaving a real grandchild in its
    process group has that grandchild killed by the time the run returns, the same
    as on a t
- id: FR-017-AC-25
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A capture thread that panics, or whose poll or read of the stream errs,
    refuses the run with `kani_output_unread`; the run is never classified from empty
    text.
- id: FR-017-AC-26
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A generated FR-035 caller Text-admission harness enters the same production
    execution path in single and compatible batch runs; its evidence carries the exact
    t
- id: FR-017-AC-27
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A genuine routed `x + 1` scalar harness from `generate_routed`, its complete
    emitted oracle package and one in-range concrete operand are passed to the paired
    r
- id: FR-017-AC-28
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Before either child starts, a package or harness whose path or bytes differ
    from the retained routed output, a selected oracle symbol or argument order that
    dif
- id: FR-017-AC-29
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The runner measures one canonical proof-content identity over the exact
    paths and bytes of every staged execution input: the complete generated package,
    retaine'
- id: FR-017-AC-30
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The public receipt exposes the measured identity, scalar identity, ordered
    operands, exact supplied and meter-used `ScalarLimits`, verified Kani evidence
    and ac
- id: FR-017-AC-31
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The paired runner executes an order-sensitive routed scalar operation with
    two symbolic operands in two concrete operand orders. Each real Kani run is verified;
- id: FR-017-AC-32
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: In the `make kani` lane, a crate whose library source holds the generated
    harness byte for byte and, beside it, an item that does not compile (a reference
    to an
- id: FR-017-AC-33
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over single-run launcher stand-ins that exit 1: a report of success beside
    the unsuccessful exit is `unknown` even when its stdout holds `Solving with CaDiCaL
    3'
- id: FR-017-AC-34
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A redacted stream of exactly 8192 bytes is kept whole with `omittedBytes`
    0; one of 8193 bytes whose final line starts within the window keeps the longest
    line-
- id: FR-017-AC-35
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'With crate directory `/h/u/ws/crate`, target directory `/h/u/ws/crate/target`
    and home directory `/h/u` supplied in the request, a stand-in whose streams print '
- id: FR-017-AC-36
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The diagnostic record never feeds classification: two runs with the same
    report and exit and different stream text classify to the same outcome, checks,
    SUCCESS'
- id: FR-017-AC-37
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over single-run launcher stand-ins that exit 1, print `Checking harness
    proofs::check...` and `CBMC failed with status 64` on stdout and `Unknown option:
    --no-s'
- id: FR-017-AC-38
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A single-run stand-in that exits 1 and exports a `Failure` report listing
    a failed assertion check with no playback on the console, whose entry for the
    harness '
- id: FR-017-AC-39
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A batch stand-in of members `a::check`, `b::check`, `c::check`, `d::check`
    and `e::check` whose process exits 1 and whose report has `a::check` `Success`
    with a
- id: FR-017-AC-40
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The cause never feeds classification: three single-run stand-ins with
    the same `Failure` report, no checks and the same exit, whose entries give `exit_code_64`,'
- id: FR-017-AC-41
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Every `KaniScalarNativeError` variant has a `code()` that is a total `&''static
    str` from exactly this set: `invalid_request`, `artifact_package`, `snapshot_io`,'
- id: FR-017-AC-42
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '`class()` is `CompletionRefusal` for `invalid_request`, `artifact_package`
    and `oracle_stop`, and `EnvironmentFault` for `snapshot_io`, `native_launch`,
    `depend'
- id: FR-017-AC-43
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A canonical-identity encoding failure is `IdentityEncode` with code `identity_encode`
    and class `EnvironmentFault`; no path returns `NativeResult` for it. `Nati
- id: FR-017-AC-44
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The code table and the mapping to class are frozen: a test lists every
    variant with its expected code and its mapping to class (one class for a fixed
    variant; f'
- id: FR-017-AC-45
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The receipt exposes its proof-content identity as a `DigestRecord`, the
    `qsl_replay::DigestRecord` re-exported at the crate root (no CG-defined twin type;
    a com
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'For a genuine routed `x + 1` package the v3 preimage has exactly ten artifacts,
    in UTF-8 byte order: `Cargo.lock`, `Cargo.toml`, `claim-map.json`, the only admi'
- id: FR-017-AC-47
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Two receipts for the same package and harness with different operands,
    different `ScalarLimits` and different native outcomes have equal `DigestRecord`s;
    operan
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The receipt exposes read-only the ordered staged `(path, bytes)` entries
    the digest was built from, including both manifests, both independently generated
    locks
- id: FR-017-AC-49
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A change to the preimage schema (artifact set, path names, ordering, encoding,
    canonicalization or hash) is made only by changing the schema string, with the
    go
- id: FR-017-AC-50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits`
    the native meter was constructed with. They are read back from the native invocat
- id: FR-017-AC-51
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A consumer compile test names and constructs `quire_contract_codegen::ScalarLimits`
    using only codegen crate paths, a typed accessor to `KaniScalarNativeRequest
- id: FR-017-AC-52
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Production emitted bytes and feature-free exact dependency remain unchanged.
    The isolated private observer uses resolver 2 and its sole example compiles the
    sin
- id: FR-017-AC-53
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Both package locks are independently freshly resolved with the existing
    bounded offline generation/update workflow, without copying a parent/other lock
    or subst
- id: FR-017-AC-54
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A successful claimed-init confirmation poll with no `IN` yields public
    `MemoryObservationFailed` with the existing outer and nested prefixes and labelled
    observ
- id: FR-017-AC-55
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Through the real public execution entry and real owned namespace, an unchanged
    real-poll control retains its original public result and confirmed cleanup; each '
- id: FR-017-AC-56
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Inspection of the cleanup change shows that init-pidfd KILL still precedes
    wrapper-group KILL, the single confirmation poll retains its `(5, 0)` timeout,
    signal
- id: FR-017-CON-1
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator SHALL NOT convert a non-verified outcome into a proof claim.
- id: FR-017-CON-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator SHALL NOT report a generation-time classification as an execution
    outcome.
- id: FR-025-AC-1
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: For every harness kind, the identifiers of the emitted `kani::any()` bindings,
    in source order, equal the identity's `arguments` identifiers in order, and those
- id: FR-025-AC-2
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: Every argument binding in a persisted identity names its parameter node
    id and declared domain.
- id: FR-025-AC-3
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A Boolean argument is a `bool` that reaches the oracle unchanged, and a
    bounded-integer argument is an `i64` that reaches the oracle as `rt::Integer::from`
    of t
- id: FR-025-AC-4
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: An argument whose domain has no declared finite bound yields no harness
    and is never narrowed to a machine range.
- id: FR-025-AC-5
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A frame obligation is accounted `unsupported` with a typed reason and no
    harness.
- id: FR-025-AC-6
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A binding that names no parameter of the selected function, and a parameter
    of that function with no binding, are each refused at generation with a typed
    reason
- id: FR-025-AC-7
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: An argument of a family with no row in the Rust-type table (rational, decimal,
    IEEE, text, enum, composite, collection or function) is accounted `unsupported`
    w
- id: FR-025-AC-8
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A harness over a state-reading obligation passes each state value to the
    subject as `&mut` to a harness-owned value and copies it before the call; the
    pre-state
- id: FR-025-AC-9
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: PLANNED (IR-806). Every emitted composite shadow leaf binding retains its
    original operand/parameter node, typed path, declared domain and primitive draw
    type i
- id: FR-028-AC-1
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every harness identity records the request's memory ceiling and wall-clock
    ceiling, and changing either ceiling changes the identity.
- id: FR-028-AC-2
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A backend Kani run that exceeds its wall-clock ceiling is stopped and classified
    KaniRunOutcome::Inconclusive with the timed-out reason naming the ceiling, neve
- id: FR-028-AC-3
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A backend Kani run that exceeds its memory ceiling is stopped and classified
    KaniRunOutcome::Inconclusive with the memory-exhausted reason naming the ceiling,
    d
- id: FR-028-AC-4
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every execution evidence records the bounds of each symbolic argument and
    the two ceilings the run was held to.
- id: FR-028-AC-5
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every harness identity, generation refusal and execution evidence names
    its family.
- id: FR-028-AC-6
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A harness of a family with no supplied shadow records proof subject `production`.
- id: FR-028-AC-7
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A family with a supplied shadow yields a harness with proof subject `bounded_shadow`
    and its refinement obligation in the same result, and never the shadow harn
- id: FR-028-AC-8
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: An argument narrowed inside its declared domain records the declared domain,
    the narrowed bound and that the harness covers only the narrowed bound, in the
    harn
- id: FR-028-AC-9
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A narrowing outside the argument's declared domain is refused with a typed
    reason and no harness.
- id: FR-028-AC-12
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: In a group of N harnesses (one launcher process of a FR-017 batch) sharing
    a wall-clock budget T, a member the backend cuts off (entry status Failure, no
    checks
- id: FR-028-AC-13
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: 'The source of a `bounded_shadow` harness, parsed, names no crate but `core`,
    `alloc`, `std` and `kani` (so no path into `quire_contract_runtime`, `quire_exact` '
- id: FR-028-AC-14
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: The result of a `bounded_shadow` family carries a refinement obligation,
    as generated Rust source in the same result, that names the production subject
    by its s
- id: FR-028-AC-15
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: For every case the refinement run executes, the production subject's complete
    outcome under an unlimited meter agrees with the shadow's result on the observable
- id: FR-028-AC-16
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A refinement run over a domain of at most the request's case cap cases
    executes every case and is classified `exhaustive`; a run over a larger domain,
    or one wh
- id: FR-028-AC-17
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A verified `production` harness reports `production_proved`; a verified
    `bounded_shadow` harness reports `shadow_proved_refinement_exhaustive`, `shadow_proved_r
- id: FR-028-AC-18
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A harness identity of a shadow family lists each abstraction the shadow
    makes by name with the evidence that discharges it (a Kani check of the harness
    or a ref
- id: FR-028-AC-19
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A refinement domain that does not contain every case the shadow harness's
    assumptions admit (a bound narrower on any argument) is refused with a typed reason
    na
- id: FR-028-AC-20
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: 'An item whose shape lies outside its family''s shadow support is refused:
    a missing bound as `requires_bound` naming the unbounded node, a shape no shadow
    covers'
- id: FR-028-AC-21
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: The backend process tree of every run is held to the harness identity's
    memory ceiling by a mechanism that observes or limits the tree's memory (the choice
    is t
- id: FR-028-AC-22
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: 'A shadow family''s machinery catches each seeded mutant: a shadow harness
    over a mutated shadow is `Falsified` with a concrete playback, a refinement run
    over a '
- id: FR-028-AC-23
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every execution evidence records the Kani version, the Rust compiler version,
    the solver name and version, the unwind bound and the complete option vector the
    r
- id: FR-028-AC-24
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: 'The refinement run is a native process launched through one typed refinement
    execution entry that reuses FR-017''s launcher and FR-028''s ceilings: it is held
    to '
- id: FR-029-AC-1
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`verified` with three SUCCESS checks maps to `Proved { success_checks:
    3 }`, and `falsified` with a reproduced replay maps to `Refuted`.'
- id: FR-029-AC-2
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`inconclusive` with the vacuous-proof reason and `cover-unsatisfied` each
    map to `Proved { success_checks: 0 }`, whose QSL category is `inconclusive` with
    cause'
- id: FR-029-AC-3
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The timed-out reason maps to `Incomplete(TimedOut)`, and the memory-exhausted
    and exhausted-unwind-bound reasons each map to `Incomplete(ResourceExhausted)`.
- id: FR-029-AC-4
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The no-verdict reason maps to `Failed`.
- id: FR-029-AC-5
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The missing-cover-summary reason maps to `Failed`. The failure-without-counterexample
    reason maps by its cause (FR-029-AC-32 to FR-029-AC-35).
- id: FR-029-AC-6
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The ordinary production/source-predicate map returns no `Tested`.
- id: FR-029-AC-8
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a replay disagreement of each `DisagreementCause` (`Verdicts`,
    `Witness` and `NoValue`) maps to `Inconclusive(ReplayParity)` carrying that `Dis'
- id: FR-029-AC-9
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)`
    carrying `ReplayRefusal::code()` of that refusal.'
- id: FR-029-AC-10
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a fault maps to `Failed` for each of: the call-site fault''s
    own reading (`ReplaySettlement::Fault`, which QSL''s replay result does not carry:
    Q'
- id: FR-029-AC-11
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with each CG-raised failure that carries no QSL code maps
    to `Failed`: `SpineReplayError::UnboundArgument`, `FieldDelimiter`, `Transcript`,
    `WrongAr'
- id: FR-029-AC-12
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: Across every replay settlement other than reproduced, `falsified` maps
    to a value other than `Refuted`.
- id: FR-029-AC-13
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`,
    each bare and wrapped in `ReplayPackageError` and `FrameReplayError`, maps to '
- id: FR-029-AC-14
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a `DependencyLockError::Input` that carries QSL''s `DuplicateIdentity`
    refusal (code `invalid_package`), as a lock whose only defect is a repeat'
- id: FR-029-AC-15
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: For the ordinary source-predicate `ReplaySettlement` input, a falsified
    outcome given no replay settlement is refused with `TerminalPairError::MissingSettlement
- id: FR-029-AC-16
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'The state-clause replay path (FR-024) settles as the other replay paths
    do: a `StateClauseReplayResult` that settles `ReproducedWithEvaluatedWitness`
    in categor'
- id: FR-029-AC-17
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-806). IR-635's Eq-only verified constructor invokes QSL with
    supplied evidence; the closed FR-028 strength producer/projection remains gated.
    The st
- id: FR-029-AC-18
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: For the `falsified` state-clause run of the wrapping debit subject `deposit_debiting`,
    whose post-state value (-1) lies outside `balance`'s declared range, this
- id: FR-029-AC-19
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). After the cross-outcome disagreement rule,
    verified shadow CeilingReached yields Incomplete(ResourceExhausted); nonzero Exhauste
- id: FR-029-AC-20
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PARTIAL (IR-666 direct F-1 to F-7 report consumption covered; IR-806 must
    reach these rows from a CG-built request over the original proving context). An
    identi
- id: FR-029-AC-21
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806): `run_terminal_value` has no proof-subject or proof-strength
    input and `TerminalPairError` has no variant for a bounded-shadow outcome; adding '
- id: FR-029-AC-22
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-806). The strength set enumerated by the map equals FR-028-AC-17's
    closed set; adding a strength without its projection and outcome row fails the
    bu
- id: FR-029-AC-23
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). A stopped backend run retains its ordinary
    timeout/memory inconclusive classification; a stopped refinement retains FR-028-AC-24
- id: FR-029-AC-24
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). After common claim/source/node/bounds/O-09
    validation, retained refinement_failed/Disagreed evidence for the same valid claim
    yi
- id: FR-029-AC-25
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806). With no retained refinement disagreement, every bounded_shadow
    inconclusive reason maps through the ordinary rows: vacuous proof and cover-uns'
- id: FR-029-AC-26
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED/GATED (IR-806/QSL-640). The converter preserves the actual QSL
    record category/cause: Proved0 remains inconclusive/KaniVacuousProof and is never
    promote'
- id: FR-029-AC-27
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). The distinct typed composite settlement
    input refuses a missing settlement or another claim's node/run/operation/operand/domain/
- id: FR-029-AC-28
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'IR-666 public converter delivered. For any full-identity-bound QSL `CompositeParityResult::Refused`
    report, CG preserves `report.terminal_value()`: non-fault re'
- id: FR-029-AC-29
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: A direct `KaniRunOutcome::Cancelled { source }` maps both IR source variants
    to QSL `IncompleteCause::Cancelled` with the matching exact cause, using the same
    e
- id: FR-029-AC-30
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806). A `bounded_shadow` outcome (verified, falsified, inconclusive
    or cover-unsatisfied) reaches only the composite parity route on the production '
- id: FR-029-AC-31
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806). The parity route fabricates no predicate replay and
    no terminal cause for a bounded-shadow outcome: its terminal value is the one
    QSL''s report'
- id: FR-029-AC-32
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). A `FailedWithoutCounterexample` run with cause `out_of_memory`
    maps to `Incomplete(ResourceExhausted)`, and the same reason with cause `proper
- id: FR-029-AC-33
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). A `FailedWithoutCounterexample` run with cause `timeout`
    maps to `Incomplete(TimedOut)`: the single-run stand-in of FR-029-AC-32 with exit
    sta'
- id: FR-029-AC-34
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). CG's own memory stop (the memory-exhausted reason) and
    a `FailedWithoutCounterexample` run with cause `out_of_memory` give the same terminal
    v
- id: FR-029-AC-35
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). The cause never creates a verdict. Over every combination
    of outcome, reason and cause (absent, `properties_failed`, `timeout`, `out_of_memory
- id: FR-029-AC-36
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). The rationale of this requirement states that Kani reports
    any `SIGKILL` of CBMC as `out_of_memory`, that an `out_of_memory` cause is accepted
- id: FR-030-AC-1
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every pair (`KaniOutcomeKind`, replay settlement) that the map's input
    can express maps to exactly one `TerminalValue`.
- id: FR-030-AC-2
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Refused`, `InvalidInput` and `IncompleteInput` map to `Declined` with
    `ProofRefusalCause::Refused`, `InvalidInput` and `IncompleteInput` respectively,
    so no re'
- id: FR-030-AC-3
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`TimedOut` and `ResourceExhausted` map to their distinct `Incomplete`
    causes. `Cancelled { source: Requested }` and `Cancelled { source: Deadline }`
    map to `Inc'
- id: FR-030-AC-4
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Proved` with three SUCCESS checks maps to `Proved { basis: Checks { success_checks:
    3 }, certification: Certified }`, zero checks maps to `Proved { basis: Chec'
- id: FR-030-AC-5
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Inconclusive` with cause `kani_vacuous_proof` maps to `Proved { basis:
    Checks { success_checks: 0 }, certification: Certified }` (category inconclusive/vacuous'
- id: FR-030-AC-6
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: No outcome maps to `Tested`.
- id: FR-030-AC-7
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: The map is one `match` over the pair (`KaniOutcomeKind`, replay settlement)
    with no wildcard arm.
- id: FR-030-AC-8
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Unavailable` with cause `kani_checker_absent` maps to `Unsupported(SolverAbsent)`;
    with `kani_backend_absent` or any other cause it maps to `Unsupported(Backen'
- id: FR-030-AC-9
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a replay disagreement maps to `Inconclusive(ReplayParity)`,
    and with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` car'
- id: FR-030-AC-10
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a fault, walked through every wrapper FR-029-AC-10
    lists, and with each CG-raised failure FR-029-AC-11 lists, maps to `Failed`.'
- id: FR-030-AC-11
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Across every replay settlement other than reproduced, `Counterexample`
    maps to a value other than `Refuted`.
- id: FR-030-AC-12
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`,
    each bare and wrapped, maps to `Inconclusive(ReplayRefused)` carrying tha'
- id: FR-030-AC-13
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a `DependencyLockError::Input` that carries QSL''s
    `DuplicateIdentity` refusal (code `invalid_package`), as a lock whose only defect
    is a r'
- id: FR-030-AC-14
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: A `Counterexample` given no replay settlement is refused with `TerminalPairError::MissingSettlement`,
    and each other kind given a settlement is refused with `Te
- id: FR-030-AC-15
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public report controls; IR-806 must reach the same
    rows from a CG-built request over the original proving context). For an identity-valid
- id: FR-030-AC-16
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: 'PARTIAL (IR-666 direct public report controls; IR-806 must reach the same
    rows from a CG-built request over the original proving context). With valid identity, '
- id: FR-030-AC-17
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: PARTIAL (IR-666 F-7 and binding controls; IR-806 original-artifact path
    planned). With valid identity and all earlier rows absent, changing only the retained
    sh
- id: FR-030-AC-18
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: 'Every Kani `Proved` row carries `ProofBasis::Checks` with the transcript
    count and `Certification::Certified`, including the zero-check row; zero checks
    remain '
- id: FR-030-AC-19
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: For both `BoundedComplete { depth }` and `Inductive { depth }`, an SMT
    proof whose certificate FR-314 verifies maps to `Proved` with the same basis/depth
    and `C
- id: FR-030-AC-20
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: For either SMT basis, a foreign query, wrong certificate shape, invalid
    checked proof step or missing final empty clause rejected by FR-314 maps to `Inconclusiv
- id: FR-030-AC-21
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: SMT/cross-check disagreement without certificate rejection keeps both verdicts
    and settles inconclusive; agreement does not change certification; missing Z3
    ret
- id: FR-030-AC-22
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every expressible accepted SMT backend/checker/replay settlement produces
    one terminal result, never `Tested`; no non-`Proved` result carries basis or certifica
- id: FR-034-AC-1
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before the exclusive caller lease, matching helper identity, verified monitor/INIT/peer,
    observer readiness and positive typed Dispatch are established, no prod
- id: FR-034-AC-2
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before any M spawn, O is positively verified outer PID 1 with armed parent-death
    protection and original-exclusive bootstrap authority. Unclaimed inner I is con
- id: FR-034-AC-3
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: In ClaimedBootstrap, InitReady or Dispatched, original-caller lease EOF
    causes guardian cancellation/exit and confirmed INIT termination; Dispatched descendants
- id: FR-034-AC-4
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Mutual authority is the specific anonymous connected pair: guardian checks
    the actual mapped creator UID and only the original caller holds its executor
    endpoin'
- id: FR-034-AC-5
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: C retains actual unreaped Child L; L/O/M/I ownership is verified through
    host pidfds, start/parent/namespace identities and actual outer/inner PID 1. O
    owns act
- id: FR-034-AC-6
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The owned tree memory observer is ready before typed Dispatch; failed observer
    preparation releases no backend instruction.
- id: FR-034-AC-7
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Bounded inner startup/identity recovery is required before Dispatch. M
    death/reparenting or malformed/missing info cancels through retained outer INIT
    authority
- id: FR-034-AC-8
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: After Dispatch, original-caller death cancels double-fork/reparented, setsid,
    late-born and nested-PID-namespace descendants through kernel-owned INIT teardown,
- id: FR-034-AC-9
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Cancelling one owned namespace leaves concurrent independent runs and unrelated
    host children unaffected.
- id: FR-034-AC-10
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Normal backend completion, backend exec failure, explicit cancellation,
    timeout, memory excess, capture failure and observation failure each require confirmed
    o
- id: FR-034-AC-11
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Guardian SIGKILL, abort or OOM-equivalent death during bootstrap, InitReady
    or immediately after Dispatch cannot release an unowned production backend: the
    guar'
- id: FR-034-AC-12
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Every startup refusal, completion and cancellation settles actual owned
    roles before conclusions. Anonymous controls and report pipe/memfd have no persistent
    na
- id: FR-034-AC-13
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'The bounded executor requires an explicit path to this package''s actual
    Cargo executable quire-kani-guardian, built and supplied with the library from
    the same '
- id: FR-034-AC-14
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Setup docs require the matched package helper roles, safe caller-context
    user/PID/private-mount permissions, verified mapping/setgroups/private proc, nested
    ins
- id: FR-034-AC-15
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Typed private controls reject malformed and unknown fields and enforce
    finite encoded-byte, pending-message and startup-work bounds. Ancillary descriptor
    count/
- id: FR-034-AC-16
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Only the original caller holds the pair's executor endpoint. Guardian receives
    only its control end as fd0 through safe CLOEXEC child-only mapping, borrows it
    w
- id: FR-034-AC-17
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Backend receives unchanged raw argument bytes except the explicitly allocated
    internal report locator: harness options then -Z unstable-options --export-json
    /p'
- id: FR-034-AC-18
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Guardian diagnostics and control use separate channels from backend stdout/stderr
    captures and cannot contaminate reports; existing bounded capture and capture-
- id: FR-034-AC-19
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Original identity ceilings and existing outcome classifications remain
    authoritative after confirmed cleanup: memory excess and wall expiry stay distinct,
    ambig'
- id: FR-034-AC-20
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Guardian connection, startup, identity verification and Dispatch use the
    original monotonic identity deadline without resetting it. An already expired
    deadline '
- id: FR-034-AC-21
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Guardian connection and handshake have a finite setup cap within the remaining
    original deadline. Cap expiry while that deadline remains live is a typed setup
    r
- id: FR-034-AC-22
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Control/capture shutdown and cleanup observation waits have finite bounds.
    Unconfirmed termination refuses with a live caller rather than accepting a proof
    or c
- id: FR-034-AC-23
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Caller-death fixtures use the actual matching normal helper and shared
    production functions/order with bounded prefix selection as input only. Actual
    caller/gro
- id: FR-034-AC-24
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Actual lease-EOF cancellation passes independent original I termination
    and no backend Dispatch/marker before escalation, plus death of a positively acknowledge
- id: FR-034-AC-25
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before backend Dispatch, the helper handshake matches the actual running
    CG library's build, protocol and lifecycle-capability identity against the actual
    invok
- id: FR-034-AC-26
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The guardian session/group is distinct from the original caller before
    Ready/Dispatch. Real feature-on caller-group deaths and feature-off positive post-Dispatc
- id: FR-034-AC-27
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: guardian-test-support is off by default, absent from default features,
    and exposes exactly one documented fixture operation only when explicitly enabled.
    A feat
- id: FR-034-AC-28
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Fixture prefix selects unchanged production transitions, never certifies
    live state. Actual original identity/right custody, independent pre-escalation
    I/worker
- id: FR-034-AC-29
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The packaged caller fixture and real helper link the same normal library
    artifact through consumer-manifest package selection with matching target/profile/featu
- id: FR-034-AC-30
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: CG publishes the test-only feature contract and allocates production-driver
    dependency-edge exclusion to IR-649's QSL driver work. The contract requires all
    dow
- id: FR-034-AC-31
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Real production fixtures fail the merged PR #295 recipe at exact post-clone/pre-internal-child_wait
    caller death and natural M/info-reader failure, and pass onl'
- id: FR-034-AC-32
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The bounded O event loop collects anonymous report pipe into memfd without
    a backend/collector completion wait cycle. A hard 16 MiB plus one detection-byte
    rete
- id: FR-034-AC-33
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Authenticated Completed precedes original-lease close, confirmed inner
    teardown and M termination/reap closing ALL pipe writers, including reopened procfd
    and d
- id: FR-034-AC-34
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Live-C production close_lease_and_observe retains L/O/M/I ownership and
    final report control independently of consumed original lease. No outer kill,
    bootstrap '
- id: FR-034-AC-35
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. L creates private network/root before O, O validates before
    M, and nested bind / / refers to confined O root. Safe backend-only seccomp/privilege
- id: FR-034-AC-36
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. C inventories actual backend fd0/fd1/fd2. Real OriginalStdin::Open
    socket input and failed fstat inspection refuse before Dispatch; actual produc
- id: FR-034-AC-37
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Trusted I retains its exclusive lease through Dispatch;
    O retains separate final delivery after original lease close. All bootstrap/ownership/rep
- id: FR-034-AC-38
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Named fixed SETTLE_RESERVE R is measured/rounded; its finite
    effective reserve stays inside original whole T; short finite ceilings stay admitted
- id: FR-034-AC-39
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Native guardian/backend-installer policy support is source/cfg
    Analysis; unreachable unsupported-native support is not a fabricated runtime Test.
- id: FR-034-AC-40
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Only declared trusted role/operation/phase causes use the
    cross-role representation. Actual kind/errno, independently authenticated optional
    fini
- id: FR-034-AC-41
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). For a named native-accounting conformance claim,
    the analysis identifies the exact consumer build configuration and compiler actually
    us
- id: FR-034-AC-42
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The proof is derived from GNU objdump or llvm-objdump
    disassembly using stable compiler inputs of the actual final linked executable
    aft
- id: FR-034-AC-43
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Every named operation and thread-entry root maps
    to final address ranges or conservatively covered host frames, and every reachable
    dire
- id: FR-034-AC-44
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). A reachable recursive call cycle, including one
    through an external callee, leaves the native-workspace bound UNPROVEN under this
    nonrec
- id: FR-034-AC-45
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The native-stack bound sums per-thread maxima for
    every concurrently live thread instance carrying named state, accounting for thread-en
- id: FR-034-AC-46
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'PLANNED/UNRUN (IR-682). The analysis covers the actual linked panic strategy:
    reachable unwind/landing-pad and cleanup paths contribute to the stack bound,
    or t'
- id: FR-034-AC-47
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Each relevant external library/runtime/libc/allocator
    frame has an explicit finite bound source matching the actual loaded runtime ABI/c
- id: FR-034-AC-48
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The named heap inventory covers actual returned
    Rust capacities times element size, metadata and simultaneous lifetimes, excluding
    incid
- id: FR-034-AC-49
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). A change to the source head, resolved consumer
    configuration/compiler, resulting executable or loaded runtime/bound premises
    invalidates
- id: FR-034-AC-50
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Missing or incomplete proof, or a proven bound
    exceeding the independently declared runtime charge, leaves the configuration
    UNPROVEN an
- id: FR-034-AC-51
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Given genuine live-I construction preconditions, the child's
    own acknowledgement establishes release-dependent birth after the actual completed
    o
- id: FR-034-AC-52
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual retained-I pidfd termination confirmation strictly
    precedes matching kernel report seal; independently observed fabricated-live-I
    and late
- id: FR-034-AC-53
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual retained M consuming Child wait/reap strictly precedes
    matching kernel report seal; independent fabricated-reap and late-reap controls
    fai
- id: FR-034-AC-54
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. On the independently grounded supported cooperative cancellation
    path, the actual original O gate owner retains the gate through actual I confirm
- id: FR-034-AC-55
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Independently checked final-source Analysis establishes
    contained teardown authority through actual namespace INIT death and retained
    owner settl
- id: FR-034-AC-56
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Published-source Analysis establishes O's final complete
    accounting after actual I confirmation, retained M reap, writer EOF and immutable
    seal f
- id: FR-034-AC-57
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: "PLANNED/UNRUN (IR-687). When L awaits its first O arm result, the newly\
    \ allocated authenticated O\u2192L OperationalFailure carries zero ancillary rights."
- id: FR-034-AC-58
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). L accepts that negative only from its actual retained
    O Child PID with kernel sender UID/GID0 and the original build/run/source/state/st
- id: FR-034-AC-59
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: "PLANNED/UNRUN (IR-687). For L\u2192C AwaitArm negatives, C rejects missing,\
    \ extra, wrong-type or substituted rights through owned cleanup, without a zero-right\
    \ fallb"
- id: FR-034-AC-60
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The negative O pidfd grants C only exact O termination
    observation and existing owned-cancellation signalling, while L retains its direc
- id: FR-034-AC-61
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Negative capability custody grants no Armed, namespace/network,
    phase, report or Dispatch admission.
- id: FR-034-AC-62
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C adopts the authentic O producer stop before L
    wait and preserves the original earliest cutoff, including absent work deadline,
    without
- id: FR-034-AC-63
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C exposes a negative original-cause result only
    after actual O Child wait at L and required L/capture/creator/control-EOF settlement
    und
- id: FR-034-AC-64
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The new negative routes reuse existing charged
    controls and ceilings without a new envelope type, ACK, window, cap or public
    category.
- id: FR-034-AC-65
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Before first-byte COMMIT, a fresh complete exhausting
    pre-send tick retires an unsent genuine OperationalFailure into existing OwnerStop
- id: FR-034-AC-66
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: "PLANNED/UNRUN (IR-687). Only the allocated L-local pre-arm negative route\
    \ forwards the same bytes to C with exactly one clone of L\u2019s retained O pidfd."
- id: FR-034-AC-67
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C authenticates the L-forwarded negative only in
    AwaitArm through the actual L chain and original O/run/build bindings.
- id: FR-034-AC-68
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C does not use the negative pidfd for pidfd_getfd,
    pidfd setns, process_madvise, other-actor authority or numeric-PID reopening.
- id: FR-034-AC-69
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Missing or abnormal whole-chain settlement overrides
    the provisional negative with existing CleanupUnconfirmed and no execution evidence
- id: FR-034-AC-70
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The first emitted terminal byte irreversibly fixes
    the candidate; partial/full publication cannot be rewritten, spliced or followed
    by a
- id: FR-034-AC-71
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). O includes every due observation and a genuine
    fresh complete named-input tick before initial terminal emission; failed current
    input ne
- id: FR-034-AC-72
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). After COMMIT, later successful complete exhaustion
    stays in actual private O accounting until genuine settlement; retention is an
    Analys
- id: FR-034-AC-73
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A claimed-startup negative publication remains
    provisional until positive actual I termination and separate real M Child reap,
    with requ
- id: FR-034-AC-74
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: "PLANNED/UNRUN (IR-687). The negative\u2019s absent peak field supplies\
    \ no zero measurement, no-history claim or current-validity substitute."
- id: FR-034-AC-75
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A later genuine observation/collector/transport
    error after COMMIT remains owned through bounded cleanup and prevents normal O
    Code0 acc
- id: FR-034-AC-76
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A genuine admitted open-file-description Test oracle
    distinguishes L's retained-pidfd clone from independently opened same-O and foreign
- id: FR-034-AC-77
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). On a fresh complete Exhausted tick after first-byte
    COMMIT, O immediately initiates the same actual owned cancellation of live claimed
    I
- id: FR-034-AC-78
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. C admits actual original O Armed only through genuine retained
    O/build/run authentication and original owned rights; wrong sender/run or unvalida
- id: FR-034-AC-79
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual production control reception rejects malformed, partial
    or forbidden-right traffic without granting positive phase/Dispatch/report authori
- id: FR-034-AC-80
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine original authenticated I Completed alone establishes
    completion reception; M exit, EOF, phase or copied metadata cannot substitute.
    The s
- id: FR-034-AC-81
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every surviving actual simultaneous named fixture/control
    storage and pipe/backing reservation is charged before exposure; retired certificate
    st
- id: FR-034-AC-82
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual supported-source/artifact Analysis and mechanical
    paired consumer/helper checks establish feature-off absence and unchanged production
    fra
- id: FR-034-AC-83
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. The original exclusive caller lease closes independently
    of retained monitor/INIT ownership and does not leak into children; a leaked alias
    canno
- id: FR-034-AC-84
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every remaining fixture reporter/control/report endpoint
    is excluded from untrusted child/exec mappings and public control handles through
    actual
- id: FR-034-AC-85
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'PLANNED/UNRUN. Actual retained M consuming reap is not established by
    repeated or cached Child Some(status); an independent genuine-operation predicate
    rejects '
- id: FR-034-AC-86
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. O establishes same-M parent-only prior-WNOWAIT/FIRST-uncached-Some/post-ECHILD
    provenance under the original sole-waiter/disposition/history; ind
- id: FR-034-AC-87
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual I confirmation/M consuming reap precede report seal.
    On AC-54's independently grounded supported cooperative path, actual I confirmation
    p
- id: FR-034-AC-88
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. O preserves every due ordinary sample and existing control/cutoff
    transition without observation ACK, pause, stored publication permission or sup
- id: FR-034-AC-89
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine original I Completed reception and original stop
    adoption precede actual original lease close, without a separate stored completion
    certi
- id: FR-034-AC-90
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Any remaining optional fixture write failure does not terminate
    O or alter ordinary sampling/control/settlement/cancellation; it cannot rescue
    ig
- id: FR-034-AC-91
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Ordinary feature-on execution remains available without
    retired observation binding, endpoint, event emission or default identity.
- id: FR-034-AC-92
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine adopted-orphan acknowledgement and release-dependent
    child birth after an actual complete ordinary O sample remain required without
    a sto
- id: FR-034-AC-93
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every surviving feature-on named native workspace has separate
    supported paired-artifact finite-bound Analysis and an independent charge; sole-pu
- id: FR-034-AC-94
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'PLANNED/UNRUN. C disposes the defined pending claim through actual close
    of its raw I right without positive admission or capability; invalid prior traffic
    and '
- id: FR-034-AC-95
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual O retains the original report backing identity through
    collection, kernel sealing and final descriptor delivery; a substituted backing
    can
- id: FR-034-AC-96
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Missing, malformed, wrong or unvalidated original O/build/run/control
    authority refuses without positive admission, retaining actual received-rig
- id: FR-034-AC-97
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Retired observation-binding echo creates no replacement
    comparison; actual original report backing equality and delivery/settlement authority
    rem
- id: FR-034-AC-98
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Feature-off paired consumers/helpers retain unchanged phase
    bytes/rights with no retired early field or branches; ordinary feature-on execution
    h
- id: FR-034-AC-99
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Any remaining optional O fixture write requires actual ignored/unblocked/untraced
    stable single-thread proof, including independent across-PID-na
- id: FR-034-AC-100
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Unsupported, malformed, unavailable or unproved profiles
    permit no optional fixture write, preserving pending signals, ordinary flow and
    original
- id: FR-040-AC-1
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A caller retains a live `Cancel`, sends its clone through a public single-run
    request, and the bounded launcher observes it before spawn and during an actual
    ru
- id: FR-040-AC-2
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: In two live single runs, issue `Requested` then `Deadline`, and reverse
    the calls. Each run's actual launcher stop, `KaniExecutionEvidence.outcome`, constructed
- id: FR-040-AC-3
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A run that reaches its execution ceiling with no cancellation event remains
    `TimedOut`. A `Deadline` event recorded only after completion, memory/capture
    stop o
- id: FR-040-AC-4
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: Completion before observed cancellation keeps the report-based result;
    measured memory exhaustion, memory observation failure, output over-limit, unread
    capture
- id: FR-040-AC-5
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A multi-member group sharing one handle stops wholly on its observed cause,
    returns `BatchCancelled { source, members }`, and classifies no member; another
    grou
- id: FR-040-AC-6
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The single-run producer constructs an IR `KaniOutcomeKind::Cancelled {
    source }` from settled execution evidence, and its real call path reaches the
    FR-030 map.
- id: FR-040-AC-7
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The direct `quire-exact::CancelCause` constructs `qsl-replay::IncompleteCause::Cancelled`
    through a type-checked exhaustive conversion, while all QSL imports re
- id: FR-041-AC-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A nonempty reviewed declaration is checked against the current compiled-module
    census and the merge-base declaration. An absent compiled module, a removed claim
- id: FR-041-AC-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside
    only the first and an UNREACHABLE check inside the second reports the second `unrea
- id: FR-041-AC-3
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The current build's transitive expectation/subject reachability yields
    the shared-helper intersection; the gate fails when the checked-in generated evidence
    omi
- id: FR-041-AC-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Each claimed module and computed shared helper has a positive unmodified
    real-Kani proof and a source mutation that compiles, executes, and reports a failed
    pro
- id: FR-041-AC-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A stubbed executor, predetermined outcome or build-only run produces no
    qualifying module evidence and cannot pass the gate.
- id: NFR-006-AC-1
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-scope` prints `required` and the path for each of `src/kani/generate/negotiate.rs`,
    `src/oracle/boolean_v1.rs`, `src/routed/generate.rs`, `src/replay'
- id: NFR-006-AC-2
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-scope` prints `not required` for each of `spec/kani/functional/FR-017-kani-execution-evidence.md`,
    `reviews/REV-018-bound-coverage-observations.md`, '
- id: NFR-006-AC-3
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A rename of `src/kani/old.rs` to `src/strategy/new.rs`, and a rename of
    `src/strategy/old.rs` to `src/kani/new.rs`, each print `required`.
- id: NFR-006-AC-4
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make -n kani` and `make -n kani-gate` expand to the same `cargo test`
    command line, and that line holds each of the six filters `kani_obligations`,
    `skeleton_s'
- id: NFR-006-AC-5
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-gate` starts the lane''s `cargo test` once with `--test-threads=1`,
    holds the lock named by `KANI_LOCK` while it runs, and does not start it while
    ano'
- id: NFR-006-AC-6
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-gate` exits non-zero and prints `result=failed` when the lane''s
    filters list zero tests, when fewer tests run than are listed, and when any test
    fail'
- id: NFR-006-AC-7
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: The passing run-result line has the fields `result`, `ran`, `expected`,
    `elapsed`, `kani`, `tree` in that order, with `ran` equal to `expected`, a non-empty
    `ka
- id: NFR-006-AC-8
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: When the tree state or the head commit read after the run differs from
    the one read before the build, `make kani-gate` prints `result=failed` and exits
    non-zero
- id: NFR-006-AC-9
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: With a dirty working tree `make kani-gate` prints `tree=dirty`, never `result=passed`,
    and exits non-zero.
- id: NFR-006-AC-10
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: 'With no `cargo-kani` launcher `make kani-gate` starts no test, prints
    `kani-gate: not run: launcher absent` and exits non-zero.'
- id: NFR-006-AC-11
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: 'The tests marked `#[ignore = "kani lane: ..."]` in `tests/it` are exactly
    the `#[ignore]`d tests of `tests/it` whose path holds one of the `make kani` filters.'
- id: NFR-006-AC-12
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make -n ci` expands to no `kani` or `kani-gate` command.'
- id: NFR-006-AC-13
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: For a Kani-touching pull request, the author runs the full gate on its
    clean, stable candidate before opening it, and the merger runs it again on the
    clean, sta
- id: NFR-006-AC-14
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: 'Given Kani-reaching paths changed on `origin/main` since the candidate''s
    merge base and main absent from the candidate, `make kani-gate` refuses before
    running '
- id: NFR-006-AC-15
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A Kani-touching pull request whose required lane did not run or failed
    remains unmerged and does not claim real-Kani verification; a written explanation
    does no
- id: NFR-006-AC-16
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A pull request whose scope is `not required` has no mandatory real-Kani
    run under this requirement.
- id: NFR-006-AC-17
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Before a version tag is pushed, the full gate actually passes on the clean,
    stable candidate to be tagged; a copied release-ticket line cannot substitute
    for th
- id: NFR-006-AC-18
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Every `include!`, `include_str!`, `include_bytes!` and `#[path]` target
    named in a file of `tests/it` that holds a lane test, and in each file those name
    in tur
- id: NFR-006-AC-19
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: On a clean current candidate, `make kani-gate` runs FR-041's module census
    and real mutation controls in its one required lane, and a failure of any claim
    or sh
- id: NFR-006-M-1
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Merged pull requests touching the Kani-touching set without an actual full
    gate passing on the current candidate immediately before merge
- id: NFR-006-M-2
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Lane runs reported `passed` with `ran` different from `expected` or `ran`
    of 0
- id: NFR-006-M-3
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`#[ignore = "kani lane: ..."]` tests the lane does not select'
- id: FR-014-AC-1
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every requested item receives exactly one generated or typed-refused disposition,
    and a refused item contributes no generated function while its siblings are ge
- id: FR-014-AC-2
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division,
    modulo), rational, ordering, decimal, IEEE arithmetic/comparison/width conversion
- id: FR-014-AC-3
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor whose arity, result or operand scalar type disagrees with
    the lowered node, a node that is not a lowered scalar expression, a node that
    is unbounde
- id: FR-014-AC-4
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Generated bytes are identical across repeated runs and across permutations
    of the request order; the generated crate is exactly `Cargo.toml`, `src/lib.rs`
    and `
- id: FR-014-AC-5
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every claim-map entry carries the node id, IR id, package id, source map,
    claims, bounds and operation identity of its item, and, for a `quire.op.quantity.conve
- id: FR-014-AC-6
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193
    vectors, and on these QSpec TC-187 vectors, yields outcomes, charges and consumed
    cou
- id: FR-014-AC-7
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Composite, collection, function, model, relation, state, temporal and
    protocol nodes are refused with their blocked or unsupported reason and never
    reported as '
- id: FR-014-AC-8
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names the direct exact/QSV
    and Runtime residue dependencies from the one profile authority without a replay-cont
- id: FR-014-AC-9
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Generated oracle functions do not panic: an invalid generated constant,
    including a decimal target, stops as `InvalidConstant`, an operand of the wrong
    width st'
- id: FR-014-AC-10
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor parameter whose admitted bound is missing, repeated or unequal
    to the reachable `bounded_domain` node, an operand that is neither a literal nor
    a r
- id: FR-014-AC-11
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A claim whose descriptor passes every check this requirement states, and
    whose descriptor agrees with the node's catalogued `operation.identity`, law definition
- id: FR-014-AC-12
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor naming a different catalogued operation than the node's own
    is not confirmed, including where the two share one operand shape and one checked
    bound
- id: FR-014-AC-13
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A descriptor whose implied identity matches the node''s but whose law
    definition is absent from that node''s `operation.laws`, or whose mode value disagrees
    with '
- id: FR-014-AC-14
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: "A claim whose node this generator never inspected \u2014 a duplicate copy,\
    \ or a record that never lowered \u2014 or inspected and refused with a typed\
    \ reason, marks its o"
- id: FR-014-AC-15
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A requested item whose node closure exceeds the 65,536-unit lowering work
    ceiling is refused as `LoweringWorkExhausted`, naming the ceiling and the consumed
    cou
- id: FR-014-AC-16
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A bound is read from `binding` members looked up by name in any order (`min`/`max`,
    `numerator_min` through `denominator_max`, `coefficient_min` through `roundi
- id: FR-014-AC-17
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `reference` operand whose target is typed by a `bounded_domain` node
    is classified by that domain''s base scalar type, so `x + 1` over a parameter
    `x` of type '
- id: FR-014-AC-18
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`derive_exact_scalar_items` returns, per node id in input order, the descriptor
    determined by the node''s `operation.identity`, `operation.mode`, `operation.laws'
- id: FR-014-AC-19
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A derived item generates an oracle whose operation is `ir_confirmed`,
    and for every item of the corpus the derived descriptor equals the descriptor
    the fixture '
- id: FR-014-AC-20
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An integer node over two distinct bounded parameters, `Int[0, 9]` and `Int[10,
    20]`, each typed by its own `integer_range` `bounded_domain`, with its result
    typ
- id: FR-014-AC-21
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node whose result is typed by a scalar type and whose reachable bounds
    are its operands' own and exactly one other takes that other as its result bound.
- id: FR-014-AC-22
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node whose result is typed by a scalar type and whose two or more reachable
    bounds of the form are not exactly one outside the operands' own bounds is refused
- id: FR-014-AC-23
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An operand, or the result, typed by a `bounded_domain` of a form other
    than the descriptor's is refused as `MissingBound`.
- id: FR-014-AC-24
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'An operand''s own bound need not lie inside the result bound: `a + wide`
    over `[0, 9]` and `[0, 50]` with a `[0, 29]` result, `-e` over `[1, 9]` with a
    `[-9, -1]'
- id: FR-014-AC-25
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `reference` operand to a plain scalar parameter or a non-literal `value`
    node is refused as `RequiresBound` naming its type, in generation and in derivation, '
- id: FR-014-AC-26
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `reference` operand whose target is a `value` node with a literal body
    is a literal operand: `x + 1` over `x: Int[0, 9]` narrowed to `Int[0, 10]`, with
    the li'
- id: FR-014-AC-27
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node whose result is typed by a scalar type and that only narrowing conversions
    (`quire.op.numeric.narrow`, exactly one argument referencing the node) consume
- id: FR-014-AC-28
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'The referenced node''s body, not its form or type, makes an operand a
    literal: `x + n`, where `n` is a plain-Integer-typed `value` node of form `literal`
    whose b'
- id: FR-014-AC-29
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`x + p` narrowed to `Int[0, 29]`, where `p` is a parameter typed by the
    plain Integer type, is refused as `RequiresBound` naming Integer, in generation
    and in d'
- id: FR-014-AC-30
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`q + q` narrowed to `Int[0, 29]`, where no operand owns a bound and `q`''s
    node reaches `[0, 29]`, is refused as `RequiresBound` naming Integer, in generation
    an'
- id: FR-014-AC-31
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A literal operand of an integer operation that is not a canonical `integer`
    literal is refused as `OperandTypeMismatch` naming its position and literal kind,
    wh
- id: FR-014-AC-32
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node that narrowing conversions narrow to two distinct bounds (`[10,
    29]` and `[0, 40]`) is refused as `AmbiguousBound`, in generation and in derivation.
- id: FR-014-AC-33
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A node narrowed to a `text_bounds` bound over Integer, or to an `integer_range`
    over Rational, is refused as `MissingBound` naming Integer and `integer_range`, '
- id: FR-014-AC-34
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node that a narrowing conversion and a node that is not a narrowing conversion
    both consume (`x + 2` narrowed to `Int[0, 11]` and an operand of `(x + 2) + x`)
- id: FR-014-AC-35
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Boolean `and`, `or`, `not`, `implies`, `eq` and `ne` nodes and integer
    `eq`, `ne`, `lt`, `le`, `gt` and `ge` nodes each derive a descriptor and generate
    an `ir_
- id: FR-014-AC-36
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every oracle's source map declares the node's implication-consequent count,
    one evaluation-entry probe on the function-entry line disjoint from every consequent
- id: FR-014-AC-37
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A differential corpus covering every node FR-014-AC-35 names compiles against
    the runtime alone, and each oracle's outcome, admitted charges and consumed counte
- id: FR-014-AC-38
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The Boolean condition in the third argument of a `state`/`state_clause`
    node's `quire.op.state.clause` body, addressed by the clause node id, derives
    a descript
- id: FR-014-AC-39
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The generated `src/lib.rs` of every crate the TC-024 corpus generates,
    and the non-test code of `src/oracle/scalar/mod.rs` (comments and every `#[cfg(test)]`
    it
- id: FR-014-AC-40
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A package whose lowering returns `failed` for the `bytes` limit (Contract
    IR FR-038-AC-95) is refused per item as `ExactScalarRefusal::LoweringByteLimitExceeded
- id: FR-014-AC-41
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A `failed` record whose `limit_kind` is `work` is still refused as `LoweringWorkExhausted`
    with the record's `limit` and `consumed` (FR-014-AC-15 unchanged), an
- id: FR-014-AC-42
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: No `limit_kind` of a `failed` record is read in `src/oracle/scalar/mod.rs`,
    `src/oracle/equality/mod.rs` or `src/oracle/function/mod.rs` but through the one
    sha
- id: FR-014-AC-44
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: For each IR-admitted direct request naming a `temporal`/`formula`, `temporal`/`fairness`
    or `expression`/`case` node, the scalar claim map has exactly one entry
- id: FR-014-AC-45
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: For a QSL-shaped outer integer operation narrowed to an `integer_range`,
    a direct reference to a plain-Integer inner application over independently bounded
    oper
- id: FR-014-AC-46
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'An inferred interval uses only the four named integer operations and canonical
    literal or `integer_range`-typed leaves. A direct reference to a raw application '
- id: FR-014-AC-47
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of every oracle-agreement
    corpus of FR-014, FR-018 and FR-021 (the TC-185, TC-186, TC-187, TC-192 and T
- id: FR-014-AC-48
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A `quire.op.quantity.convert` node whose `type_argument` names a quantity
    unit and whose `operation.mode` is `exact` derives and generates an `ir_confirmed`
    ora
- id: FR-014-AC-49
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: When the runtime refuses a `quire.op.quantity.convert` with `IllTyped`
    before any charge, the oracle stops as `ill_typed` with zero admitted charges
    and no chan
- id: FR-014-AC-50
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'No oracle generated for `quire.op.quantity.convert` returns a scalar:
    its result is always a `Quantity` in the supplied target unit, and a conversion
    node whose'
- id: FR-014-AC-51
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A selected `quire.op.quantity.convert` claim settles through capability
    settlement like any other quantity operation, as `OperationNotRendered` (Planned,
    IR-269
- id: FR-018-AC-1
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every requested item receives exactly one generated or typed-refused disposition;
    a refused item contributes no function, no environment constructor and no symb
- id: FR-018-AC-2
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: On every vector of the composite-equality corpus, the generated oracle's
    `Outcome<bool>`, its admitted charge sequence and its consumed counters are equal
    to th
- id: FR-018-AC-3
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "Each generated item's claim-map entry records the descriptor the request\
    \ supplied \u2014 operator, both operand source types and both conversion targets\
    \ \u2014 and it is "
- id: FR-018-AC-4
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each generated item's claim-map entry records an `EqualitySchedule` equal
    to `CheckedEquality::schedule()` of the `CheckedEquality` its descriptor admits,
    for a
- id: FR-018-AC-5
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A descriptor with a `convert<T>` operand outside `admits_equality_conversion`,
    distinct text profiles, distinct enum declarations, incompatible dimensions, dist
- id: FR-018-AC-6
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "An operand type bearing an IEEE value at any depth \u2014 including a\
    \ `float64` field of a record nested inside a `sequence` \u2014 is refused as\
    \ `OperatorIneligible`, an"
- id: FR-018-AC-7
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: On the FR-330 package path, model, function, state and temporal roots outside
    the equality lowering profile retain their exact typed `Unsupported` feature-accou
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation
    time carrying that `DeclarationCause`, including both recursion passes. The authen
- id: FR-018-AC-9
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Every generated oracle function returns `Outcome<bool>`, never `bool`:
    with a denial injected at each of `equality.plan-form`, `equality.plan`, `equality.pair`,'
- id: FR-018-AC-10
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Generated bytes are identical across repeated runs and across permutations
    of the request order, including a request carrying two descriptors over one node
    id, '
- id: FR-018-AC-11
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every claim marks its operation `caller_declared`, the claim map carries
    the blocked item "operation identity not consumed by codegen's generators", and
    two ite
- id: FR-018-AC-12
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names direct exact/QSV
    and root replay dependencies through the one manifest authority, contains no charge
    amoun
- id: FR-018-AC-13
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every claim-map entry carries the node id, IR id, package id, source map,
    claims, reconstructed declaration keys and selected schedule of its item, and
    its genu
- id: FR-018-AC-14
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Admitted `bounded_domain` nodes an operand type reaches retain the exact
    named bound values FR-014 specifies. For `integer_range` and `collection_bounds`,
    Contr
- id: FR-018-AC-15
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A `binary` node''s operand is read through its `reference`: a reference
    to a `convert` expression node is read as the type of what it converts, following
    nested '
- id: FR-018-AC-16
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Genuine public tuple members naming `integer_range`, `decimal_range`, `rational_range`
    and `text_bounds` nodes reconstruct their exact own bounds, text profile,
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The `src/lib.rs` that `generate_composite_equality_oracles` returns for
    the TC-029 step 1 package (every generated item) contains zero occurrences of
    the panic '
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`render_value_type` called with `ValueType::Quantity` and with `ValueType::Reference`
    returns `Err(RenderError::UnsupportedValueType { family })` with `family` '
- id: FR-018-AC-19
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The non-test code of `src/oracle/equality/mod.rs` (comments and every `#[cfg(test)]`
    item removed wherever the item sits, string literals the generator emits co
- id: FR-018-AC-20
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: When an admitted FR-330 package contains a `Failed` feature-accounting
    record, its equality claim is refused as `CompositeEqualityRefusal::PackageAccounting`
    wi
- id: FR-018-AC-21
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each generated item's claim-map entry records its declaration closure as
    every declaration's key, form (record, tuple or option), members in declaration
    order w
- id: FR-018-AC-22
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'For every pair of operand values in the item''s refinement domain (FR-028-AC-16:
    every case where the domain fits the case cap, the boundary and seeded cases oth'
- id: FR-018-AC-23
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: For every item of the TC-029 corpus whose leaves are Boolean or bounded
    integers, the claim-map closure's member names, presence and each integer leaf's
    inclusi
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader
    and List/Tree shape assertions; independently authored genuine admitted source
    with thos
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The per-item type-resolution counter charges one unit for each root type-node
    entry or followed type-node reference or bounded-domain `semantic_type` edge,
    incl
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reading the QSpec List `next` field as `binding(next, aggregate([binding(optional,
    reference Option<List>)]))` yields exactly one `FieldDeclaration` named `next
- id: FR-018-AC-27
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reading the QSpec Tree `kids` field's direct reference to `collection_bounds`
    follows that node's `semantic_type` to `Sequence<Tree>`, reads its sole element
    re
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and
    `cg.equality.source_bytes` raised to the measured flat source size, an admitted
    acycli
- id: FR-018-AC-29
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A cycle detected after a charged repeated entry reports `TypeResolutionCycle`
    without following a child; if that repeated entry is the first denied work entry, '
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized
    constructor expression; under the 1,048,576-byte default it returns `SourceT
- id: FR-018-AC-31
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A caller-selected type-resolution work limit of `u64::MAX` refuses as `InvalidTypeResolutionWorkLimit`
    before lowering or charging; with `u64::MAX - 1`, the fir
- id: FR-018-AC-32
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each IR-admitted direct `temporal`/`formula`, `temporal`/`fairness` and
    `expression`/`case` request has one claim-map entry keyed by its requested node
    id and r
- id: FR-018-AC-33
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A genuine root-facade compilation of authentic retained source, original
    dependencies and admitted request limits has equal compiled/context/original package
    id
- id: FR-018-AC-34
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The actual generated ABI executes ordered and unordered enum comparisons
    over all declared cases, including a case absent from every selected literal and
    an enu
- id: FR-018-AC-35
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Unknown declaration, unknown case and a directly or transitively imported
    declaration produce the original typed Unknown/Foreign details at consumed and
    generat
- id: FR-018-AC-36
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emitted source uses only WireNodeId and case identifiers for lookup and
    contains no NodeKey constructor/decoder or generated nominal preimage/hash. Only
    the pre
- id: FR-018-AC-37
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Equality''s generated manifest and native scratch consumers use the one
    profile authority''s direct exact/QSV/replay dependencies, while ordinary Boolean/integer '
- id: FR-018-AC-38
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'With `None`, standalone generation and emitted preparation accept successfully
    reconstructed non-nominal primitive and nested Option/Collection closures; their '
- id: FR-018-AC-39
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`None` at the first reached nominal identity returns typed MissingAdmittedContext
    naming that declaration. Cover left and right source operands, left and right '
- id: FR-018-AC-40
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Some(wrong-package context) fails the whole standalone call after existing
    setting/accounting checks and before item classification, for primitive-only,
    nominal
- id: FR-018-AC-41
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A reached enum still reconstructs every owner-admitted case, including
    unused cases and an enum parameter with no member literal, using the supplied
    genuine key
- id: FR-018-AC-42
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generated oracle invocations under Some or valid context-free None preserve
    private package/item/descriptor/index binding, both check_type guards, same-index
    ch
- id: FR-018-AC-43
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'For genuine source with a: Int[0,9] and b: Integer, the admitted left
    parameter names its integer_range and the right parameter names canonical base
    Integer. Di'
- id: FR-021-AC-1
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Every requested `call` item receives exactly one scheduled or typed-refused
    disposition; a refused item contributes no instruction root, concrete symbol or
    loca
- id: FR-021-AC-2
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For each function-application corpus vector, the oracle's outcome, admitted
    charge sequence and consumed counters equal a direct application independently
    assem
- id: FR-021-AC-3
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Each scheduled item records the requested function name and declaring
    V2 function node id, plus its request-order function index; any independent application
    a '
- id: FR-021-AC-4
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'The oracle returns the arity, wrong-value-kind, dangling-reference and
    unknown-function input refusals unchanged to the caller before any execution charge;
    the '
- id: FR-021-AC-5
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The schedule orders arity validation before per-argument validation, validates
    arguments in declared parameter order, then places `function.call` before body
    wo
- id: FR-021-AC-6
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A function whose declared operator requirements name a capability no registered
    backend can discharge is marked `unsupported` at generation time, naming the cap
- id: FR-021-AC-7
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A tree-shaped 100,000-node V2 `expression`/`call` graph whose nested children
    are separate admitted nodes reached by `reference` produces a flat SSA instruction
- id: FR-021-AC-8
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The schedule marks only linked-package admission as applicable and never
    marks a kernel-mode package applicable. Concrete mode admission through a QSL
    applicati
- id: FR-021-AC-9
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'The schedule preserves a `function.call` charge point and an incomplete
    stop before body work, and the denied charge is not applied; owner-path execution
    of it '
- id: FR-021-AC-10
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A declared parameter or result type reaching a `reference` composite form
    is refused as blocked on quire-spec-language#120, with no schedule root or concrete
    co
- id: FR-021-AC-11
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Model and relation lowering records map to the quire-spec-language#120
    blocker; state, temporal nodes other than directly requested `formula` and `fairness`
    roo
- id: FR-021-AC-12
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: "A declared function whose own body fails to lower \u2014 an unlowerable\
    \ node, a form none of this generator's classifiers admits, or a nested `call`\
    \ naming a functio"
- id: FR-021-AC-13
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Schedule source and claim-map bytes are identical across runs and request
    permutations; entries order by call-node id, declaring function-node id, then
    source a
- id: FR-021-AC-14
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The schedule crate declares `publish = false`, compiles without a Contract
    Runtime `exact` dependency, contains no charge amount or literal outcome/value
    standi
- id: FR-021-AC-15
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Each scheduled call point has one constant-size location record with function
    origin, stable occurrence ordinal, parent ordinal, child ordinal and instruction
    s
- id: FR-021-AC-16
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'The schedule and its claim/location maps neither read nor depend on dynamic
    invocation `location` or `losses` fields. No oracle makes a claim that those fields '
- id: FR-021-AC-17
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Each location record uses the request function binding and function-body
    ordering for its origin; the cross-check against an origin published by a QSL-owned
    che
- id: FR-021-AC-18
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of the function-application
    corpus, no TC-031 step and no oracle-matrix row claims, or presents as evid
- id: FR-021-AC-19
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The emitted schedule crate for scalar, equality and nested-call corpus
    cases contains zero `.unwrap(`, `.expect(`, `unreachable!`, `panic!`, `todo!`
    and `unimpl
- id: FR-021-AC-20
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For each scalar and equality instruction, the target-neutral schedule preserves
    completed, undefined, refused, incomplete and checked-invariant result distincti
- id: FR-021-AC-21
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'A scalar `Negate` body is refused as `UnsupportedOperator` with no schedule
    root, whether or not it reaches a `quire.op.quantity.convert` node (a conversion
    is '
- id: FR-021-AC-22
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: When two or more declarations share one declaring node id, none of them
    appears in the instruction table or in `location-map.json`, and every item naming
    one of
- id: FR-021-AC-23
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For an admitted FR-330 package carrying a `Failed` feature-accounting record,
    a function item retains that exact typed record, its `limit_kind`, `limit`, `consu
- id: FR-021-AC-24
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Items on one `call` node, with equal argument operands, that name functions
    absent from the request's declarations each get their own claim-map entry. Requestin
- id: FR-021-AC-25
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Across one body, the counter charges each node entry, child edge, emitted
    instruction and location record once in source traversal order. A setting equal
    to the
- id: FR-021-AC-26
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The target-neutral schedule's branch/stop edges order a denied left-operand
    charge to stop before the right operand charge, and, after the left completes,
    the r
- id: FR-021-AC-27
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: An admitted nested body consists of separate V2 expression nodes connected
    only by `reference` leaves under QSpec FR-322, with each expression node reached
    from
- id: FR-021-AC-28
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Under the default 1,048,576-byte source limit, a 100,000-instruction body
    that needs more bytes returns `SourceTooLarge` naming the default; raising the
    caller '
- id: FR-021-AC-29
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A 100,000-call linear body produces 100,000 constant-size parent-linked
    location records; requesting its deepest path under sufficient query limits returns
    exac
- id: FR-021-AC-30
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A requested generation work limit of `u64::MAX` returns `InvalidGenerationWorkLimit`
    before lowering or charging; at `u64::MAX - 1` the first denied count is re
- id: FR-021-AC-31
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of the 100,000-node
    case, no TC-031 step and no oracle-matrix row claims, or presents as evidence,
    that
- id: FR-021-AC-32
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Neither the schedule crate nor any concrete oracle requires `quire_contract_runtime::exact`,
    an alias, wrapper or compatibility layer; CG emits no owner-path or
- id: FR-021-AC-33
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: With an IR-admitted `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case`
    node used directly as an item's call-node id, the claim-map entry retains
- id: FR-021-AC-34
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Given one authentic strict-reader-admitted `CheckedPackageV2` of less than
    1,048,576 bytes containing 129 distinct functions with two bound Integer parameters
    e
- id: FR-031-AC-1
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). For each of Add, Subtract and Multiply over a `reject`
    integer type, the generated source of the native oracle calls `exact::evaluate_inte
- id: FR-031-AC-2
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite. For Divide and Remainder
    over a `reject` integer type, the generated source of the native oracle calls
    `exact:'
- id: FR-031-AC-3
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). A native oracle with an arithmetic node returns
    `Outcome<bool>` and takes a trailing `&mut Meter`; an oracle with none returns
    `bool` and '
- id: FR-031-AC-4
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Each of the six comparisons over two arithmetic-free
    operands is emitted as the one native operator the table names, and over the seven
    va
- id: FR-031-AC-5
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Under a `reject` integer type, an add, subtract or
    multiply result outside the type's interval evaluates to `Refused(IntegerOutOfDomain)`,
- id: FR-031-AC-6
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer
    type, a zero divisor evaluates to `Undefined(DivisionByZero)` for both Divide
    and Rem
- id: FR-031-AC-7
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). An add, subtract or multiply over a `saturate` integer
    type is refused with `UnsupportedSaturatingArithmetic` at the node''s span, with
    no '
- id: FR-031-AC-8
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). In a generated crate compiled and executed against
    `quire-contract-runtime`, the outcome of a native add, subtract or multiply oracle
    equa
- id: FR-031-AC-9
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). The first non-completed outcome in evaluation order
    is the oracle''s outcome: an arithmetic stop in the left operand of a short-circuit
    con'
- id: FR-031-AC-10
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). The tri-state harness generator, the bound strategy
    generators and the Kani obligation clause lowering each refuse a clause holding
    an ari
- id: FR-031-AC-11
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED IN CG (IR-596); THE QSL EXEMPLAR RUN IS PENDING. The clause
    that runs QSL's IT-011 and IT-010-SC-05 against this branch is pending the quire-integra
- id: FR-031-AC-12
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Apart from the generated Rust header lines required
    by NFR-002, the oracle generated for an expression with no arithmetic node is
    byte-ide
- id: FR-031-AC-13
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). Generating the oracle of an expression with an arithmetic
    node twice, and from a permuted request, yields identical bytes. This criterion '
- id: FR-031-AC-14
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). The generated source of an oracle with an arithmetic
    node contains no `unwrap`, `expect` or panic macro. This criterion passes on the
    tree
- id: FR-031-AC-15
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). In the native oracle, each of the six comparisons
    that has an arithmetic operand is emitted as the exact-kernel call the table names
    (for '
- id: FR-031-AC-16
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer
    type of `-5..=5`, the minimum divided by negative one is out of domain and the
    minimu
- id: FR-031-AC-17
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. In a generated crate,
    the outcome of a divide or remainder oracle equals a plain-integer model of IR's
    member-
- id: FR-031-AC-18
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602). A divide or remainder node over a `saturate` integer
    type is refused with `UnsupportedSaturatingArithmetic` at the node's span, with
    no artifa
- id: FR-031-AC-27
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite (it cannot land before
    the native division of AC-2 exists). The code `UnsupportedIntegerDivision` does
    not exis
- id: FR-031-AC-28
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-664). The V1 Boolean oracle admits a typed IR integer domain
    or literal only when its value and both inclusive domain endpoints fit i64. A
    minim
- id: FR-031-AC-19
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). The Kani bundle oracle of an expression whose arithmetic
    nodes are add, subtract and multiply renders them as the infix `+`, `-` and `*`
    o
- id: FR-031-AC-20
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). Under real Kani, a harness that calls a bundle oracle
    with unconstrained operands at which IR''s discharge does not hold fails with
    Kani''s '
- id: FR-031-AC-21
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). For each of add, subtract and multiply, the bool
    the Kani bundle oracle returns equals the `bool` inside `Completed` of the native
    oracle '
- id: FR-031-AC-22
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The
    generated source of a native oracle with an arithmetic node aliases `quire_exact`
    as `rt`
- id: FR-031-AC-26
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The
    generated source of a native oracle holds no `quire_contract_runtime::operators`
    path and
- id: FR-031-AC-23
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602). The Kani bundle oracle renders Divide over a `reject`
    type as the infix `/` and Remainder as `wrapping_rem` on `i64`, one operator or
    call per
- id: FR-031-AC-24
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602). Under real Kani, a harness that calls the bundle oracle
    of the construction Z-1 (a divide) with unconstrained operands fails the checks
    "attem
- id: FR-031-AC-25
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite (its native side calls
    the kernel). For divide and for remainder, the bool the Kani bundle oracle returns
    equal
- id: FR-035-AC-1
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: A generated caller-ingress oracle accepts `&TextPayload`, the declared
    `TextType` and `&mut Meter`, calls `rt::admit_text` once, and returns the runtime
    outcome
- id: FR-035-AC-2
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8`
    before admission and leave the admission meter unchanged; `admit_text` itself
    is never cl
- id: FR-035-AC-3
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: 'For each profile, `Text[1,1]` admits or refuses according to the profile
    length of the retained sequence, including `e` plus U+0301 under NFC and NFD;
    a length '
- id: FR-035-AC-4
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real
    Kani harness symbolically ranges over all 21 sequences of zero to two scalars
    from the
- id: FR-035-AC-5
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: In a copy of the generated oracle with its admission call or declared `TextType`
    altered so the oracle accepts a payload the direct runtime call refuses, real
    K
- id: FR-035-AC-6
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: A request outside the declared finite proof class has a typed unsupported
    or requires-bound disposition and no harness, and no caller-ingress claim is marked
    `i
- id: FR-042-AC-1
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: CG's native Boolean, equality, function and pre-check placeholder constructors
    use the exact nine CG-owned causes in the table, retaining `IllTypedCause` from
    `
- id: FR-016-AC-1
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Every retained witness decodes into typed values or is refused as malformed.
- id: FR-016-AC-2
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: An out-of-domain witness is reported as such and never as a contract failure.
- id: FR-016-AC-3
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: An in-domain witness is reported as a failure only when native replay reproduces
    the same typed outcome.
- id: FR-016-AC-4
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A native replay that runs and disagrees with the harness result yields
    a typed mismatch, and never a failure or a replay error.
- id: FR-016-AC-5
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A witness bound to a different harness identity is reported as malformed
    and never replayed.
- id: FR-016-AC-6
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Every witness decode receives required caller-supplied byte and value-count
    bounds at its entry point; replay inputs carry both as non-optional fields with
    no d
- id: FR-016-AC-8
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: 'A witness schema binds the obligation''s persisted argument bindings,
    in the order the obligation persists them, position for position to the harness''s
    symbolic '
- id: FR-016-AC-9
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification whose witness makes the native function evaluate
    the clause to false settles as `reproduced-with-evaluated-witness` with category
    `viola
- id: FR-016-AC-10
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification at which the native function evaluates the clause
    to true settles `inconclusive` with the proved violation and the replayed success
    both
- id: FR-016-AC-11
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: 'The adapter refuses, with a distinct typed error each, a decoded value
    no replay parameter binds, a transcript field holding a delimiter, a transcript
    QSL does '
- id: FR-016-AC-12
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A `replay` call that returns `ReplayRefusal::Fault` is returned as `SpineReplayError::Refused`
    carrying that fault.
- id: FR-016-AC-13
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A `Witness` arm that settles `reproduced-with-evaluated-witness` with a
    category other than `violation` yields a typed mismatch, and never a reproduced
    failure.
- id: FR-016-AC-14
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A proved unit that imports a locked dependency is compiled by `qsl_replay::call_site`
    together with that dependency's lock source, and `qsl_replay::replay` sett
- id: FR-016-AC-15
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: The replay request's `package.dependencies` carries one entry per lock
    dependency selection, in ascending identity order, with the lock's recorded `package_id`.
- id: FR-016-AC-16
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A unit whose import no lock dependency selection supplies is refused at
    the call site.
- id: FR-016-AC-17
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock recording another `package_id` for a dependency the unit imports
    is refused by QSL as a dependency identity mismatch.
- id: FR-016-AC-18
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock selecting a library the unit does not import is refused by QSL as
    unselected.
- id: FR-016-AC-19
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Lock libraries whose sources share a source owner are refused as no dependency
    input, before the call site compiles anything.
- id: FR-016-AC-20
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock library whose source has the unit's own source owner is refused
    by the call site.
- id: FR-016-AC-21
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: 'The function path''s request `obligation_identity` holds the function-contract
    obligation digest of quire-spec-language ADR-013 O-09: the digest of the RFC 8785 '
- id: FR-016-AC-22
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: For two functions with the same parameters, the identities differ. The
    identity of one function is the same across recompiles that only add comments
    or blank li
- id: FR-016-AC-23
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: 'The function-contract identity function derives nothing from the package:
    with the `FunctionSite`''s `function` and `declaration`, the kind and the arguments
    hel'
- id: FR-016-AC-24
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock whose only defect is one library identity selected twice is refused
    by QSL's `DependencyInput::new` as `DuplicateIdentity`, code `invalid_package`
    with c
- id: FR-024-AC-1
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The envelope''s obligation identity is QSL''s `ObligationIdentity` built
    as the ADR-013 O-09 digest by the one scheme of FR-024-AC-20: the subject node
    id (functi'
- id: FR-024-AC-2
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: Every backend-witness transcript the generator passes to `Witness::parse`
    is produced by the one adapter rendering function from decoded values, and that
    functi
- id: FR-024-AC-3
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A counterexample with any value outside its declared domain is reported
    out-of-domain and neither `replay` nor `replay_frame` is called. The endpoints
    of each d
- id: FR-024-AC-4
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'For each `WitnessPacket` member in turn, a packet missing that member
    is refused by `WitnessEnvelope::reconstruct` with QSL''s `MissingMember` naming
    it, and no '
- id: FR-024-AC-6
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A frame counterexample is submitted as a `WitnessEnvelope<FrameCounterexample>`
    through `qsl_replay::replay_frame`, and its clause, frame, anchor and occurrence
- id: FR-024-AC-7
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A reduced `Witness`-arm candidate is retained only when a backend re-run
    and native replay both preserve domain validity and the failure; a candidate that
    chang
- id: FR-024-AC-8
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness`-arm envelope carries its own backend re-run's
    transcript, never the original counterexample's.
- id: FR-024-AC-10
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: No type named `Witness`, `ReplaySource`, `WitnessEnvelope`, `TerminalRecord`
    or `ObligationIdentity` is defined under `src/`, and no source file under `src/`
    im
- id: FR-024-AC-11
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`StateClauseReplay::replay` returns exactly the `StateClauseReplayResult`
    that `qsl_replay::replay_state_clause` returns when a test calls it directly with
    the '
- id: FR-024-AC-12
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The state-clause envelope's `clause_node` equals the `ClauseSite` node,
    and its `occurrence_key` the `ClauseSite` occurrence, that `qsl_replay::call_site`
    retur
- id: FR-024-AC-13
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The pre snapshot holds, for every declared field of the state object, the
    value the Kani playback bound to that field, and the post snapshot holds the post-stat
- id: FR-024-AC-14
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: For a document vector whose members are in non-sorted source order, whose
    text holds characters JSON escapes (a quote, a backslash, a control character,
    a non-A
- id: FR-024-AC-15
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A playback that binds no value for a declared state field returns `StateClauseReplayError::MissingField`
    naming that field, the executor is not called, and no s
- id: FR-024-AC-16
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: For a subject mutated to violate the postcondition, the replay settles
    `reproduced-with-evaluated-witness` with category `violation` and an evaluated
    `false`. F
- id: FR-024-AC-17
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A state field value outside its declared integer range returns `StateClauseReplayError::OutOfDomain`
    naming the field and the executor is not called. The range'
- id: FR-024-AC-18
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: With the installed backend, the falsified operation-contract harness of
    a postcondition state clause over a subject mutated to debit is replayed from
    its real K
- id: FR-024-AC-19
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: An operation that declares a parameter, and one that declares a result,
    each return `StateClauseReplayError::UnsupportedOperationShape` carrying the operation
    a
- id: FR-024-AC-20
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'One obligation-identity function in `src/replay/obligation.rs` mints the
    identity of the function path and of the frame path: the SHA-256 of the RFC 8785
    encodi'
- id: FR-024-AC-21
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The identity changes when any one of these changes alone: the kind, the
    `function` node and the `declaration` key (node, role or ordinal); it does not
    change wh'
- id: FR-024-AC-22
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A frame obligation's identity takes `function` and `declaration` from the
    `OperationSite`'s `frame` and `frame_occurrence`, so they equal the envelope's
    `clause
- id: FR-024-AC-23
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`StateFrameIdentity` and its persisted record carry `state_fields`, every
    field the harness draws in draw order, which `domains` (ranged fields only) and
    `prope'
- id: FR-024-AC-24
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`FrameReplayInputs` has no `obligation_identity` member. The frame request''s
    `obligation_identity` and the envelope''s both equal the identity minted from
    the si'
- id: FR-024-AC-25
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The frame witness is built from the real playback: the playback text decodes
    through the one decoder of `src/replay/witness.rs` against bindings built from
    the '
- id: FR-024-AC-26
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A decoded state-field value outside the field's declared range returns
    `FrameReplayError::OutOfDomain` naming the field and its value, and calls neither
    `call_s
- id: FR-024-AC-27
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The decoded pre-state is tied to the invocation: for every state field,
    the integer the pre snapshot (the provided document whose `sha256-jcs` digest
    the invoca'
- id: FR-024-AC-28
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'A harness whose `scope.operation` is not the operation requested returns
    `FrameReplayError::ScopeMismatch` naming `operation` before `call_site` is called;
    one '
- id: FR-024-AC-29
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The frame envelope's `declared_domains` is the empty list, no source of
    `src/replay/frame.rs` builds a `DeclaredDomain` or a `DomainKey`, and the module's
    heade
- id: FR-024-AC-30
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: With the installed backend, the real playback of the falsified frame harness
    of a subject that writes a forbidden field is replayed through `FrameReplay::new`
    a
- id: FR-024-AC-31
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. For a QSL-emitted package, `StateClauseReplay` takes names and order '
- id: FR-024-AC-32
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A frame harness generated from the package QSL emits for the twin is '
- id: FR-024-AC-33
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A postcondition harness generated from the package QSL emits for the '
- id: FR-024-AC-34
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. With the installed backend, the real playback of the falsified frame '
- id: FR-024-AC-35
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A falsified frame or postcondition harness draws a present model fiel
- id: FR-024-AC-36
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'Two distinct test backend adapters each produce a typed outcome: success
    carries equivalent assignments, each adapter''s backend identity, the same opaque
    obliga'
- id: FR-024-AC-37
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The common intake maps an adapter's decode-refusal arm to a typed decode
    evidence failure retaining its cause, with no envelope or replay. Before native
    replay,
- id: FR-024-AC-38
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: Starting from one admitted original failure, one valid reduced candidate
    that preserves the failure becomes a new CG lineage revision referring to its
    exact adm
- id: FR-024-AC-39
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: From a retained failure revision, an out-of-domain reduction and a domain-valid
    reduction that changes the native verdict each create no revision. After each
    re
- id: FR-024-AC-40
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness` envelope holds a transcript from its own backend
    re-run, bound to its own reduced assignments; it cannot reuse any ancestor's trans
- id: FR-032-AC-1
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    A public consumer passes the routed harness's `ScalarObligationIdentity` dir
- id: FR-032-AC-2
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631). The builder decodes actual routed-harness playback in
    persisted symbolic call order through the Kani output adapter; another harness
    name, mis
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    Each operand at its own lower or upper bound is admitted and the adjacent ou
- id: FR-032-AC-4
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The driver's actual native observation of the proved generated oracle is req
- id: FR-032-AC-5
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer
    conformance). The public consumer executes the same proved generated artifact
    and invokes '
- id: FR-032-AC-6
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    For every `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Inc
- id: FR-032-AC-7
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer
    conformance). QSL''s divergence between generated and authoritative exact outcomes
    becomes '
- id: FR-032-AC-8
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The real QSL-emitted `x + 1` with input `Int[0,9]` and result `Int[0,10]` ex
- id: FR-032-AC-9
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (authoritative metadata access and CG retention).
    Independent canonical-preimage checks include only the claimed application node,
- id: FR-032-AC-10
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family retention and actual
    CG/QSL conformance). When the authoritative scalar kind and retained positional
    metadata
- id: FR-032-AC-11
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention
    and actual CG/QSL route). When the obligation digest differs from the retained
    positional '
- id: FR-032-AC-12
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention
    and actual CG/QSL route). When a correctly minted claim invents an application
    occurrence '
- id: FR-032-AC-13
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and
    CG builder). When the owning mint cannot encode the preimage, the public `parity_oblig
- id: FR-032-AC-14
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (actual CG scalar builder/converter). Adding
    the scalar builder/converter preserves existing function/clause carry-only behavior
    a
- id: FR-033-AC-1
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-635 verified Eq graph-child constructor controls and Ne parameter/parameter
    and parameter/literal controls, including operator mismatch refusal; Ne '
- id: FR-033-AC-2
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Canonical round trips preserve every listed leaf family,
    including beyond-i64 composite integer leaves as ExactInteger (while top-level
    Integer r
- id: FR-033-AC-3
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Noncanonical JCS/tag/scalar/escape spelling, unknown declaration
    or unit/reference identity, wrong enum/union member, missing/extra or wrongly
    pr
- id: FR-033-AC-4
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Public caller input exactly at the configured encoded-byte
    limit is admitted and one byte over refuses before parsing; occurrence/work exhaustion
- id: FR-033-AC-5
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. The retained actual assertion playback reconstructs operand
    values by the FR-025 original-node/leaf bindings; another harness or persisted
    bindin
- id: FR-033-AC-6
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. For falsified replay the driver executes the same proved
    generated artifact and supplies its actual native outcome/count; QSL receives
    this obser
- id: FR-033-AC-7
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public F-7 converter controls; IR-806 original-artifact
    path planned). For an identity-valid claim reaching row F-7, actual QSL exact
    equ
- id: FR-033-AC-8
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. CG submits the original claim's operand declarations and
    literal singleton domains and consumes QSL-derived bound keys without a CG-authored
    decl
- id: FR-033-AC-9
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-666 report binding covered; IR-635 Eq-only verified source/package/node/occurrence
    prechecks and genuine invocation covered; generated-artifact auth
- id: FR-033-AC-10
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Each lawful generated family exercised by the completed
    route has a real verify and falsify control with retained backend evidence and
    mutation-k
- id: FR-033-AC-11
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-635 Eq graph-child positional O-09 controls and Ne parameter/parameter
    and parameter/literal controls; Eq self-comparison is covered; Ne literal/lit
- id: FR-033-AC-12
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-666 direct QSL F-1 precedence controls; IR-635 Eq-only verified
    constructor and QSL membership/unknown-bound refusals before supplied Disagreed;
    fal
- id: FR-033-AC-13
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public F-2 to F-6 controls; IR-806 must reach the
    same rows from a CG-built request over the original proving context). On a valid
    falsif
- id: FR-033-AC-14
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814; literal operands are admitted one-field `Integer` records,
    with bounded-field narrowing literals (IR-691) and collection-constructor literals
    g
- id: FR-033-AC-15
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814). When the original application node is a `structural.ne`
    comparing one parameter with itself and the requested operator is NotEqual, the
    verifi
- id: FR-033-AC-16
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814; verified by Inspection of the single conversion that AC-17
    tests). If the owning operand projection refuses an inline integer literal operand
    (
- id: FR-033-AC-17
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814). If the owning operand projection refuses an inline non-integer
    literal operand (`InlineNonInteger`), then the builder returns that refusal thr
- id: FR-019-AC-1
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Settlement dispatches one arm per variant of the closed backend kind, and
    every variant reaches an arm that settles rather than falling through.
- id: FR-019-AC-2
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Every requested item receives exactly one of `supported`, `requires-bound`,
    `unsupported` and `invalid-request`, settled from its `candidates` and extent
    classi
- id: FR-019-AC-3
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: On the Kani arm, an unbounded extent against a `bounded`-only advertisement
    settles `requires-bound` when a finite bound is available, rather than `unsupported`
- id: FR-019-AC-4
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An item with more than one candidate and no named backend settles `invalid-request`
    with `invalid_capability`/`ambiguous-backend`, naming every candidate in can
- id: FR-019-AC-10
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Only an item settled `supported` routes.
- id: FR-019-AC-45
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A manifest with a second descriptor of one backend identity causes a whole-envelope
    `EnvelopeRefusal::ManifestRepeatsBackendIdentity { identity: String }`, whet'
- id: FR-019-AC-46
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: With several repeated identities, the refusal names the bytewise-first
    repeated identity independent of manifest order; identities differing only in
    case are di
- id: FR-019-AC-47
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: QSL's registry snapshot holds each backend identity at most once (FR-290:258-265;
    FR-331:57), so no driver-built FR-331 manifest repeats an identity.
- id: FR-019-AC-48
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A `Refused(refusal)` outcome from the current Kani oracle-function preflight
    read produces `Err(EnvelopeRefusal::ContractPackageReadRefused { refusal: ContractP'
- id: FR-019-AC-49
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An `Incomplete(incomplete)` outcome from the current Kani oracle-function
    preflight read produces `Err(EnvelopeRefusal::ContractPackageReadIncomplete {
    incomple
- id: FR-019-AC-50
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An `Admitted` Kani function-application preflight read whose exact generator
    produces a claim for an otherwise supported item retains the existing `supported`
    s
- id: FR-019-AC-51
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Among several request items with non-`Admitted` preflight outcomes, the
    first failing item in manifest/item request order determines the whole-request
    `Contract
- id: FR-019-AC-52
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'An `Admitted` Kani function-application read whose exact generator emits
    no claim, and a structural preflight rejection before any read, each retain item-level '
- id: FR-019-AC-53
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Given an admitted package with bound driver items at driver indexes 0 and
    2 around an excluded accounting-only position at index 1 whose accounting is `lowered`
- id: FR-019-AC-7
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An envelope whose `capability_vocabulary` is not exactly `quire.capability-kind/v1`,
    or is absent, is refused as `invalid_capability`/`unsupported-version` with
- id: FR-019-AC-8
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An item with an absent extent classification settles `invalid-request`
    with `invalid_capability`/`absent-extent`, and an absent or unknown kind settles
    before t
- id: FR-019-AC-9
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A backend kind added without a negotiation arm does not compile: the dispatch
    is an exhaustive `match` over the closed kind with no catch-all arm.'
- id: FR-019-AC-11
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). `BackendKind` has `Process(id)` beside `Kani`; `ALL`
    lists finite built-in kinds only, while CG enumerates process candidates from
    descriptors
- id: FR-019-AC-12
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; non-empty bounded rows depend on QSL-654's producer and
    admit-side enforcement). A bounded process item with an uncovered `extent.bounds[].kind
- id: FR-019-AC-16
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). An unbounded process item settles `supported` when its
    descriptor advertises `unbounded` for its kind, regardless of whether any `extent.domai
- id: FR-019-AC-17
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded
    item settles `requires-bound` exactly when `finite_bound_available` is true and
    every bou
- id: FR-019-AC-18
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). Given the same process advertisements and item extent,
    changing only backend identity, manifest position, manifest run-limit defaults
    or ambie
- id: FR-019-AC-19
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; bounded row depends on QSL-654's producer). The process
    arm reads each explicit `extent.bounds[].kind` and never derives a domain kind
    from `Do
- id: FR-019-AC-20
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; driver projection remains paired work). CG's process descriptor
    retains the FR-331 manifest's advertised (kind, mode) pairs, `domains` and opti
- id: FR-019-AC-42
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Two otherwise equal CG Process descriptors with absent and present-empty
    manifest `bounds` remain distinct typed values. Changing only between those forms
    leave
- id: FR-019-AC-21
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded
    item with any unadvertised boundable `extent.domains[].kind` settles `unsupported`,
    warne
- id: FR-019-AC-22
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded
    item with every boundable `extent.domains[].kind` advertised and the item's `finite_bound
- id: FR-019-AC-23
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; bounded row depends on QSL-654). Changing only a bounded
    item's proof-bound numeric maximum leaves its process-provider `supported` or
    `unsuppo
- id: FR-019-AC-13
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'Settling a process-provider item reaches no plugin: a descriptor whose
    identity names a non-existent executable settles identically to one with an ordinary
    iden'
- id: FR-019-AC-14
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'The process-provider arm''s return type is `Disposition`, which has no
    terminal-value, verification-result or artifact member, so settling returns none
    of them. '
- id: FR-019-AC-15
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-633). CG's `BackendDescriptor` has a typed `ProviderOrigin`
    projecting exactly QSL layer R's `Linked` and `Process` meanings (QSL FR-288,
    ADR-029 PV
- id: FR-019-AC-24
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'PLANNED (IR-338). Given QSL FR-335''s `value-validity` claim rooted at
    an unbounded `s: Set<Int[0,9999]>`, an empty candidate registry settles `unsupported`
    with'
- id: FR-019-AC-25
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'The normative Kani form set is the complete V1 proof set defined by IR-269
    and FR-015: scalar families Boolean, integer, rational, decimal, IEEE float, text,
    en'
- id: FR-019-AC-26
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For any form in FR-019-AC-25 with one validated Kani candidate, a bounded
    extent settles `supported` exactly when the form's FR-015 lowering succeeds and
    the ca
- id: FR-019-AC-27
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: With one Kani candidate, a typed form outside the normative set in FR-019-AC-25
    settles `unsupported`, warned, with `unsupported_projection`/`unsupported-reques
- id: FR-019-AC-33
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For a valid bounded typed form inside FR-019-AC-25 that matches one specific
    Planned subpath in TC-030's status register and does not match an Implemented
    subpa
- id: FR-019-AC-28
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: A form refusal under FR-019-AC-27 is distinct from an empty candidate set,
    which retains the missing-capability cause, and from multiple candidates without
    a na
- id: FR-019-AC-29
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: After request and candidate/manifest validation select one Kani candidate,
    Kani checks the typed IR form before its extent and advertised-mode rules; if
    both th
- id: FR-019-AC-30
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For the same typed IR tag/form, changing only its display or debug text
    leaves Kani's disposition, cause and named form identity unchanged.
- id: FR-019-AC-31
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For a Kani-supported unbounded form against a `bounded`-only advertisement,
    `finite_bound_available=true` settles `requires-bound`, with no substituted bound
    an
- id: FR-019-AC-32
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: When a caller supplies a finite bound in a new bounded request for a Kani-supported
    form, negotiation settles `supported` and preserves the exact supplied proof
- id: FR-019-AC-34
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'PLANNED (IR-516). With one EN-1 candidate, an advertised admitted TP-1
    through TP-4, TP-5, SG-1 through SG-3, or HP-1 model/HP-2/HP-3/HP-6 form settles
    through '
- id: FR-019-AC-35
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With one SMT temporal candidate advertising the item's
    kind and form, TP-1 through TP-4 route only to an EN-2 or EN-3 method ADR-018
    assigns t
- id: FR-019-AC-36
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). A TP-5 whole-refinement `temporal-satisfaction` item
    retains its safety and optional liveness halves on the concrete subject through
    EN-1, and
- id: FR-019-AC-37
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). HP-1 model, HP-2, HP-3 and HP-6 reach an advertising
    EN-1 arm; only HP-2 reaches an advertising SMT arm under its applicable EN-2 or
    EN-3 meth
- id: FR-019-AC-38
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). An advertised SG-1, SG-2 or SG-3 item reaches EN-1 with
    its recorded form; only advertised SG-1 reaches an SMT arm. SG-2 or SG-3 against
    an SM
- id: FR-019-AC-39
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). A selected reduction reaches only a candidate advertising
    that exact reduction, with the selection preserved in the routed request. Symmetry
    d
- id: FR-019-AC-40
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'PLANNED (IR-516). For an item over a weak `parallel`, a candidate advertising
    the subject''s resolved `tso` or `ra` model receives that exact model; a candidate '
- id: FR-019-AC-41
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With otherwise equal form and kind advertisement, model
    and protocol subject classes are read separately; an arm that discharges only
    one clas
- id: FR-019-AC-43
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Two driver items selecting distinct `expression` occurrences of one checked
    `integer.add` node and different direct `numeric.narrow` occurrences in one admitted
- id: FR-019-AC-44
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For a V2 scalar claim, an absent `OwnResult`, an application occurrence
    absent from the admitted source, a conversion occurrence from another source or
    unrelate
- id: FR-022-AC-1
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Generation dispatches one arm per variant of the closed `BackendKind` through
    an exhaustive `match` with no catch-all, and `GenerationContexts::has` handles
    eac
- id: FR-022-AC-2
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: For a set of routed Kani items, each record and harness in the output equals
    what `negotiate_kani_obligations` returns for the same items in ascending request-i
- id: FR-022-AC-3
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The entry point accepts no backend registry, candidate set, extent or capability
    kind, and constructs no FR-019 `Disposition`.
- id: FR-022-AC-4
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629 internal-invariant test migration). A crate-internal malformed
    routed Kani item whose private backend identity has no CG built-in kind, or conve
- id: FR-022-AC-5
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Two routed items with one request index refuse as `DuplicateRequestIndex`
    naming it. A routed Kani item without Kani context refuses as `MissingKindContext`
    nam
- id: FR-022-AC-6
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: A Kani group-level refusal (for example an unwind outside `1..=1024`, or
    an unparsable subject path) is returned as `Kani` carrying the unchanged `KaniObligatio
- id: FR-022-AC-7
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'A Kani group containing an `invalid_request` item is listed in `rejected`,
    and every Kani item''s record is returned with no harness. A `DuplicateItem` reported '
- id: FR-022-AC-8
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Every routed item appears exactly once in `items`, in ascending request
    index, under its routed backend and the `KindOutput` variant of its routed kind.
    An item
- id: FR-022-AC-9
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Regeneration from equal inputs is byte-identical, and a permutation of
    the routed-item slice yields an identical result. The same node routed at a different
    req
- id: FR-022-AC-10
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'The Kani arm derives each item''s operation and domain from the package
    alone; the caller supplies no claim map, and `x + 1` over a parameter `Int[0,
    9]` routes '
- id: FR-022-AC-11
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: An item with no derivable descriptor keeps its typed refusal and no harness,
    and its siblings' records and harnesses are unaffected. On the FR-330 package
    path,
- id: FR-022-AC-12
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: '`RoutedGeneration.claim_map` is `Some` after a Kani group and contains
    one claim per selected node/occurrence admitted past FR-039 accounting, retaining
    any ear'
- id: FR-022-AC-13
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The Kani arm routes an integer node over bounded parameters to a supported
    harness whose arguments are one range per operand and whose result assertion uses
    the
- id: FR-022-AC-14
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: '`RoutedGeneration.oracle_artifacts` is `Some` after a Kani group and `None`
    when nothing is routed. Its `claim-map.json` equals the complete package-derived
    cla'
- id: FR-022-AC-15
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'The Kani arm routes the packages QSL emits for `x + 1` over `x: Int[0,
    9]` into `Int[0, 10]`, `x + y` over `Int[0, 9]` and `Int[10, 20]` into `Int[10,
    29]`, and'
- id: FR-022-AC-16
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Two Kani harnesses with one `harness_symbol` refuse the call as `DuplicateHarness`
    naming the second harness's `module::harness` path, and no harness is overwri
- id: FR-022-AC-17
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). A routed `Process(id)` item appears exactly once with
    its own identity and empty `KindOutput::Process`, without a generation context
    or member
- id: FR-022-AC-18
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). A crate-internal routed `Process(id)` item whose private
    backend and kind disagree refuses the whole call as `BackendKindDisagrees` with
    `conv
- id: FR-022-AC-19
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). Permuting a routed-item slice containing Kani and multiple
    process identities leaves its request-index-ordered output unchanged.
- id: FR-022-AC-20
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `RoutedGenerationItem.backend` and `.kind` are private
    while `request_index` and `node_id` remain public; external callers cannot create
    an it
- id: FR-022-AC-21
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `from_descriptor` constructs Kani for a linked `kani`
    descriptor and `Process(id)` for a process descriptor with the same identity text.
    An un
- id: FR-022-AC-22
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `from_descriptor` rejects a backend identity different
    from the descriptor identity as `DescriptorBackendMismatch`, naming both; it rejects
    an
- id: FR-022-AC-23
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: After FR-019 authenticates two `integer.add` driver items on one admitted
    ContractPackage with distinct selected application/direct-`numeric.narrow` pairs,
    rout
- id: FR-022-AC-24
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: A routed scalar item's retained authenticated selection and its driver
    lowering position must name the same application/conversion pair in the same admitted
    Con
- id: FR-022-AC-25
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The public CG API exposes the concrete `ContractScalarError` carried by
    `RoutedGenerationError::ContractScalar`, so an external driver can match `ContractScalar
- id: FR-022-AC-26
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Given the checked QSL RR-16 package with two `x + 1` occurrences of one
    application node, directly narrowed to `Int[0, 10]` and `Int[0, 20]`, the driver
    supplie
- id: FR-022-AC-27
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'For `if p then x + 1 else x + 1`, each occurrence''s generated scalar
    claim is conditional on its own QSL path guard: the same guard occurrence with
    `holds=true`'
- id: FR-022-AC-28
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: If a routed scalar item's occurrence has no entry in the supplied requirement
    records, its key's node disagrees with the authenticated application selection,
    or
- id: FR-022-AC-29
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Given an admitted package whose complete lowering sequence has bound driver
    items at indexes 0 and 2 and an excluded accounting-only position at index 1,
    a rout
- id: FR-026-AC-1
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: The Kani adapter implements the adapter trait, and a function generic over
    that trait reaches the Kani adapter's generation arm, execution, transcript parser
    an
- id: FR-026-AC-4
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: The execution evidence type is an associated type of the adapter trait,
    and the Kani adapter's is `KaniExecutionEvidence`.
- id: FR-026-AC-5
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: PLANNED (IR-629). `Process(id)` has no CG adapter trait implementation
    or CG execution path; an inspection of all adapter implementations and execution
    dispatch
- id: FR-002-AC-1
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Rejected inputs remain distinct from passing cases in generated reports.
- id: FR-002-AC-2
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Boundary strategies exercise immediately inside and outside supported constraints.
- id: FR-002-AC-3
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Supported correlated constraints do not rely on improbable filtering.
- id: FR-002-AC-4
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Shrinking preserves generated constraints or records a residual rejection.
- id: FR-002-AC-5
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: The generated campaign runner owns execution, explicit-discard accounting,
    and conclusion, returns a typed terminal result, and rejects campaigns below either
    c
- id: FR-002-AC-6
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Integer and generated Boolean harness campaign cases bind exact values
    to executable accepted/rejected expectations; enum cases execute an all-admitted
    membersh
- id: FR-002-AC-7
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For one precondition and one postcondition selected from
    an admitted `CheckedPackageV2`, the generated harness embeds each selected clause's
    F
- id: FR-002-AC-8
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). In the V2 harness, an instrumented snapshot, precondition
    oracle, subject and postcondition oracle record that the snapshot precedes precondit
- id: FR-002-AC-9
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 harness generator opens
    a `CheckedPackageV2` body only through the `core/ir/` layer of AD-004, and names
    no `Bo
- id: FR-002-AC-10
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause identity that is unknown, of the wrong
    clause kind, or the same for both selections yields a typed, located diagnostic
    namin
- id: FR-002-AC-11
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause body outside the FR-014-AC-38 shape
    yields a structured unsupported diagnostic and no artifact, and the generator
    emits no f
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Repeated V2 harness generation for the same admitted
    package, selected clause identities and campaign policy is byte-identical. The
    requiremen
- id: FR-002-AC-13
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which both oracles complete true
    runs the subject once and is a pass.
- id: FR-002-AC-14
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    false is a rejected precondition and the subject is not invoked.
- id: FR-002-AC-15
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    true and the postcondition oracle completes false is a failed postcondition.
- id: FR-002-AC-16
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition or postcondition
    oracle returns an outcome that does not complete (stopped or undefined) is reporte
- id: FR-002-AC-17
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not complete does not invoke the subject.
- id: FR-002-AC-18
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not complete does not evaluate the postcondition oracle.
- id: FR-008-AC-1
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: A fresh QSL-produced, IR-admitted ConfigVersion V2 postcondition with direct
    `self.versionNumber = pre(self.versionNumber)` yields two `State` census entries
    in
- id: FR-008-AC-2
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: An IR-admitted V2 `amount < 7` invariant with a typed current read of range
    0..=1000 yields operator `Less`, primary `amount`, literal 7 and that inclusive
    cens
- id: FR-008-AC-3
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: A valid selected comparison generates despite a comparison-oracle-refused
    sibling claim in the same package; requesting the refused claim returns its typed
    sele
- id: FR-008-AC-4
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: 'Unknown clause, absent claim, unsupported clause kind and malformed comparison
    report distinct typed causes in the stated first-defect order. Diagnostics carry '
- id: FR-008-AC-5
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: A request with several applicable defects returns the first only. Every
    refusal returns zero artifacts; a successful request returns the complete source,
    census
- id: FR-008-AC-6
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: Either read endpoint or a literal at i64::MIN - 1, i64::MAX + 1, i128::MIN
    or i128::MAX refuses `UnsupportedRelation` at that operand without narrowing.
    Two ind
- id: FR-008-AC-7
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Beyond the per-operator agreement stated in FR-008-AC-3, which
    this criterion does not repeat, for each of integer.eq, ne, lt, le, gt and ge
    the select
- id: FR-008-AC-8
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. The V2 comparison oracle takes only the accessor's typed result
    for the selected claim, and no V1 bound oracle or `BoundPackage` is an input to
    it.
- id: FR-008-AC-9
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. A CG integration test calls IR's public clause-context accessor
    through its public API only, and the generator's census, ordered relation and
    diagnosti
- id: FR-008-AC-10
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Source inspection shows the generator reads no `CheckedSemanticNodeV2.body`
    field and decodes no body JSON (the verification of FR-008-CON-1).
- id: FR-008-CON-1
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: 'The generator SHALL read V2 operand identity, provenance, observation,
    operator and bounds only through IR''s public typed accessor. A private decoder
    or copied '
- id: FR-008-CON-2
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: Admission SHALL finish before any strategy, population, census or runner
    source is published; a refusal returns no artifact or partial bundle.
- id: FR-008-CON-3
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL check each read endpoint for i64 representability before
    constructing the shared-domain relation.
- id: FR-008-CON-4
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL check each comparison literal for i64 representability
    before conversion.
- id: FR-009-AC-1
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: For the QSL-produced, IR-admitted V2 `VersionUnchanged` selected claim
    (FR-008-AC-1) over 0..=1000, every `Satisfying` case has post = pre, every `Violating`
    ca
- id: FR-009-AC-2
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: For every V2 comparison operator, every domain of 1, 2, 3, and 4 members,
    and both a two-read relation and a read-versus-literal relation with the literal
    at ea
- id: FR-009-AC-3
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: 'Generated population source contains no filter, assume, reject, or discard
    call, and a seeded 10,000-case run of each population records zero discards and
    zero '
- id: FR-009-AC-4
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: For `amount < 0` over 0..=1000, `Satisfying` and `Broad` are refused with
    `EmptyPopulation` naming `Satisfying`, and `Violating` generates; for `amount
    >= 0`, `
- id: FR-009-AC-5
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: A domain of `i64::MIN..=i64::MAX` with `NotEqual`, `Less`, and `Equal`
    compiles as generated Rust and generates 10,000 seeded cases without overflow
    or panic, e
- id: FR-009-AC-6
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: Repeated generation for the same `CheckedPackageV2`, selected clause id,
    authentic claim occurrence, and population is byte-identical.
- id: FR-009-AC-7
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: PLANNED. For each noncommutative operator, a comparison written with the
    literal in the first position produces the same satisfying and violating sets
    as its mi
- id: FR-010-AC-1
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: For `amount < 7` over 0..=1000, the in-domain census is exactly [0 Holds,
    1 Holds, 6 Holds, 7 Violated, 8 Violated, 999 Violated, 1000 Violated], and the
    out-of
- id: FR-010-AC-2
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: For the QSL-produced, IR-admitted V2 `VersionUnchanged` selected claim
    (primary post, partner pre; FR-008-AC-1) over 0..=1000, the in-domain census is
    exactly t
- id: FR-010-AC-3
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: 'A selected claim whose accessor domain is exactly `i64::MIN..=i64::MAX`
    (representable under FR-008-AC-6) compiles as generated Rust without overflow
    or panic, '
- id: FR-010-AC-4
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: '`amount <= 1000` over 0..=1000 refuses `Boundary` with `UnsupportedCampaignConstraint`
    and emits no artifact for that request, while its `Satisfying` bundle sti'
- id: FR-010-AC-5
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: Every in-domain census tag equals an independent evaluation of the relation,
    every out-of-domain case has at least one value outside the domain, and repeated
    ge
- id: FR-011-AC-1
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: Over V2 selected claims, a QSL-produced `VersionUnchanged` (`Postcondition`)
    `Broad` campaign records `Holds` cases as `Passed` and `Violated` cases as `FailedP
- id: FR-011-AC-2
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: A V2 comparison oracle deliberately swapped for its negation fails each
    campaign with `ConformanceMismatch` carrying the case values, expected tag, and
    observed
- id: FR-011-AC-3
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: The census runner evaluates each of the 10 `VersionUnchanged` in-domain
    census cases exactly once, in census order, and evaluates none of the out-of-domain
    case
- id: FR-011-AC-4
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: A campaign over a report seeded with fixed prior counts returns `discard_rate()
    == Some((discarded, attempted))` and `rejection_rate() == Some((rejected, attemp
- id: FR-011-AC-5
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: 'A fresh 10,000-case `Broad` campaign for each fixture returns `discard_rate()
    == Some((0, attempted))` with `attempted > 0`, passes a zero discard ceiling,
    and '
- id: FR-012-AC-1
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: For every V2 comparison operator and every domain of 1 to 4 members from
    FR-009-AC-2, an exhaustive walk of every valid `simplify` and `complicate` path
    of `Sat
- id: FR-012-AC-2
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: For 1,000 seeded `VersionUnchanged` value trees of each population over
    0..=1000, every candidate on the greedy shrink path stays in domain and keeps
    its tag.
- id: FR-012-AC-3
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: A `Violating` V2 `VersionUnchanged` campaign whose comparison oracle is
    replaced by `post == pre
- id: FR-012-AC-4
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: After a shrinking campaign, `attempted` equals the number of oracle evaluations
    the runner performed, including shrink replays.
- id: FR-013-AC-1
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: A consumer fixture crate that includes the generated Rust source and whose
    manifest depends only on `proptest` and `quire-contract-runtime` compiles under
    denie
- id: FR-013-AC-2
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: The generated bundle contains no serialized case, census, or summary file,
    and the change adds no file under `schemas/`.
- id: FR-013-AC-4
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: The generated Rust header carries the selected checked clause id and authentic
    claim occurrence, and generating two distinct selected claims from one package
    pr
- id: FR-013-AC-5
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: interface-001 `bound_strategy_slice.consumer` records the case, census,
    and runner surface a downstream consumer compiles against.
- id: NFR-004-AC-1
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Fresh 10,000-case and 256-case campaigns of every population over the V2
    selected claims `VersionUnchanged`, `amount < 7`, and a `Precondition` fixture
    record z
- id: NFR-004-AC-2
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: For every admitted fixture in TC-019, the in-domain census plus out-of-domain
    array holds at most 20 cases.
- id: NFR-004-M-1
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Discarded / attempted on a fresh 10,000-case campaign per population and
    fixture
- id: NFR-004-M-2
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Proptest global rejects on the same campaigns
- id: NFR-004-M-3
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Campaigns ending `Exhausted` on the same campaigns, and on 256-case default-config
    campaigns
- id: NFR-004-M-4
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: In-domain plus out-of-domain census cases per admitted clause

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Inherited FR-005-AC-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/core/functional/FR-005-cli-conformance.md:49 |
| FND-002 | medium | Inherited FR-039-AC-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/core/functional/FR-039-contract-package-consumption.md:213 |
| FND-003 | medium | Inherited NFR-001-M-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/core/non-functional/NFR-001-reproducible-atomic.md:1 |
| FND-004 | medium | Inherited NFR-002-M-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/core/non-functional/NFR-002-provenance-boundary.md:1 |
| FND-005 | medium | Inherited NFR-002-M-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/core/non-functional/NFR-002-provenance-boundary.md:1 |
| FND-006 | medium | Inherited FR-004-AC-6 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-004-vacuity-evidence.md:186 |
| FND-007 | medium | Inherited FR-004-AC-8 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-004-vacuity-evidence.md:188 |
| FND-008 | medium | Inherited FR-004-AC-10 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-004-vacuity-evidence.md:190 |
| FND-009 | medium | Inherited FR-004-AC-11 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-004-vacuity-evidence.md:191 |
| FND-010 | medium | Inherited FR-004-AC-12 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-004-vacuity-evidence.md:192 |
| FND-011 | medium | Inherited FR-004-AC-13 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-004-vacuity-evidence.md:193 |
| FND-012 | medium | Inherited FR-004-AC-14 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-004-vacuity-evidence.md:194 |
| FND-013 | medium | Inherited FR-004-AC-15 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-004-vacuity-evidence.md:195 |
| FND-014 | medium | Inherited FR-004-AC-18 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-004-vacuity-evidence.md:198 |
| FND-015 | medium | Inherited FR-036-AC-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-036-source-obligation-census.md:140 |
| FND-016 | medium | Inherited FR-036-AC-2 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-036-source-obligation-census.md:141 |
| FND-017 | medium | Inherited FR-036-AC-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-036-source-obligation-census.md:142 |
| FND-018 | medium | Inherited FR-036-AC-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-036-source-obligation-census.md:143 |
| FND-019 | medium | Inherited FR-036-AC-5 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-036-source-obligation-census.md:144 |
| FND-020 | medium | Inherited FR-036-AC-6 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-036-source-obligation-census.md:145 |
| FND-021 | medium | Inherited FR-036-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/evidence/functional/FR-036-source-obligation-census.md:146 |
| FND-022 | medium | Inherited FR-015-AC-19 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:755 |
| FND-023 | medium | Inherited FR-015-AC-21 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:757 |
| FND-024 | medium | Inherited FR-015-AC-22 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:39 |
| FND-025 | medium | Inherited FR-015-AC-23 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:469 |
| FND-026 | medium | Inherited FR-015-AC-24 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:760 |
| FND-027 | medium | Inherited FR-015-AC-30 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:766 |
| FND-028 | medium | Inherited FR-015-AC-31 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:767 |
| FND-029 | medium | Inherited FR-015-AC-32 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:768 |
| FND-030 | medium | Inherited FR-015-AC-38 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:38 |
| FND-031 | medium | Inherited FR-015-AC-39 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:402 |
| FND-032 | medium | Inherited FR-015-AC-40 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:406 |
| FND-033 | medium | Inherited FR-015-AC-41 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:406 |
| FND-034 | medium | Inherited FR-015-AC-42 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:413 |
| FND-035 | medium | Inherited FR-015-AC-43 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:416 |
| FND-036 | medium | Inherited FR-015-AC-44 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:41 |
| FND-037 | medium | Inherited FR-015-AC-45 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:41 |
| FND-038 | medium | Inherited FR-015-AC-46 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:424 |
| FND-039 | medium | Inherited FR-015-AC-47 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:426 |
| FND-040 | medium | Inherited FR-015-AC-48 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:429 |
| FND-041 | medium | Inherited FR-015-AC-49 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:39 |
| FND-042 | medium | Inherited FR-015-AC-51 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:442 |
| FND-043 | medium | Inherited FR-015-AC-52 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:445 |
| FND-044 | medium | Inherited FR-015-AC-56 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:453 |
| FND-045 | medium | Inherited FR-015-AC-57 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:458 |
| FND-046 | medium | Inherited FR-015-AC-69 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:55 |
| FND-047 | medium | Inherited FR-015-AC-70 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:680 |
| FND-048 | medium | Inherited FR-015-AC-71 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:683 |
| FND-049 | medium | Inherited FR-015-AC-72 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:686 |
| FND-050 | medium | Inherited FR-015-AC-73 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:58 |
| FND-051 | medium | Inherited FR-015-AC-74 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:690 |
| FND-052 | medium | Inherited FR-015-AC-75 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:692 |
| FND-053 | medium | Inherited FR-015-AC-76 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:55 |
| FND-054 | medium | Inherited FR-015-AC-80 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:816 |
| FND-055 | medium | Inherited FR-015-AC-84 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:820 |
| FND-056 | medium | Inherited FR-015-AC-85 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:821 |
| FND-057 | medium | Inherited FR-015-AC-87 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:823 |
| FND-058 | medium | Inherited FR-015-AC-89 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:825 |
| FND-059 | medium | Inherited FR-015-AC-90 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:826 |
| FND-060 | medium | Inherited FR-015-AC-91 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:827 |
| FND-061 | medium | Inherited FR-015-AC-92 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:828 |
| FND-062 | medium | Inherited FR-015-AC-93 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:829 |
| FND-063 | medium | Inherited FR-015-AC-95 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:88 |
| FND-064 | medium | Inherited FR-015-AC-98 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:138 |
| FND-065 | medium | Inherited FR-015-AC-105 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:841 |
| FND-066 | medium | Inherited FR-015-AC-106 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:104 |
| FND-067 | medium | Inherited FR-015-AC-107 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:843 |
| FND-068 | medium | Inherited FR-015-AC-108 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:844 |
| FND-069 | medium | Inherited FR-015-AC-110 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:87 |
| FND-070 | medium | Inherited FR-015-AC-111 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:87 |
| FND-071 | medium | Inherited FR-015-AC-117 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:831 |
| FND-072 | medium | Inherited FR-015-AC-119 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:373 |
| FND-073 | medium | Inherited FR-015-AC-120 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:856 |
| FND-074 | medium | Inherited FR-015-AC-121 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:857 |
| FND-075 | medium | Inherited FR-015-AC-122 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-015-bounded-kani-obligations.md:858 |
| FND-076 | medium | Inherited FR-017-AC-7 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-017-kani-execution-evidence.md:859 |
| FND-077 | medium | Inherited FR-017-AC-19 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-017-kani-execution-evidence.md:868 |
| FND-078 | medium | Inherited FR-017-AC-26 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-017-kani-execution-evidence.md:875 |
| FND-079 | medium | Inherited FR-017-AC-27 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-017-kani-execution-evidence.md:650 |
| FND-080 | medium | Inherited FR-017-AC-30 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-017-kani-execution-evidence.md:879 |
| FND-081 | medium | Inherited FR-017-AC-47 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-017-kani-execution-evidence.md:896 |
| FND-082 | medium | Inherited FR-017-AC-52 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-017-kani-execution-evidence.md:901 |
| FND-083 | medium | Inherited FR-017-AC-53 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-017-kani-execution-evidence.md:902 |
| FND-084 | medium | Inherited FR-017-AC-54 computes untagged at exact base and candidate; owner IR-877, actual decisions 0b9a2853-962b-4073-8474-03b0d5986ce3 and 4227b5e7-0d5e-43fc-8162-952c1dc0f7af. No gap closure or execution is claimed. | spec/kani/functional/FR-017-kani-execution-evidence.md:903 |
| FND-085 | medium | Inherited FR-017-AC-55 computes untagged at exact base and candidate; owner IR-877, actual decisions 0b9a2853-962b-4073-8474-03b0d5986ce3 and 4227b5e7-0d5e-43fc-8162-952c1dc0f7af. No gap closure or execution is claimed. | spec/kani/functional/FR-017-kani-execution-evidence.md:904 |
| FND-086 | medium | Inherited FR-017-CON-1 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-017-kani-execution-evidence.md:848 |
| FND-087 | medium | Inherited FR-025-AC-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-025-generated-subject-abi.md:94 |
| FND-088 | medium | Inherited FR-025-AC-2 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-025-generated-subject-abi.md:95 |
| FND-089 | medium | Inherited FR-025-AC-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-025-generated-subject-abi.md:96 |
| FND-090 | medium | Inherited FR-025-AC-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-025-generated-subject-abi.md:97 |
| FND-091 | medium | Inherited FR-025-AC-5 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-025-generated-subject-abi.md:98 |
| FND-092 | medium | Inherited FR-025-AC-6 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-025-generated-subject-abi.md:99 |
| FND-093 | medium | Inherited FR-025-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-025-generated-subject-abi.md:100 |
| FND-094 | medium | Inherited FR-025-AC-8 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-025-generated-subject-abi.md:101 |
| FND-095 | medium | Inherited FR-025-AC-9 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-025-generated-subject-abi.md:102 |
| FND-096 | medium | Inherited FR-028-AC-5 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:206 |
| FND-097 | medium | Inherited FR-028-AC-6 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:207 |
| FND-098 | medium | Inherited FR-028-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:208 |
| FND-099 | medium | Inherited FR-028-AC-8 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:209 |
| FND-100 | medium | Inherited FR-028-AC-9 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:210 |
| FND-101 | medium | Inherited FR-028-AC-13 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:33 |
| FND-102 | medium | Inherited FR-028-AC-14 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:81 |
| FND-103 | medium | Inherited FR-028-AC-15 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:143 |
| FND-104 | medium | Inherited FR-028-AC-16 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:70 |
| FND-105 | medium | Inherited FR-028-AC-17 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:81 |
| FND-106 | medium | Inherited FR-028-AC-18 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:84 |
| FND-107 | medium | Inherited FR-028-AC-19 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:163 |
| FND-108 | medium | Inherited FR-028-AC-20 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:168 |
| FND-109 | medium | Inherited FR-028-AC-22 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:221 |
| FND-110 | medium | Inherited FR-028-AC-23 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:178 |
| FND-111 | medium | Inherited FR-028-AC-24 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:33 |
| FND-112 | medium | Inherited FR-029-AC-17 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:377 |
| FND-113 | medium | Inherited FR-029-AC-19 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:379 |
| FND-114 | medium | Inherited FR-029-AC-21 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:381 |
| FND-115 | medium | Inherited FR-029-AC-22 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:382 |
| FND-116 | medium | Inherited FR-029-AC-23 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:383 |
| FND-117 | medium | Inherited FR-029-AC-24 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:384 |
| FND-118 | medium | Inherited FR-029-AC-25 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:385 |
| FND-119 | medium | Inherited FR-029-AC-26 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:386 |
| FND-120 | medium | Inherited FR-029-AC-27 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:387 |
| FND-121 | medium | Inherited FR-029-AC-30 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:207 |
| FND-122 | medium | Inherited FR-029-AC-31 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:391 |
| FND-123 | medium | Inherited FR-030-AC-19 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:237 |
| FND-124 | medium | Inherited FR-030-AC-20 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:238 |
| FND-125 | medium | Inherited FR-030-AC-21 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:239 |
| FND-126 | medium | Inherited FR-030-AC-22 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:240 |
| FND-127 | medium | Inherited FR-034-AC-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1844 |
| FND-128 | medium | Inherited FR-034-AC-2 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1845 |
| FND-129 | medium | Inherited FR-034-AC-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1846 |
| FND-130 | medium | Inherited FR-034-AC-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1847 |
| FND-131 | medium | Inherited FR-034-AC-5 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1848 |
| FND-132 | medium | Inherited FR-034-AC-6 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1849 |
| FND-133 | medium | Inherited FR-034-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1850 |
| FND-134 | medium | Inherited FR-034-AC-8 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1851 |
| FND-135 | medium | Inherited FR-034-AC-9 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1852 |
| FND-136 | medium | Inherited FR-034-AC-10 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1853 |
| FND-137 | medium | Inherited FR-034-AC-11 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1854 |
| FND-138 | medium | Inherited FR-034-AC-12 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1855 |
| FND-139 | medium | Inherited FR-034-AC-13 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1856 |
| FND-140 | medium | Inherited FR-034-AC-14 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1857 |
| FND-141 | medium | Inherited FR-034-AC-15 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1858 |
| FND-142 | medium | Inherited FR-034-AC-16 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1859 |
| FND-143 | medium | Inherited FR-034-AC-17 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1860 |
| FND-144 | medium | Inherited FR-034-AC-18 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1861 |
| FND-145 | medium | Inherited FR-034-AC-19 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1862 |
| FND-146 | medium | Inherited FR-034-AC-20 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1863 |
| FND-147 | medium | Inherited FR-034-AC-21 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1864 |
| FND-148 | medium | Inherited FR-034-AC-22 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1865 |
| FND-149 | medium | Inherited FR-034-AC-23 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1866 |
| FND-150 | medium | Inherited FR-034-AC-24 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1867 |
| FND-151 | medium | Inherited FR-034-AC-25 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1868 |
| FND-152 | medium | Inherited FR-034-AC-26 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1869 |
| FND-153 | medium | Inherited FR-034-AC-27 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1870 |
| FND-154 | medium | Inherited FR-034-AC-28 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1871 |
| FND-155 | medium | Inherited FR-034-AC-29 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1872 |
| FND-156 | medium | Inherited FR-034-AC-31 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1874 |
| FND-157 | medium | Inherited FR-034-AC-32 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1875 |
| FND-158 | medium | Inherited FR-034-AC-33 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1876 |
| FND-159 | medium | Inherited FR-034-AC-34 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1877 |
| FND-160 | medium | Inherited FR-034-AC-35 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1878 |
| FND-161 | medium | Inherited FR-034-AC-36 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1879 |
| FND-162 | medium | Inherited FR-034-AC-37 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1880 |
| FND-163 | medium | Inherited FR-034-AC-38 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1881 |
| FND-164 | medium | Inherited FR-034-AC-39 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1882 |
| FND-165 | medium | Inherited FR-034-AC-40 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1883 |
| FND-166 | medium | Inherited FR-034-AC-51 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1894 |
| FND-167 | medium | Inherited FR-034-AC-52 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1895 |
| FND-168 | medium | Inherited FR-034-AC-53 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1896 |
| FND-169 | medium | Inherited FR-034-AC-54 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1897 |
| FND-170 | medium | Inherited FR-034-AC-57 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1900 |
| FND-171 | medium | Inherited FR-034-AC-58 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1901 |
| FND-172 | medium | Inherited FR-034-AC-59 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1902 |
| FND-173 | medium | Inherited FR-034-AC-60 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1903 |
| FND-174 | medium | Inherited FR-034-AC-61 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1904 |
| FND-175 | medium | Inherited FR-034-AC-62 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1905 |
| FND-176 | medium | Inherited FR-034-AC-63 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1906 |
| FND-177 | medium | Inherited FR-034-AC-65 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1908 |
| FND-178 | medium | Inherited FR-034-AC-66 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1909 |
| FND-179 | medium | Inherited FR-034-AC-67 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1910 |
| FND-180 | medium | Inherited FR-034-AC-69 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1912 |
| FND-181 | medium | Inherited FR-034-AC-70 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1913 |
| FND-182 | medium | Inherited FR-034-AC-71 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1914 |
| FND-183 | medium | Inherited FR-034-AC-73 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1916 |
| FND-184 | medium | Inherited FR-034-AC-74 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1917 |
| FND-185 | medium | Inherited FR-034-AC-75 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1918 |
| FND-186 | medium | Inherited FR-034-AC-76 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1919 |
| FND-187 | medium | Inherited FR-034-AC-77 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1920 |
| FND-188 | medium | Inherited FR-034-AC-78 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1921 |
| FND-189 | medium | Inherited FR-034-AC-79 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1922 |
| FND-190 | medium | Inherited FR-034-AC-80 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1923 |
| FND-191 | medium | Inherited FR-034-AC-81 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1924 |
| FND-192 | medium | Inherited FR-034-AC-82 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1925 |
| FND-193 | medium | Inherited FR-034-AC-83 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1926 |
| FND-194 | medium | Inherited FR-034-AC-84 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1927 |
| FND-195 | medium | Inherited FR-034-AC-85 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1928 |
| FND-196 | medium | Inherited FR-034-AC-86 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1929 |
| FND-197 | medium | Inherited FR-034-AC-87 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1930 |
| FND-198 | medium | Inherited FR-034-AC-88 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1931 |
| FND-199 | medium | Inherited FR-034-AC-89 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1932 |
| FND-200 | medium | Inherited FR-034-AC-90 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1933 |
| FND-201 | medium | Inherited FR-034-AC-91 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1934 |
| FND-202 | medium | Inherited FR-034-AC-92 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1935 |
| FND-203 | medium | Inherited FR-034-AC-94 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1937 |
| FND-204 | medium | Inherited FR-034-AC-95 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1938 |
| FND-205 | medium | Inherited FR-034-AC-96 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1939 |
| FND-206 | medium | Inherited FR-034-AC-97 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1940 |
| FND-207 | medium | Inherited FR-034-AC-98 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1941 |
| FND-208 | medium | Inherited FR-034-AC-99 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1942 |
| FND-209 | medium | Inherited FR-034-AC-100 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-034-caller-death-ownership.md:1943 |
| FND-210 | medium | Inherited FR-041-AC-4 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/functional/FR-041-claimed-module-proof-gate.md:69 |
| FND-211 | medium | Inherited NFR-006-M-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:1 |
| FND-212 | medium | Inherited NFR-006-M-2 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:1 |
| FND-213 | medium | Inherited NFR-006-M-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:1 |
| FND-214 | medium | Inherited FR-018-AC-21 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:577 |
| FND-215 | medium | Inherited FR-018-AC-22 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:581 |
| FND-216 | medium | Inherited FR-018-AC-23 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:588 |
| FND-217 | medium | Inherited FR-018-AC-33 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:636 |
| FND-218 | medium | Inherited FR-018-AC-34 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:637 |
| FND-219 | medium | Inherited FR-018-AC-35 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:638 |
| FND-220 | medium | Inherited FR-018-AC-36 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:639 |
| FND-221 | medium | Inherited FR-018-AC-37 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:640 |
| FND-222 | medium | Inherited FR-018-AC-38 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:641 |
| FND-223 | medium | Inherited FR-018-AC-39 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:642 |
| FND-224 | medium | Inherited FR-018-AC-40 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:643 |
| FND-225 | medium | Inherited FR-018-AC-41 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:644 |
| FND-226 | medium | Inherited FR-018-AC-42 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:645 |
| FND-227 | medium | Inherited FR-018-AC-43 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-018-composite-equality-oracles.md:646 |
| FND-228 | medium | Inherited FR-021-AC-30 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-021-function-application-oracles.md:277 |
| FND-229 | medium | Inherited FR-031-AC-2 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:69 |
| FND-230 | medium | Inherited FR-031-AC-6 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:481 |
| FND-231 | medium | Inherited FR-031-AC-16 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:518 |
| FND-232 | medium | Inherited FR-031-AC-17 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:189 |
| FND-233 | medium | Inherited FR-031-AC-27 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:269 |
| FND-234 | medium | Inherited FR-031-AC-20 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:524 |
| FND-235 | medium | Inherited FR-031-AC-22 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:71 |
| FND-236 | medium | Inherited FR-031-AC-26 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:527 |
| FND-237 | medium | Inherited FR-031-AC-23 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:513 |
| FND-238 | medium | Inherited FR-031-AC-24 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:529 |
| FND-239 | medium | Inherited FR-031-AC-25 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:166 |
| FND-240 | medium | Inherited FR-035-AC-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-035-caller-text-admission.md:107 |
| FND-241 | medium | Inherited FR-035-AC-2 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-035-caller-text-admission.md:108 |
| FND-242 | medium | Inherited FR-035-AC-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-035-caller-text-admission.md:109 |
| FND-243 | medium | Inherited FR-035-AC-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-035-caller-text-admission.md:110 |
| FND-244 | medium | Inherited FR-035-AC-5 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-035-caller-text-admission.md:111 |
| FND-245 | medium | Inherited FR-035-AC-6 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-035-caller-text-admission.md:112 |
| FND-246 | medium | Inherited FR-042-AC-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:31 |
| FND-247 | medium | Inherited FR-024-AC-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:75 |
| FND-248 | medium | Inherited FR-024-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:352 |
| FND-249 | medium | Inherited FR-024-AC-8 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:353 |
| FND-250 | medium | Inherited FR-024-AC-18 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:362 |
| FND-251 | medium | Inherited FR-024-AC-34 computes tagged-by-ignored-test at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:378 |
| FND-252 | medium | Inherited FR-024-AC-35 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:379 |
| FND-253 | medium | Inherited FR-024-AC-36 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:380 |
| FND-254 | medium | Inherited FR-024-AC-37 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:381 |
| FND-255 | medium | Inherited FR-024-AC-38 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:382 |
| FND-256 | medium | Inherited FR-024-AC-39 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:383 |
| FND-257 | medium | Inherited FR-024-AC-40 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:384 |
| FND-258 | medium | Inherited FR-032-AC-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:382 |
| FND-259 | medium | Inherited FR-032-AC-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:385 |
| FND-260 | medium | Inherited FR-032-AC-5 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:386 |
| FND-261 | medium | Inherited FR-032-AC-6 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:387 |
| FND-262 | medium | Inherited FR-032-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:388 |
| FND-263 | medium | Inherited FR-032-AC-8 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:389 |
| FND-264 | medium | Inherited FR-032-AC-9 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:390 |
| FND-265 | medium | Inherited FR-032-AC-10 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:391 |
| FND-266 | medium | Inherited FR-032-AC-11 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:392 |
| FND-267 | medium | Inherited FR-032-AC-12 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:393 |
| FND-268 | medium | Inherited FR-032-AC-13 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:394 |
| FND-269 | medium | Inherited FR-032-AC-14 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:395 |
| FND-270 | medium | Inherited FR-033-AC-2 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:362 |
| FND-271 | medium | Inherited FR-033-AC-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:363 |
| FND-272 | medium | Inherited FR-033-AC-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:364 |
| FND-273 | medium | Inherited FR-033-AC-5 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:365 |
| FND-274 | medium | Inherited FR-033-AC-6 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:366 |
| FND-275 | medium | Inherited FR-033-AC-8 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:368 |
| FND-276 | medium | Inherited FR-033-AC-10 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:370 |
| FND-277 | medium | Inherited FR-033-AC-14 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:374 |
| FND-278 | medium | Inherited FR-033-AC-15 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:375 |
| FND-279 | medium | Inherited FR-033-AC-17 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:377 |
| FND-280 | medium | Inherited FR-019-AC-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-019-capability-settlement.md:519 |
| FND-281 | medium | Inherited FR-019-AC-15 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-019-capability-settlement.md:547 |
| FND-282 | medium | Inherited FR-019-AC-24 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-019-capability-settlement.md:548 |
| FND-283 | medium | Inherited FR-019-AC-34 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-019-capability-settlement.md:558 |
| FND-284 | medium | Inherited FR-019-AC-35 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-019-capability-settlement.md:559 |
| FND-285 | medium | Inherited FR-019-AC-36 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-019-capability-settlement.md:560 |
| FND-286 | medium | Inherited FR-019-AC-37 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-019-capability-settlement.md:561 |
| FND-287 | medium | Inherited FR-019-AC-38 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-019-capability-settlement.md:562 |
| FND-288 | medium | Inherited FR-019-AC-39 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-019-capability-settlement.md:563 |
| FND-289 | medium | Inherited FR-019-AC-40 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-019-capability-settlement.md:564 |
| FND-290 | medium | Inherited FR-019-AC-41 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-019-capability-settlement.md:565 |
| FND-291 | medium | Inherited FR-022-AC-26 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-022-routed-generation.md:361 |
| FND-292 | medium | Inherited FR-022-AC-27 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-022-routed-generation.md:362 |
| FND-293 | medium | Inherited FR-022-AC-28 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-022-routed-generation.md:363 |
| FND-294 | medium | Inherited FR-026-AC-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-026-backend-adapter-contract.md:60 |
| FND-295 | medium | Inherited FR-026-AC-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/routed/functional/FR-026-backend-adapter-contract.md:61 |
| FND-296 | medium | Inherited FR-002-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-002-tristate-proptest.md:91 |
| FND-297 | medium | Inherited FR-002-AC-8 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-002-tristate-proptest.md:92 |
| FND-298 | medium | Inherited FR-002-AC-10 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-002-tristate-proptest.md:94 |
| FND-299 | medium | Inherited FR-002-AC-11 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-002-tristate-proptest.md:95 |
| FND-300 | medium | Inherited FR-002-AC-12 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-002-tristate-proptest.md:96 |
| FND-301 | medium | Inherited FR-002-AC-13 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-002-tristate-proptest.md:97 |
| FND-302 | medium | Inherited FR-002-AC-14 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-002-tristate-proptest.md:98 |
| FND-303 | medium | Inherited FR-002-AC-15 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-002-tristate-proptest.md:99 |
| FND-304 | medium | Inherited FR-002-AC-16 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-002-tristate-proptest.md:100 |
| FND-305 | medium | Inherited FR-002-AC-17 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-002-tristate-proptest.md:101 |
| FND-306 | medium | Inherited FR-002-AC-18 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-002-tristate-proptest.md:102 |
| FND-307 | medium | Inherited FR-008-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:123 |
| FND-308 | medium | Inherited FR-008-AC-9 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:125 |
| FND-309 | medium | Inherited FR-009-AC-7 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/functional/FR-009-constructive-correlated-populations.md:77 |
| FND-310 | medium | Inherited NFR-004-M-1 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 |
| FND-311 | medium | Inherited NFR-004-M-2 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 |
| FND-312 | medium | Inherited NFR-004-M-3 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 |
| FND-313 | medium | Inherited NFR-004-M-4 computes untagged at exact base and candidate; owner existing IR-878/program ownership, actual 3db226b2-9381-476f-a880-cf37f082ac9e / 04483a66-6049-4c81-99f7-4fd6563d9b26; IR-942 where applicable. No gap closure or execution is claimed. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 |
| FND-314 | medium | Inherited authentic duplicate-source admission evidence remains absent on the reviewed branch; existing synthetic generator tags do not discharge merged AC8 stage prerequisites. Owner IR-17, as retained in PR496 and IR877 spec review. | spec/oracle/functional/FR-018-composite-equality-oracles.md:611 |
| FND-315 | medium | New AC16 public bounded-positive/private two-form-equivalence and precise malformed-member stage allocation is not established by unchanged synthetic-public fixture tags. Full exact-source equivalence, original bounds/order/rounding, precise private refusals/no code/siblings and genuine source/context positive remain unqualified. Owner IR-17, actual 538eac99 claim. | spec/oracle/functional/FR-018-composite-equality-oracles.md:153 |
| FND-316 | medium | New AC24 exact external-QSpec strict-reader/shape versus independent genuine-source public-generation separation is not implemented by the unchanged fixture-appending/key-decoding helper. Tags and an unset-QSPEC_REPO early return do not discharge either evidence stage. Owner IR-17, actual 538eac99 claim; existing IR643 recursion ownership remains unchanged. | spec/oracle/functional/FR-018-composite-equality-oracles.md:627 |

## Evidence and coverage

Selected QSL owner source was independently examined, rather than trusting the author report: FR-092:302-328 requires bounded-domain references; qsl-semantics/src/check/type_form.rs:340-363 represents the bounded numeric families; assemble.rs:2958-2968 resolves an alias to the same ValueType; lowering.rs:1957-2040 emits the numeric bounded nodes and :2201-2230 descends into actual tuple positions. Contract IR crates/quire-contract-model/src/checked_package/v2/lower.rs:524-590 follows exact successors and :664-670 clones each actual node and semantic_type. A profile/root selection does not rewrite member encoding. QSL qsl-replay/src/admitted.rs:198-202 binds context to package identity and :407-445 builds it only from recompiled admitted source. CG's draft src/oracle/equality/mod.rs:735-746 binds reconstruction to the source package identity. These facts support this bounded three-family stage split; they do not establish that every conceivable public alternate source is impossible.

The candidate keeps exact public Int[-100,100], Text[0,16;nfc], Decimal[-100,100;0,0;nearest-even], Rational[-10,10;1,1], tuple order, each selected text bound and no sibling substitution. The old two-form generated-source equivalence remains mandatory at a labelled private seam; the currently drafted test merely checking reconstruction substrings is not qualified as full equivalence. Exact MissingBound(integer_range)/bound identity, unsupported bounded_domain/bound identity, no code and independent siblings remain mandatory. Float/operator-ineligible reader controls keep their actual cause/path/locus. No public malformed nominal context, key minting or unrelated sibling ownership is licensed. The unbounded Integer/sibling-range ambiguity is explicitly unchanged.

The exact external QSpec recursive fixture retains its own strict-reader admission and original List optional wrapper/Tree [0,3] shape evidence. The independent source positive must compile_admitted, lower its unchanged package, generate both genuine declarations and preserve its own keys/context; it is explicitly not the external fixture extended with equalities. Private fixture-shaped reconstruction retains field presence, keys, cardinality and exact defense assertions. An unavailable QSPEC_REPO check grants no evidence. Synthetic Option/Sequence cycles retain repeated-node TypeResolutionCycle, subprocess protection, no symbol and healthy sibling. AC8/26 from PR496 are unchanged.

Quire validation and git diff --check pass. No applicable enforced AssuranceProfile is installed in the scoped spec. No Cargo, formatter, metadata, native test, backend or proof was run; Rust gates are inapplicable to this two-Markdown-file/no-embedded-consumer diff and root holds the sole heavy workload reservation. EARS was not selected: the FR Description requirement statement is unchanged. Dependency analysis was not selected: no relationship edge changes; the new body citation resolves to the actual QSL FR092 owner. Object/risk analyses were not selected: no object schema, algorithm or resource budget changes. Calibrated criterion-strength/Jev analysis was not selected or claimed. No optional repository-wide semantic review was requested or performed.

Both exact base and candidate compute887 criteria:523 tagged,295 untagged,51 method-without-symbol,18 ignored-only. Criterion set, methods, statuses and binders are identical; only AC16/24 text changes. The313 computed gaps are311 program-owned plus two planned IR877 controls; the extra three manual stage gaps are inherited AC8 plus new AC16/24. Total316 findings. AC56 Inspection remains pending IR877 implementation/actual inspection and is not falsely counted as a missing Test binding or inspected PASS. IR621 AC123/124 now compute tagged after merged494; their historical missing bindings are not recreated. Existing tags on AC8/16/24 are not falsely judged wrong solely because new stage obligations remain unmet.

Whole-source reverse audit inventoried library/public exports, core profile/CLI, publication, evidence/replay, strategy, Kani generation/execution/corpus and equality source/test seams against their owning core/oracle/proptest/kani spec families; no newly unspecified surface or stub/hollow implementation was identified in this two-doc amendment. Existing planned feature gaps remain in the complete findings inventory. Plan completion: not assessed

## Disposition round 1 verification

Same frozen head ebb26afd36236bda990224c48a2b69fe4ce80a00; base 6b1ba8b4e4906c2f2e5e2b398fd59e7f2fb6c7f8. Actual root scope decision 68f36578-9a30-4687-a4f7-4780518c184f independently read back: partial specification delivery only. No source or test change, no new introduced spec defect, no product qualification, no assertion weakening or changed member-association semantics. All six non-gap methods retain PASS. Full gap retains FAIL; all316 findings remain open under their exact existing owners. AC56 inspection remains pending IR-877 and IR-643 recursion ownership is unchanged. No heavy job ran.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | FR-005-AC-3 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-002 | deferred | FR-039-AC-4 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-003 | deferred | NFR-001-M-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-004 | deferred | NFR-002-M-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-005 | deferred | NFR-002-M-3 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-006 | deferred | FR-004-AC-6 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-007 | deferred | FR-004-AC-8 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-008 | deferred | FR-004-AC-10 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-009 | deferred | FR-004-AC-11 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-010 | deferred | FR-004-AC-12 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-011 | deferred | FR-004-AC-13 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-012 | deferred | FR-004-AC-14 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-013 | deferred | FR-004-AC-15 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-014 | deferred | FR-004-AC-18 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-015 | deferred | FR-036-AC-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-016 | deferred | FR-036-AC-2 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-017 | deferred | FR-036-AC-3 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-018 | deferred | FR-036-AC-4 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-019 | deferred | FR-036-AC-5 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-020 | deferred | FR-036-AC-6 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-021 | deferred | FR-036-AC-7 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-022 | deferred | FR-015-AC-19 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-023 | deferred | FR-015-AC-21 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-024 | deferred | FR-015-AC-22 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-025 | deferred | FR-015-AC-23 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-026 | deferred | FR-015-AC-24 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-027 | deferred | FR-015-AC-30 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-028 | deferred | FR-015-AC-31 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-029 | deferred | FR-015-AC-32 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-030 | deferred | FR-015-AC-38 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-031 | deferred | FR-015-AC-39 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-032 | deferred | FR-015-AC-40 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-033 | deferred | FR-015-AC-41 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-034 | deferred | FR-015-AC-42 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-035 | deferred | FR-015-AC-43 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-036 | deferred | FR-015-AC-44 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-037 | deferred | FR-015-AC-45 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-038 | deferred | FR-015-AC-46 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-039 | deferred | FR-015-AC-47 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-040 | deferred | FR-015-AC-48 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-041 | deferred | FR-015-AC-49 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-042 | deferred | FR-015-AC-51 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-043 | deferred | FR-015-AC-52 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-044 | deferred | FR-015-AC-56 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-045 | deferred | FR-015-AC-57 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-046 | deferred | FR-015-AC-69 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-047 | deferred | FR-015-AC-70 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-048 | deferred | FR-015-AC-71 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-049 | deferred | FR-015-AC-72 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-050 | deferred | FR-015-AC-73 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-051 | deferred | FR-015-AC-74 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-052 | deferred | FR-015-AC-75 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-053 | deferred | FR-015-AC-76 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-054 | deferred | FR-015-AC-80 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-055 | deferred | FR-015-AC-84 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-056 | deferred | FR-015-AC-85 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-057 | deferred | FR-015-AC-87 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-058 | deferred | FR-015-AC-89 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-059 | deferred | FR-015-AC-90 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-060 | deferred | FR-015-AC-91 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-061 | deferred | FR-015-AC-92 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-062 | deferred | FR-015-AC-93 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-063 | deferred | FR-015-AC-95 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-064 | deferred | FR-015-AC-98 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-065 | deferred | FR-015-AC-105 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-066 | deferred | FR-015-AC-106 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-067 | deferred | FR-015-AC-107 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-068 | deferred | FR-015-AC-108 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-069 | deferred | FR-015-AC-110 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-070 | deferred | FR-015-AC-111 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-071 | deferred | FR-015-AC-117 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-072 | deferred | FR-015-AC-119 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-073 | deferred | FR-015-AC-120 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-074 | deferred | FR-015-AC-121 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-075 | deferred | FR-015-AC-122 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-076 | deferred | FR-017-AC-7 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-077 | deferred | FR-017-AC-19 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-078 | deferred | FR-017-AC-26 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-079 | deferred | FR-017-AC-27 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-080 | deferred | FR-017-AC-30 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-081 | deferred | FR-017-AC-47 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-082 | deferred | FR-017-AC-52 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-083 | deferred | FR-017-AC-53 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-084 | deferred | FR-017-AC-54 remains planned under existing IR-877, actual decisions 0b9a2853-962b-4073-8474-03b0d5986ce3 and 4227b5e7-0d5e-43fc-8162-952c1dc0f7af. AC56 inspection also remains pending; no implementation or genuine control acceptance is claimed. |
| FND-085 | deferred | FR-017-AC-55 remains planned under existing IR-877, actual decisions 0b9a2853-962b-4073-8474-03b0d5986ce3 and 4227b5e7-0d5e-43fc-8162-952c1dc0f7af. AC56 inspection also remains pending; no implementation or genuine control acceptance is claimed. |
| FND-086 | deferred | FR-017-CON-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-087 | deferred | FR-025-AC-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-088 | deferred | FR-025-AC-2 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-089 | deferred | FR-025-AC-3 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-090 | deferred | FR-025-AC-4 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-091 | deferred | FR-025-AC-5 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-092 | deferred | FR-025-AC-6 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-093 | deferred | FR-025-AC-7 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-094 | deferred | FR-025-AC-8 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-095 | deferred | FR-025-AC-9 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-096 | deferred | FR-028-AC-5 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-097 | deferred | FR-028-AC-6 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-098 | deferred | FR-028-AC-7 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-099 | deferred | FR-028-AC-8 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-100 | deferred | FR-028-AC-9 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-101 | deferred | FR-028-AC-13 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-102 | deferred | FR-028-AC-14 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-103 | deferred | FR-028-AC-15 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-104 | deferred | FR-028-AC-16 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-105 | deferred | FR-028-AC-17 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-106 | deferred | FR-028-AC-18 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-107 | deferred | FR-028-AC-19 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-108 | deferred | FR-028-AC-20 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-109 | deferred | FR-028-AC-22 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-110 | deferred | FR-028-AC-23 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-111 | deferred | FR-028-AC-24 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-112 | deferred | FR-029-AC-17 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-113 | deferred | FR-029-AC-19 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-114 | deferred | FR-029-AC-21 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-115 | deferred | FR-029-AC-22 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-116 | deferred | FR-029-AC-23 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-117 | deferred | FR-029-AC-24 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-118 | deferred | FR-029-AC-25 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-119 | deferred | FR-029-AC-26 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-120 | deferred | FR-029-AC-27 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-121 | deferred | FR-029-AC-30 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-122 | deferred | FR-029-AC-31 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-123 | deferred | FR-030-AC-19 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-124 | deferred | FR-030-AC-20 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-125 | deferred | FR-030-AC-21 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-126 | deferred | FR-030-AC-22 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-127 | deferred | FR-034-AC-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-128 | deferred | FR-034-AC-2 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-129 | deferred | FR-034-AC-3 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-130 | deferred | FR-034-AC-4 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-131 | deferred | FR-034-AC-5 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-132 | deferred | FR-034-AC-6 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-133 | deferred | FR-034-AC-7 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-134 | deferred | FR-034-AC-8 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-135 | deferred | FR-034-AC-9 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-136 | deferred | FR-034-AC-10 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-137 | deferred | FR-034-AC-11 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-138 | deferred | FR-034-AC-12 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-139 | deferred | FR-034-AC-13 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-140 | deferred | FR-034-AC-14 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-141 | deferred | FR-034-AC-15 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-142 | deferred | FR-034-AC-16 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-143 | deferred | FR-034-AC-17 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-144 | deferred | FR-034-AC-18 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-145 | deferred | FR-034-AC-19 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-146 | deferred | FR-034-AC-20 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-147 | deferred | FR-034-AC-21 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-148 | deferred | FR-034-AC-22 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-149 | deferred | FR-034-AC-23 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-150 | deferred | FR-034-AC-24 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-151 | deferred | FR-034-AC-25 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-152 | deferred | FR-034-AC-26 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-153 | deferred | FR-034-AC-27 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-154 | deferred | FR-034-AC-28 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-155 | deferred | FR-034-AC-29 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-156 | deferred | FR-034-AC-31 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-157 | deferred | FR-034-AC-32 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-158 | deferred | FR-034-AC-33 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-159 | deferred | FR-034-AC-34 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-160 | deferred | FR-034-AC-35 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-161 | deferred | FR-034-AC-36 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-162 | deferred | FR-034-AC-37 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-163 | deferred | FR-034-AC-38 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-164 | deferred | FR-034-AC-39 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-165 | deferred | FR-034-AC-40 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-166 | deferred | FR-034-AC-51 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-167 | deferred | FR-034-AC-52 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-168 | deferred | FR-034-AC-53 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-169 | deferred | FR-034-AC-54 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-170 | deferred | FR-034-AC-57 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-171 | deferred | FR-034-AC-58 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-172 | deferred | FR-034-AC-59 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-173 | deferred | FR-034-AC-60 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-174 | deferred | FR-034-AC-61 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-175 | deferred | FR-034-AC-62 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-176 | deferred | FR-034-AC-63 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-177 | deferred | FR-034-AC-65 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-178 | deferred | FR-034-AC-66 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-179 | deferred | FR-034-AC-67 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-180 | deferred | FR-034-AC-69 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-181 | deferred | FR-034-AC-70 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-182 | deferred | FR-034-AC-71 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-183 | deferred | FR-034-AC-73 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-184 | deferred | FR-034-AC-74 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-185 | deferred | FR-034-AC-75 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-186 | deferred | FR-034-AC-76 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-187 | deferred | FR-034-AC-77 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-188 | deferred | FR-034-AC-78 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-189 | deferred | FR-034-AC-79 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-190 | deferred | FR-034-AC-80 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-191 | deferred | FR-034-AC-81 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-192 | deferred | FR-034-AC-82 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-193 | deferred | FR-034-AC-83 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-194 | deferred | FR-034-AC-84 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-195 | deferred | FR-034-AC-85 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-196 | deferred | FR-034-AC-86 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-197 | deferred | FR-034-AC-87 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-198 | deferred | FR-034-AC-88 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-199 | deferred | FR-034-AC-89 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-200 | deferred | FR-034-AC-90 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-201 | deferred | FR-034-AC-91 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-202 | deferred | FR-034-AC-92 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-203 | deferred | FR-034-AC-94 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-204 | deferred | FR-034-AC-95 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-205 | deferred | FR-034-AC-96 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-206 | deferred | FR-034-AC-97 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-207 | deferred | FR-034-AC-98 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-208 | deferred | FR-034-AC-99 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-209 | deferred | FR-034-AC-100 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-210 | deferred | FR-041-AC-4 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-211 | deferred | NFR-006-M-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-212 | deferred | NFR-006-M-2 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-213 | deferred | NFR-006-M-3 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-214 | deferred | FR-018-AC-21 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-215 | deferred | FR-018-AC-22 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-216 | deferred | FR-018-AC-23 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-217 | deferred | FR-018-AC-33 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-218 | deferred | FR-018-AC-34 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-219 | deferred | FR-018-AC-35 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-220 | deferred | FR-018-AC-36 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-221 | deferred | FR-018-AC-37 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-222 | deferred | FR-018-AC-38 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-223 | deferred | FR-018-AC-39 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-224 | deferred | FR-018-AC-40 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-225 | deferred | FR-018-AC-41 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-226 | deferred | FR-018-AC-42 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-227 | deferred | FR-018-AC-43 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-228 | deferred | FR-021-AC-30 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-229 | deferred | FR-031-AC-2 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-230 | deferred | FR-031-AC-6 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-231 | deferred | FR-031-AC-16 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-232 | deferred | FR-031-AC-17 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-233 | deferred | FR-031-AC-27 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-234 | deferred | FR-031-AC-20 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-235 | deferred | FR-031-AC-22 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-236 | deferred | FR-031-AC-26 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-237 | deferred | FR-031-AC-23 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-238 | deferred | FR-031-AC-24 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-239 | deferred | FR-031-AC-25 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-240 | deferred | FR-035-AC-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-241 | deferred | FR-035-AC-2 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-242 | deferred | FR-035-AC-3 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-243 | deferred | FR-035-AC-4 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-244 | deferred | FR-035-AC-5 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-245 | deferred | FR-035-AC-6 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-246 | deferred | FR-042-AC-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-247 | deferred | FR-024-AC-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-248 | deferred | FR-024-AC-7 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-249 | deferred | FR-024-AC-8 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-250 | deferred | FR-024-AC-18 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-251 | deferred | FR-024-AC-34 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-252 | deferred | FR-024-AC-35 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-253 | deferred | FR-024-AC-36 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-254 | deferred | FR-024-AC-37 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-255 | deferred | FR-024-AC-38 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-256 | deferred | FR-024-AC-39 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-257 | deferred | FR-024-AC-40 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-258 | deferred | FR-032-AC-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-259 | deferred | FR-032-AC-4 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-260 | deferred | FR-032-AC-5 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-261 | deferred | FR-032-AC-6 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-262 | deferred | FR-032-AC-7 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-263 | deferred | FR-032-AC-8 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-264 | deferred | FR-032-AC-9 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-265 | deferred | FR-032-AC-10 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-266 | deferred | FR-032-AC-11 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-267 | deferred | FR-032-AC-12 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-268 | deferred | FR-032-AC-13 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-269 | deferred | FR-032-AC-14 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-270 | deferred | FR-033-AC-2 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-271 | deferred | FR-033-AC-3 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-272 | deferred | FR-033-AC-4 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-273 | deferred | FR-033-AC-5 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-274 | deferred | FR-033-AC-6 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-275 | deferred | FR-033-AC-8 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-276 | deferred | FR-033-AC-10 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-277 | deferred | FR-033-AC-14 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-278 | deferred | FR-033-AC-15 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-279 | deferred | FR-033-AC-17 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-280 | deferred | FR-019-AC-3 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-281 | deferred | FR-019-AC-15 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-282 | deferred | FR-019-AC-24 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-283 | deferred | FR-019-AC-34 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-284 | deferred | FR-019-AC-35 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-285 | deferred | FR-019-AC-36 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-286 | deferred | FR-019-AC-37 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-287 | deferred | FR-019-AC-38 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-288 | deferred | FR-019-AC-39 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-289 | deferred | FR-019-AC-40 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-290 | deferred | FR-019-AC-41 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-291 | deferred | FR-022-AC-26 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-292 | deferred | FR-022-AC-27 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-293 | deferred | FR-022-AC-28 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-294 | deferred | FR-026-AC-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-295 | deferred | FR-026-AC-4 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-296 | deferred | FR-002-AC-7 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-297 | deferred | FR-002-AC-8 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-298 | deferred | FR-002-AC-10 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-299 | deferred | FR-002-AC-11 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-300 | deferred | FR-002-AC-12 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-301 | deferred | FR-002-AC-13 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-302 | deferred | FR-002-AC-14 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-303 | deferred | FR-002-AC-15 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-304 | deferred | FR-002-AC-16 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-305 | deferred | FR-002-AC-17 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-306 | deferred | FR-002-AC-18 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-307 | deferred | FR-008-AC-7 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-308 | deferred | FR-008-AC-9 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-309 | deferred | FR-009-AC-7 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-310 | deferred | NFR-004-M-1 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-311 | deferred | NFR-004-M-2 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-312 | deferred | NFR-004-M-3 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-313 | deferred | NFR-004-M-4 retains existing IR-878/program ownership under actual comments 3db226b2-9381-476f-a880-cf37f082ac9e and 04483a66-6049-4c81-99f7-4fd6563d9b26, with IR-942 where applicable. Root scope decision 68f36578-9a30-4687-a4f7-4780518c184f permits only this partial spec delivery, not evidence acceptance, owner transfer or a native/proof gate waiver. |
| FND-314 | deferred | FR-018-AC-8 remains owned by this same open IR-17 implementation, actual claim 538eac99-26d3-493e-84f3-c1210053067b and scope decision 68f36578-9a30-4687-a4f7-4780518c184f. Exact public/private/external stage evidence must be implemented and qualified before closure; existing IR-643 recursion ownership is unchanged. |
| FND-315 | deferred | FR-018-AC-16 remains owned by this same open IR-17 implementation, actual claim 538eac99-26d3-493e-84f3-c1210053067b and scope decision 68f36578-9a30-4687-a4f7-4780518c184f. Exact public/private/external stage evidence must be implemented and qualified before closure; existing IR-643 recursion ownership is unchanged. |
| FND-316 | deferred | FR-018-AC-24 remains owned by this same open IR-17 implementation, actual claim 538eac99-26d3-493e-84f3-c1210053067b and scope decision 68f36578-9a30-4687-a4f7-4780518c184f. Exact public/private/external stage evidence must be implemented and qualified before closure; existing IR-643 recursion ownership is unchanged. |
