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
  - target: ix://agent-ix/quire-spec-language/FR-314
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-375
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
- Finite caller-selected per-query input-byte, wall-clock, memory and
  captured-output byte limits. Each of the two queries gets its own limit accounting and
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
When query bytes exceed the input limit, CG SHALL return `Stopped` with that
limit before spawning a solver.
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

### Refinement SMT run records

For a QSpec FR-375 refinement with `N` step rows, CG SHALL identify the
FR-315 `Initial` query as `Base` and each FR-315 `Step { row: r }` query as
`Step { row: r }`, where `r` is the zero-based position in the recompiled
refinement's canonically ordered `steps` array, for every `r` in `0..N`.
The row index does not use the source text's row order. CG SHALL retain one
typed `Base` run record and one typed `Step { row: r }` run record per row,
including when `N = 0`. These records carry the refinement item and concrete
and abstract subject association, the invocation's distinct attempt identity,
the exact FR-315 canonical query bytes submitted to the solver, the solver
status and process evidence, and a separate optional field for the exact
Alethe proof bytes emitted by that invocation. Query bytes are never inferred
from proof output, and proof bytes are never inserted into or substituted for
the query field. A present empty proof remains present and empty.

The supported direct cvc5 proof-capture mode selects `--dump-proofs` and
`--proof-format-mode=alethe` in the retained argument vector. Those arguments
request proof output after an unsatisfiable `(check-sat)` without changing any
FR-315 query byte or appending a command to stdin (the
[cvc5 Alethe output example](https://cvc5.github.io/docs-ci/docs-main/proofs/output_alethe.html)
shows this command-line mode). For this mode CG SHALL take
the status from the first response line after optional SMT-LIB comments and
`success` notices. Only a completed, successful invocation whose status is
`Unsat` may supply proof bytes. CG SHALL identify the proof as the exact
contiguous stdout bytes after that status line, including its whitespace and
newlines, when that tail contains one or more complete top-level SMT-LIB
S-expressions and no other token. It SHALL NOT take proof bytes from stderr,
another attempt, a caller-supplied field or a later process. A second status,
an SMT-LIB error response, an incomplete expression or any other trailing
non-proof token SHALL yield `MalformedResponse` and no proof-bearing handoff.
CG does not interpret Alethe rules or remove an outer proof expression; QSL
FR-314 judges the exact extracted bytes. A status with no proof expression
leaves the proof field absent and gives `MissingField`, not an inferred
present-empty proof. Other solver modes may carry a present empty proof only
when their selected, explicit output framing distinguishes its presence from
no proof; absent such framing they SHALL leave the proof field absent.

CG SHALL validate the complete record collection against the selected
refinement before producing the certificate handoff: exactly one `Base`,
exactly one `Step { row: r }` for each `r` in `0..N`, no row outside that
range, and no duplicate role or row. CG SHALL compare each record's item,
subjects, kind, row, attempt and query bytes with the expected invocation and
FR-315 output. A missing, duplicate, out-of-range, stale or swapped record,
including a `Base` record in a step position or a step record in the base
position, SHALL yield a typed association or shape refusal and no certificate
handoff. A record with an absent query field or absent proof field SHALL yield
a typed missing-field refusal and no certificate handoff. A query that FR-315
refuses to encode SHALL retain that refusal and yield no invented query or
certificate handoff. A non-`Unsat` or incomplete solver response SHALL remain
run evidence and yield no proof-bearing handoff.

For a complete validated collection, CG SHALL serialize the QSL adapter's
input deterministically as an ordered sequence: `Base` first, followed by
`Step { row: 0 }` through `Step { row: N-1 }`. Each serialized entry SHALL
carry a role tag (`Base` or `Step` with its zero-based `row`), a query field
containing the exact FR-315 canonical bytes, and a separate proof field
containing the exact Alethe bytes; byte fields are length-delimited so
arbitrary content, including zero bytes, survives unchanged. Two validated
collections with identical typed fields SHALL serialize to byte-identical
sequences regardless of input record order. The QSL FR-314 adapter consumes
this sequence to form `RefinementSmtCertificate { format: Alethe, base,
steps }`; CG SHALL NOT assert that the certificate verifies or settle the
refinement's terminal result.

The handoff's byte format SHALL be a little-endian unsigned 64-bit record
count, followed by exactly that many entries in the order above. Each entry
SHALL be a one-byte role tag (`0` for `Base`, `1` for `Step`); a `Step` tag
SHALL be followed by its zero-based row as a little-endian unsigned 64-bit
integer. Each entry then SHALL carry the query byte length as a little-endian
unsigned 64-bit integer and that many raw query bytes, followed by the proof
byte length in the same encoding and that many raw proof bytes. The count
includes `Base`. CG SHALL refuse serialization if a count, row or byte length
cannot be represented, and a consumer SHALL reject a truncated stream,
unknown tag, mismatched count or trailing bytes. The item, subjects, attempts,
solver status and process evidence remain in CG's typed source records; the
validated handoff contains only the fields FR-314's adapter needs.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-045-AC-1 | For an admitted HP-2 safety clause and `k = 1`, CG requests QSL's base and step queries using FR-315's clause, ordered subjects, instance and Meter shape, submits each canonical byte sequence unchanged to the selected solver, and retains the product, item, instance, depth, kind and exact bytes in each query record. Matching `Unsat`/`Unsat` records are eligible for QSL-owned `Inductive { depth: 1 }`; QSL FR-182-AC-7, rather than CG, settles that outcome as `proved`, `decisive-witness`, `Uncertified`. | Test (TC-059) |
| FR-045-AC-2 | A base `Sat` response remains only a candidate for QSL reconstruction and replay; base `Unsat` with step `Sat` is not induction; solver `Unknown`, QSL encoding refusal, resource stop, missing or malformed response, process failure and a response paired with another product, item, instance, depth, kind or query bytes remain distinct typed non-proof evidence. A resource stop is handed to FR-182 V-7 as incomplete. Every branch retains its available identities and depth, and no CG branch claims terminal refutation or certification. | Test (TC-059) |
| FR-045-AC-3 | CG invokes the selected solver executable directly, one process per QSL query, with exact query bytes on stdin and finite per-query input-byte, wall-clock, memory and captured-output limits; it retains the executable, arguments, limits, completion, status, captured output and any emitted model/proof bytes in a CG-owned run record. An input-byte excess refuses before spawn, a missing executable gives `MissingTool`, and a reached limit gives `Stopped` with that limit; none becomes `Unsat` or a QSL outcome. Separate attempt and query identities prevent a stale or swapped response from satisfying the other query. | Test (TC-059) |
| FR-045-AC-4 | For a refinement with seven step rows, CG retains one `Base` and exactly seven typed `Step { row }` records numbered 0 through 6 from the recompiled, canonically ordered `steps` array, even when records arrive out of order or the source rows were written in another order. Each record binds the refinement item, subjects, distinct invocation attempt, role and row, and exact FR-315 `Initial` or `Step { row }` query bytes sent on stdin; changing a query byte or exchanging the base and a step role refuses the handoff. With zero step rows, the complete shape has only `Base`. | Test (TC-059) |
| FR-045-AC-5 | Given seven completed `Unsat` step records and one completed `Unsat` base record with present Alethe proofs extracted from each record's own proof-producing solver invocation, CG emits one deterministic handoff sequence in `Base`, `Step { row: 0 }` through `Step { row: 6 }` order for either input permutation. Every entry has separate length-delimited query and proof byte fields equal to the original FR-315 query and emitted Alethe proof, including a present empty proof when explicit output framing reports one. Repeating serialization of the same typed fields gives identical bytes; changing a proof byte changes only that entry's proof field. The QSL adapter can construct `RefinementSmtCertificate` without reinterpreting or rewriting either byte field. | Test (TC-059) |
| FR-045-AC-6 | Starting from AC-5's collection, removing `Base`, removing row 3, duplicating `Base` or row 3, or adding row 7 yields a typed shape refusal and no certificate handoff. Replacing row 3 with a record from another item, subject or attempt, or swapping row 3's record with row 4's while retaining the old row labels, yields a typed association refusal and no handoff. | Test (TC-059) |
| FR-045-AC-7 | Starting from AC-5's collection, making row 3's query or proof field absent yields a typed missing-field refusal and no handoff; making the proof present but empty preserves the empty bytes in the handoff. An FR-315 refusal, a `Sat` or `Unknown` answer, a missing or malformed response, a failed solver invocation or a reached limit remains typed non-proof evidence and produces no proof-bearing handoff. | Test (TC-059) |

## Dependencies

- QSL FR-176 owns the self-composition product and its `μ_U` semantics.
- QSL FR-315 exports `encode_hyper_query` and the refinement `Initial` and
  `Step { row }` query contract. Its clause/subjects/instance/Meter signature
  is authoritative for HP-2. CG does not introduce a hyper-product encoder.
- QSL FR-127 and FR-182 own outcome types and terminal mapping; FR-314 and
  QSL-908 own refinement certificate assembly and checking. A CG run record
  is execution evidence.

## Status

PARTIAL (IR-974). CG's bounded single-query solver execution and associated
base/step evidence classifier exist in `src/routed/smt/`; QSL exports the
hyper query encoder and outcome types, but their CG integration remains
planned. The refinement Base/Step record
collection, validation and deterministic FR-314 handoff are PLANNED
(IR-1120), with no implementation or tagged tests yet.
