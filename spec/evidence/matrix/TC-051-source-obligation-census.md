---
id: TC-051
title: "Source census is independent of attempts and preserves producer facts"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-036
    type: verifies
---
# TC-051: Source census is independent of attempts and preserves producer facts

## Description

Plan falsifying controls for [FR-036](../functional/FR-036-source-obligation-census.md)
using genuine typed owner inventories and the reviewed IR-759 correspondence.
The producer/consumer implementation is Planned under IR-740 and Gated on IR-759;
these procedures are not executed tests or a claimed proof baseline.

## Test Procedure

1. Build a small selected source fixture through the real checked-package and
   production-symbol authorities: two criteria, three semantic obligations, two
   production functions in one module and explicit many-to-many relations. Retain
   one zero-mapping entity. Exercise missing criterion, missing production mapping
   and explicitly authored absence of a production function independently. Compare
   census identities/links/reasons to the fixture's authored relations and actual
   owner obligation inventory (FR-036-AC-1).
2. Enumerate before invoking negotiation, generation or a backend. Keep the selected
   source inputs fixed while independently omitting a run, reporting unsupported,
   reporting pre-run refusal, retrying and changing method/profile/subject. Assert
   equality of the full census; remove one obligation in the owning source fixture
   and regenerate its inventory/relations to assert a changed census (AC-1, AC-2).
3. Export actual typed capability/refusal/outcome fixtures and assert retained
   producer, stage, cause and the absence of an invented run for unsupported and
   pre-run refusal. Independently exercise absent decisions, absent observations,
   unfamiliar producer states and every terminal class FR-036-AC-3 lists. Missing
   execution remains absent evidence, not an invented backend outcome (AC-3).
4. Exercise the execution export over a fact fixture carrying exact claim-bound
   checks and production reachability. Independently remove the checks, substitute
   a check location without binding, tags, LLVM-only observations or incidental
   reachability. Assert that the exported observations track the changed input and
   the missing evidence never becomes a positive bound observation (AC-4).
5. Vary production/shadow subject, declared/effective bounds and separately executed
   sampled refinement. Assert exact retained bounds, subject, samples and assumptions;
   removing refinement removes that observation without changing the census (AC-5).
6. In a downstream consumer above CG and IR, borrow the public result into the
   reviewed neutral accounting port. Supply duplicate result records and change
   source, profile, subject and domain independently. Assert that each record and
   its actual association remains inspectable while census membership is unchanged;
   repeat identical inputs to compare ordering. No source/transcript/test-name parser
   or local schema copy is used (AC-6).
7. Remove each required authoritative inventory or mark it incomplete. Independently
   corrupt correspondence with a foreign selection and dangling endpoint. Assert the
   precise typed refusal and absence of a complete census; contrast with the valid
   missing-mapping candidate gap from step 1 (AC-7).

## Expected Results

Source removal changes the census, while producer-attempt mutations do not.
All selected entities and obligations remain visible with actual links or typed gaps.
Unsupported and pre-run refusal remain distinct from executed outcomes; missing and
unfamiliar facts stay explicit. Zero-check, incidental, shadow, narrower-domain and
sampled-refinement observations preserve their measured limitations. Duplicate and
mismatched records do not mint candidates or silently disappear.

Only complete authoritative inputs and valid relation endpoints yield a complete
census. Borrowed consumer conformance establishes the acyclic seam. Tests assert CG
producer facts; IR FR-045/TC-446 separately test eligibility, joins, credit and
projection totals. Neither this planned test nor spec validation claims Cargo or
Kani execution.
