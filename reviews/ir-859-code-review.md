---
id: SR-12840
title: "IR-859 supplemental code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-859 PR424 own spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md and spec/kani/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-859. Supplemental method for collection status/evidence annotations and the isolated upstream context correction. Original reviews remain unchanged.

## Verdict

**PASS** — no new defect. Existing partial/planned disclosures remain required; no whole-repository coverage or fresh proof claimed.

## Evidence and rationale

Own frozen delta: #424 b42908a7a7ce410baff1333b146857da4a9d1bc8 against parent #417 cbc03c87251848009171d2ef490907414c4676e2. Parent content is context only and being corrected separately. Six edited AC obligation prefixes are byte-identical; AC99 removes only its old status lead-in. No normative requirement or verification method changes.

Fresh computed AC96..102 binder counts are 6/8/0/3/5/5/5; graph AC105..108 remain untagged. Inspected actual test assertions: ordered-set refusal binds AC97/102 rather than AC96 uniqueness; symbolic draw/cover assertions bind AC96/100 rather than AC99; KaniExecutionEvidence still has no version fields; representational helper remains untagged and declaration test is helper-only. Their partial disclosures remain accurate. AC97 refusal tests and AC102 mixed supported/refused accounting exercise the claimed collection behaviors. Existing gaps are disclosed, not introduced by this patch.

IR268 historical close-out comment b1bad83a-f1d6-4c47-9cce-a99162c90d27 confirms one passing test, healthy 94 SUCCESS, mutant 93 SUCCESS and 1 FAILURE with in-domain playback, Kani 0.68.0, CBMC 6.11.0, CaDiCaL 3.0.0 and unwind 8. This is historical evidence, not a new run. The ignored Kani wrapper receives no ordinary-test run credit. Live receipts show IR851/852 Done and QSL820 In Progress; actual package population/link emission remains outstanding. Graph stays Planned.

EARS is inapplicable to status annotations with unchanged normative obligations. Integrity, objects, dependency, architecture and security methods are inapplicable because no structure, object, relationship edge, boundary or security behavior changes. Completed base SR12210 and gap SR12211 are retained verbatim rather than rerun. No Cargo/Kani or branch edits occurred.

Inspected duplication/vendoring, integrity and spec/code/test faithfulness in the own delta. No new second normative definition, helper, vendored payload, suppressed gate or file tracking machinery. Test/status summaries reference the owning obligations. No applicable AssuranceProfile was found under spec/. Rust lane is skipped because this change is Markdown-only; source reads inspect evidence only.

## Examined scope

```yaml
scope:
- id: FR-015-AC-96
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An admitted QSL V2 clause/function body that contains a `quire.op.collection.forall`
    or `quire.op.collection.exists` operation over a selected model collection field
    is a collection claim. For a field with typed inclusive cardinality `[minimum,
    maximum]` and supported finite elements, the generator emits a harness with symbolic
    length and symbolic elements. Its assumptions admit both cardinality endpoints
    and every length between them; for `IntRange` elements they admit both inclusive
    integer endpoints and every integer between them, and for `Boolean` elements they
    admit both `false` and `true
- id: FR-015-AC-97
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For an admitted collection claim whose selected field has multiplicity
    lower bound `0` and no finite upper bound, QSL admits an unbounded collection
    type; when its element type is supported, this generator settles `requires_bound`
    naming the selected field or unbounded collection type and emits no harness. An
    admitted field with no finite element domain likewise settles `requires_bound`
    naming that element type and emits no harness. A field with multiplicity lower
    bound greater than `0` and no finite upper bound has no QSL kernel type ([QSL
    FR-056-AC-10](ix://agent-ix/quire-spec-language/FR-05
- id: FR-015-AC-98
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: A graph claim (one reachability application of either form above, at its
    `state_clause` node id and argument path) generates a harness when the admitted
    package gives its covering object type `E` a finite population `[0, N]` through
    `select_population` and the selected edge has finite multiplicity (for `quire.op.model.reaches_field`
    the field's `Reference`, `Option` or bounded `Collection` member; for `quire.op.model.reaches`
    the destination end's finite multiplicity). Every symbolic object and edge is
    bounded inclusively by those declarations, including zero and maximum where admitted.
    Reacha
- id: FR-015-AC-99
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The collection harness asserts the admitted clause/function body over symbolic
    inputs drawn from the selected model field's typed domain, ends with the FR-015-AC-7
    cover after its assertions, and never replaces those inputs with the finite-input
    corpus's concrete `Vec<i128>`. The independent native oracle replays the same
    authored function through `qsl_replay::replay`; it does not substitute a separately
    authored predicate. A healthy QSL-emitted admitted V2 case yields installed-Kani
    `Verified` with its cover satisfied; a seeded claim or subject mutant yields `Falsified`
    with a concrete in-dom
- id: FR-015-AC-100
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For the collection family, changing the selected field's authored cardinality
    or element bound, admitted body claim, customer subject or unwind changes the
    harness identity; equal admitted inputs regenerate byte-identically. The identity
    records every symbolic collection domain and the solver and complete ordered option
    vector, and FR-017 run evidence records the actual Kani, Rust and solver versions,
    unwind and every option used by the installed backend. For the graph family (AC-98),
    changing the claim's clause id or argument path, the reachability form, relationship
    or field, its role or nam
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A request whose symbolic collection domain exceeds the generator''s representational
    or resource ceiling settles `unsupported` with a typed reason naming the failing
    domain or ceiling and no harness; it never tightens the model domain to make a
    proof tractable. A missing or malformed selected declaration likewise refuses
    with its typed reason and no harness. For a graph claim, a population or edge
    domain whose symbolic object or edge count exceeds the ceiling likewise settles
    `unsupported` naming the domain, with no harness and no reduced population or
    multiplicity. PARTIAL (IR-268, CG #396 fe2'
- id: FR-015-AC-102
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'An admitted collection body claim is settled through the FR-015 V2 obligation
    path, one disposition per requested item; supported items alone emit harnesses.
    The finite-input lowerer and hand-built corpus of AC-86 to AC-95 neither supply
    the symbolic bounds nor classify the proof verdict. Graph items take the same
    V2 path (IR-805), one disposition per requested item, and are not implementation
    or proof evidence for IR-268. IMPLEMENTED for the collection family (IR-268, CG
    #396 fe20ecc): `tests/it/kani_obligations_symbolic_domains.rs` `collection_negotiation_accounts_for_each_supported_or_refus'
- id: FR-015-AC-105
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'For a graph claim the harness draws an object count in `[0, N]` of the
    one covering population and symbolic objects within it; for every object, the
    selected edge as its shape states: a `Reference` field has exactly one target,
    an `Option` field an absent or one target, a `Collection` field or finite multi-valued
    relationship end a count in `[lower, upper]` (a unique end admits no repeated
    target, a non-unique end repeats, and `lower > 0`, such as `{2,3}`, forces at
    least `lower` targets) and every target is an object of the population. Start
    and target are drawn from that population and may b'
- id: FR-015-AC-106
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'The independent native oracle replays the same authored clause over each
    concrete graph through QSL''s replay (`qsl_replay`), and does not substitute a
    separately authored reachability. With the installed backend, a QSL-emitted, IR-admitted
    healthy case yields `Verified` with its cover satisfied for one field-edge claim
    and one relationship-end claim over a `{0,3}` unique and a `{0,3}` non-unique
    end; each seeded mutant yields `Falsified` with an in-domain playback: reachability
    that allows a zero-length path (so an isolated `start = target` reads reachable),
    one that rejects a self-loop, one f'
- id: FR-015-AC-107
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'Graph claims are located by `state_clause` node id and argument path:
    one reachability written twice in one condition yields two items at positions
    0 and 1; one node under two clauses yields two items with different identities;
    a field-edge and a relationship-end reachability yield separate items; a collection
    claim, a clause with no reachability and a path that does not name a reachability
    application each settle `unsupported` naming the missing claim with no harness.
    A mixed request settles one disposition per item and a supported item''s harness
    is unchanged by a refused sibling. PLANNED (IR'
- id: FR-015-AC-108
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'A graph claim settles by the disposition sequence in the Description (the
    paragraph "Disposition sequence for a graph claim"), with no harness, domain reduction
    or caller `max_expansions` substitute. While the population selection for the
    claim is `Unavailable` (no IR mapping from the claim to a population, IR-852),
    the claim settles `unsupported` naming it. Given a selection, `Unbounded` or `UnknownClosureOpenExtent`,
    that is a claim whose population has no finite authored bound (after QSL-819 and
    IR-852), settles `requires_bound` naming the population; an unbounded `Collection`
    edge settles '
- id: TC-025
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: run evidence, including actual Kani, Rust and solver versions. The collection-family
    tests exist and are tagged
- id: spec/kani/matrix/tests.md
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: "| FR-015 | FR-015-AC-96 through FR-015-AC-102 | TC-025 | \U0001F6A7 Partly\
    \ covered (IR-268, CG #396 fe20ecc): FR-015-AC-97 is covered by tagged tests in\
    \ `tests/it/kani_obligations_symbolic_domains.rs`; the Kani-lane test `tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies`\
    \ (:2080) is recorded in IR-268 evidence as `Verified` (94 SUCCESS, IR-268 close-out\
    \ run at PR head f43adca) with the mutant `Falsified` (93 SUCCESS and 1 FAILURE,\
    \ in-domain playback). FR-015-AC-102 is covered for the collection family only.\
    \ FR-015-AC-96, FR-015-AC-99, FR-015-AC-100 and FR-015-AC-101 are partial: AC"
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
