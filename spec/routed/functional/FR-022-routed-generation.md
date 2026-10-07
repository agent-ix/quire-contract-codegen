---
id: FR-022
title: "Generate for routed items per backend kind without re-negotiating"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: depends_on
---
# FR-022: Generate for routed items per backend kind without re-negotiating

## Description

When the orchestrating driver hands this generator the items that FR-019
settled `supported` and that QSL `route` then routed, each with its request
index, its routed backend and that backend's closed backend kind, together
with the admitted IR `CheckedPackageV2`, the generator shall run the
generation arm of each item's backend kind and return that arm's output keyed
by the item's request index. It shall not settle, select or route a backend
again.

This is the CG generation arm of seam S9 (QSL ADR-012 §5 and §7.2 step 4;
ADR-011 T-13 driver step 7 and §9 scenario 7). FR-019 is the `negotiate_*`
arm of the same seam. The two are separate requirements because settling a
disposition and generating for a settled item are different claims: a
settlement names a backend, and generation returns the backend kind's output.
The linked Kani arm may produce an artifact; the process-provider arm produces
none (QSL ADR-029 PV-4).

### What "without re-negotiating" means

The router's results are inputs and are taken as given:

- that the item settled `supported` (only `supported` items route, FR-019-AC-10);
- which backend the item routed to;
- that backend's closed backend kind;
- the extent and advertised-mode settlement FR-019 made for the item.

This generator does not receive a backend registry, a candidate set, an extent or a
capability kind on this path, so it cannot recompute any of them. It
constructs no FR-019 `Disposition` here.

Two checks remain, and neither is a settlement. First, the routed backend
kind must agree with the routed identity: a built-in kind uses
`BackendKind::from_identity`, while `Process(id)` carries the same identity
string as the routed backend (FR-019, QSL ADR-029 PV-4). An item is never sent to
another kind's arm.
Second, the kind's generation arm still accounts every item it is given with
its own typed per-item record. For Kani that record is FR-015's
`ObligationRecord`: a routed item can still be refused at lowering, for
example an IR node with no finite encoding or an unknown node. That record
says what generation did. It does not change the FR-019 settlement, and the
item is never handed to another backend kind's arm instead.

### How the existing Kani generator composes

`negotiate_kani_obligations` selects no backend. Its per-item dispositions are FR-015's lowering accounting
under the Contract IR FR-036 vocabulary. So nothing in it re-runs FR-019, and
it is not split. The Kani generation arm calls it unchanged, once, with one
FR-015 `ObligationItem::ScalarClaim` per routed Kani item in ascending
request-index order, and maps each FR-015 position back to the driver's
request index. `negotiate_kani_obligations` stays public as the direct FR-015
entry point for callers that are not the driver.

### Where the input type lives

The routed-item type is this crate's. It carries the backend as this crate's
`Candidate` (the FR-331 `backend` wire value),
the kind as this crate's `BackendKind`, and the node as the IR
`CheckedNodeId`. The driver converts its own routed value into it, as it
already does for the FR-331 envelope.

For that envelope, the driver projects each descriptor from QSL `Registry::descriptors()` once
into CG's own `BackendDescriptor`, copying identity, advertised (kind, mode) pairs
and `origin()` without changing their meaning (FR-019-AC-15). QSL layer R owns the closed origin
vocabulary (QSL FR-288, ADR-029 PV-1); CG's type is its local projection under ADR-013 T-7. The
driver's registration conversion supplies the origin to QSL's descriptor; its registry
holds and conflict-checks that value. Neither this routed-item conversion nor CG infers one from
the backend identity, provider bytes or a side map. The current built-in
Kani route remains linked. Process-kind classification and its empty generation
output belong to IR-629; they do not change this crate boundary.

- Not `quire-driver`: the driver depends on this crate, so the reverse edge is
  a Cargo package cycle.
- Not `qsl-route`'s `BackendId`: QSL ADR-011 §2.1 and ADR-013 T-7 put no
  shared Rust crate between QSL and CG for these types; each side converts
  its data at the driver boundary. Taking it would also widen this crate's one QSL edge
  (`qsl-replay`) that quire-driver FR-001-AC-1 asserts.
- Not the IR model crate: the IR is target-neutral and cannot name a CG
  backend kind, and this crate depends on the IR, not the other way round.

## Inputs

- The admitted IR `CheckedPackageV2`, by reference. This crate does not
  re-export `CheckedPackageV2` or `CheckedNodeId`, so a caller depends on
  `quire-contract-ir` directly to name them. It is the IR reader's
  output for the E4 bytes (QSL ADR-011 E5). This generator does not re-check
  the package identity the driver expected; that is the IR reader's.
- The routed items, each a `RoutedGenerationItem`:
  - `request_index`: the item's index in the driver's request, as FR-019's
    `ItemSettlement::request_index` numbered it;
  - `node_id`: the IR `CheckedNodeId` of the item's checked node. The driver
    reads it from its own request item at that index;
  - `backend`: the routed `Candidate`;
  - `kind`: the routed `BackendKind` from CG's descriptor-to-kind conversion
    (FR-019). The driver calls that conversion on the same CG descriptor it
    projected for negotiation; it does not construct `Process(id)` itself.
- `GenerationContexts`: the optional Kani generation context and exhaustive
  kind handling required by this entry point. The `kani` field holds a
  `KaniGenerationContext`:
  - `subject_path` and `unwind`, with FR-015's meanings.

  The caller supplies the Kani context when routing Kani items. `Process` needs
  no generation context because its output is empty. Adding a `BackendKind`
  variant without an arm in generation dispatch or `GenerationContexts::has`
  does not compile.

## Outputs

- `RoutedGeneration`:
  - `items`: one `RoutedItemOutput` per routed item, in ascending
    `request_index`, each carrying its `request_index`, its routed `backend`
    and a `KindOutput`;
  - `rejected`: the backend kinds whose arm rejected its whole group; only
    Kani can reject its group in this version, so this list is empty or `[Kani]`;
  - `claim_map`: `Some` when a Kani group ran, holding the FR-014
    `ClaimMap<ExactScalarClaim>` the arm derived for that group, and `None`
    otherwise;
  - `oracle_artifacts`: `Some` when a Kani group ran, holding the FR-014 oracle
    crate (`Cargo.toml`, `src/lib.rs`, `claim-map.json`) that
    `generate_exact_scalar_oracles` returned for the group's derived items,
    and `None` otherwise. A `Generated` claim's oracle symbol is
    defined in that `src/lib.rs`, which is the standalone FR-014 oracle crate
    source. Each harness embeds its own oracle's source (FR-015), including
    that oracle's definition and a `use quire_contract_runtime::exact
    as rt;` line, so it needs a crate that depends on `quire-contract-runtime`.
    The load-bearing returned artifact is `Cargo.toml`, which carries that
    dependency (with `features = ["exact"]`) and
    `[workspace]`. To run a harness the driver writes the returned `Cargo.toml`
    and the harness's `rust.contents` as `src/lib.rs`, because
    `execute_kani_obligation` refuses with `HarnessNotInCrate` unless the
    crate's `src/lib.rs` contains the harness source byte for byte.
    `execute_kani_obligation` accepts this `KaniScalarObligationHarness`
    directly (FR-017-AC-11): the driver assembles the request the same way
    for either harness kind, and runs it through the one execution path
    FR-017 owns. The driver
    does not write the returned `src/lib.rs` as well: appending the harness to
    it duplicates the `use ... as rt` line and the oracle definitions. The
    `claim-map.json` covers the derivable items only; the in-memory `claim_map`
    is authoritative for the routed group, since it also holds the
    `NoDerivableClaim` claims. `oracle_artifacts` is `Some` even when the Kani
    arm rejected the whole group (`rejected` lists `Kani`, no harness exists),
    matching `claim_map`.
- `KindOutput` has one variant per `BackendKind`. `KindOutput::Process` is empty.
  `KindOutput::Kani` carries
  the item's FR-015 `ObligationRecord` and its
  `Option<KaniScalarObligationHarness>`. Every request index inside that
  record, meaning `request_index` and a `DuplicateItem`'s `first_index`, is
  the driver's request index, not a position in the Kani group.
- Or one `RoutedGenerationError` for the whole call, including `Oracle` when
  FR-014 generation over the derived items fails as a whole.

## Behavior

- The generator shall dispatch generation through one arm per variant of
  `BackendKind`, with no catch-all arm, so that a backend kind with no
  generation arm fails to compile.
- The generator shall group routed items without relying on `BackendKind::ALL`
  to enumerate dynamic `Process(id)` values. It shall produce the specified
  request-index-ordered output for every permutation of the routed-item slice.
- The generator shall take each item's backend and backend kind from its
  routed input.
- The generator shall compute no candidate set, read no backend registry, settle no
  disposition and choose no backend.
- If an item's routed kind does not match its routed backend's identity under
  the built-in lookup or `Process(id)` identity comparison, then the generator shall refuse
  the whole call with `BackendKindDisagrees`, naming the request index, the
  routed backend, the routed kind and the converted kind or its absence, with
  nothing generated.
- If two routed items share one request index, then the generator shall
  refuse the whole call with `DuplicateRequestIndex`, naming that index,
  with nothing generated.
- If any routed Kani item lacks its `GenerationContexts` field,
  then the generator shall refuse the whole call with `MissingKindContext`,
  naming that kind, with nothing generated.
- The generator shall check the three refusals above in that order and report
  the first failing check. The first two name the lowest offending request
  index; `MissingKindContext` names a kind and no request index.
- The generator shall ignore a context whose kind has no routed item.
- When no item is routed, the generator shall return an empty
  `RoutedGeneration`.
- When generating for the Kani kind, the generator shall derive an FR-014
  item for each distinct node of the group with `derive_exact_scalar_items`,
  generate oracles for the derived items with
  `generate_exact_scalar_oracles`, and record, for each node with no derivable
  item, a claim whose result is the refusal derivation returned: the lowering
  or bound refusal FR-014 gives that node, or `NoDerivableClaim` carrying the
  `ClaimDerivationRefusal`. It shall order the claims by node id.
- When generating for the Kani kind, the generator shall return the artifacts
  `generate_exact_scalar_oracles` produced, unchanged, in
  `RoutedGeneration.oracle_artifacts`, including when no node of the group is
  derivable and when the group is rejected.
- When generating for the Kani kind, the generator shall call
  `negotiate_kani_obligations` exactly once, with one
  `ObligationItem::ScalarClaim` per routed Kani item in ascending
  `request_index` order, over the package, the claim map so built and the
  item's `node_id`, and with the context's subject path and unwind. The caller
  supplies no claim map.
- If `negotiate_kani_obligations` refuses the Kani group as a whole, then the
  generator shall refuse the whole call with `Kani`, carrying that
  `KaniObligationError` unchanged, with nothing generated.
- If the Kani group is rejected because an item is `invalid_request`, then the
  generator shall list `Kani` in `rejected` and return every Kani item's record
  with no harness.
- The generator shall join each Kani harness to its item through the record's
  `harness_symbol` and never by position.
- If the Kani arm emits two harnesses with one `harness_symbol`, then the
  generator shall refuse the whole call with `DuplicateHarness`, naming the
  `module::harness` path of the second, with nothing generated.
- If FR-015 reports a number of records other than the number of items in
  the Kani group, then the generator shall refuse the whole call with
  `RoutedGenerationError::KaniRecordCountMismatch { records, items }`, with
  nothing generated, and shall not pair the shorter of the two (NFR-005).
- The generator shall rewrite every position FR-015 reports into the driver's
  request index.
- If FR-015 reports a `DuplicateItem` whose `first_index` is not a position in the Kani group, then
  the generator shall refuse the whole call with
  `RoutedGenerationError::KaniDuplicatePositionOutOfRange { first_index, items }`, with nothing
  generated, and shall not index the group by it. `generate_kani` pairs, rewrites and joins the
  records through `route_records`, which rewrites each position with `rewrite_duplicate_position`
  (NFR-005-AC-6).
- The generator shall report every routed item exactly once, under its routed
  backend and the `KindOutput` variant of its routed kind. An item the arm
  refuses keeps its typed refusal and no artifact, and no other kind's arm is
  invoked for it.
- When generating for a process-provider kind, the generator shall return an
  empty `KindOutput::Process` for each routed item, with no CG artifact or CG
  adapter, execution or terminal-record call. The driver's plugin host owns
  the process adapter and terminal result (QSL ADR-029 PV-4).
- The generator shall return byte-identical output for equal inputs,
  whatever the order of the routed-item slice.
- The generator shall render a harness whose bytes and identity are
  independent of the item's request index and of which candidate of its backend routed it.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-022-AC-1 | Generation dispatches one arm per variant of the closed `BackendKind` through an exhaustive `match` with no catch-all, and `GenerationContexts::has` handles each variant; adding a variant without either arm does not compile. `Process` requires no context. | Analysis |
| FR-022-AC-2 | For a set of routed Kani items, each record and harness in the output equals what `negotiate_kani_obligations` returns for the same items in ascending request-index order with the same context. The one difference is that every index inside a record is the driver's request index, and each harness is paired with the record whose `harness_symbol` names it. | Test (TC-033) |
| FR-022-AC-3 | The entry point accepts no backend registry, candidate set, extent or capability kind, and constructs no FR-019 `Disposition`. | Test (TC-033) |
| FR-022-AC-4 | A routed Kani item whose backend identity has no CG built-in kind, or converts to a kind other than Kani, refuses the whole call as `BackendKindDisagrees`, naming the request index, backend, routed kind and converted kind or its absence, and no artifact is returned. | Test (TC-033) |
| FR-022-AC-5 | Two routed items with one request index refuse as `DuplicateRequestIndex` naming it. A routed Kani item without Kani context refuses as `MissingKindContext` naming Kani. Each returns no artifact. | Test (TC-033) |
| FR-022-AC-6 | A Kani group-level refusal (for example an unwind outside `1..=1024`, or an unparsable subject path) is returned as `Kani` carrying the unchanged `KaniObligationError`, with no artifact. | Test (TC-033) |
| FR-022-AC-7 | A Kani group containing an `invalid_request` item is listed in `rejected`, and every Kani item's record is returned with no harness. A `DuplicateItem`'s `first_index` names the driver's request index of the first occurrence. | Test (TC-033) |
| FR-022-AC-8 | Every routed item appears exactly once in `items`, in ascending request index, under its routed backend and the `KindOutput` variant of its routed kind. An item the arm refuses at lowering keeps its typed FR-015 refusal and no harness. An empty routed set returns an empty result. | Test (TC-033) |
| FR-022-AC-9 | Regeneration from equal inputs is byte-identical, a permutation of the routed-item slice yields an identical result, and the same node routed at a different request index or under another candidate of the same backend yields a byte-identical harness. | Test (TC-033) |
| FR-022-AC-10 | The Kani arm derives each item's operation and domain from the package alone; the caller supplies no claim map, and `x + 1` over a parameter `Int[0, 9]` routes to a supported `quire.op.integer.add` harness with arguments `[0, 9]` and `[1, 1]` (the literal at its own value, FR-015-AC-16). | Test (TC-033) |
| FR-022-AC-11 | An item with no derivable descriptor keeps its typed refusal and no harness, and its siblings' records and harnesses are unaffected. A node absent from the graph is `invalid_request` and rejects the group; a node that does not lower or has a refused bound keeps the FR-015 disposition it has when its claim is generated by FR-014 (`requires_bound`, `blocked_on_upstream`, `unsatisfiable_bound`); a node routed twice is one claim and one `duplicate_item`. | Test (TC-033) |
| FR-022-AC-12 | `RoutedGeneration.claim_map` is `Some` after a Kani group and equals the FR-014 claim map over the derived items, with one refused claim, of provenance `underived`, per underivable node, ordered by node id. | Test (TC-033) |
| FR-022-AC-13 | The Kani arm routes an integer node over bounded parameters to a supported harness whose arguments are one range per operand and whose result assertion uses the result bound: `a + b` over `[0, 9]` and `[10, 20]` with result `[0, 29]` has arguments `[0, 9]` and `[10, 20]`; `-e` over `[1, 9]` with result `[-9, -1]` and `e * f` over `[1, 9]` and `[100, 200]` with result `[100, 1800]` are supported; a node with a plain-typed `reference` operand beside bounded typing (FR-014-AC-25) settles `requires_bound` and has no harness. | Test (TC-033) |
| FR-022-AC-14 | `RoutedGeneration.oracle_artifacts` is `Some` after a Kani group and `None` when nothing is routed. It equals, byte for byte, the artifacts `generate_exact_scalar_oracles` returns over the derived items, so a group with no derivable node returns that call's artifacts for an empty item set. For `x + 1` over a parameter `Int[0, 9]`, each `Generated` claim's oracle symbol is defined in the returned `src/lib.rs` and appears in the supported harness's Rust source. | Test (TC-033) |
| FR-022-AC-15 | The Kani arm routes the packages QSL emits for `x + 1` over `x: Int[0, 9]` into `Int[0, 10]`, `x + y` over `Int[0, 9]` and `Int[10, 20]` into `Int[10, 29]`, and `-z` over `Int[0, 9]` into `Int[-9, 0]` (a literal as a reference to its own `value` node, the declared bound on a narrowing `conversion` consuming the plain-typed arithmetic node) to supported harnesses with arguments `[0, 9]` and `[1, 1]`; `[0, 9]` and `[10, 20]`; and `[0, 9]`, each asserting the result against the conversion's bound (`[0, 10]`, `[10, 29]`, `[-9, 0]`). In that shape a plain-Integer parameter beside a bounded one, and a reference to a `value` node whose body is not a literal, settle `requires_bound`, and a node narrowed to two distinct bounds is `oracle_refused` `AmbiguousBound`, none with a harness. | Test (TC-033) |
| FR-022-AC-16 | Two Kani harnesses with one `harness_symbol` refuse the call as `DuplicateHarness` naming the second harness's `module::harness` path, and no harness is overwritten or dropped. | Test (TC-033) |
| FR-022-AC-17 | PLANNED (IR-629). A routed `Process(id)` item appears exactly once with its own identity and empty `KindOutput::Process`, without a generation context or membership in `BackendKind::ALL`. | Test (TC-046) |
| FR-022-AC-18 | PLANNED (IR-629). A routed `Process(id)` item whose embedded identity differs from its routed backend refuses the whole call as `BackendKindDisagrees`, with no generated output. | Test (TC-046) |
| FR-022-AC-19 | PLANNED (IR-629). Permuting a routed-item slice containing Kani and multiple process identities leaves its request-index-ordered output unchanged. | Test (TC-046) |

## Dependencies

- **Upstream**: [FR-019](./FR-019-capability-settlement.md) settles and
  names the backend kind. [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) is
  the Kani generation arm. [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) derives
  the claim map the Kani arm builds.
- **Downstream**: [TC-033](../matrix/TC-033-routed-generation.md);
  quire-driver FR-001 step 7, which calls this entry point with
  `Driven::routed` converted to `RoutedGenerationItem`s.

## Out of Scope

- Linked backend kinds other than Kani. The process-provider variant returns
  empty output under QSL ADR-029 PV-4. The
  crate's other generators (FR-002 tri-state harnesses and strategies, FR-008
  to FR-013 bound strategies, and the FR-014, FR-018 and FR-021 oracles) are
  not selected by a backend kind. Callers invoke them directly, and this
  requirement does not route them. A new linked backend kind registers as
  [FR-026](./FR-026-backend-adapter-contract.md) states: a `BackendKind`
  variant, its FR-019 `negotiate_*` arm, its generation arm here, its context
  field and its adapter, and each of these is a compile error until it exists.
- Running a harness, and probing its tool. Those are FR-017 and FR-019.
- Verifying the package's identity against the driver's expected
  `package_id`. That is the IR reader's (E5).

## Open items

1. **Whether FR-019's Kani arm should read the IR form.** QSL ADR-012 §7.2
   step 3 has `negotiate_*` receive the IR form. FR-019's Kani arm reads only
   extent and advertised modes. So a `supported` item can still be refused at
   lowering, and it surfaces here as an FR-015 record. *Recommendation:* keep
   that for this version and file a separate FR-019 ticket. *Blocks:* nothing
   here. The driver reports both the settlement and the generation record.
2. **Renaming `negotiate_kani_obligations`.** It settles no capability, but
   its `negotiate_` prefix reads like an FR-019 settlement point, and QSL
   ADR-012 §5 cites it as one. *Recommendation:* leave the name in this
   ticket. A rename is churn and needs the owner's go-ahead. *Blocks:*
   nothing.
3. **Where the driver gets the Kani context values** (subject path and
   unwind). *Recommendation:* they are driver or command inputs.
   *Blocks:* the quire-driver step 7 wiring, not this crate.
