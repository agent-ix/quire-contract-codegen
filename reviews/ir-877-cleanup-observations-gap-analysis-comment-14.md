<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 14/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-235 | high | coverage | Inherited FR-034-AC-34 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-236 | high | coverage | Inherited FR-034-AC-35 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-237 | high | coverage | Inherited FR-034-AC-36 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-238 | high | coverage | Inherited FR-034-AC-37 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-239 | high | coverage | Inherited FR-034-AC-38 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-240 | high | coverage | Inherited FR-034-AC-39 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-241 | high | coverage | Inherited FR-034-AC-40 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-242 | high | coverage | Inherited FR-034-AC-51 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-243 | high | coverage | Inherited FR-034-AC-52 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-244 | high | coverage | Inherited FR-034-AC-53 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-245 | high | coverage | Inherited FR-034-AC-54 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-246 | high | coverage | Inherited FR-034-AC-57 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-247 | high | coverage | Inherited FR-034-AC-58 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-248 | high | coverage | Inherited FR-034-AC-59 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-249 | high | coverage | Inherited FR-034-AC-60 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-250 | high | coverage | Inherited FR-034-AC-61 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-251 | high | coverage | Inherited FR-034-AC-62 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-252 | high | coverage | Inherited FR-034-AC-63 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-031-AC-20
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). Under real Kani, a harness that calls a bundle oracle with unconstrained
    operands at which IR''s discharge does not hold fails with Kani''s arithmetic-overflow check and a
    counterexample: the oracle of `x < 5 && x * 2 <= 10` over `0..=10` under `reject`, called with `x`
    unconstrained, fails the "attempt to multiply with overflow" check, and the counterexample''s playback
    value satisfies `x < 5` and overflows `x * 2` (any such value is valid; the assertion does not name
    one); a hand-written probe harness that assumes the declared domain on `x`, calls the same bundle
    oracle an'
- id: FR-031-AC-21
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). For each of add, subtract and multiply, the bool the Kani bundle oracle
    returns equals the `bool` inside `Completed` of the native oracle of the same typed expression, on
    every vector of a grid inside the declared domain at which the expression's guards pass and IR's discharge
    holds, including the domain's edges, with the bundle oracle compiled and run natively in a generated
    crate against `quire-contract-runtime` in plain `cargo test`, with no Kani. This is the differential
    that holds the two implementations of the rule in step.
- id: FR-031-AC-22
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The generated source of
    a native oracle with an arithmetic node aliases `quire_exact` as `rt`, takes `&mut rt::Meter`, returns
    `rt::Outcome<bool>`, and contains no `quire_contract_runtime::exact` path; a generated crate that
    compiles and runs it, with divide and remainder nodes and with add, subtract and multiply nodes in
    the same oracle (the construction Z-3, `y <= 0 && x % (y + 11) <= 10`, is one such oracle), resolves
    one `quire-exact`; CG''s own `Cargo.toml` depends on `quire-exact` directly from `agent-ix/quire-exact`,
    and '
- id: FR-031-AC-26
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The generated source of
    a native oracle holds no `quire_contract_runtime::operators` path and no `and_short_circuit`, `or_short_circuit`
    or `implies_short_circuit`; a short-circuit connective is an inline `match` over the left operand's
    `rt::Outcome` that returns the first stop and does not evaluate the right operand when the left decides,
    and a total connective evaluates both operands in order and returns the first stop, none charging
    any point and none calling `evaluate_boolean`; in a generated crate the outcomes of FR-031-AC
- id: FR-031-AC-23
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602). The Kani bundle oracle renders Divide over a `reject` type as the infix `/`
    and Remainder as `wrapping_rem` on `i64`, one operator or call per arithmetic node, contains no `exact::`
    call and no `modulo`, `div_euclid`, `rem_euclid`, `checked_`, `saturating_` or `overflowing_` method,
    no `wrapping_` method other than `wrapping_rem` on a remainder node, takes no meter, returns `bool`,
    and its file states that its arithmetic is checked by Kani. `generate_kani_bundle` over a divide or
    a remainder over a `reject` type generates a bundle, and over a `saturate` divide or remainder it
- id: FR-031-AC-24
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-602). Under real Kani, a harness that calls the bundle oracle of the construction
    Z-1 (a divide) with unconstrained operands fails the checks "attempt to divide by zero" and "attempt
    to divide with overflow", each with a counterexample, and one that calls the bundle oracle of the
    construction Z-3 (a remainder) with unconstrained operands fails the check "attempt to calculate the
    remainder with a divisor of zero" and no other arithmetic check; a probe harness that assumes the
    declared domain and the guards, calls the same oracle and discards its value verifies, for the divide
    `10 / '
- id: FR-031-AC-25
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite (its native side calls the kernel). For divide
    and for remainder, the bool the Kani bundle oracle returns equals the `bool` inside `Completed` of
    the native oracle of the same typed expression, on every vector of a grid inside the declared domain
    at which the expression's guards pass and the divisor is non-zero and IR's discharge holds, including
    the domain's edges and R-1 and R-2, and including the remainder `y != 0 && x % y <= 0` over `i64::MIN..=i64::MAX`
    at `x = i64::MIN`, `y = -1` (native `Completed(true)`, bundle `true`, where a raw `%` pa
- id: FR-032-AC-1
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). A public
    consumer passes the routed harness's `ScalarObligationIdentity` directly to the scalar builder; its
    typed call constructs neither `KaniObligationIdentity` nor V1 `ReplayPackage`. After the ordered setup
    checks, an otherwise valid request returns unsupported-capability only while a named step-5 capability
    is unavailable; when all are available it returns an admitted plan, and authenticated completion enables
    replay/conversion. Conversion of a capability refusal returns no settlement rather than a synthe
- id: FR-032-AC-2
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631). The builder decodes actual routed-harness playback in persisted symbolic
    call order through the Kani output adapter; another harness name, missing/extra operand, wrong byte
    width, playback above the builder-owned 8 MiB per-harness ceiling or malformed assertion block yields
    the distinct typed refusal under the stated precedence before replay.
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). Each operand
    at its own lower or upper bound is admitted and the adjacent out-of-domain value is refused before
    replay. The admitted QSL inputs preserve the catalogued operator, operands, operand ranges, result
    range and harness limits. Another node/operation, altered operand schema or replay limit differing
    from the retained original generation context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-16.
- id: FR-032-AC-4
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The driver's
    actual native observation of the proved generated oracle is required and tied to this operand vector,
    limits and canonical proof-content identity; the checked completion requires driver authentication;
    a bare outcome/digest echo, missing observation, another artifact's observation or fresh regeneration
    that removes the arithmetic mutation refuses before QSL evaluation. Playback bytes and Kani falsification
    alone cannot supply a generated outcome.
- id: FR-032-AC-5
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The public
    consumer executes the same proved generated artifact and invokes QSL between the builder and converter.
    QSL compares its actual native observation with authoritative exact operator evaluation using the
    four-operator projection owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-37, without adding refusal-cause or accounting-counter equality; changing the generated arithmetic
    while keeping original node/operator identity changes the measured parity result. `Undefined`, `Inco
- id: FR-032-AC-6
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). For every
    `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete` and other `Refused` outcome,
    the converter compares the report's full claim to the retained sent claim and actual sent wire against
    the original proving context before conversion; a changed package/function/source/dependency or original
    limit refuses even with an equal claim. At the immutable-report/retained-binding boundary, an absent
    report, another result/run, or changing only the obligation, node, occurrence, kind, canonica
- id: FR-032-AC-7
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). QSL's divergence
    between generated and authoritative exact outcomes becomes a CG lowering fault and `Failed`; agreement
    despite Kani falsification becomes `Inconclusive(ScalarAgrees)`; native `Incomplete`/execution fault
    becomes `GeneratedFault` and `Failed` while exact `Incomplete` becomes `Incomplete(ResourceExhausted)`.
    `RefusedInput` preserves `invalid_runtime_input`; other typed QSL refusals follow the existing refusal
    map and non-fault refusals retain their code. This scalar route never returns `Reproduce
- id: FR-032-AC-8
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The real
    QSL-emitted `x + 1` with input `Int[0,9]` and result `Int[0,10]` exercises routed generation, actual
    Kani arithmetic-mutation playback and same-proved-artifact native observation. A separate bounded-addition
    fixture with both operands and result in `[-1000,1000]` exercises an admitted operand pair whose exact
    result is outside the result range. The expected exact result is derived from the actual retained
    playback, and its correct refusal is never labelled a source violation. The fixture ranges and wit
- id: FR-032-AC-9
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (authoritative metadata access and CG retention). Independent
    canonical-preimage checks include only the claimed application node, authentic expression occurrence,
    authoritative scalar kind and one argument per operand position in actual draw order, each with position
    ordinal, typed `OperandIdentity` and harness operand range. Actual graph references use `GraphChild
    { node_id }`; inline literals use `InlineLiteral { node_id, occurrence_key, position }` from the authentic
    application/occurrence and position, with their value in the singleton range. QSL emits no ext
- id: FR-032-AC-10
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family retention and actual CG/QSL conformance).
    When the authoritative scalar kind and retained positional metadata are available, the public shared
    mint produces the identity for the independently checked exact O-09 canonical scalar preimage, and
    the actual QSL operator route accepts it. A copied encoder, stand-in verifier or synthetic success
    cannot pass.
- id: FR-032-AC-11
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and actual CG/QSL route).
    When the obligation digest differs from the retained positional preimage, after an otherwise membership-valid
    preimage member change with the old digest, the consumer observes QSL's typed `ScalarIdentity::Obligation`
    refusal with claimed/recomputed digests and `stale_dependency`/`revision-mismatch` before comparison.
- id: FR-032-AC-12
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and actual CG/QSL route).
    When a correctly minted claim invents an application occurrence or operand child, or misstates an
    inline literal's value/singleton range, the consumer observes the corresponding merged typed membership
    refusal before identity comparison; self-consistent hashing cannot admit the invented metadata.
- id: FR-032-AC-13
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and CG builder). When
    the owning mint cannot encode the preimage, the public `parity_obligation` returns `IdentityEncodeError`
    with no identity, and the CG builder returns typed setup refusal without replay. No claimed digest
    or usable sentinel can replace that error.
- id: FR-032-AC-14
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (actual CG scalar builder/converter). Adding the scalar builder/converter
    preserves existing function/clause carry-only behavior and function/frame identity encoding; none
    is redirected through scalar parity or changed by its positional argument rule.
- id: FR-033-AC-1
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-635 verified Eq graph-child constructor controls and Ne parameter/parameter and
    parameter/literal controls, including operator mismatch refusal; Ne literal/literal is AC-14 and inline
    operands are AC-16 and AC-17, all PLANNED under IR-814; falsified and generated-artifact routes planned,
    IR-806). A public consumer builds the equality-parity request directly from the composite node and
    original proving context. CG submits the original source/package and node selector to QSL and accepts
    only its admitted same-package/node result under QSpec FR-322. CG validates admitted node/occurren
- id: FR-033-AC-2
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Canonical round trips preserve every listed leaf family, including beyond-i64
    composite integer leaves as ExactInteger (while top-level Integer remains i64), IEEE NaN and signed-zero
    bits, normalized rational, retained decimal, delimiter-containing Unicode text, enum member, exact
    quantity/unit and reference identity, and every listed composite kind. Mixed nested values retain
    declaration/slot/member/presence/arity/order/multiplicity. No decoder claim depends on a currently
    supported Boolean/i64 harness alone.
- id: FR-033-AC-3
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PLANNED/GATED. Noncanonical JCS/tag/scalar/escape spelling, unknown declaration or unit/reference
    identity, wrong enum/union member, missing/extra or wrongly present record slot, wrong tuple arity,
    excess cardinality, out-of-domain leaf, duplicate set/ordered-set element under authoritative equality
    or altered canonical element order yields a typed operand/decode refusal before exact evaluation when
    that stage is reached, after common claim checks and same-valid-claim Disagreed precedence. An admitted
    claim whose operand field x is declared Int[0, 9] but supplied as x: 12 with Disagreed yields'
- id: FR-033-AC-4
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Public caller input exactly at the configured encoded-byte limit is admitted
    and one byte over refuses before parsing; occurrence/work exhaustion refuses with checked accounting.
    A 100,000-link admitted recursive value decodes, clones, compares, formats redacted Debug and drops
    on a 512 KiB native stack without a depth refusal. Its one-byte-too-small budget refuses truthfully;
    launcher capture limits alone cannot satisfy this check.
- id: FR-033-AC-5
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. The retained actual assertion playback reconstructs operand values by the FR-025
    original-node/leaf bindings; another harness or persisted binding schema refuses claim binding. For
    the same admitted claim, swapped playback leaf order, missing/extra playback leaf or wrong primitive
    width refuses reconstruction only after Disagreed precedence; retained Disagreed remains Failed/CgDefect.
    Literal payloads remain their singleton value and absent/null/present states remain distinct. Removing
    or changing a leaf binding changes reconstruction or refuses, rather than silently decoding an
- id: FR-033-AC-6
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. For falsified replay the driver executes the same proved generated artifact
    and supplies its actual native outcome/count; QSL receives this observation, the retained shadow result/count
    and the canonical content tie. Another artifact/context, fresh regeneration that removes the mutation
    or changed original limits refuses at claim binding. For the same admitted claim, missing observation
    cannot mask Disagreed/Failed/CgDefect; without Disagreed it refuses a replay continuing beyond F-1
    before exact evaluation, with no fabricated observation. A canned observation or Kani transcript
- id: FR-033-AC-7
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public F-7 converter controls; IR-806 original-artifact path planned).
    For an identity-valid claim reaching row F-7, actual QSL exact equality/count divergence from a falsified
    shadow maps to CG-defect `Failed`; an assertion-only mutation with otherwise agreeing equality/count
    maps to `Inconclusive(ScalarAgrees)` retaining the CompositeEquality claim/outcome. Neither maps to
    `Refuted`. Independently mutating equality result and occurrence-pair count is detected, including
    no early exit after an unequal pair.
- id: FR-033-AC-8
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. CG submits the original claim's operand declarations and literal singleton domains
    and consumes QSL-derived bound keys without a CG-authored declared-bound list. Omitting a bounded
    position or using an empty list with declared/unbounded/recursive keys leaves it uncovered; unknown/duplicate/kind-mismatched
    keys refuse. Exact declared coverage and a wider legal harness range cover; a tightened range/cardinality/depth
    does not silently cover under ADR-021 TX3. Enum Variants bounds cover only when every source-declared
    variant is included; a Variants bound naming an undeclared varia
- id: FR-033-AC-9
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-666 report binding covered; IR-635 Eq-only verified source/package/node/occurrence
    prechecks and genuine invocation covered; generated-artifact authentication and falsified invocation
    planned, IR-806). CG compares `report.claim()` with `CompositeIdentity::new` over the sent obligation,
    node, occurrence, operator, obligation kind, harness bounds, limits, content identity and complete
    observation: falsified operands, shadow verdict/count, native observation/cause and refinement, or
    verified SUCCESS count and refinement. Changing each CG-owned retained sent-identity member independent'
- id: FR-033-AC-10
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Each lawful generated family exercised by the completed route has a real verify
    and falsify control with retained backend evidence and mutation-killed same-artifact replay; all canonical
    families have the AC-2/3 decode checks. Families awaiting shadow generation or upstream admission
    remain explicitly unsupported/gated and are reported as gaps, rather than counted as passing end-to-end
    coverage. Open real Text-profile, nested set/bag performance and source-unbounded coverage limitations
    remain explicit; API presence or rational substitution supplies no generated-family acceptanc
- id: FR-033-AC-11
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-635 Eq graph-child positional O-09 controls and Ne parameter/parameter and parameter/literal
    controls; Eq self-comparison is covered; Ne literal/literal (AC-14), Ne self-comparison (AC-15) and
    the inline-operand refusals (AC-16, AC-17) are PLANNED under IR-814. Native integration follows from
    AC-6 (IR-806); the harness identity record is FR-015-AC-76, and only the Population-keyed metadata
    part has no owning criterion yet; the planner will specify it as a new FR-033 criterion under IR-806).
    CG computes O-09 through the owning typed parity_obligation from the exact claimed applicati
- id: FR-033-AC-12
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-666 direct QSL F-1 precedence controls; IR-635 Eq-only verified constructor and
    QSL membership/unknown-bound refusals before supplied Disagreed; falsified and cross-outcome integration
    planned, IR-806). Both paths carry the same four-state Refinement. Claim/source/node/occurrence/bounds/O-09
    refusal precedes Disagreed; for a valid claim Disagreed wins over operand refusal (including an operand
    field x declared Int[0, 9] but supplied as x: 12), missing native observation, native fault, exact
    limit, refinement ceiling and agreeing replay. No early operand/native setup refusal erases '
- id: FR-033-AC-13
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-666 direct public F-2 to F-6 controls; IR-806 must reach the same rows from a
    CG-built request over the original proving context). On a valid falsified claim without Disagreed,
    a native Incomplete or ExecutionFault wins over an invalid operand, too-small admission limit, too-small
    exact limit and CeilingReached: CG observes GeneratedFault/Failed retaining its NativeCause. With
    a Completed native, an operand outside its declared domain yields RefusedInput/ReplayRefused even
    with CeilingReached; a request limit reached during admission yields Incomplete/Admission with its
    counter, an'
- id: FR-033-AC-14
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814; literal operands are admitted one-field `Integer` records, with bounded-field
    narrowing literals (IR-691) and collection-constructor literals gated). When the original application
    node is a `structural.ne` whose two operands are both composite literals and the requested operator
    is NotEqual, the verified request carries NotEqual and an O-09 of two positional `graph_child` arguments,
    each the literal's own node with empty Bounds and no parameter position.
- id: FR-033-AC-15
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814). When the original application node is a `structural.ne` comparing one parameter
    with itself and the requested operator is NotEqual, the verified request carries NotEqual and an O-09
    of two positional `graph_child` arguments with the same node identity, each retaining the drawn Node-keyed
    bounds naming that parameter and neither deduplicated.
- id: FR-033-AC-16
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814; verified by Inspection of the single conversion that AC-17 tests). If the
    owning operand projection refuses an inline integer literal operand (`InlineInteger`), then the builder
    returns that refusal through `CompositeBuildError::Operands` before any QSL invocation, with no report
    or terminal value and no built argument. The owning contract treats this branch as defensive because
    reader admission already refuses an integer operand of `structural.eq` and `structural.ne`, so no
    admitted fixture can reach it. Both inline refusals pass through the same `CompositeBuildError::Operand
- id: FR-033-AC-17
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814). If the owning operand projection refuses an inline non-integer literal operand
    (`InlineNonInteger`), then the builder returns that refusal through `CompositeBuildError::Operands`
    before any QSL invocation, with no report or terminal value and no new CG error variant. No test of
    this path exists yet.
- id: FR-034-AC-1
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before the exclusive caller lease, matching helper identity, verified monitor/INIT/peer, observer
    readiness and positive typed Dispatch are established, no production backend instruction is created.
    Caller SIGKILL, abort or OOM-equivalent forced death during initialization may start only bounded
    guardian bootstrap, which exits without backend on absent lease or authorization.
- id: FR-034-AC-2
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before any M spawn, O is positively verified outer PID 1 with armed parent-death protection
    and original-exclusive bootstrap authority. Unclaimed inner I is contained by O even if M dies before
    internal child_wait handoff or startup-info reader loss aborts M. Owned outer pidfd termination confirms
    teardown; missing authority refuses before spawn. No host scan, monitor/group exit or successful signal
    proves cleanup.
- id: FR-034-AC-3
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: In ClaimedBootstrap, InitReady or Dispatched, original-caller lease EOF causes guardian cancellation/exit
    and confirmed INIT termination; Dispatched descendants are torn down by the kernel. The test controller
    observes dead-caller cleanup without manufacturing that caller's outcome or evidence.
- id: FR-034-AC-4
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Mutual authority is the specific anonymous connected pair: guardian checks the actual mapped
    creator UID and only the original caller holds its executor endpoint. The bounded executor binds kernel
    credentials of Ready''s actual sender to claimed host INIT/start/pidfd/parent/namespace, not socketpair
    creator credentials alone. Guardian''s creator PID0 is not a host identity. A foreign actor cannot
    rebind a public name or supply replacement lease/Dispatch; another pair, mismatched UID or replayed
    authority refuses.'
- id: FR-034-AC-5
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: C retains actual unreaped Child L; L/O/M/I ownership is verified through host pidfds, start/parent/namespace
    identities and actual outer/inner PID 1. O owns actual M Child and I is M's inner child. Reused, stale,
    incomplete, translated incorrectly or dead identity refuses before Dispatch.
- id: FR-034-AC-6
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The owned tree memory observer is ready before typed Dispatch; failed observer preparation
    releases no backend instruction.
- id: FR-034-AC-7
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Bounded inner startup/identity recovery is required before Dispatch. M death/reparenting or
    malformed/missing info cancels through retained outer INIT authority and yields typed setup refusal;
    complete outer termination is independently confirmed. No host scan or gate EOF authorizes a backend.
- id: FR-034-AC-8
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: After Dispatch, original-caller death cancels double-fork/reparented, setsid, late-born and
    nested-PID-namespace descendants through kernel-owned INIT teardown, including descendants absent
    from previous samples. Observed-PID lists or normal-operation host scans are not cancellation authority.
- id: FR-034-AC-9
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Cancelling one owned namespace leaves concurrent independent runs and unrelated host children
    unaffected.
- id: FR-034-AC-10
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Normal backend completion, backend exec failure, explicit cancellation, timeout, memory excess,
    capture failure and observation failure each require confirmed owned teardown before any proof conclusion.
    Backend exit or valid success output alone never authorizes acceptance.
- id: FR-034-AC-11
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Guardian SIGKILL, abort or OOM-equivalent death during bootstrap, InitReady or immediately
    after Dispatch cannot release an unowned production backend: the guardian is INIT, and its death tears
    down that namespace. With a live original caller, guardian failure always yields a typed refusal and
    never verified/falsified evidence, even after confirmed teardown and beside a valid success report.'
- id: FR-034-AC-12
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Every startup refusal, completion and cancellation settles actual owned roles before conclusions.
    Anonymous controls and report pipe/memfd have no persistent names and end at final close. All report-owner
    death reclaims kernel report backing without Drop/unlink; other assigned artifacts retain surviving-owner
    cleanup and truthful all-owner-death limits. No dead actor is claimed to reap or unlink.
findings:
- fnd: FND-235
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-34
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1877'
  excerpt: Live-C production close_lease_and_observe retains L/O/M/I ownership and final report control
    independently of consumed original lease. No outer kill, bootstrap EOF or parent-death cascade masks
    AC-24's independent pre-escalation EOF and termination checks; ignored-EOF still fails its named actual-behavior
    predicate before cleanup. Missing conservative backing accounting or immutable kernel report sealing
    gives typed refusal and owned cancellation. All roles, private namespace/proc setup, controls, report
    backing and observation use existing whole-run ceilings and original deadline, with no res
  finding: 'Inherited FR-034-AC-34 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-236
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-35
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1878'
  excerpt: PLANNED/UNRUN. L creates private network/root before O, O validates before M, and nested bind
    / / refers to confined O root. Safe backend-only seccomp/privilege installation before Dispatch preserves
    the same positively owned PID and original recipe, with no unfiltered Dispatch or extra surviving
    process/ownership/deadline; policy installation failure is pre-Dispatch, actual recipe exec/failure
    is post-Dispatch with existing bounded handling/no fabricated evidence; trusted installer starts in
    sanitized loader environment and restores original backend environment only at filtered recipe exec;
    I
  finding: 'Inherited FR-034-AC-35 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-237
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-36
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1879'
  excerpt: PLANNED/UNRUN. C inventories actual backend fd0/fd1/fd2. Real OriginalStdin::Open socket input
    and failed fstat inspection refuse before Dispatch; actual production fd1/fd2 are capture pipes verified
    by mapping/inventory Analysis, not caller socket cases. Caller fd0..2 stability throughout setup is
    a trusted precondition; observed capture inconsistency refuses, with no claim to detect every ambient
    mutation. C internally captures/pins OriginalStdin before child/control fd reuse, without a public
    request field; authoritative initial absence/CLOEXEC yields Closed, while later Open inspection EBA
  finding: 'Inherited FR-034-AC-36 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-238
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-37
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1880'
  excerpt: PLANNED/UNRUN. Trusted I retains its exclusive lease through Dispatch; O retains separate final
    delivery after original lease close. All bootstrap/ownership/report/reporter controls remain owned/CLOEXEC
    outside intended mappings and unavailable to arbitrary backend, sibling exec and descendants. I confirms
    non-dumpability after final credentials and backend cannot hold or regain CAP_SYS_PTRACE in I owning
    user namespace; real backend /proc/1/fd, pidfd_getfd and ptrace gates prove protection independently
    of host Yama. Actual leaked-control/protection mutants fail before emergency cleanup; rest
  finding: 'Inherited FR-034-AC-37 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-239
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-38
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1881'
  excerpt: PLANNED/UNRUN. Named fixed SETTLE_RESERVE R is measured/rounded; its finite effective reserve
    stays inside original whole T; short finite ceilings stay admitted with R_eff=min(R,T/2), no minimum-budget
    cause. At finite workdeadline=T-R_eff stop/cancel; normal returns positively settle/reap all owned
    roles, captures and existing creator thread by original T before conclusions; original None/overflow
    never-elapsing work remains admitted and FIRST actual stop starts one R settlement deadline. Confirmed
    workdeadline expiry preserves existing TimedOut classification naming T; unconfirmed settlement
  finding: 'Inherited FR-034-AC-38 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-240
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-39
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1882'
  excerpt: PLANNED/UNRUN. Native guardian/backend-installer policy support is source/cfg Analysis; unreachable
    unsupported-native support is not a fabricated runtime Test. Actual installation failures are separate
    production Tests when available, otherwise unavailable/UNRUN with no Test completion credit. Planned
    mandatory admission context distinguishes BackendIpcExclusion capability support/filter installation
    from TrustedOwnerProtection privilege installation, retaining typed cause provenance without message
    parsing or fabricated errno. The policy terminates the calling process on an unsupported sysca
  finding: 'Inherited FR-034-AC-39 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-241
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-40
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1883'
  excerpt: PLANNED/UNRUN. Only declared trusted role/operation/phase causes use the cross-role representation.
    Actual kind/errno, independently authenticated optional finite provenance and loss survive privately;
    public projection applies only to existing io::Error-cause-bearing refusals. Post-admission observation/settlement
    detail-only results keep their existing shape/mapping, private finite facts/loss and diagnostic-only
    text, with no public marker/carrier/downcast promise or Tool reclassification; generic original custom
    sources/chains are lost explicitly, including valid original boxed declared ope
  finding: 'Inherited FR-034-AC-40 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-242
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-51
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1894'
  excerpt: PLANNED/UNRUN. Given genuine live-I construction preconditions, the child's own acknowledgement
    establishes release-dependent birth after the actual completed ordinary O sample.
  finding: 'Inherited FR-034-AC-51 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-243
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-52
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1895'
  excerpt: PLANNED/UNRUN. Actual retained-I pidfd termination confirmation strictly precedes matching
    kernel report seal; independently observed fabricated-live-I and late-poll controls fail without stored-record
    or ordinal proof.
  finding: 'Inherited FR-034-AC-52 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-244
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-53
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1896'
  excerpt: PLANNED/UNRUN. Actual retained M consuming Child wait/reap strictly precedes matching kernel
    report seal; independent fabricated-reap and late-reap controls fail without cached Some, copied history
    or ordinal proof.
  finding: 'Inherited FR-034-AC-53 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-245
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-54
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1897'
  excerpt: PLANNED/UNRUN. On the independently grounded supported cooperative cancellation path, the actual
    original O gate owner retains the gate through actual I confirmation before actual close; early-close
    and omitted-confirmation controls fail without marker, stage or refusal-label proof. Genuine unavailable
    confirmation at the original cutoff follows AC-38 CleanupUnconfirmed with explicit actual-owner abandonment/right
    disposal and no ordering or cleanup credit, never a renamed fatal close or diagnosis of kernel fault.
    Healthy restored order and unavailable-confirmation refusal are separate control
  finding: 'Inherited FR-034-AC-54 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-246
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-57
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1900'
  excerpt: PLANNED/UNRUN (IR-687). When L awaits its first O arm result, the newly allocated authenticated
    O→L OperationalFailure carries zero ancillary rights.
  finding: 'Inherited FR-034-AC-57 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-247
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-58
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1901'
  excerpt: PLANNED/UNRUN (IR-687). L accepts that negative only from its actual retained O Child PID with
    kernel sender UID/GID0 and the original build/run/source/state/stamp authority.
  finding: 'Inherited FR-034-AC-58 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-248
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-59
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1902'
  excerpt: PLANNED/UNRUN (IR-687). For L→C AwaitArm negatives, C rejects missing, extra, wrong-type or
    substituted rights through owned cleanup, without a zero-right fallback.
  finding: 'Inherited FR-034-AC-59 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-249
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-60
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1903'
  excerpt: PLANNED/UNRUN (IR-687). The negative O pidfd grants C only exact O termination observation
    and existing owned-cancellation signalling, while L retains its direct Child/wait obligation.
  finding: 'Inherited FR-034-AC-60 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-250
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-61
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1904'
  excerpt: PLANNED/UNRUN (IR-687). Negative capability custody grants no Armed, namespace/network, phase,
    report or Dispatch admission.
  finding: 'Inherited FR-034-AC-61 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-251
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-62
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1905'
  excerpt: PLANNED/UNRUN (IR-687). C adopts the authentic O producer stop before L wait and preserves
    the original earliest cutoff, including absent work deadline, without receipt-time reset.
  finding: 'Inherited FR-034-AC-62 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-252
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-63
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1906'
  excerpt: PLANNED/UNRUN (IR-687). C exposes a negative original-cause result only after actual O Child
    wait at L and required L/capture/creator/control-EOF settlement under that cutoff.
  finding: 'Inherited FR-034-AC-63 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 14
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++