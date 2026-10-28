---
id: SR-619014
title: "spec-review/integrity review of IR-1079 PR 625"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-021-function-application-oracles.md; spec/oracle/matrix/TC-031-function-application-oracles.md"
review_set: subset
org: agent-ix
---

## Summary

Ticket IR-1079, CG #625 at a2a24481a43bced820c01d0eab440bc816d34e33. The plain-reader predicate in AC-41 is not observable from the V1 accounting record. The error locus for a declaration-body failure is also unspecified. Authenticated outcomes are internally consistent.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | AC-41 conditions the plain-read refusal on a requested-root kind that plain V1 cannot reveal; specify an observable trigger such as all temporal-tag Unsupported records, and align TC-031. | FR-021-AC-41; TC-031 step 13 | wrong-requirement |
| FND-002 | medium | AC-41 does not say whether a blocked declaration body reports its body request index/root or the call item index/root; TC-031 tests only direct items on the plain-read path. | FR-021-AC-41; FR-021-AC-33; TC-031 step 13 | wrong-requirement |

## Scope and evidence

Examined FR-021's new source-admission paragraph, amended behavior, FR-021-AC-1, FR-021-AC-11, FR-021-AC-33, new FR-021-AC-41 and TC-031 step 13 and result summary. IR a97babf FR-035-AC-27 through AC-30 says plain strict read returns typed NotAdmitted for every Unsupported requested-root-kind query, even if the key is represented elsewhere. QSpec FR-330 retains the first unsupported reachable id and tag on V1. Thus the direct fairness and temporal-clause records can carry the same tag, while only authenticated source yields the root kind. Case is a separately lowered represented form and does not rely on that unsupported-kind view.

## Verdict

Needs revision: specify a predicate available to CG on a plain V1 package before it chooses AC-41, and make TC-031 assert it.

## Validation

`make spec` and `git diff --check origin/main...HEAD` passed. Quire reported only baseline EARS warnings in FR-014 and FR-017.

## Examined units

### FR-021-AC-1 (examined)

Source: `spec/oracle/functional/FR-021-function-application-oracles.md`

> For a request whose required source views are admitted, every requested `call` item receives exactly one scheduled or typed-refused disposition; a refused item contributes no instruction root, concrete symbol or location entry, while an unrelated admitted item is unchanged. A plain-read V1 request needing an unauthenticated root-kind view returns the whole-call AC-41 refusal before dispositions. Concrete generated disposition remains Planned through the FR-485 consumer (IR-595).

### FR-021-AC-11 (examined)

Source: `spec/oracle/functional/FR-021-function-application-oracles.md`

> With an admitted requested-root kind, model and relation lowering records map to the quire-spec-language#120 blocker; state, temporal nodes other than directly requested `formula` and `fairness` roots, and protocol lowering records map to quire-spec-language#121; the blockers remain distinct. A requested temporal clause keeps that blocker even if its first unsupported reachable node is a formula or fairness child. A plain-read V1 record whose requested-root kind is `NotAdmitted` follows AC-41 instead of guessing from the first reachable tag. TC-031's defensive Model/State classifier

### FR-021-AC-33 (examined)

Source: `spec/oracle/functional/FR-021-function-application-oracles.md`

> With an IR-admitted `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node used directly as an item's call-node id, and an authenticated requested-root kind for an IR `Unsupported` lowering record, the claim-map entry retains that id and is `Refused { refusal: ExactFunctionRefusal::Unsupported }`. For the separate declared-body-root case, an item names that function through a distinct, IR-admitted `expression`/`call` node that lowers successfully and has no duplicate, capability, or earlier item refusal; its claim retains the call-node id and carries the body's `Exac

### FR-021-AC-41 (examined)

Source: `spec/oracle/functional/FR-021-function-application-oracles.md`

> For a plain strict-reader-admitted V1 package with an `Unsupported` accounting record whose requested-root kind is required to distinguish direct `temporal`/`formula` or `temporal`/`fairness` from a temporal clause reaching one, the function generator returns whole-call `OracleGenerationError::UnauthenticatedRequestedRootKind { request_index, node_id }` for the lowest affected request index, independent of item slice order, with no claim map, schedule, oracle or partial publication. `node_id` is the requested root; the first reachable `unsupported_node_id` and `node_tag` do not subst

### FR-021 (examined)

Source: `spec/oracle/functional/FR-021-function-application-oracles.md`

> - For an `Unsupported` function-profile accounting record, the requested
>   root's kind is an admitted fact only on the producer package or Contract
>   IR's `read_with_source` handoff authenticated against the exact V2 source
>   (FR-035-AC-27 through AC-30). The V1 wire records the first unsupported
>   reachable node id and tag; those fields do not identify the requested
>   root's kind. Plain strict `read` admits the V1 package but returns typed
>   `NotAdmitted` for this view. When a requested item's classification needs
>   that view, `generate_exact_function_oracles` shall return the whole

### TC-031 (examined)

Source: `spec/oracle/matrix/TC-031-function-application-oracles.md`

> 13. Direct unsupported roots (FR-021-AC-11, FR-021-AC-33 and FR-021-AC-41).
>     Admit formula, fairness and case roots through Contract IR's public V2 reader, using QSpec FR-370/FR-440
>     for formula/case shapes and Contract IR FR-038-AC-107 for fairness.
>     Request each root first as an item's call-node id. For the separate body
>     leg, use that root as a declared function's body and supply a distinct,
>     IR-admitted `expression`/`call` item naming the function. Verify that the
>     call item lowers successfully and has no duplicate, capability or earlier
>     item refusal, so ite


## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The guard now uses selected Unsupported records with admitted `node_tag: "temporal"` and typed NotAdmitted after existing precedence; FR-021 and TC-031 use the same observable predicate. |
| FND-002 | fixed | AC-41 now identifies the direct item driver index/root and the declaration body's own driver index/root, with a lowest-index tie rule; TC-031 adds both loci and slice permutations. |
