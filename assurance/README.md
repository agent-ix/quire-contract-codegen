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
  mapping — no longer has a subject in this repository. The epic records that it
  closes as moot rather than as fixed once the campaign repositories have dropped
  their retained records; it names four, so this repository does not close it
  unilaterally and it remains open upstream at the time of writing.
- **`unsupported` and `malformed`** were two of the twelve shared verification
  states. Measured before anything was deleted, per state and per source: the
  assurance chain alone demonstrated ten, and the compatibility census supplied
  exactly these two. FR-006-AC-5 now requires ten and both claims are withdrawn.

  `unsupported` is not in the assurance chain's producer vocabulary and never was,
  so there is nothing to restate it over. The generation corpus reaches an
  `unsupported` *Interface-001 terminal state*, which is a different vocabulary on
  a different axis and is not borrowed to fill the gap.

  `malformed` is in that vocabulary, and it was still withdrawn — this is the part
  worth recording, because the first attempt got it wrong. A scenario was added
  that fed the adapter a producer stream declaring `outcome: malformed`, on the
  reasoning that a declared key is a state that travels the chain. Measured against
  the existing `fail` scenario, its receipt reasons were byte-identical:
  `ROW_RESULTS` and `CONFORMANCE_OUTCOMES` both map `malformed` onto `fail`, which
  is exactly why the chain's own `non-success-states-stay-distinguishable`
  scenario could not have included it. It was `attested-failed` under a second
  name, and it was removed. Ten states that are real beats eleven with one painted
  on.

  What stops the gate weakening quietly is not a manufactured demonstration. It is
  `tests/shared_assurance.rs` asserting that both states stay **absent**, so a
  later change that re-acquires one goes red and has to argue for it.

## Two schemas remain, and the third was not live after all

The change that deleted the retained evidence kept
`schemas/pgm01-derivation-evidence-envelope-v1.schema.json`, describing it here
and in `assurance/pins.json` as a live output contract that the generator
validated every emitted derivation manifest against. That was wrong, and it is
recorded rather than quietly corrected because the reasoning is the reusable part.

`src/oracle.rs` bound the file with `include_bytes!` as `PGM_SCHEMA` and used it
in exactly one place: `generator_implementation_digest()`, where it was one of ten
byte blobs fed to a hasher. Nothing in the crate validated against it. The only
schema validation lived in three integration tests, which compiled the file
themselves. Deleting the file did break the build — and that is what the earlier
review observed and read as liveness — but an `include_bytes!` of a missing path
is a compile error, not a contract under load. "It doesn't compile without this"
and "something depends on this" are different claims, and only the second one
justifies a keep.

It was the deprecated PGM-01 derivation-evidence envelope, the same format the
campaign removed everywhere else, and it is now deleted under
`agent-ix/quire-contract-codegen#18`. Removing a hash input moved the generator's
own identity digest. That was accepted deliberately, not overlooked: this is
pre-release software with no consumers, and both statements of the hash-input
list — the one in `src/oracle.rs` and the independent one in
`tests/oracle_generation.rs` — moved together.

The two that stay are live on the stronger test. `schemas/generated-rust-oracle-v1.schema.json`
and `schemas/oracle-source-map-v1.schema.json` are each included by
`src/oracle.rs`, which emits their SHA-256 as the schema identity of the output
artifact they describe, and `tests/oracle_generation.rs` compiles both and
validates the generated Rust and the generated source map against them with a
positive and a negative case each. Removing either breaks an assertion, not just a
path. A test asserts both are still named by the generator, so a later tidy-up
cannot fold them into the deleted set on the strength of the directory they share.
