---
id: Task-003
title: "Upstream dependency reconciliation"
type: Task
status: done
track: A
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-ir/PGM-01
    type: references
---
# Task-003: Upstream dependency reconciliation

## Scope

Pin and review the accepted PGM-01 policy, authoritative IR schema/corpus, and runtime API before
semantic implementation begins.

## Current State

PGM-01, runtime, and IR are pinned to merged revisions. IR PR #19 was accepted and merged as
`5c49ebfd1c87415f74420ad047392bd03b1bd202`; PR #10 is reconciled to that exact identity and its
locked dependency graph. Task-004 remains independently reviewable as a draft until its own
acceptance criteria and review findings are closed.
