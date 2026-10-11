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

1. Build the AC-1 source fixture through the real checked-package and production-symbol
   authorities: criteria C1/C2, obligations O1/O2/O3, source identities S1/S2, domain D,
   two production functions in one module and these four authored complete tuples:
   (C1,O1,S1,D), (C2,O1,S1,D), (C1,O2,S1,D), (C1,O2,S2,D). Assert three distinct obligation
   inventory entries and exactly four candidate keys with those members. Give O3 no
   criterion correspondence and assert its inventory/gap record, with no fabricated
   candidate. Retain a zero-mapping source symbol; exercise missing-production and
   authored-no-function gaps on complete candidate keys. Remove only C2's relation:
   assert three remaining candidates and an explicit zero-mapping C2 inventory/gap.
   Add a second symbol projection to an existing tuple: assert the new link without
   a new candidate key. Compare actual owner inventories and authored relations,
   rather than treating obligations as candidate rows (FR-036-AC-1).
2. Enumerate before invoking negotiation, generation or a backend. Keep the selected
   source inputs fixed while independently omitting a run, reporting unsupported,
   reporting pre-run refusal, retrying and changing method/profile/subject/artifact/run.
   Assert equality of inventories, all four complete candidate tuples and mapping
   gaps. Remove O1 in the owning source fixture and regenerate its inventory/relations:
   assert two remaining obligations and exactly the two remaining O2 candidate keys,
   without editing an auxiliary catalog (AC-2).
3. Export actual typed capability/refusal/outcome fixtures and assert retained
   producer, stage, cause and the absence of an invented run for unsupported and
   pre-run refusal. Assert absent artifact/run identity stays absent; an actual
   generated artifact in a pre-run refusal does not become an executed run.
   Independently exercise absent decisions, absent observations,
   unfamiliar producer states and every terminal class FR-036-AC-3 lists. Missing
   execution remains absent evidence, not an invented backend outcome (AC-3).
4. Exercise the execution export over a fact fixture carrying actual typed artifact
   and run identities separately from attempt identity, exact claim-bound checks
   and production reachability. Independently replace each artifact/run identity
   with a wrong or mismatched typed identity, then remove each identity: assert
   the received identity or typed missing reason survives for IR joining without
   repair, reconstruction or candidate mutation. Independently remove the checks, substitute
   a check location without binding, tags, LLVM-only observations or incidental
   reachability. Assert that the exported observations track the changed input and
   the missing evidence never becomes a positive bound observation (AC-4).
5. Vary production/shadow subject, declared/effective bounds and separately executed
   sampled refinement. Assert exact retained bounds, subject, samples and assumptions;
   removing refinement removes that observation without changing the census (AC-5).
6. In a downstream consumer above CG and IR, borrow the public result into the
   reviewed neutral accounting port. Supply duplicate result records and change
   artifact/run identity, source, profile, subject and domain independently. Assert that each record and
   its actual association remains inspectable while census membership is unchanged;
   repeat identical inputs to compare ordering. No source/transcript/test-name parser
   or local schema copy is used (AC-6).
7. Remove each required authoritative inventory or mark it incomplete. Independently
   corrupt correspondence with a foreign selection and dangling endpoint. Assert the
   precise typed refusal and absence of a complete census; contrast with the valid
   valid unmapped-entity gaps and production gaps on complete candidate keys from
   step 1 (AC-7).

## Expected Results

Removing O1 removes its inventory entry and both criterion-keyed O1 candidates,
while producer-attempt mutations do not change either inventory or candidate tuples.
All selected entities and obligations remain visible with actual links or typed gaps;
unmapped entities have no invented criterion-keyed candidate.
Unsupported and pre-run refusal remain distinct from executed outcomes; missing and
unfamiliar facts stay explicit. Zero-check, incidental, shadow, narrower-domain and
sampled-refinement observations preserve their measured limitations. Duplicate and
mismatched records do not mint candidates or silently disappear. Actual typed artifact
and run identities remain distinct from attempt identity; wrong or missing associations
stay explicit so IR can decide evidence joins.

Only complete authoritative inputs and valid relation endpoints yield a complete
census. Borrowed consumer conformance establishes the acyclic seam. Tests assert CG
producer facts; IR FR-045/TC-446 separately test eligibility, joins, credit and
projection totals. Neither this planned test nor spec validation claims Cargo or
Kani execution.
