<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 10/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-163 | high | coverage | Inherited FR-029-AC-27 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-164 | high | coverage | Inherited FR-029-AC-30 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-165 | high | coverage | Inherited FR-029-AC-31 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-166 | high | coverage | Inherited FR-030-AC-19 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-167 | high | coverage | Inherited FR-030-AC-20 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-168 | high | coverage | Inherited FR-030-AC-21 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-169 | high | coverage | Inherited FR-030-AC-22 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-170 | high | coverage | Inherited FR-031-AC-2 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-171 | high | coverage | Inherited FR-031-AC-6 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-172 | high | coverage | Inherited FR-031-AC-16 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-173 | high | coverage | Inherited FR-031-AC-17 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-174 | high | coverage | Inherited FR-031-AC-27 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-175 | high | coverage | Inherited FR-031-AC-20 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-176 | high | coverage | Inherited FR-031-AC-22 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-177 | high | coverage | Inherited FR-031-AC-26 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-178 | high | coverage | Inherited FR-031-AC-23 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-179 | high | coverage | Inherited FR-031-AC-24 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-180 | high | coverage | Inherited FR-031-AC-25 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-021-AC-14
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Emit a hardcoded charge or result value, require the deleting Runtime `exact` dependency, or
    produce a crate that does not compile.
- id: FR-021-AC-15
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Store every full root-to-node path or an empty path at every call point; the former is quadratic
    and the latter loses source position.
- id: FR-021-AC-16
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Read dynamic location or losses as if a QSL application API populated them.
- id: FR-021-AC-17
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Take function origin from request ordinal or emitted source instead of the binding, allowing
    claim and source to drift together.
- id: FR-021-AC-18
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Claim or present as evidence agreement with the QSL-owned application authority in a test name,
    trace tag, output line, doc comment, TC-031 step or matrix row.
- id: FR-021-AC-19
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Leave a panic token in schedule source.
- id: FR-021-AC-20
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Fold one result distinction (completed, undefined, refused, incomplete, checked-invariant)
    into another in the schedule.
- id: FR-021-AC-21
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Allow `Negate` to reach an unreachable arm instead of refusing it before schedule emission.
- id: FR-021-AC-22
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Let duplicate declaring-node ids survive into the instruction table, collapse pair members,
    or choose a request-order-dependent refusal.
- id: FR-021-AC-23
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Mislabel an admitted `Failed(Bytes)` record as work exhaustion, or synthesize a ContractPackage
    when Contract IR's byte ceiling returns no package so CG fabricates claims or artifacts.
- id: FR-021-AC-24
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Key an item by the call node, applied function's declaring node id and arguments only, so every
    unknown name on one call node shares one key and the second name is reported as `DuplicateRequest`
    and lost (the `zz_unknown` and `aa_unknown` example fails); or key an item by its request position,
    so the same unknown name requested twice yields two entries (the `zz_unknown` twice example fails);
    or put the name ahead of the declaring node id in the key, so `add_fn` sorts before `zz_unknown` (the
    `add_fn` and `zz_unknown` example fails); or compare names case-insensitively, so `aa_unknown` sorts
    be
- id: FR-021-AC-25
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Skip instruction or location charges, check after emission, or expose a partial instruction
    root on refusal.
- id: FR-021-AC-26
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Compute a right operand before testing the left stop condition; a denied left charge then admits
    a right charge.
- id: FR-021-AC-27
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Treat a nested application as an inline recursive V2 term, bind callee by name alone, or accept
    an unbound/shared/cyclic expression edge.
- id: FR-021-AC-28
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Keep the fixed 1 MiB source ceiling or check it only after publishing; the raised-limit deep
    source refuses or leaks partial output.
- id: FR-021-AC-29
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Copy a full path into every record or fail to charge path expansion; the deep map grows quadratically
    or bypasses query limits.
- id: FR-021-AC-30
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Accept `u64::MAX` and increment a `u64` at the first denied charge; the refusal wraps or panics.
- id: FR-021-AC-31
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Count schedule-source compilation as execution evidence, or present an owner-path run as evidence
    without a QSL application API.
- id: FR-021-AC-32
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Keep the deleted Contract Runtime `exact` port reachable through a re-export, wrapper or compatibility
    layer.
- id: FR-021-AC-33
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Classify a directly requested formula or fairness root as the clause's QSL-121 blocker, classify
    a lowered `case` body or call item as `FormMismatch`, substitute the unsupported body root for the
    distinct valid call item and thereby mask the body's refusal, or omit the affected call item's claim
    while its healthy sibling is scheduled.
- id: FR-021-AC-34
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Submit all 129 Call body roots to one Contract IR lowering request, duplicating their transitive
    suffixes until the unchanged 1,048,576-byte ceiling refuses an admitted public-generation chain.
- id: FR-022-AC-1
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Generation dispatches one arm per variant of the closed `BackendKind` through an exhaustive
    `match` with no catch-all, and `GenerationContexts::has` handles each variant; adding a variant without
    either arm does not compile. `Process` requires no context.
- id: FR-022-AC-2
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: For a set of routed Kani items, each record and harness in the output equals what `negotiate_kani_obligations`
    returns for the same items in ascending request-index order with the same context. The one difference
    is that every index inside a record is the driver's request index, and each harness is paired with
    the record whose `harness_symbol` names it.
- id: FR-022-AC-3
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The entry point accepts no backend registry, candidate set, extent or capability kind, and
    constructs no FR-019 `Disposition`.
- id: FR-022-AC-4
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629 internal-invariant test migration). A crate-internal malformed routed Kani
    item whose private backend identity has no CG built-in kind, or converts to a kind other than Kani,
    refuses the whole call as `BackendKindDisagrees`, naming the request index, backend, routed kind and
    converted kind or its absence, with no artifact.
- id: FR-022-AC-5
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Two routed items with one request index refuse as `DuplicateRequestIndex` naming it. A routed
    Kani item without Kani context refuses as `MissingKindContext` naming Kani. Each returns no artifact.
- id: FR-022-AC-6
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: A Kani group-level refusal (for example an unwind outside `1..=1024`, or an unparsable subject
    path) is returned as `Kani` carrying the unchanged `KaniObligationError`, with no artifact.
- id: FR-022-AC-7
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: A Kani group containing an `invalid_request` item is listed in `rejected`, and every Kani item's
    record is returned with no harness. A `DuplicateItem` reported by FR-015 has `first_index` rewritten
    to the driver's request index of the first occurrence. On the FR-330 package path, an identical selected
    scalar occurrence pair also retains FR-039's earlier typed `DuplicateRequest` claim refusal at both
    positions.
- id: FR-022-AC-8
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Every routed item appears exactly once in `items`, in ascending request index, under its routed
    backend and the `KindOutput` variant of its routed kind. An item the arm refuses at lowering keeps
    its typed FR-015 refusal and no harness. An empty routed set returns an empty result.
- id: FR-022-AC-9
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Regeneration from equal inputs is byte-identical, and a permutation of the routed-item slice
    yields an identical result. The same node routed at a different request index or under another candidate
    of the same backend yields a byte-identical harness when the admitted FR-330 package and selected
    occurrence are the same.
- id: FR-022-AC-10
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The Kani arm derives each item's operation and domain from the package alone; the caller supplies
    no claim map, and `x + 1` over a parameter `Int[0, 9]` routes to a supported `quire.op.integer.add`
    harness with arguments `[0, 9]` and `[1, 1]` (the literal at its own value, FR-015-AC-16).
- id: FR-022-AC-11
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'An item with no derivable descriptor keeps its typed refusal and no harness, and its siblings''
    records and harnesses are unaffected. On the FR-330 package path, an absent root retains its exact
    admitted `InvalidInput` accounting and `PackageRefused` output before Kani. On a direct FR-015 request,
    a node absent from the graph is `invalid_request` and rejects the group. A node that does not lower
    or has a refused bound keeps the FR-015 disposition it has when its claim is generated by FR-014 (`requires_bound`,
    `blocked_on_upstream`, `unsatisfiable_bound`). A repeated selected pair on the FR-330 '
- id: FR-022-AC-12
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: '`RoutedGeneration.claim_map` is `Some` after a Kani group and contains one claim per selected
    node/occurrence admitted past FR-039 accounting, retaining any earlier typed `DuplicateRequest` refusal;
    an underivable node has one refused claim of provenance `underived`, ordered by node id and then selected
    occurrence key.'
- id: FR-022-AC-13
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'The Kani arm routes an integer node over bounded parameters to a supported harness whose arguments
    are one range per operand and whose result assertion uses the result bound: `a + b` over `[0, 9]`
    and `[10, 20]` with result `[0, 29]` has arguments `[0, 9]` and `[10, 20]`; `-e` over `[1, 9]` with
    result `[-9, -1]` and `e * f` over `[1, 9]` and `[100, 200]` with result `[100, 1800]` are supported;
    a node with a plain-typed `reference` operand beside bounded typing (FR-014-AC-25) settles `requires_bound`
    and has no harness.'
- id: FR-022-AC-14
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: '`RoutedGeneration.oracle_artifacts` is `Some` after a Kani group and `None` when nothing is
    routed. Its `claim-map.json` equals the complete package-derived claim map, including typed refused
    items; a group with no generated claim still returns its crate. For `x + 1` over a parameter `Int[0,
    9]`, each `Generated` claim''s oracle symbol is defined in the returned `src/lib.rs` and appears in
    the supported harness''s Rust source.'
- id: FR-022-AC-15
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'The Kani arm routes the packages QSL emits for `x + 1` over `x: Int[0, 9]` into `Int[0, 10]`,
    `x + y` over `Int[0, 9]` and `Int[10, 20]` into `Int[10, 29]`, and `-z` over `Int[0, 9]` into `Int[-9,
    0]` (a literal as a reference to its own `value` node, the declared bound on a narrowing `conversion`
    consuming the plain-typed arithmetic node) to supported harnesses with arguments `[0, 9]` and `[1,
    1]`; `[0, 9]` and `[10, 20]`; and `[0, 9]`, each asserting the result against the conversion''s bound
    (`[0, 10]`, `[10, 29]`, `[-9, 0]`). In that shape a plain-Integer parameter beside a bounded one,
    and'
- id: FR-022-AC-16
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Two Kani harnesses with one `harness_symbol` refuse the call as `DuplicateHarness` naming the
    second harness's `module::harness` path, and no harness is overwritten or dropped.
- id: FR-022-AC-17
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). A routed `Process(id)` item appears exactly once with its own identity and
    empty `KindOutput::Process`, without a generation context or membership in `BackendKind::ALL`.
- id: FR-022-AC-18
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). A crate-internal routed `Process(id)` item whose private backend and kind
    disagree refuses the whole call as `BackendKindDisagrees` with `converted=Some(Process(item.backend.identity))`
    and no generated output.
- id: FR-022-AC-19
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). Permuting a routed-item slice containing Kani and multiple process identities
    leaves its request-index-ordered output unchanged.
- id: FR-022-AC-20
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `RoutedGenerationItem.backend` and `.kind` are private while `request_index`
    and `node_id` remain public; external callers cannot create an item with a hand-built `Process(id)`
    or reassign its backend after `from_descriptor` construction.
- id: FR-022-AC-21
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `from_descriptor` constructs Kani for a linked `kani` descriptor and `Process(id)`
    for a process descriptor with the same identity text. An unknown linked identity never produces Process.
- id: FR-022-AC-22
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `from_descriptor` rejects a backend identity different from the descriptor
    identity as `DescriptorBackendMismatch`, naming both; it rejects an unknown linked identity as `UnknownLinkedBackend`,
    naming the backend. Neither refusal produces a routed item.
- id: FR-022-AC-23
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: After FR-019 authenticates two `integer.add` driver items on one admitted ContractPackage with
    distinct selected application/direct-`numeric.narrow` pairs, routed generation retains the pair for
    each item in its claim and harness identity and asserts against its own selected result bound. The
    two generated harnesses have distinct identities and expected ranges; a fresh settlement of a changed
    selection changes or refuses only that item, without borrowing the other bound.
- id: FR-022-AC-24
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: A routed scalar item's retained authenticated selection and its driver lowering position must
    name the same application/conversion pair in the same admitted ContractPackage. A changed pair, missing
    conversion context root or occurrence absent from that package refuses the whole call with a typed
    selection error before any harness. No package-wide bound search, source-text reparse or copied QSL
    context supplies a replacement.
- id: FR-022-AC-25
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The public CG API exposes the concrete `ContractScalarError` carried by `RoutedGenerationError::ContractScalar`,
    so an external driver can match `ContractScalarError::Oracle(OracleGenerationError::SourceTooLarge
    { bytes, limit, setting })` and distinguish the scalar source ceiling from request-accounting and
    other oracle failures without parsing text or naming a private module.
- id: FR-022-AC-26
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Given the checked QSL RR-16 package with two `x + 1` occurrences of one application node, directly
    narrowed to `Int[0, 10]` and `Int[0, 20]`, the driver supplies two distinct request indexes and occurrence
    keys. CG matches each to its own requirement record and selected narrow; both supported scalar outputs
    use the shared application operation and their respective result bounds, and their obligation and
    harness identities differ by application occurrence. Reversing routed-item order leaves these pairings
    unchanged.
- id: FR-022-AC-27
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'For `if p then x + 1 else x + 1`, each occurrence''s generated scalar claim is conditional
    on its own QSL path guard: the same guard occurrence with `holds=true` for the then item and `holds=false`
    for the else item. CG preserves the full outermost-first guard sequence and uses it when forming the
    proof obligation; a result under `p=false` cannot prove the then claim and one under `p=true` cannot
    prove the else claim. An empty path condition remains unguarded.'
- id: FR-022-AC-28
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: If a routed scalar item's occurrence has no entry in the supplied requirement records, its
    key's node disagrees with the authenticated application selection, or the keyed record disagrees with
    the item's result bound, direct conversion occurrence or ordered path condition, CG returns a typed
    refusal identifying the request index and occurrence before emitting an oracle, harness or obligation
    identity for that request; it does not use another record with the same node or reconstruct missing
    values from the IR graph or source text.
- id: FR-022-AC-29
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'Given an admitted package whose complete lowering sequence has bound driver items at indexes
    0 and 2 and an excluded accounting-only position at index 1, a routed item with `request_index` 1
    and the excluded position''s exact `node_id` refuses the whole call as `ExcludedDriverIndex { request_index:
    1, node_id }` before any generation arm, oracle or harness. The same routed backend and descriptor
    at indexes 0 and 2 remain valid controls, and CG never substitutes an envelope position or the next
    bound item''s selection for index 1.'
findings:
- fnd: FND-163
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-029-AC-27
  related_ids: []
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '387'
  excerpt: PLANNED/GATED (IR-806/QSL-640). The distinct typed composite settlement input refuses a missing
    settlement or another claim's node/run/operation/operand/domain/limits/content binding with no value;
    ordinary source-predicate AC-15 does not consume or reject a valid verified-parity settlement as UnexpectedSettlement.
  finding: 'Inherited FR-029-AC-27 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-164
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-029-AC-30
  related_ids: []
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '390'
  excerpt: PLANNED (IR-806). A `bounded_shadow` outcome (verified, falsified, inconclusive or cover-unsatisfied)
    reaches only the composite parity route on the production path and never `run_terminal_value`, because
    `KaniRunOutcome` cannot tell a shadow from a production proof and a verified shadow outcome there
    would read as `Proved`, which FR-028-AC-17 forbids.
  finding: 'Inherited FR-029-AC-30 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-165
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-029-AC-31
  related_ids: []
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '391'
  excerpt: 'PLANNED (IR-806). The parity route fabricates no predicate replay and no terminal cause for
    a bounded-shadow outcome: its terminal value is the one QSL''s report carries under AC-17 and AC-19
    to AC-27.'
  finding: 'Inherited FR-029-AC-31 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-166
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-030-AC-19
  related_ids: []
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  lines: '237'
  excerpt: For both `BoundedComplete { depth }` and `Inductive { depth }`, an SMT proof whose certificate
    FR-314 verifies maps to `Proved` with the same basis/depth and `Certified`; no certificate or an `Unverifiable`
    unchecked rule maps to `Proved` with the same basis/depth and `Uncertified`, retaining the unverifiable
    locus when present. No SMT proof maps to `Trusted`.
  finding: 'Inherited FR-030-AC-19 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-167
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-030-AC-20
  related_ids: []
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  lines: '238'
  excerpt: For either SMT basis, a foreign query, wrong certificate shape, invalid checked proof step
    or missing final empty clause rejected by FR-314 maps to `Inconclusive(CertificateRejected { rule,
    at })` retaining the exact rejection; none produces `Proved`, basis or certification. When cvc5 and
    Z3 also disagree, `CertificateRejected` remains the one terminal cause while the parity failure and
    both verdicts are retained alongside it.
  finding: 'Inherited FR-030-AC-20 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-168
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-030-AC-21
  related_ids: []
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  lines: '239'
  excerpt: SMT/cross-check disagreement without certificate rejection keeps both verdicts and settles
    inconclusive; agreement does not change certification; missing Z3 retains its warning and cvc5 settlement.
    An SMT counterexample becomes `Refuted` only after FR-197 replay reproduces it; replay disagreement,
    non-fault refusal and fault map to their distinct non-proof results.
  finding: 'Inherited FR-030-AC-21 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-169
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-030-AC-22
  related_ids: []
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  lines: '240'
  excerpt: Every expressible accepted SMT backend/checker/replay settlement produces one terminal result,
    never `Tested`; no non-`Proved` result carries basis or certification. Negotiation refusals produce
    no terminal from this map, and neither SMT nor Kani map consumes the other backend's outcome.
  finding: 'Inherited FR-030-AC-22 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-170
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-031-AC-2
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: '504'
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. For Divide and Remainder over a `reject`
    integer type, the generated source of the native oracle calls `exact::divide` with `DivisionProfile::Truncating`,
    with `DivisionMember::Quotient` for a divide and `DivisionMember::Remainder` for a remainder, and
    with `IntegerDomain::Bounded` over the node's interval as the domain, and the oracle's body holds
    no Rust `/` or `%` and none of `checked_div`, `checked_rem`, `wrapping_rem`, `div_euclid`, `rem_euclid`
    or `modulo`, and no division of a quotient and remainder pair.
  finding: 'Inherited FR-031-AC-2 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-171
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-031-AC-6
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: '508'
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer type, a zero divisor
    evaluates to `Undefined(DivisionByZero)` for both Divide and Remainder, for the cases Z-1 `x / 0`,
    Z-2 `0 / 0`, Z-3 `x % 0` and Z-4 `0 % 0`, each built as TC-044 step 5 gives (Z-3 and Z-4 reach the
    zero through the add `y + 11`, which completes with 0), and the oracle does not panic and does not
    return `Refused`. On the tree before the change each case is refused at generation with the interim
    code, and a raw operator would panic with a division-by-zero message.
  finding: 'Inherited FR-031-AC-6 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-172
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-031-AC-16
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: '518'
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer type of `-5..=5`,
    the minimum divided by negative one is out of domain and the minimum remainder negative one is not,
    and neither panics: O-3 `i64::MIN / -1` evaluates to `Refused(DivisionOutOfDomain)` whose member is
    `Quotient` and whose domain is `-5..=5`, with the refusal''s code `division_out_of_domain` and cause
    `quotient-outside-domain`; O-5 `i64::MIN % -1` evaluates to `Completed(true)`, the remainder being
    the exact 0 and `0 <= 5` true. Each is built as TC-044 step 4 gives. On the tree before the change
    each cons'
  finding: 'Inherited FR-031-AC-16 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-173
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-031-AC-17
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: '519'
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite. In a generated crate, the outcome of a divide
    or remainder oracle equals a plain-integer model of IR''s member-only semantics under the truncating
    law, computed with `i128` independently of the emitter and of any `exact::` call (zero divisor: `Undefined(DivisionByZero)`;
    the selected member in the type''s interval: `Completed` with the comparison''s Boolean over that
    member; otherwise `Refused(DivisionOutOfDomain)` with that member), on a grid of dividends and divisors
    inside the declared domain, at its edges and outside it, that includes the two '
  finding: 'Inherited FR-031-AC-17 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-174
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-031-AC-27
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: '521'
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite (it cannot land before the native division
    of AC-2 exists). The code `UnsupportedIntegerDivision` does not exist: the crate''s `GenerationErrorCode`
    has no such variant, nothing under `src/`, `tests/` or `schemas/` spells it, no stable code string
    of that name is emitted, and no diagnostic message that generation can emit names IR-601. On the tree
    before the change the variant exists and a divide or remainder over `reject` is refused with it.'
  finding: 'Inherited FR-031-AC-27 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-175
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-031-AC-20
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: '524'
  excerpt: 'IMPLEMENTED (IR-596). Under real Kani, a harness that calls a bundle oracle with unconstrained
    operands at which IR''s discharge does not hold fails with Kani''s arithmetic-overflow check and a
    counterexample: the oracle of `x < 5 && x * 2 <= 10` over `0..=10` under `reject`, called with `x`
    unconstrained, fails the "attempt to multiply with overflow" check, and the counterexample''s playback
    value satisfies `x < 5` and overflows `x * 2` (any such value is valid; the assertion does not name
    one); a hand-written probe harness that assumes the declared domain on `x`, calls the same bundle
    oracle an'
  finding: 'Inherited FR-031-AC-20 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-176
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-031-AC-22
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: '526'
  excerpt: 'PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The generated source of
    a native oracle with an arithmetic node aliases `quire_exact` as `rt`, takes `&mut rt::Meter`, returns
    `rt::Outcome<bool>`, and contains no `quire_contract_runtime::exact` path; a generated crate that
    compiles and runs it, with divide and remainder nodes and with add, subtract and multiply nodes in
    the same oracle (the construction Z-3, `y <= 0 && x % (y + 11) <= 10`, is one such oracle), resolves
    one `quire-exact`; CG''s own `Cargo.toml` depends on `quire-exact` directly from `agent-ix/quire-exact`,
    and '
  finding: 'Inherited FR-031-AC-22 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-177
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-031-AC-26
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: '527'
  excerpt: PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The generated source of
    a native oracle holds no `quire_contract_runtime::operators` path and no `and_short_circuit`, `or_short_circuit`
    or `implies_short_circuit`; a short-circuit connective is an inline `match` over the left operand's
    `rt::Outcome` that returns the first stop and does not evaluate the right operand when the left decides,
    and a total connective evaluates both operands in order and returns the first stop, none charging
    any point and none calling `evaluate_boolean`; in a generated crate the outcomes of FR-031-AC
  finding: 'Inherited FR-031-AC-26 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-178
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-031-AC-23
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: '528'
  excerpt: PLANNED (IR-602). The Kani bundle oracle renders Divide over a `reject` type as the infix `/`
    and Remainder as `wrapping_rem` on `i64`, one operator or call per arithmetic node, contains no `exact::`
    call and no `modulo`, `div_euclid`, `rem_euclid`, `checked_`, `saturating_` or `overflowing_` method,
    no `wrapping_` method other than `wrapping_rem` on a remainder node, takes no meter, returns `bool`,
    and its file states that its arithmetic is checked by Kani. `generate_kani_bundle` over a divide or
    a remainder over a `reject` type generates a bundle, and over a `saturate` divide or remainder it
  finding: 'Inherited FR-031-AC-23 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-179
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-031-AC-24
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: '529'
  excerpt: 'PLANNED (IR-602). Under real Kani, a harness that calls the bundle oracle of the construction
    Z-1 (a divide) with unconstrained operands fails the checks "attempt to divide by zero" and "attempt
    to divide with overflow", each with a counterexample, and one that calls the bundle oracle of the
    construction Z-3 (a remainder) with unconstrained operands fails the check "attempt to calculate the
    remainder with a divisor of zero" and no other arithmetic check; a probe harness that assumes the
    declared domain and the guards, calls the same oracle and discards its value verifies, for the divide
    `10 / '
  finding: 'Inherited FR-031-AC-24 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-180
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-031-AC-25
  related_ids: []
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  lines: '530'
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite (its native side calls the kernel). For divide
    and for remainder, the bool the Kani bundle oracle returns equals the `bool` inside `Completed` of
    the native oracle of the same typed expression, on every vector of a grid inside the declared domain
    at which the expression's guards pass and the divisor is non-zero and IR's discharge holds, including
    the domain's edges and R-1 and R-2, and including the remainder `y != 0 && x % y <= 0` over `i64::MIN..=i64::MAX`
    at `x = i64::MIN`, `y = -1` (native `Completed(true)`, bundle `true`, where a raw `%` pa
  finding: 'Inherited FR-031-AC-25 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 10
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++