---
id: SR-5185
title: "IR-717 PR342 scope-boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen@02a98dd3d7fadbc8284785a4ea3e7629c7d1f9ef; IR-717; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md"
review_set: subset
---

## Summary

Boundary allocation: C reports a typed refusal; actual original O cleanup/Drop or O-exit kernel close disposes gate references. Unobserved aliases/remaining ownership remain unconfirmed. Signal, namespace, retained-pin and observation premises require source/runtime controls; absence of confirmation does not define a supported profile. Endpoint alias exclusion, safe C release, ancillary widening and finite native/kernel reservation remain outside this amendment and unresolved.

## Verdict

**PASS** — no new defect found in this confined normative diff. This is specification review, not source activation, runtime acceptance or merge authorization.

## Reviewed Units

```json
[
  {
    "id": "FR-034-AC-54",
    "path": "spec/kani/functional/FR-034-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "PLANNED/UNRUN. On the independently grounded supported cooperative cancellation path, the actual original O gate owner retains the gate through actual I confirmation before actual close; early-close and omitted-confirmation controls fail without marker, stage or refusal-label proof. Genuine unavailable confirmation at the original cutoff follows AC-38 CleanupUnconfirmed with explicit actual-owner abandonment/right disposal and no ordering or cleanup credit, never a renamed fatal close or diagnosis of kernel fault. Healthy restored order and unavailable-confirmation refusal are separate control"
  },
  {
    "id": "FR-034-AC-87",
    "path": "spec/kani/functional/FR-034-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "PLANNED/UNRUN. Actual I confirmation/M consuming reap precede report seal. On AC-54's independently grounded supported cooperative path, actual I confirmation precedes retained-gate close; late-operation/early-close mutants fail. AC-38 unavailable-confirmation abandonment at the original cutoff grants no seal, order or cleanup credit; a skipped poll, lost pin or owner defect is not a kernel-fault diagnosis. Producer ordinal certificates are retired."
  },
  {
    "id": "FR-034-AC-38",
    "path": "spec/kani/functional/FR-034-caller-death-ownership.md",
    "role": "context_only",
    "excerpt": "PLANNED/UNRUN. Named fixed SETTLE_RESERVE R is measured/rounded; its finite effective reserve stays inside original whole T; short finite ceilings stay admitted with R_eff=min(R,T/2), no minimum-budget cause. At finite workdeadline=T-R_eff stop/cancel; normal returns positively settle/reap all owned roles, captures and existing creator thread by original T before conclusions; original None/overflow never-elapsing work remains admitted and FIRST actual stop starts one R settlement deadline. Confirmed workdeadline expiry preserves existing TimedOut classification naming T; unconfirmed settlement"
  },
  {
    "id": "FR-034-AC-27",
    "path": "spec/kani/functional/FR-034-caller-death-ownership.md",
    "role": "context_only",
    "excerpt": "guardian-test-support is off by default, absent from default features, and exposes exactly one documented fixture operation only when explicitly enabled. A feature-off consumer cannot use that operation. No public lease, process-ownership handle, cancellation entry or cleanup-deferring callback is exported. The private initialized death witness transfers exactly one actual owned monitor or validated INIT pin, with its typed authority; BeforeMonitor transfers none. Reporter startup stdout is safely duplicated into a non-stdio CLOEXEC OwnedFd and itself marked CLOEXEC before any spawn; every chi"
  },
  {
    "id": "FR-034",
    "path": "spec/kani/functional/FR-034-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "For supported cooperative retained-gate cancellation, the actual original O gate owner shall retain\nthe gate through actual owned I termination confirmation and only then perform actual gate close.\nThis operation-order claim is conditional on the independently established signal/namespace and\nconfirmation-availability fault premises below. Early-close and omitted-confirmation mutants remain\nindependent and shall not pass from marker absence, later teardown or a CleanupUnconfirmed label.\nA cached C phase or serialized gate-retained flag cannot establish either operation. Genuine bounded\nproduct",
    "unit": "FR-034 cooperative-order behavior"
  },
  {
    "id": "FR-034",
    "path": "spec/kani/functional/FR-034-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "actual SIGKILL delivery and PID-namespace teardown permit owned settlement confirmation by the\napplicable original settlement deadline. Its signal/namespace and observation premises shall be\nestablished independently from the actual supported kernel/runtime/owner paths and genuine restored\ncontrols, not defined as “the confirmation happened.” The actual original validated I pin shall remain\nowned, and the required bounded signal/confirmation attempts and due ordinary observations shall run\nunder that deadline. Source Analysis shall identify the concrete signal, namespace, retained-pin and\nobse",
    "unit": "FR-034 fault premises"
  },
  {
    "id": "FR-034",
    "path": "spec/kani/functional/FR-034-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "required actual bounded attempts and retained original authority, execution shall follow AC-38's\nCleanupUnconfirmed return with no evidence, verdict, outcome, confirmed cleanup or operation-order\ncredit. There shall be no deadline reset, post-return custodian, leaked ownership or unbounded Drop.\nThis is an explicit live unavailable-confirmation abandonment case, not fatal-owner cancellation\nand not successful cooperative close. Remaining original O gate references shall be disposed by\ntheir actual owned cleanup/Drop or, on actual O exit, by kernel descriptor-table closure; the\nlast-reference c",
    "unit": "FR-034 unavailable-confirmation behavior"
  },
  {
    "id": "FR-034",
    "path": "spec/kani/functional/FR-034-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "not AC-54/87's successful ordering. Fatal C/group or O kernel closure remains a distinct abrupt-death\ncase and grants neither claim. Original caller/group-death, early-release and whole-chain obligations\nremain owed. Gate/lease alias exclusion, a safe C-owned release API, a five-right Start/ancillary\nallocation and finite additive stream/kernel/native reservation are not established or allocated by\nthis fault-domain amendment; current O gate topology and exact production rights remain unchanged.\nThe ignored-inner-EOF mutant shall remain separate and unrescued by outer cancellation. AC-77's\npos",
    "unit": "FR-034 preserved boundaries"
  },
  {
    "id": "TC-049",
    "path": "spec/kani/matrix/TC-049-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "    d. AC-54: on the independently grounded supported cooperative path, retain the real original O\n       gate during I cancellation; require actual I-positive confirmation before its actual owner\n       closes it. Independently close early or omit actual confirmation with independent checks\n       unchanged. Both must fail actual order despite no backend marker or CleanupUnconfirmed label;\n       restored healthy order passes. Ground the signal/namespace/retained-pin and observation fault\n       premises through actual supported source/runtime paths and genuine controls, not by defining",
    "unit": "step28d supported path"
  },
  {
    "id": "TC-049",
    "path": "spec/kani/matrix/TC-049-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "       Separately use only the already permitted private settlement-observation boundary in step25\n       to withhold positive confirmation from the actual owner until its applicable original cutoff,\n       while retaining the genuine original I pin and required bounded attempts/due observations.\n       Do not manufacture kernel D-state or an unkillable task. Require only bounded AC-38\n       CleanupUnconfirmed, no evidence/verdict/outcome, no order/cleanup credit and no fresh cutoff,\n       leak or post-return custodian. Identify the actual original O gate reference disposal by owned",
    "unit": "step28d unavailable confirmation"
  },
  {
    "id": "TC-049",
    "path": "spec/kani/matrix/TC-049-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "    j. AC-87: Independently observe actual I/M-before-report-seal and, on AC-54's supported cooperative path, I-before-original-owner-gate-close. Move real operations after seal or close gate before actual I confirmation and require failure. The distinct step28d original-cutoff unavailable-confirmation abandonment grants no report-seal, order or cleanup credit and never excuses a cooperative early-close mutant. Stored ordinal/receipt time cannot repair order; unavailable construction is UNBACKED.\n    k. AC-88: Inspect and exercise every due complete ordinary accounting sample, control/cutoff t",
    "unit": "step29j order"
  },
  {
    "id": "TC-049",
    "path": "spec/kani/matrix/TC-049-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "| FR-034-AC-54 | PLANNED/UNRUN: Supported cooperative retained-gate I confirmation before actual original-owner close; separate original-cutoff unavailable-confirmation abandonment/refusal (step28d). CG CODE author supplies per-predicate source/bounds and applicable runtime receipts; independent CODE reviewer checks before CODE acceptance. | No existing test supplies backing; Analysis is not Test credit. Genuine authority, independently grounded fault premises and bounded integrated schedule remain owed; abandonment supplies no successful order or cleanup. |",
    "unit": "matrix entry line 121"
  },
  {
    "id": "TC-049",
    "path": "spec/kani/matrix/TC-049-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "| FR-034-AC-54 | Supported cooperative actual I confirmation while original O gate is owned precedes its actual owner's close. Original-cutoff unavailable confirmation gives only AC-38 CleanupUnconfirmed and honest abandonment. | Early close/omitted confirmation fail independently; marker or refusal category gives no order. Withheld observation supplies no kernel-fault diagnosis or order/cleanup credit; restored healthy confirmation-before-close passes. |",
    "unit": "matrix entry line 1277"
  },
  {
    "id": "TC-049",
    "path": "spec/kani/matrix/TC-049-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "| FR-034-AC-87 | PLANNED/UNRUN: step 29, independent AC-87 check below. | Actual I confirmation/M consuming reap precede report seal; on AC-54's supported cooperative path actual I confirmation precedes retained-gate close. Independent late-operation/early-close mutants fail; original-cutoff unavailable-confirmation abandonment supplies no order/cleanup credit. Producer ordinal certificates are retired. No prior Test or matrix row supplies completion. |",
    "unit": "matrix entry line 204"
  },
  {
    "id": "TC-049",
    "path": "spec/kani/matrix/TC-049-caller-death-ownership.md",
    "role": "examined",
    "excerpt": "| FR-034-AC-87 | Actual I confirmation/M consuming reap precede report seal; supported cooperative I confirmation precedes actual retained-gate-owner close. Independent late-operation/early-close mutants fail; original-cutoff unavailable-confirmation abandonment supplies no seal/order/cleanup credit. Producer ordinal certificates are retired. | Independently observe actual I/M-before-report-seal and supported cooperative I-before-gate-close. Move real operations after seal or close gate before actual I confirmation and require failure; a refusal label cannot rescue that mutant. Separate withho",
    "unit": "matrix entry line 1299"
  }
]
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Evidence and Limits

Exact local head and complete two-path diff inspected. Prospective Linear custody faa7e403-5acd-49ec-bd04-641d7dd1aff7 was read in full on IR-717. Scoped Quire document validation exit0: 2/2 grammar-clean, zero grammar findings. No applicable AssuranceProfile/review_selection was found in the scoped repository scan. GitHub published-head readback was unavailable to this reviewer (HTTP401); root must verify publication/head separately. Installed Quoin 0.28.3, Quire0.36.2/engine0.50.2; module discovery printed duplicate-archetype/inverse-edge and inline-schema advisories without document failure. No Rust/source/CI delta, Cargo/Kani/native probe, lock, full spec sweep or computed matrix claim. Production gap analysis does not apply to a Markdown-only diff; all existing PLANNED/UNRUN controls remain owed. Earlier GATE-DESIGN-001..004 are preserved as unresolved implementation/design duties; this PR claims no mechanism fix. Model id and native harness id are not exposed to this reviewer and are recorded unavailable, not guessed.

## Dispositions

No real findings; no disposition round required.
