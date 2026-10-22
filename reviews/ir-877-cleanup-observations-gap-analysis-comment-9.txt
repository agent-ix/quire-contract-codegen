<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 9/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-145 | high | coverage | Inherited FR-028-AC-14 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-146 | high | coverage | Inherited FR-028-AC-15 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-147 | high | coverage | Inherited FR-028-AC-16 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-148 | high | coverage | Inherited FR-028-AC-17 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-149 | high | coverage | Inherited FR-028-AC-18 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-150 | high | coverage | Inherited FR-028-AC-19 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-151 | high | coverage | Inherited FR-028-AC-20 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-152 | high | coverage | Inherited FR-028-AC-22 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-153 | high | coverage | Inherited FR-028-AC-23 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-154 | high | coverage | Inherited FR-028-AC-24 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-155 | high | coverage | Inherited FR-029-AC-17 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-156 | high | coverage | Inherited FR-029-AC-19 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-157 | high | coverage | Inherited FR-029-AC-21 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-158 | high | coverage | Inherited FR-029-AC-22 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-159 | high | coverage | Inherited FR-029-AC-23 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-160 | high | coverage | Inherited FR-029-AC-24 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-161 | high | coverage | Inherited FR-029-AC-25 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-162 | high | coverage | Inherited FR-029-AC-26 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-019-AC-7
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An envelope whose `capability_vocabulary` is not exactly `quire.capability-kind/v1`, or is
    absent, is refused as `invalid_capability`/`unsupported-version` with none of its kinds read.
- id: FR-019-AC-8
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An item with an absent extent classification settles `invalid-request` with `invalid_capability`/`absent-extent`,
    and an absent or unknown kind settles before the candidate table is consulted.
- id: FR-019-AC-9
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A backend kind added without a negotiation arm does not compile: the dispatch is an exhaustive
    `match` over the closed kind with no catch-all arm.'
- id: FR-019-AC-11
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). `BackendKind` has `Process(id)` beside `Kani`; `ALL` lists finite built-in
    kinds only, while CG enumerates process candidates from descriptors. CG's descriptor-to-kind conversion
    returns `Process(id)` exactly when a descriptor has process origin, including identity text `kani`.
    A named, registered process backend reaches that arm and receives one disposition; no process item
    disappears for being absent from `ALL`.
- id: FR-019-AC-12
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; non-empty bounded rows depend on QSL-654's producer and admit-side enforcement).
    A bounded process item with an uncovered `extent.bounds[].kind` settles `unsupported`, warned, with
    `unsupported_projection`/`unsupported-requested-capability` naming the offending domain kind and candidate
    (QSpec FR-290-AC-13). An item that passes the domain check, including one with empty `bounds`, settles
    `supported` exactly when its descriptor advertises `bounded` for its kind; otherwise it settles `unsupported`,
    warned, with `unsupported_projection`/`unsupported-requested-capability`.
- id: FR-019-AC-16
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). An unbounded process item settles `supported` when its descriptor advertises
    `unbounded` for its kind, regardless of whether any `extent.domains[].kind` belongs to manifest `domains`.
- id: FR-019-AC-17
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded item settles `requires-bound`
    exactly when `finite_bound_available` is true and every boundable `extent.domains[].kind` is in manifest
    `domains` (QSpec FR-290-AC-13).
- id: FR-019-AC-18
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). Given the same process advertisements and item extent, changing only backend
    identity, manifest position, manifest run-limit defaults or ambient state leaves the disposition and
    cause unchanged apart from the backend named in the output.
- id: FR-019-AC-19
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; bounded row depends on QSL-654's producer). The process arm reads each explicit
    `extent.bounds[].kind` and never derives a domain kind from `DomainKey`; two otherwise equal bounded
    items with the same `DomainKey` and different explicit kinds can settle differently under one manifest
    `domains` set.
- id: FR-019-AC-20
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; driver projection remains paired work). CG's process descriptor retains the
    FR-331 manifest's advertised (kind, mode) pairs, `domains` and optional `bounds` beside the IR-633
    `id` and `origin` projection.
- id: FR-019-AC-42
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Two otherwise equal CG Process descriptors with absent and present-empty manifest `bounds`
    remain distinct typed values. Changing only between those forms leaves process settlement's disposition
    and request-validation precedence unchanged; run-limit defaults are not proof bounds.
- id: FR-019-AC-21
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded item with any unadvertised
    boundable `extent.domains[].kind` settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability`
    naming the offending domain kind and candidate, regardless of `finite_bound_available`; non-boundable
    kinds are not compared (QSpec FR-290-AC-13).
- id: FR-019-AC-22
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded item with every boundable
    `extent.domains[].kind` advertised and the item's `finite_bound_available=false` settles `unsupported`,
    warned, with `unsupported_projection`/`unbounded-extent`. CG reads that flag from the item and does
    not derive it from the domain kinds (QSpec FR-290-AC-13).
- id: FR-019-AC-23
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; bounded row depends on QSL-654). Changing only a bounded item's proof-bound
    numeric maximum leaves its process-provider `supported` or `unsupported` disposition and cause unchanged.
- id: FR-019-AC-13
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'Settling a process-provider item reaches no plugin: a descriptor whose identity names a non-existent
    executable settles identically to one with an ordinary identity, apart from the backend each names,
    and starts nothing, and a descriptor whose identity names an executable that records its own start
    leaves no record. A mutant arm that starts or resolves the identity as a process either changes the
    first disposition or leaves the record, and fails this.'
- id: FR-019-AC-14
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: The process-provider arm's return type is `Disposition`, which has no terminal-value, verification-result
    or artifact member, so settling returns none of them. A change that returns one does not compile against
    that type.
- id: FR-019-AC-15
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-633). CG's `BackendDescriptor` has a typed `ProviderOrigin` projecting exactly
    QSL layer R's `Linked` and `Process` meanings (QSL FR-288, ADR-029 PV-1); layer R owns that vocabulary.
    The driver projects each descriptor from `Registry::descriptors()` once into CG's descriptor, copying
    `id` and advertised pairs unchanged and mapping `origin()` exhaustively, `Linked` to `Linked` and
    `Process` to `Process`, with no wildcard or origin inference from identity, manifest or provider bytes
    or a side map. Two registry descriptors identical except for origin produce CG descriptors identical
    e
- id: FR-019-AC-24
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'PLANNED (IR-338). Given QSL FR-335''s `value-validity` claim rooted at an unbounded `s: Set<Int[0,9999]>`,
    an empty candidate registry settles `unsupported` with a warning naming `value-validity` and emits
    no artifact or Kani outcome; one linked Kani candidate advertising (`value-validity`, `bounded`) with
    `finite_bound_available=true` settles the unbounded item `requires-bound` without substituting a bound
    or emitting an artifact; a separately requested bounded `ProofBound::Cardinality{maximum: 8}` item
    with that same sole candidate settles `supported` under its own request index, while the un'
- id: FR-019-AC-25
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'The normative Kani form set is the complete V1 proof set defined by IR-269 and FR-015: scalar
    families Boolean, integer, rational, decimal, IEEE float, text, enum and quantity; record and tuple
    composite equality; function application; precondition, postcondition and invariant `BoundClause`
    roles; and bounded collection and graph forms. V1 `ClauseKind` has no frame variant. State before/after
    and frame obligations are typed V2 `StateFrame` items with the `contract` or `frame` role, included
    where FR-015 defines their typed lowering. Each item is identified by its typed IR tag/form, family,
    ope'
- id: FR-019-AC-26
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For any form in FR-019-AC-25 with one validated Kani candidate, a bounded extent settles `supported`
    exactly when the form's FR-015 lowering succeeds and the candidate advertises the bounded mode; negotiation
    creates no artifact as part of settlement.
- id: FR-019-AC-27
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: With one Kani candidate, a typed form outside the normative set in FR-019-AC-25 settles `unsupported`,
    warned, with `unsupported_projection`/`unsupported-requested-capability` naming its typed tag/form
    and Kani; no artifact is emitted and no generator is invoked. A malformed in-set form settles with
    the applicable typed FR-015 validation refusal.
- id: FR-019-AC-33
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For a valid bounded typed form inside FR-019-AC-25 that matches one specific Planned subpath
    in TC-030's status register and does not match an Implemented subpath, negotiation returns `unsupported`
    with that subpath's single exact typed refusal cause, emits no artifact, and never settles `supported`
    before its renderer is implemented; this refusal remains distinct from the out-of-V1 refusal in AC-27.
- id: FR-019-AC-28
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: A form refusal under FR-019-AC-27 is distinct from an empty candidate set, which retains the
    missing-capability cause, and from multiple candidates without a named backend, which retains `invalid_capability`/`ambiguous-backend`
    naming each candidate in candidate order.
- id: FR-019-AC-29
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: After request and candidate/manifest validation select one Kani candidate, Kani checks the
    typed IR form before its extent and advertised-mode rules; if both the form and mode/extent are unsupported,
    the form refusal of FR-019-AC-27 settles the item.
- id: FR-019-AC-30
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For the same typed IR tag/form, changing only its display or debug text leaves Kani's disposition,
    cause and named form identity unchanged.
- id: FR-019-AC-31
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For a Kani-supported unbounded form against a `bounded`-only advertisement, `finite_bound_available=true`
    settles `requires-bound`, with no substituted bound and no artifact.
- id: FR-019-AC-32
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: When a caller supplies a finite bound in a new bounded request for a Kani-supported form, negotiation
    settles `supported` and preserves the exact supplied proof-bound value, domain kind and domain identity.
- id: FR-019-AC-34
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With one EN-1 candidate, an advertised admitted TP-1 through TP-4, TP-5,
    SG-1 through SG-3, or HP-1 model/HP-2/HP-3/HP-6 form settles through its explicit-state arm with the
    recorded form and model-or-protocol subject intact; an unadvertised form or HP-1 code claim does not
    settle `supported` through EN-1.
- id: FR-019-AC-35
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With one SMT temporal candidate advertising the item's kind and form, TP-1
    through TP-4 route only to an EN-2 or EN-3 method ADR-018 assigns to that form and subject; TP-4 does
    not route to EN-3, and a form absent from the candidate's advertised form set settles `unsupported`
    with no engine request.
- id: FR-019-AC-36
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). A TP-5 whole-refinement `temporal-satisfaction` item retains its safety and
    optional liveness halves on the concrete subject through EN-1, and the same whole item reaches an
    advertising SMT arm for RE-3 refutation; a separately requested RE-2 `operation-contract` item reaches
    the SMT arm for per-step safety checking, but its disposition cannot substitute for the whole-refinement
    item. An arm lacking the required TP-5 form settles the item `unsupported`.
- id: FR-019-AC-37
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). HP-1 model, HP-2, HP-3 and HP-6 reach an advertising EN-1 arm; only HP-2
    reaches an advertising SMT arm under its applicable EN-2 or EN-3 method. HP-4 and an unadvertised
    HP form settle unsupported before an engine, HP-5 stays on its possible-property path, and an HP-1
    code claim is not settled as an EN-1 model claim.
- id: FR-019-AC-38
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). An advertised SG-1, SG-2 or SG-3 item reaches EN-1 with its recorded form;
    only advertised SG-1 reaches an SMT arm. SG-2 or SG-3 against an SMT-only candidate settles unsupported
    and reaches no SMT engine.
- id: FR-019-AC-39
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). A selected reduction reaches only a candidate advertising that exact reduction,
    with the selection preserved in the routed request. Symmetry declarations and POR enter the obligation
    identity, so their reduced and unreduced requests for one claim have distinct obligation identities;
    a state constraint changes request identity but leaves obligation identity unchanged. An unadvertised
    selection or POR on a subject with no footprints settles `unsupported` without an artifact, and no
    arm drops or adds a reduction.
- id: FR-019-AC-40
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). For an item over a weak `parallel`, a candidate advertising the subject's
    resolved `tso` or `ra` model receives that exact model; a candidate lacking it settles `unsupported`
    with no artifact, and no arm weakens the resolved model. An `sc` subject is a separate strong-model
    control and does not take the weak-memory branch.
- id: FR-019-AC-41
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With otherwise equal form and kind advertisement, model and protocol subject
    classes are read separately; an arm that discharges only one class settles the other `unsupported`,
    while a supported protocol item retains its protocol transition-system subject identity through routing.
- id: FR-019-AC-43
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Two driver items selecting distinct `expression` occurrences of one checked `integer.add` node
    and different direct `numeric.narrow` occurrences in one admitted ContractPackage settle independently
    under the same Kani descriptor. Each preflight result is classified from its own selected result context;
    changing the selected narrow changes that item's typed result identity or refusal without changing
    the other. A node-only classifier or package-wide choice of either bound fails this control.
- id: FR-019-AC-44
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For a V2 scalar claim, an absent `OwnResult`, an application occurrence absent from the admitted
    source, a conversion occurrence from another source or unrelated to the application, a missing conversion
    context root, or a ContractPackage source identity different from the checked source settles `invalid-request`
    with a typed form/context cause and no artifact or harness. Request and candidate/manifest defects
    retain their earlier precedence; for a valid candidate this selected-form refusal precedes extent
    and advertised-mode refusal. Neither a source message nor a caller maximum repairs the se
- id: FR-021-AC-1
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Abort the whole request on one refusal or assign a schedule root to a refused item.
- id: FR-021-AC-2
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Compare only with the former runtime port or build the direct application from emitted source,
    so two matching errors pass.
- id: FR-021-AC-3
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Read function identity from emitted source, so a mutated schedule and claim map agree falsely.
- id: FR-021-AC-4
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Collapse distinct input refusals into one generic cause.
- id: FR-021-AC-5
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Validate arguments in a different order or schedule `function.call` after body work.
- id: FR-021-AC-6
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Schedule an unsupported-capability function as applicable or defer its refusal to a meter charge.
- id: FR-021-AC-7
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Keep a fixed depth ceiling, recursively traverse the V2 graph, or emit nested Rust expressions;
    the deep compile case refuses or overflows.
- id: FR-021-AC-8
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Mark a kernel-mode package applicable.
- id: FR-021-AC-9
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Turn a denied `function.call` into a completed value or apply the denied charge.
- id: FR-021-AC-10
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Schedule a function with a reference-typed signature instead of its typed blocker.
- id: FR-021-AC-11
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Merge distinct upstream blockers or schedule a model, state, temporal or protocol node.
- id: FR-021-AC-12
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Insert a stub instruction for a body whose lowering or classifier failed.
- id: FR-021-AC-13
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Order claims by function name alone or by request position, changing output under permutation.
findings:
- fnd: FND-145
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-14
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '231'
  excerpt: Return the shadow harness without its refinement obligation, or emit an obligation whose abstraction
    function lives in the shadow's own module, so the shadow's representation is read back into the production
    value and the comparison follows any shadow mutation.
  finding: 'Inherited FR-028-AC-14 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-146
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-15
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '232'
  excerpt: Build production values by a constructor that ignores the oracle's environment, or skip a case
    the environment refuses to construct, so an oracle that declares an optional field required agrees
    on every verdict and count; skip a case whose production outcome is `Refused`, or compare verdicts
    only, so a production subject that charges one pair too many, or refuses a legal pair, agrees with
    the shadow.
  finding: 'Inherited FR-028-AC-15 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-147
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-16
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '233'
  excerpt: Report every run as `exhaustive`, or enumerate only the first cap cases of a larger domain
    and call it `exhaustive`, so a sampled run reads as a complete one.
  finding: 'Inherited FR-028-AC-16 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-148
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-17
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '234'
  excerpt: Map a verified shadow harness to `production_proved`, or report the shadow harness's verdict
    when the refinement run disagrees.
  finding: 'Inherited FR-028-AC-17 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-149
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-18
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '235'
  excerpt: Leave an abstraction undischarged (for example the omitted payload of an absent slot) and pass;
    or let the shadow and its expectation share one bound-reading helper and list none, so a mutation
    of that helper moves both and the proof holds vacuously (ADR-011 FB-10); or mark a bound-reading helper
    `order_only` to escape the mutation rule.
  finding: 'Inherited FR-028-AC-18 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-150
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-19
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '236'
  excerpt: Refine over a smaller integer range than the shadow draws, so the cases the shadow proves and
    the production subject was never compared on coincide only in the cases that were.
  finding: 'Inherited FR-028-AC-19 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-151
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-20
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '237'
  excerpt: Treat an unbounded integer leaf as the whole `i64`, a recursive declaration as depth one, or
    an unsupported family as a production harness, so a harness exists that neither covers the declared
    domain nor says it does not.
  finding: 'Inherited FR-028-AC-20 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-152
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-22
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '240'
  excerpt: A suite that mutates only the production side, so a shadow that always answers `true` verifies;
    or only the shadow side, so a production subject that never compares a field agrees; or counts a permutation
    of an order-only helper as a mutant the machinery must catch.
  finding: 'Inherited FR-028-AC-22 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-153
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-23
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '241'
  excerpt: Record the version of the first `kani` or `rustc` on `PATH` rather than the one the run launched,
    or omit the solver, so two runs under different backends read as one.
  finding: 'Inherited FR-028-AC-23 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-154
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-028-AC-24
  related_ids: []
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  lines: '239'
  excerpt: Run the refinement in the test process with no ceiling, so an unbounded domain hangs the lane;
    or report a refinement that hit a ceiling as `not_run` or `sampled`, so a run that never finished
    reads as one that did.
  finding: 'Inherited FR-028-AC-24 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-155
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-029-AC-17
  related_ids: []
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '402'
  excerpt: Add a proof strength to FR-028-AC-17 without a row here, or map it through a wildcard arm to
    `Proved` or `Failed`, so a verified shadow harness whose refinement hit a ceiling has no defined output
    or reads as proved.
  finding: 'Inherited FR-029-AC-17 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-156
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-029-AC-19
  related_ids: []
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '379'
  excerpt: PLANNED/GATED (IR-806/QSL-640). After the cross-outcome disagreement rule, verified shadow
    CeilingReached yields Incomplete(ResourceExhausted); nonzero Exhausted with QSL-derived complete coverage
    yields Proved n; remaining completed nonzero verification yields Tested. Tested is excluded for disagreement,
    ceilings and zero checks; tightening or missing coverage cannot promote it to Proved.
  finding: 'Inherited FR-029-AC-19 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-157
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-029-AC-21
  related_ids: []
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '381'
  excerpt: 'PLANNED (IR-806): `run_terminal_value` has no proof-subject or proof-strength input and `TerminalPairError`
    has no variant for a bounded-shadow outcome; adding either to the map is a deliberate, tested change
    rather than a wildcard arm.'
  finding: 'Inherited FR-029-AC-21 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-158
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-029-AC-22
  related_ids: []
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '382'
  excerpt: PLANNED (IR-806). The strength set enumerated by the map equals FR-028-AC-17's closed set;
    adding a strength without its projection and outcome row fails the build or test rather than entering
    a wildcard mapping.
  finding: 'Inherited FR-029-AC-22 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-159
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-029-AC-23
  related_ids: []
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '383'
  excerpt: PLANNED/GATED (IR-806/QSL-640). A stopped backend run retains its ordinary timeout/memory inconclusive
    classification; a stopped refinement retains FR-028-AC-24's CeilingReached projection and recorded
    ceiling. Neither stop, nor QSL settlement resource refusal, becomes completed NotExhausted evidence
    or Tested.
  finding: 'Inherited FR-029-AC-23 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-160
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-029-AC-24
  related_ids: []
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '384'
  excerpt: 'PLANNED/GATED (IR-806/QSL-640). After common claim/source/node/bounds/O-09 validation, retained
    refinement_failed/Disagreed evidence for the same valid claim yields Failed/CgDefect for verified,
    falsified, inconclusive and cover-unsatisfied backend outcomes, including zero checks, an operand
    field x declared Int[0, 9] but supplied as x: 12, missing/fault native observation and a falsified
    replay agreeing on its retained case; no early operand/native setup refusal loses this disagreement,
    which precedes resource, vacuity and agreement rows. Missing observations are never fabricated.'
  finding: 'Inherited FR-029-AC-24 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-161
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-029-AC-25
  related_ids: []
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '385'
  excerpt: 'PLANNED (IR-806). With no retained refinement disagreement, every bounded_shadow inconclusive
    reason maps through the ordinary rows: vacuous proof and cover-unsatisfied are Proved0; timeout is
    Incomplete(TimedOut); memory and unwind exhaustion are Incomplete(ResourceExhausted); no-verdict and
    missing-summary are Failed; failure-without-counterexample takes the cause rows (FR-029-AC-32 to FR-029-AC-35).
    No vacuous outcome is coerced into verified evidence.'
  finding: 'Inherited FR-029-AC-25 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-162
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-029-AC-26
  related_ids: []
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '386'
  excerpt: 'PLANNED/GATED (IR-806/QSL-640). The converter preserves the actual QSL record category/cause:
    Proved0 remains inconclusive/KaniVacuousProof and is never promoted by its payload. Zero-count backend
    cases use AC-25, not an unreachable verified-count-zero row.'
  finding: 'Inherited FR-029-AC-26 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 9
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++