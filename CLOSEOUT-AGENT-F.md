# Agent F bounded closeout — 2026-09-10

The owner redirected this lane from expanding codegen #6 to a bounded closeout,
followed by a model switch and a separately scoped OB01–03 baseline. Do not start
that pivot as part of this checkpoint.

## Disposition

- Retain only factual corrections to MP-001 and SUR-001 on the active baseline:
  implemented coverage observations, historical LLVM/Rust identities, public IR
  decoder availability, and removal of a duplicated suite description.
  These facts were covered by focused review SR-019 in the parked snapshot.
  This is not acceptance of the full specification or qualification of a backend.
- Preserve all 36 files of accumulated specification/review work in commit
  `4779a65` on `park/agent-f-codegen6-expansion-2026-09-10`. That branch is an
  unaccepted historical draft, not an implementation-ready specification.
- Park CLI/configuration expansion, conformance execution adapters, platform and
  difference profiles, native observation/seed plumbing, Kani result integration,
  fixture admission, and dependent consumer migration. Do not merge the parked
  branch wholesale or resume it automatically after a model switch.
- No CLI or executable conformance was shipped. Issues #1–5 remain partial;
  #6 and parent #7 are not discharged. There were no open codegen PRs when checked
  for this checkpoint. No ticket closure or review approval is manufactured.

## Constraints and handoff

No CI runs or dispatches; hosted workflows remain manual-dispatch-only. Evidence
review is waived by the owner and is not a blocking gate. Local document checks
are not CI or backend qualification. No source, dependency, toolchain, workflow,
third-party fixture, or upstream repository changes are included.

Consume Rust 1.98.1; all first-party implementation and integration must be Rust.
The owner requires AGPL-3.0-only with no carve-outs; baseline metadata/headers
remain unmigrated. Native Quire is the sole authored clause authority. No Java,
JVM, Maven, Eclipse or Electron dependency is authorized. Exact-file fixture
rights remain a gate; CL-11/12/13 aliases still need authoritative confirmation.

IR remains read-only. Its coverage-state work must not hold this closeout open.
The consumed runtime already exposes CampaignSnapshot and the optional
snapshot-json codec; old draft statements that transport is absent are stale.
Structural decoding and Quoin receipts do not authenticate native execution.

Next session: obtain the authoritative OB01–03 brief, prioritize observation,
correlation, completeness, replay and incremental-monitoring specifications,
then reconcile executable/output consumers. Do not infer these contracts from
their names, wait for C's campaign, or treat this checkpoint as program completion.
