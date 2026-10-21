---
id: SR-15320
title: "IR-641 CG consumer specification code-review review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-641; spec/core/functional/FR-043-owned-lock-coherence-gate.md; spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-043
    type: references
---

## Summary

No product code, Makefile, dependency, or executable checker changes. The two-file spec diff introduces consumer wiring obligations rather than a second coherence algorithm. No hashes, tool pins, tracking catalogs, runtime dependency, copied upstream normalization logic or fixture implementation are introduced. The user explicitly requires preservation of existing Cargo manifests and lock; AC-5 implements that scope constraint without adding a digest or tracking catalog.

## Verdict

PASS. Reviewed frozen PR #502 head 935f53cc9e43a1141405cdc9b521a554c12fbb38 against live main e9dafb178873fb2ad45560c2655c0f73afa631b3. Ticket: IR-641.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

No applicable AssuranceProfile or required review selection was found in the repository spec tree. Rust-review is not applicable: no Cargo or Rust change; no Rust gate was run. No optional semantic review, live backend, aggregate gate, Cargo, formatting or provisioning was run. Spec base, EARS, integrity, dependency and requested full computed gap review ran; no domain-object change warrants object review. Jev criterion strength was not selected.

Scope units (clean units included):

```yaml
- id: FR-043-AC-1
  path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  excerpt: The normal `make deny` recipe invokes the released `quire check-lock Cargo.lock` command on this repository's
    committed lock. On a coherent scratch lock the real command succeeds and the recipe completes when its other
    policy checks pass; an invocation record identifies the executable and argument without substituting checker
    logic.
- id: FR-043-AC-2
  path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  excerpt: Through the same normal `make deny` recipe, a scratch lock with duplicate first-party Git package names
    and a scratch lock with distinct crate names at split resolved revisions of one repository each cause a non-zero
    Make exit and the real command's coherence diagnostic. The aligned multi-crate control passes the command. Every
    agent-ix Git source in these coherence controls has a valid full resolved fragment admitted by CLI FR-027; a
    malformed-input diagnostic cannot satisfy a duplicate or split-revision control. These controls exercise CLI
    FR-027; they do not define a second normalization al
- id: FR-043-AC-3
  path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  excerpt: A missing checker executable, malformed scratch lock, or checker refusal cannot produce successful `make
    deny`. The recipe propagates failure without an ignored exit, success fallback, or skipped check.
- id: FR-043-AC-4
  path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  excerpt: The committed `scripts/check_one_copy.awk` file is absent, the normal recipe contains no reference to
    it, and no replacement local implementation, executable copy, compatibility shim, or revision allowance bypasses
    the owned command.
- id: FR-043-AC-5
  path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  excerpt: Adoption leaves the committed Cargo manifests and lock byte-for-byte unchanged and adds no runtime dependency
    on developer tooling. Qualification runs the real command on the committed lock and reports any existing incoherence
    as a failing gate with an owning blocker; a scratch passing control or an unrelated gate failure cannot qualify
    that lock.
- id: FR-043
  path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  excerpt: 'When a developer runs `make deny`, quire-contract-codegen shall execute the released

    `quire check-lock Cargo.lock` command and propagate its failure to Make.'
- id: spec/spec.md
  path: spec/spec.md
  role: examined
  excerpt: '[FR-043](core/functional/FR-043-owned-lock-coherence-gate.md) defines planned consumption of the single
    owned CLI lock

    coherence gate under IR-641. It does not claim consumer implementation or qualification.'
- id: FR-027
  path: quire-cli/spec/functional/FR-027-check-lock-subcommand.md
  role: context_only
  excerpt: 'The distributed `quire` executable SHALL expose `quire check-lock <Cargo.lock>...`

    as the single owned developer gate for agent-ix Git dependency coherence.'
```
