# Foundation evidence index

Foundation evidence is produced by `scripts/collect_foundation_evidence.sh`. It creates a new
revision-and-UTC-timestamp-scoped directory, refuses to overwrite an existing record, and requires a
clean source tree plus a clean, exact IR validator checkout. It proves only foundation gates; it
cannot support semantic generation, parity, or release claims.

The collector emits canonical `quire.derivation-evidence/v1` JSON plus separately versioned input
and manifest schemas. `PGM01_SCHEMA` and `PGM01_VALIDATOR` are mandatory and their resolved paths,
digests, checkout revision, and Python package set are retained. CI workflows are manual-only;
hosted CI is currently deferred and no remote result is claimed by these records.

The current authoritative record is the foundation directory whose `sha256sums.txt` is named
directly by `evidence/ANCHORS`. Its envelope and `source-revision.txt` carry the full source revision.
The record retains every foundation outcome, both PGM-01 validators, and the exact IR validator
checkout and digests. Coverage remains explicitly inconclusive while upstream status-column
classification is unavailable, so the overall foundation result is `pending`, never conclusive.
Collection verifies a new record directly but does not rewrite `evidence/ANCHORS`; updating the
anchor census and running the complete verifier are separate review-boundary steps before commit.

Run `make verify-evidence` with the exact packages in `requirements-evidence.txt` to verify the
committed `evidence/ANCHORS` record-set boundary. The verifier checks the complete file/checksum
census, the recursively anchored historical and remote trees, envelope and manifest schemas plus
formats, every manifest artifact digest and size, local envelope input/output digests,
source-identity agreement, external PGM-01 schema identity, complete parameter-tool digest, and
independently re-derived outcome values, result status, summary, and limitations.

Directories below `evidence/historical/` are quarantined development history and are not
authoritative. In particular, `historical/untrusted-pre-exit-status/` contains records whose
historical conclusive claims predate retained numeric exit statuses.

`historical/retired-oracle-rebase-round6/` retains the preceding Round 6 foundation authority with
an in-envelope retraction. It is superseded by the deterministic-oracle source record and remains
available for audit rather than being deleted during the stacked-branch rebase.

`historical/retired-harness-rebase-round6/` retains the deterministic-oracle authority with an
in-envelope retraction. It is superseded by the source-bound harness/strategy record and remains
available for audit as the PR #10 boundary beneath the stacked PR #12 work.

`historical/retired-harness-review-round1/` retains the pre-review harness/strategy authority with
an in-envelope retraction. It is superseded by the PR #12 Round 1 remediation record, whose source
implements campaign conclusions, executable expected-domain binding, complete terminal states,
and harness/strategy derivation manifests.
