---
id: FR-045
title: "Run QSL-encoded HP-2 induction queries and map solver outcomes"
type: FR
relationships:
  - target: ix://agent-ix/quire-spec-language/FR-315
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-176
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-182
    type: references
---
# FR-045: Run QSL-encoded HP-2 induction queries and map solver outcomes

## Description

When QSL supplies canonical SMT queries for an admitted HP-2 safety item,
CG's SMT backend SHALL submit those bytes to the selected solver and retain
a CG-owned run record for QSL's outcome and terminal settlement. QSL FR-315
owns transition-relation encoding, bounded unrolling and induction base/step
query construction. CG SHALL NOT independently encode the self-composition
product, its `μ_U` joint-step guard, automaton, definedness or fairness
semantics. QSL FR-182 and QSL-991 own the inductive outcome type and terminal
settlement; a CG run record alone claims neither proof nor refutation.

## Inputs

- The admitted HP-2 safety item's QSL-owned clause, ordered subjects,
  object-parameter instance and Meter, passed to QSL's actual FR-315
  `encode_hyper_query(HyperQueryRequest { clause, subjects, instance, meter },
  SmtQueryKind)` API. QSL owns the resulting canonical base and step SMT-LIB
  bytes. The caller retains the selected product identity, item identity and
  instance identity for association; CG does not reconstruct the product.
- One caller-selected induction depth `k >= 1`, the FR-315 base and step
  query kinds for that same depth, and a selected solver executable with an
  argument vector. The executable path and arguments are caller supplied;
  CG invokes it directly, without a shell or command-string interpretation.
- Finite caller-selected per-query wall-clock, memory and captured-output
  byte limits. Each of the two queries gets its own limit accounting and
  process. CG does not choose the program's trust or proof status.

## Outputs

CG returns one run record with the selected product, item and instance
identities, depth, solver selection and two separately identified query
records. A query identity is the run-local tuple of product, item, instance,
depth, query kind and the **exact retained query bytes**; a distinct attempt
identity separates repeat invocations. No hash or file pin is introduced.
Each query record retains the exact bytes sent to the solver, limits used,
process completion and captured stdout/stderr within those limits, parsed
`Sat`, `Unsat` or `Unknown` status when present, and any raw model or proof
bytes the solver actually emitted. CG does not synthesize model or proof
bytes or alter QSL's query to request them.

The typed non-success dispositions are `EncodingRefused` (the original QSL
refusal), `MissingTool`, `Stopped` (which limit was reached), `MissingResponse`,
`MalformedResponse`, `ProcessFailed` and `AssociationMismatch`. They retain
product, item, instance, depth and query kind where available. A process
that produces no recognized `sat`, `unsat` or `unknown` response is never
classified as `Unsat`. A resource stop is handed to QSL as incomplete,
not as solver unknown or inconclusive.

| CG run evidence | QSL-owned handoff |
| --- | --- |
| Matching base `Unsat` and step `Unsat` | Eligible for QSL's `Inductive { depth: k }` method outcome; QSL FR-182 decides any `proved` settlement and certification |
| Base `Sat` with raw model bytes | Candidate only; QSL-owned reconstruction and replay decide whether a product path exists or refutes the item |
| Base `Unsat`, step `Sat` | Induction is not closed at `k`; QSL settles inconclusive |
| `Unknown`, missing or malformed response, process failure or association mismatch | Typed non-proof evidence; no inductive or terminal verdict |
| `Stopped` or `MissingTool` | Typed incomplete or unavailable execution; no inductive or terminal verdict |
| `EncodingRefused` | Original QSL refusal; no query is invented or submitted |

## Behavior

CG SHALL invoke one process per QSL query, write exactly that query's bytes
on stdin and capture only that process's response under its per-query limits.
The selected executable, argument vector and limits are retained in the run
record. A missing executable SHALL return `MissingTool` before claiming a
solver response. When the process exceeds a time, memory or capture limit, CG SHALL
stop it and retain the reached limit. A nonzero exit, absent status,
additional contradictory status or malformed status SHALL not become a
proof-bearing response. CG SHALL keep the response bound to the invocation's
product, item, instance, depth, kind and exact bytes; a swapped, stale or
missing response SHALL not satisfy the other query.

CG SHALL offer the two associated run records to the QSL-owned outcome map
only when both required queries completed. It SHALL report the `Unsat` pair
as evidence eligible for QSL induction only when both required queries
returned `Unsat` and neither has an unknown, stop or unexamined result. A
base `Sat` is not a terminal violation without QSL product-path replay.
QSL FR-127 and FR-182 own terminal settlement and QSL-908 owns certificate
rows. This contract sets no default Kani or SMT memory ceiling.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-045-AC-1 | For an admitted HP-2 safety clause and `k = 1`, CG requests QSL's base and step queries using FR-315's clause, ordered subjects, instance and Meter shape, submits each canonical byte sequence unchanged to the selected solver, and retains the product, item, instance, depth, kind and exact bytes in each query record. Matching `Unsat`/`Unsat` records are eligible for QSL-owned `Inductive { depth: 1 }`; QSL FR-182-AC-7, rather than CG, settles that outcome as `proved`, `decisive-witness`, `Uncertified`. | Test (TC-059) |
| FR-045-AC-2 | A base `Sat` response remains only a candidate for QSL reconstruction and replay; base `Unsat` with step `Sat` is not induction; solver `Unknown`, QSL encoding refusal, resource stop, missing or malformed response, process failure and a response paired with another product, item, instance, depth, kind or query bytes remain distinct typed non-proof evidence. A resource stop is handed to FR-182 V-7 as incomplete. Every branch retains its available identities and depth, and no CG branch claims terminal refutation or certification. | Test (TC-059) |
| FR-045-AC-3 | CG invokes the selected solver executable directly, one process per QSL query, with exact query bytes on stdin and finite per-query wall-clock, memory and captured-output limits; it retains the executable, arguments, limits, completion, status, captured output and any emitted model/proof bytes in a CG-owned run record. A missing executable gives `MissingTool`, and a reached limit gives `Stopped` with that limit; neither becomes `Unsat` or a QSL outcome. Separate attempt and query identities prevent a stale or swapped response from satisfying the other query. | Test (TC-059) |

## Dependencies

- QSL FR-176 owns the self-composition product and its `μ_U` semantics.
- QSL-661 must first implement FR-315's single-subject SMT encoder and its
  public query vocabulary; QSL-1010 must then implement `encode_hyper_query`
  before CG can run the full producer. Current QSL source has neither
  exported encoder. FR-315's clause/subjects/instance/Meter signature is
  authoritative. CG does not introduce a temporary hyper-product encoder
  or fallback.
- QSL FR-127, FR-182 and QSL-991 own outcome types and terminal mapping;
  QSL-908 owns certificate settlement. A CG run record is execution evidence.

## Status

PLANNED (IR-974). No HP-2 SMT producer or QSL hyper query encoder exists
in the current source tree. CG-owned solver-run evidence and refusal logic
can be built first; the QSL query call and outcome handoff wait on their
owning APIs.
