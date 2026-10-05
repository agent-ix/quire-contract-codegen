---
id: TC-046
title: "Verify the process-provider backend kind settles from its manifest alone"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: verifies
---
# TC-046: Verify the process-provider backend kind settles from its manifest alone

## Description

Verify that `BackendKind::Process(BackendId)` carries the plugin's identity, that its `negotiate_*` arm
settles from the descriptor's advertised (kind, mode) pairs, domains and bounds and the extent
classification alone, that it never reaches the plugin, and that every other arm over the kind is a typed
pass-through, generation yielding `KindOutput::Process` with no artifact (QSL ADR-029 PV-4; QSL-637).

## Test Procedure

1. Settle an item whose one candidate converts to `Process(id)` for two different ids, and call
   `BackendKind::from_identity` with `kani` and with a plugin's identity.
2. Settle one item against two process-provider descriptors with equal advertised pairs, domains and bounds
   and different identity text, manifest position and ambient state, and compare the dispositions apart
   from the backend each names. Settle an item whose domain the descriptor does not advertise, and an item
   whose bound the advertised bound does not cover, against it and against a descriptor that advertises
   them.
3. Settle an item against a descriptor whose identity names a non-existent executable and compare it, apart
   from the backend named, with one whose identity is ordinary. Settle another against a descriptor
   whose identity names an executable that records its own start, and look for the record.
4. Settle a bounded item against a descriptor advertising `bounded` with a covering bound and against one
   advertising `unbounded` only; settle an unbounded item against `unbounded`, and against `bounded` only
   with a finite bound available and with none.
5. Call routed generation with a `Process(id)` item beside a Kani item, with a `Process("kani")` item and
   with a `Process(id)` item whose identity differs from `id`; scan the crate's matches over `BackendKind`.

## Expected Results

1. The two settlements name their own ids; `from_identity("kani")` is `Some(Kani)` and the plugin identity
   gives `None` (FR-019-AC-11).
2. The two dispositions are identical apart from the named backend, which is each descriptor's own
   identity; the unadvertised domain and the uncovered bound each settle `unsupported`, warned, with the
   FR-290 cause, and settle `supported` against the descriptor that advertises them (FR-019-AC-12).
3. The dispositions are identical apart from the named backend, nothing is started, and no record exists
   (FR-019-AC-13).
4. The rows settle as FR-019-AC-15 lists, and an unbounded extent never settles `supported` against
   `bounded` only (FR-019-AC-15).
5. The `Process` item yields one `KindOutput::Process` with no artifact and no change to the Kani item's
   output, the two disagreeing items refuse `BackendKindDisagrees`, no process arm panics or calls
   `unreachable!`, and the scan finds no wildcard arm (FR-019-AC-16, FR-022-AC-17).

FR-019-AC-14 is verified by analysis of the arm's `Disposition` return type, not by a step here.

## Seeded mutants

| Mutant | Caught by |
|---|---|
| `from_identity` returns `Process` for an unknown identity | step 1 |
| The variant drops its id, or the arm names a fixed identity | step 1 |
| The arm treats `Process("kani")` as `Kani` | steps 1 and 5 |
| The disposition or cause depends on the identity text beyond echoing it as the named backend | step 2 |
| The arm ignores the domains, or ignores the bounds | step 2 |
| The arm starts or resolves the descriptor's identity as a process | step 3 |
| Unmatched rows default to `supported`, or an unbounded extent settles `supported` against `bounded` only | step 4 |
| A bounded item settles `supported` against `unbounded` only | step 4 |
| Generation emits a harness or artifact for a `Process` item, drops it, or requires a context for it | step 5 |
| A `Process` arm panics, calls `unreachable!`, or falls into a wildcard | step 5 |
| The `BackendKindDisagrees` check accepts `Process("kani")` or compares nothing for `Process` | step 5 |

## Status

Planned. The variant does not exist at this revision. QSL-637 answers FR-019's open questions 1, 2, 3, 5
and 6; the cause for an unadvertised domain or bound waits on QSpec FR-290 (step 2), and the variant's
serialized label (open question 4) is open and asserted by no step.
