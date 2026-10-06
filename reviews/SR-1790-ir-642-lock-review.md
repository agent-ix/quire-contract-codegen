---
id: SR-1790
title: "Code review of IR-642 locked dependency graph"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@2d9c3458080778c5c157d341d44fa25ea8b5a8c4; Cargo.lock: qsl-attrs, qsl-cst, qsl-eval, qsl-forms, qsl-foundation, qsl-package, qsl-replay, qsl-semantics, quire-contract-ir, quire-contract-model, quire-exact, quire-semantic-value, quire-verification-contracts; Cargo.toml and scripts/check_one_copy.awk as context"
review_set: subset
---

## Summary

Ticket: IR-642. Reviewed the single-file `Cargo.lock` change against CG main `28553daeb1e9cfd88bb6620125bb1df27273c68e`, QSL's `dd5605a185f9cab0387e75b123843bfcd9d2382b` lock, and CG's declared dependencies. The diff updates eight QSL crates and five shared first-party entries; it adds no source, test, schema, fixture, or CI file.

## Verdict

**PASS** — the changed lock graph is coherent and contains one copy of every Agent IX git crate. No review finding. This is a confined lock/config review; no production behavior or test trace binding changed, so a separate gap-analysis artifact is inapplicable. The lead owns the full CI and Kani gates.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- Examined each of the eight changed QSL entries in `Cargo.lock:1277-1380`: all resolve to `dd5605a185f9cab0387e75b123843bfcd9d2382b`. The QSL commit exists in the local upstream repository and is the `#646` adaptation commit.
- Examined `quire-contract-ir` and `quire-contract-model` in `Cargo.lock:1443-1460`: both resolve to `3c47de24dfa7fe266cb0c3cc5deccc71629e25b1`. That commit descends from `3be6ff70a6e20372b5181c2c53d242c86dba46d3`, the Contract Model revision in QSL's own lock.
- Examined `quire-exact`, `quire-semantic-value`, and `quire-verification-contracts` in `Cargo.lock:1485-1550`: their revisions match QSL's lock exactly (`efd4a22846ed69a5cf942797923fd6dd4f950acc`, `e0ada80708fe73469923a9d6c2263cfebc0087a0`, and `ec4563ff33e03135ac4d0d9500cc53b0a956ecb6`). All other shared first-party source entries also match QSL's lock.
- Examined `Cargo.toml` direct dependencies and `scripts/check_one_copy.awk` as context. `awk -f scripts/check_one_copy.awk Cargo.lock`, `git diff --check`, and `cargo metadata --locked --offline --format-version 1 --no-deps` exited zero. A parsed lock inventory found no duplicate Agent IX git crate name or source/revision for the same crate.
- `SR-1790` through `SR-1799` were absent in the reviewed PR head and pinned CG main before this artifact was authored. No applicable `AssuranceProfile` was found under `spec/`.
- No full gate or Kani run was made in this reviewer pass. The pre-PR `make ci` and Kani outcome in the ticket is external lead evidence, not independently reproduced here.
