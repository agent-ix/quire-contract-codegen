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
Quire static export, the retained-evidence compatibility view, and the MSRV
build, and writes their structured output to `target/assurance/`.

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

## The compatibility answer, stated plainly

This repository never retained a `quire.pgm01-evidence` record. Its 44 retained
envelopes are `quire.derivation-evidence/v1` — a different schema family, which
the PGM-01 programme governed but did not define. The pinned mapping
`engineering_assurance.verification_semantics.map_pgm01_bytes` therefore answers
`incompatible` for every one of them, with the reason "unknown PGM-01 schema
version".

That is the mapping declining to interpret a shape it has never seen, which is
exactly what it should do and is one of the twelve states this migration is
required to keep distinguishable. It is not a pass, it is not a failure of these
records, and it is not a licence to write a local mapper that would return a
friendlier answer. The gap is filed upstream as
`agent-ix/engineering-assurance#21`.

The mapping is shown to accept as well as refuse: the pinned release's own
`fixtures/verification-semantics/pgm01-v1.json` and `pgm01-v2.json` are read as
positive controls in the same run. A refusal that has never been seen to accept
is indistinguishable from a step that never worked.

## What was frozen rather than deleted

Three files under `schemas/` are frozen. Retained envelopes name each of them by
SHA-256, so deleting one would not remove a generic evidence family from this
repository; it would break a reference inside bytes this migration is required to
leave untouched. `pins.json` records the digests and where each reference sits,
and a test asserts nothing executable references them.

Two other files under `schemas/` are live domain artifacts describing generated
output — the generated-oracle shape and the source-region map shape — and stay in
use. They are not evidence machinery and no retained record names them.
