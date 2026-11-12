---
id: SR-17940486709564847047724
title: "IR-983 CG #802 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-983; PR #802; src/replay/function.rs; tests/it/composite_artifact_binding.rs; tests/it/composite_original_eq.rs; tests/it/scalar_replay_binding.rs; tests/it/scalar_settlement.rs; tests/it/skeleton_spine.rs; tests/it/terminal_map.rs; tests/state_frame_support/native_twin.rs"
review_set: subset
---

## Summary

Ticket: IR-983. Reviewed head 0b4c1a801458970cd028a923ea7e4b9c8bbf7f84 against origin/main e6a5e96a (`git diff origin/main...HEAD`), with the Rust lane (rust-review) folded in. Examined FR-024-AC-44 (value-parity half), FR-024-AC-46 and TC-035, and QSL FR-484 at the locked qsl-replay rev 434869e8 as the request-member authority.

Superseded PR #553 findings re-checked against this code, not the author report:

- SR-16300 FND-001 (caller-supplied obligation): removed. `value_parity_packet(generator, assignments)` and `value_parity_request(packet)` take no `ObligationIdentity`; both select `FunctionSite.value_parity.obligation_identity` from QSL `call_site` through `value_parity_claim()`.
- SR-16300 FND-002 (effective supplied limits): present. An `Input` request sets `supplied_limits = Some(self.inputs.supplied_limits.unwrap_or_default())`; `SuppliedLimits::default()` is QSL's published A/H/B.

Focused checks: `cargo test --locked --lib tc_035` 14/14 pass; `cargo clippy --locked --all-targets -- -D warnings` clean; `cargo fmt --all -- --check` and `git diff --check` clean. No Cargo.toml, lock, deny or CI workflow change.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The value-parity request copies `ReplayInputs.declared_domains` into the request while the admitted Input packet must carry `[]`; QSL FR-484 fixes the request's declared domains empty, and the test's emptiness assertion cannot fail with the fixture's empty domains | src/replay/function.rs:500; src/replay/function.rs:1036-1046; src/replay/function.rs:1956 |
| FND-002 | low | `value_parity_request` doc says the package binds the packet's package; no package member is compared (WitnessPacket has none), package binding exists only by constructing the request from the checked package | src/replay/function.rs:926-928 |

## Verdict

Not mergeable until FND-001 is fixed. The obligation fix and supplied-limit fix from #553 hold. The `Witness`-arm Backend check is pre-existing main behaviour narrowed to that arm; it binds the only producer that emits Witness packets to the constant the request repeats (`wire()` writes "kani"), so it is load-bearing, not ceremony. The `Input` arm copies the packet's generator `BackendId` unchecked, consistent with the #553 ruling that a comparison against a caller-chosen label guards nothing. `NoValueParityClaim` for a Boolean function is correct and tested on both producer and request paths. The new public `ReplayInputs.supplied_limits` field is threaded through every constructor; `wire()` now also carries configured limits on the Witness path, which every current caller sets to `None`.

## Dispositions

Round 1, fix head 54f46f42556a4f376e4be2923a51aeaee1acc3ab. `cargo test --locked --lib tc_035` 14/14, all-targets clippy `-D warnings`, fmt and diff-check clean.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The Input branch sets `request.declared_domains = envelope.declared_domains().to_vec()`, which intake has already required to be empty; the positive test now gives `ReplayInputs` a non-empty proving-run domain, so its emptiness assertion can fail |
| FND-002 | fixed | The doc now lists target, occurrence, subject, Input arm, empty domains and obligation, and says the package binds because the request is built from this checked package |
