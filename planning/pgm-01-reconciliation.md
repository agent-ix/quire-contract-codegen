---
id: REV-003
title: "Codegen PGM-01 candidate reconciliation"
type: Review
---

# PGM-01 candidate reconciliation

PGM-01 candidate: `agent-ix/quire-contract-ir#12` at
`d8d376d887c40255e87ef9656bc0faf79216b321`.

Envelope schema: `quire.derivation-evidence/v1`, SHA-256
`0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256`.

The foundation adopts the canonical policy URI, direct-development-tool classification, required
producer/input/backend/output/environment/provenance/result identities, typed non-success states,
dual-license boundary, exact development pins, and open human authority. Generated Rust is separately
classified as linked runtime and remains subject to consuming-project verification.

The foundation evidence envelope validates against the exact PGM-01 candidate. This is provisional:
PGM-01 review and merge, the authoritative IR schema/corpus, runtime review, deliberate remote CI,
CODEOWNER approval, and the human source-release decision remain open. Final dependency identities
must be reconciled before semantic implementation leaves draft.

The current candidate retains the same envelope schema digest. Its policy repository gates pass
21/21 Quire documents, 28/28 backed criteria, the 13/13 Draft 7 corpus, and all seven schema/format
mutation probes. The foundation envelope is accepted by both the candidate schema and its custom
validator with zero errors. PGM-01's remaining review findings stay open and are not converted into
codegen claims.

The candidate's release-only verifier also matches its unique retained record across 66/66 complete
HEAD/worktree inputs and 5/5 committed outputs, including the published PGM evidence-manifest schema.

The exact candidate schema is vendored at
`schemas/pgm01-derivation-evidence-envelope-v1.schema.json`. The foundation builder derives its
digest, fails on disagreement with the executable pin, and its MP-001 unit tests require the
executable revision and digest to agree with this reconciliation and the dependency-pin review.
