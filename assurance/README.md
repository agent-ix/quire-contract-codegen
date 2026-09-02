# Shared assurance

Two files and no evidence.

`change-assurance.json` is what this repository *states* about the change under
[issue #13](https://github.com/agent-ix/quire-contract-codegen/issues/13): the
requirements it claims to meet, the things it promises not to break, the proofs
it offers, and the questions it cannot answer. `pins.json` is the Engineering
Assurance release it adopts and the digests of the artifacts it actually reads
from that release.

## Why there is no evidence in here

Because retention is Quoin's job. `make assurance` seals the declaration into a
Quoin change-assurance record, seals a proof attestation over each producer's
already-written result file, hands those bytes to Quoin's intake, and asks for a
verification receipt. The record, the attestations, the retained bytes, and the
receipt all live in Quoin's store under `target/`, which is ignored.

The repository that produced a result does not also get to be the place that
result is kept, digested, and pronounced upon. That arrangement is the thing this
migration removed — a collector, an envelope builder, a verifier, an anchor
writer, a coverage reimplementation, and a Makefile that policed its own
execution controls — and putting a smaller version of it back under a new
directory name would be the same mistake in a nicer font.

## What runs what

One target produces:

```
make assurance-inputs
```

It runs the generation conformance corpus, the upstream-identity check, the
Quire static export, and the MSRV build, and writes their structured output to
`target/assurance/`.

Everything downstream consumes those files. `scripts/assurance_chain.py` refuses
to run a producer; if an input is missing it says so and names the target that
makes it. Quire exports and does not execute. Quoin transcribes and does not
execute. That separation is asserted by a test with a control, not just described
here.

## The generation producer

`examples/generation_conformance.rs` is this repository's headline producer. It
walks the bounded corpus through the crate's public API and publishes
`codegen.generation-conformance/v1` — one row per case, each carrying the
Interface-001 terminal state the case reached, the diagnostic code it produced,
and the number of declared checks that held against a floor.

The floor is the part that matters. A case that holds every check it ran but runs
fewer than its floor reads `vacuous`, not `pass`. Without it a corpus can go green
by getting smaller, which is the same failure as a proof that simplified away.

Its rejection cases exist to keep the terminal states apart. `unsupported` — a
construct the generator declines to lower — and `invalid-input` — a construct it
rejects as wrong — are different facts for a caller, and a census row fails if the
corpus stops reaching both.

## What is not here, and why that is the honest answer

FR-003 (Kani obligation lowering) and FR-004 (vacuity and rejection evidence) are
specified and have no implementation at this revision. There is no Kani code and
no vacuity code under `src/`, and every TM-001 row for them is 🚧 Planned.

So there is no suite for them and no proof obligation over them. A proof
obligation whose subject does not exist is the most complete false green
available: it would run, report `pass`, and mean nothing. The specification is
preserved, the planned status is preserved, and nothing was manufactured.

## The decision that is not here

A verification receipt for this change reads `incomplete`, and the reason it
gives is that no human decision event exists. That is correct. An ix-flow
decision is an attributed human act; only the repository owner can create one,
and an agent that synthesized one would be forging the single field in the whole
chain that exists to say a person looked.

## What was deleted with the retained evidence

This repository used to carry 2,205 files under `evidence/` — 44 envelopes of
`quire.derivation-evidence/v1`, a schema family the PGM-01 programme governed but
did not define, which the pinned mapping refused as an unknown schema version for
every one of them. It also carried the reader that asked the mapping, the fixtures
that exercised it, a proof obligation over its answer, a `compat-view` Make
target, and two schemas frozen only because retained records named them by digest.

All of it is deleted. The repository owner released the preservation constraint
for the pre-stable phase on 2026-09-02; the authority is the
"Preservation constraint released for the pre-stable phase" section of
`agent-ix/engineering-assurance#7`. Evidence becomes something to protect when
these repositories move toward stable releases, and the constraint re-applies
unchanged at that point.

Nothing was rewritten, backdated, or re-sealed to look as though it still
verifies. The claims that rested on those records were removed with them:

- **FR-006-AC-4** asserted retained bytes were read through the shared mapping
  and reported without collapsing. There are no retained bytes, so the criterion,
  its TM-001 rows, its test and its proof obligation are gone rather than
  restated over a smaller tree.
- **`agent-ix/engineering-assurance#21`** — the missing `quire.derivation-evidence/v1`
  mapping — closes as moot rather than as fixed. The records it was about no
  longer exist here.
- **`unsupported`** was one of the twelve shared verification states, and on this
  repository the compatibility census was its only demonstration. It is not in the
  assurance chain's producer vocabulary and never was, so FR-006-AC-5 withdraws it
  rather than restating it over a refusal or borrowing the Interface-001 terminal
  state of the same name. `tests/shared_assurance.rs` asserts it stays absent, so
  a later change cannot quietly re-acquire it.

  **`malformed` did not go with it**, and that is the measurement worth recording.
  Taken before anything was deleted, per state and per source: the chain alone
  demonstrated ten of the twelve, and the census supplied `unsupported` *and*
  `malformed`. `malformed` is a declared member of the adapter's producer outcome
  vocabulary, so the `attested-malformed` scenario now demonstrates it from a
  producer stream that really reports it, derived from the real corpus run. Had it
  simply been dropped, TC-012 would have gone on passing at ten and reported the
  same green — a gate weakening silently rather than a claim being withdrawn.

## Three schemas remain, and all three are live

`schemas/pgm01-derivation-evidence-envelope-v1.schema.json` looks like evidence
machinery and is not: `src/oracle.rs` includes it at compile time as `PGM_SCHEMA`,
the generator validates every derivation manifest it emits against it, and it is
part of the generator's own `executable_digest`. The `pgm01` in its filename names
the programme that governed the shape, not the records that were deleted. A
filename is not a dependency.

That distinction was measured on this repository rather than inherited. A sibling
froze four artifacts including its own vendored copy of this schema; inheriting
that list would have described a live dependency as dead machinery, and deleting
on the strength of it would have broken every generation.

The generated-oracle shape and the source-region map shape are live for the same
kind of reason — both are included by `src/oracle.rs` and validated against in
`tests/oracle_generation.rs`. All three stay in use, and a test asserts that each
is still named by the generator so a later tidy-up cannot fold them into the
deleted set on the strength of the directory they share.
