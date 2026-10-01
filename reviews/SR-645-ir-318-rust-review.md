---
id: "SR-645"
title: "CG design audit (rust-review, whole repo) at origin/main 5a924e1"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@5a924e1fdeaf60aab6e4ba9e2401634b5a076707; src/**, tests/**, Makefile, .github/workflows/ci.yml, spec/assurance/AD-001-codegen-architecture.md, spec/index.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
---

# SR-645: CG design audit (rust-review, whole repo)

## Summary

Ticket: IR-318. This is a read-only `/rust-review` design audit of the whole repository at
origin/main 5a924e1 (after #202 to #206), not a PR diff. It re-measures the earlier audit comment on
IR-318 (taken at 23dcc3d) and treats every claim in it as unverified. It also maps each finding
onto an M4 or existing ticket. PRs #207 (IR-462) and #208 (IR-458) are open and are not on main.
Findings are judged on main only.

Counts: 4 high, 12 medium, 9 low.

## Method

- I read `src/lib.rs`, `CLAUDE.md`, `Cargo.toml`, `clippy.toml`, `Makefile`, `ci.yml`, AD-001 and
  `spec/index.md`.
- I read the panic, hand-off and identity paths of `kani_obligations.rs`, `kani_execution.rs`,
  `bounded_kani_corpus.rs`, `state_frame.rs`, `frame_replay.rs`, `routed_generation.rs`,
  `exact_function.rs`, `bound_coverage.rs` and `publication.rs`.
- I ran grep sweeps over all of `src/` for these:
  - panics outside `#[cfg(test)]`
  - `unreachable!` on RT `#[non_exhaustive]` enums
  - duplicated helpers
  - `crate::` import edges, for cycles
  - String identity fields
  - raw `serde_json::Value` access
  - error types and their `Error` impls
  - bare `as` casts
  - `kani::cover!` sites
- Three read-only sub-agents swept the Kani launcher and transcript, the test conventions, and the
  state/frame and replay paths. I re-read every line cited below myself before recording it.
- I ran `cargo clippy --locked --all-targets -- -D warnings` with a private target dir. The result
  is under Gates.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The bounded-Kani corpus generator invents the Kani verdict. `generate_bounded_kani_corpus_case` returns `KaniOutcome::proved` or `::counterexample` from IR's lowered value at generation time, and no Kani run happens. Arithmetic is hard-coded `true`. The emitted harness is `assert!(corpus_oracle())` over literals, with no `kani::any()` and no `kani::cover!`, so it is ground evaluation and not a proof. Failure: a consumer reads `BoundedCorpusCase.outcome == Proved` for a case whose harness was never compiled or run, or does not compile. The module has no caller in `src/`, and IR-311 Q1 rules FR-015 the only Kani generator. Ticket: IR-453, with retirement in IR-348. | src/bounded_kani_corpus.rs:254-260, src/bounded_kani_corpus.rs:289-299, src/bounded_kani_corpus.rs:377-381 |
| FND-002 | high | The scalar obligation harness only asserts what the oracle already enforces. `let sound = match &outcome { Ok(Completed(v)) => domain.contains(v), _ => true }`. Failure: an oracle that returns a wrong value that is still in range (for example `sub` rendered as `add` where the sum stays in range) verifies. So does any `Err`, `Refused` or `Undefined` outcome on inputs where the operation is defined, provided the cover is met somewhere. Ticket: IR-458, fixed by open PR #208, which is not on main. | src/kani_obligations.rs:2057-2064 |
| FND-003 | high | Still live. 37 `unreachable!` arms on RT `#[non_exhaustive]` enums panic during generation if an RT bump adds a variant: exact_scalar 26, composite_equality 9, and exact_function 2 inside generated code. `OracleGenerationError::UnknownRuntimeVariant` exists and is used at only two sites (exact_scalar.rs:1221, :1269). Failure: an RT revision adds `IeeeWidth::W16`, and `generate_exact_scalar_oracles` panics in the caller's process instead of returning a typed error. Ticket: IR-352. | src/exact_scalar.rs:336, src/exact_scalar.rs:1753, src/exact_scalar.rs:2039, src/exact_scalar.rs:2324, src/exact_scalar.rs:2799, src/composite_equality.rs:249, src/composite_equality.rs:1422, src/composite_equality.rs:1548, src/exact_function.rs:1331, src/exact_function.rs:1363, src/generation.rs:85 |
| FND-004 | high | Two modules read Kani's console prose, and text they do not recognise becomes a verdict instead of a refusal. `classify_transcript` falls through to `Inconclusive { NoVerdict }` on unrecognised text. The 8 MiB capture keeps the tail and silently drops the head, with no cap refusal. `kani_witness_join` scans playback text separately (`check_clause`, `select_assertion_block`, `read_block`, `concrete_entries`). That contradicts AD-001 ("Kani's printed wording is read in exactly one module"). Failure: a Kani release rewords its banner, and every run reads Inconclusive rather than failing with a stable code. Ticket: IR-277 (and IR-288 for the JSON path). | src/kani_execution.rs:443, src/kani_execution.rs:578-588, src/kani_execution.rs:733-735, src/kani_witness_join.rs:187-347, spec/assurance/AD-001-codegen-architecture.md:167-169 |
| FND-005 | medium | Still live, and it grew. Cross-module invariants are enforced by panics keyed on string operator ids. `lower_scalar_claim` has two `unreachable!` that rely on `exact_scalar::check_parameters` facts. `reachable_range` ends in `_ => unreachable!` over `("quire.op.integer.*", arity)` strings that `scalar_arity` also lists. `exact_function` unwraps or expects Stage-1 results. Failure: someone adds `quire.op.integer.div` to `scalar_arity` without a `reachable_range` arm, and `generate_routed` panics on a valid package. Ticket: IR-344 (typed operator enum) and IR-348. | src/kani_obligations.rs:1398, src/kani_obligations.rs:1414, src/kani_obligations.rs:1503, src/exact_function.rs:854, src/exact_function.rs:911-913 |
| FND-006 | medium | The V1 and V2 dual input path is still live, against the IR-311 ruling (V1 retired, FR-015 the one generator). `negotiate_kani_obligations` takes both `ObligationItem::BoundClause` (V1) and `ScalarClaim` (V2), refuses a mix as `MixedBoundPackages`, and returns both `harnesses` and `scalar_harnesses`. `generate_kani_bundle` (proof_for_contract, FR-003 retired) is public with no owning FR. `bound`, `bound_coverage` and `bound_strategy` read `BoundPackage`. That makes five Kani generation entry points over three input shapes. Failure: a fix lands on one path and the other keeps the defect; FND-002 and FND-009 already differ by path. Ticket: IR-348 / IR-344. | src/kani_obligations.rs:103-108, src/kani_obligations.rs:393, src/kani_obligations.rs:564-571, src/kani.rs:314, src/lib.rs:111 |
| FND-007 | medium | `generate_state_frame_obligations` stops at the first refusal (`?` at each step, and early `Err` for an unknown field, nothing forbidden or an unresolved bound). Failure: a package whose clause has one unsupported effect gets no disposition for its other constructs, which QSL-20 requires. Ticket: IR-461. | src/state_frame.rs:428-433, src/state_frame.rs:445-463, src/state_frame.rs:477-491 |
| FND-008 | medium | Frame replay runs on caller-supplied stand-in identities. `obligation_identity` and `counterexample_identity` are raw `[u8; 32]` inputs, and nothing derives them from `StateFrameIdentity`. The witness is the placeholder `<<<assertion|{operation}|frame|>>>`, and `declared_domains` and `profile_selections` are empty. No `src` code builds the envelope from Kani playback or calls `replay_state_clause`. Failure: a replay settles against an obligation identity that names no harness CG generated. Ticket: IR-459 / IR-460. | src/frame_replay.rs:47-49, src/frame_replay.rs:134, src/frame_replay.rs:165-171 |
| FND-009 | medium | The proof_for_contract harness from `generate_kani_bundle` and the corpus harness emit no `kani::cover!`. Since #202 the classifier reads vacuity from the cover summary, so a healthy run on these paths classifies `Inconclusive { MissingCoverSummary }` and can never be `Verified`. Ticket: IR-464. | src/kani.rs:806-814, src/bounded_kani_corpus.rs:377-381, src/kani_execution.rs:697-709 |
| FND-010 | medium | Default `cargo test` runs real `cargo kani` outside the Kani lane. Three non-ignored tests in `tests/it/bounded_kani_corpus.rs` spawn `cargo kani` and assert on the exit status. Failure: on a host or CI runner without Kani, `make test` and `ci.yml`'s `cargo test` fail with "cargo kani should launch". On a host with Kani, they run unserialised outside `flock`. Ticket: NEW (needs ticket; IR-453's corpus retirement would also remove them). | tests/it/bounded_kani_corpus.rs:251-261, tests/it/bounded_kani_corpus.rs:307, tests/it/bounded_kani_corpus.rs:373 |
| FND-011 | medium | Still live. CI never runs on its own: `ci.yml` is `on: workflow_dispatch` only, `make ci` excludes the Kani lane, and no job runs `--ignored`. Every mutation control that runs real Kani (FND-019 lists them) runs only when someone types `make kani`. Failure: a merged change makes the seeded-defect harness verify, and nothing goes red. Ticket: NEW (needs ticket). | .github/workflows/ci.yml:3-4, Makefile:79-82, Makefile:207 |
| FND-012 | medium | Still live. Errors are not one crate-level envelope. There are 23 public `*Error`/`*Refusal`/`*Diagnostic`/`*Failure` types, 7 implement `std::error::Error`, and none uses `thiserror`. `RoutedGenerationError`, `KaniDiagnostic` and `OracleGenerationError` have no `Display`. `DecodeFailure.code` is a `String`, not a catalogued enum. Failure: a caller composing `generate_routed` with `?` into `Box<dyn Error>` does not compile, and refusal kinds can only be matched by string. Ticket: IR-344. | src/routed_generation.rs:116, src/kani.rs:222, src/generation.rs:73, src/kani_witness_join.rs:47-53 |
| FND-013 | medium | Still live. Identities are stringly typed. `module_symbol`, `harness_symbol` and `solver` are `pub String` although `KaniSolver` exists (the value is always "cadical"). `generate_routed` pairs harnesses to records by a `harness_symbol` string key in a `BTreeMap::collect`, which keeps the last of any duplicates, so a second record would get `harness: None` with no error. Vacuity is decided by `checks_outcome.code == "kani_vacuous_proof"`. Bounds travel as decimal `String` and are re-parsed with `.parse()`. Ticket: IR-344 (identity newtypes). | src/kani_obligations.rs:478-490, src/kani_obligations.rs:535-541, src/state_frame.rs:218-222, src/kani_execution.rs:357, src/routed_generation.rs:281, src/routed_generation.rs:308-309, src/kani_execution.rs:689-690, src/kani_obligations.rs:1421 |
| FND-014 | medium | Operator identity is one fact kept in three places as string literals: 166 `"quire.op.*"` literals in exact_scalar, 9 in kani_obligations and 10 in state_frame, with `_ => unreachable!` closing two of the exact_scalar tables. Adding an operator means hand-editing parallel tables that the compiler does not check against each other (see FND-005). Ticket: IR-344. | src/exact_scalar.rs:1925-1937, src/exact_scalar.rs:2074, src/kani_obligations.rs:1490-1505 |
| FND-015 | medium | Still live, and grown by #203. IR body terms are walked by hand over `serde_json::Value`. In non-test code: exact_scalar 72 `.get("`/`as_*` calls, state_frame 51 (`Graph::follow`, `ClauseShape::read`, `frame_grants`, `condition`, `read_scope`, ...), composite_equality 17, exact_function 15. Tag and form checks compare strings, and no typed IR node decoder (`CheckedNodeKind`) is used anywhere in `src/`. Failure: IR renames a body member, and every walker silently returns `None` and becomes a refusal, with no compile error. Ticket: IR-344 (typed node access), with the IR half on IR-343. | src/state_frame.rs:568-890, src/exact_scalar.rs:924, src/composite_equality.rs:884, src/exact_function.rs:649 |
| FND-016 | medium | AD-001's "Current state" is stale after #206. It says the domain check before replay is not built and that no `WitnessEnvelope` is built. `replay_counterexample` now runs `first_out_of_domain` before replay, and `FrameReplay` builds a `WitnessPacket` and reconstructs a `WitnessEnvelope`. Failure: a reader plans work that already exists, or misses the stand-in identity gap in FND-008. Ticket: IR-329. | spec/assurance/AD-001-codegen-architecture.md:193-201, src/spine_replay.rs:498, src/frame_replay.rs:159-174, src/frame_replay.rs:184-189 |
| FND-017 | low | Duplicated helpers. Still live: `fn artifact` appears 9 times (it was 10). `deterministic_json` appears twice with different error types (`SerializationError` vs `String`), and `unsupported_family` and `application_arguments` 3 times each. Fixed: the `src/` sha256 helpers were removed by #189 and #191. Still live in tests: two identical `sha256_hex` (IR-377). Ticket: IR-344 (one helper module). | src/oracle.rs:1107, src/kani.rs:1051, src/harness.rs:1075, src/oracle.rs:1111, src/kani.rs:1045, src/exact_scalar.rs:897, src/composite_equality.rs:857, src/exact_function.rs:629, tests/exact_scalar_support/package.rs:398, tests/composite_equality_support/package.rs:268 |
| FND-018 | low | Release builds truncate silently. `generate_routed` checks `records.len() == group.len()` and the duplicate-index lookup with `debug_assert` only, then `zip`s. Failure: in release, a record-count mismatch from FR-015 drops items from the output with no error. Ticket: NEW (needs ticket; small). | src/routed_generation.rs:287-294, src/routed_generation.rs:302 |
| FND-019 | low | Test conventions: 89 of 373 tests are not `tc_NNN_` named (most in kani_execution 14, kani_witness_join 12, kani_generation 10, bound_coverage 9, publication 9), and 21 `tc_` tests carry a bare `/// TC-NNN.` instead of `/// Trace:`. About 10 assertions check only `is_err()` or `matches!(.., V(_))` (for example oracle_generation.rs:491-530, kani_obligations.rs:1287). A wall-clock threshold (`elapsed < 2s`) is in the default suite. Mutation controls exist but only in the Kani lane: kani_obligations.rs:1694, :1971; kani_obligations_state_frame.rs:979, :998; kani_witness_join.rs:354. Ticket: NEW (needs ticket; test hygiene). | src/kani_execution.rs:1287, tests/it/oracle_generation.rs:491, tests/it/kani_obligations.rs:1287 |
| FND-020 | low | Launcher residue after #202 (IR-353). A panicked capture thread becomes empty output through `join().unwrap_or_default()`, and that classifies `NoVerdict` rather than a launch error. A timeout too large for `Instant::checked_add` waits forever; this is documented. On a normal exit, leftover processes in the launcher's group are not killed. Ticket: NEW (needs ticket), or fold into IR-277. | src/kani_execution.rs:488-496 |
| FND-021 | low | Still live. A fault-injection parameter is threaded through the production publish path: `PublicationFault` with six arms is checked at five points inside `publish`, and the public entry passes `::None`. This is a test seam inside production control flow. Ticket: IR-348. | src/publication.rs:142-149, src/publication.rs:162-262 |
| FND-022 | low | Still live. There are about 229 names in `pub use` re-exports from `lib.rs`. Pass-through modules (`finite_reference_graphs`, `definedness_arithmetic`, `bounded_collections`) only forward to IR lowerings, which IR-313 OQ-2 moves into CG's backend adapter. Ticket: IR-348. | src/lib.rs:52-159, src/finite_reference_graphs.rs:12-19 |
| FND-023 | low | Still live. There are bare `as` casts on persisted or counted values: `usize as u32` for the consequent count compared to and stored in coverage maps, `(start + token_length) as u32` for columns, and `u32 as usize` indexing. None is reachable to wrap at today's sizes. Ticket: IR-348. | src/bound_coverage.rs:376, src/bound_coverage.rs:479, src/oracle.rs:1159-1160, src/oracle.rs:468 |
| FND-024 | low | Still live. Kani outcome vocabularies overlap: `KaniRunOutcome` / `KaniInconclusiveReason`, `LaunchOutcome` (`TimedOut` in both), `ObligationDisposition`, `KaniObligationOutcome`, `kani_transcript::KaniBanner` and IR's `KaniOutcomeKind`. Vacuity is split across `CoverUnsatisfied`, `MissingCoverSummary` and `VacuousProof`, and no public map goes to QSL `TerminalValue` (C-09). Ticket: IR-465 (C-09 map) and IR-344. | src/kani_execution.rs:287-330, src/kani_execution.rs:418 |
| FND-025 | low | Still live. Module size and churn: exact_scalar.rs is 4170 lines and 62 fns, kani_obligations.rs 2679 lines. lib.rs has 41 commits and oracle.rs 26. Ticket: IR-344 (subsystem directories). | src/exact_scalar.rs, src/kani_obligations.rs |

## Prior audit re-measure (IR-318 comment at 23dcc3d, unverified claims)

| Prior | Status at 5a924e1 |
| --- | --- |
| H1 ~37 unreachable! on RT enums | still-live: exactly 37 (26/9/2); FND-003, IR-352 |
| H2 invariants by panic | still-live, lines moved to kani_obligations.rs:1398/1414/1503, exact_function.rs:854; FND-005 |
| H3 raw serde_json walking | still-live and grown (state_frame.rs from #203); FND-015 |
| H4 launcher unbounded capture, leaked threads, untimed probes, PATH kill, Instant overflow | fixed-by #202 (IR-353): 8 MiB tail cap, threads joined, no probes, rustix group kill, checked_add. Residue in FND-020 |
| M1 ~20 error types, 3 impl Error | still-live: 23 types, 7 impl Error; FND-012 |
| M2 four Kani entry points over two input models | still-live: five entry points; FND-006 |
| M3 cycle kani_obligations<->kani_execution | fixed-by #189 (6f4beea removed KaniToolPins import) |
| M3 cycle kani_execution<->kani_transcript | wrong: the back-edge is in kani_transcript's test module only (kani_transcript.rs:261), at 23dcc3d too |
| M3 exact_scalar as utility library | still-live: kani_obligations.rs:60-63 and state_frame.rs:39 import its walkers |
| M4 artifact x10, sha256 x7, deterministic_json x3, unsupported_family/application_arguments x3 | artifact x9 still-live; sha256 x7 fixed-by #189/#191 in src (tests x2, IR-377); deterministic_json now x2; the x3 pairs still-live; FND-017 |
| M4 hand-written length-delimited hashing x2 | not re-found in src (no length-prefix hashing helper by grep); treat as removed by #189/#191, unconfirmed |
| M5 31 pub String sha256/digest/_symbol fields; harness_symbol joins | partly fixed: no pub String digest/sha256 field remains; symbol and solver strings still-live; FND-013 |
| M6 parallel outcome enums | still-live; FND-024 |
| M7 file size and churn | still-live; FND-025 |
| M8 tc_026 ignored and never run by make kani | fixed-by #202 (IR-354): `make kani` filter includes kani_witness_join |
| L1 ~200 re-exports / L2 pass-through modules | still-live (~229); FND-022 |
| L3 PublicationFault in production path | still-live; FND-021 |
| L4 bare casts | still-live (lines moved); FND-023 |
| L5 66 non-tc tests, is_err-only, ci.yml dispatch-only | still-live: 89 of 373; FND-019, FND-011 |

## Not verified

- I did not run `cargo test`, `make kani` or `make ci`. Whether the suite or the Kani lane is green
  at 5a924e1 is not measured. FND-010's failure on a host without Kani is inferred from the code
  (`Command::new("cargo").args(["kani", ...]).output().expect(...)` plus a status assert), not run.
- FND-002's wrong-in-range-value scenario was not run under Kani. It is read from the harness
  template.
- FND-001: I did not check whether any external consumer (driver, QSL) reads
  `BoundedCorpusCase.outcome`. There is no caller in `src/`.
- PRs #207 and #208 were read only as diffs. Their effect on FND-001 and FND-002 is taken from the
  diff, not measured.
- The panic surface was swept by grep up to each file's first top-level `#[cfg(test)]`. A panic in a
  nested non-test module after that line would be missed.

## Gates

- `cargo clippy --locked --all-targets -- -D warnings` (CARGO_TARGET_DIR private): exit 0, no warnings (4m 31s).

## Verdict

Four high findings. Two are proof-honesty defects in generated evidence: FND-001 is a Kani verdict
invented without a run, and FND-002 is a scalar harness that asserts only the oracle's own bound
check (fix open in #208). FND-003 is the RT-variant panic surface (IR-352). FND-004 is verdicts
read from prose with no refusal for unknown text (IR-277).

The structural debt the earlier audit named is still present: the V1/V2 dual path, stringly
identities, raw JSON walking, error sprawl and duplicated helpers. It all belongs to the IR-344 AD
and the IR-348 refactor. The launcher hardening (#202) and the removal of one import cycle (#189)
are real fixes.

What is right:
- `kani_execution`, `kani_transcript` and `kani_witness_join` have no `unwrap`, `expect` or `panic!`
  outside tests, and no bare casts.
- The kani_obligations and state_frame harnesses carry `kani::cover!`.
- Mutation controls exist for the scalar, precondition, state-frame and witness paths.
- Every cited `Trace:` id resolves under `spec/`.
- The crate forbids `unsafe` code.

Four items need new tickets: FND-010, FND-011, FND-018 and FND-019, plus FND-020 if it is not
folded into IR-277.
