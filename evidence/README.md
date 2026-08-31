# Foundation evidence index

Foundation evidence is produced by `scripts/collect_foundation_evidence.sh`. It creates a new
revision-and-UTC-timestamp-scoped directory, refuses to overwrite an existing record, and requires a
clean source tree plus a clean, exact IR validator checkout. It proves only foundation gates; it
cannot support semantic generation, parity, or release claims.

The collector emits canonical `quire.derivation-evidence/v1` JSON plus separately versioned input
and manifest schemas. `PGM01_SCHEMA` and `PGM01_VALIDATOR` are mandatory and their resolved paths,
digests, checkout revision, and Python package set are retained. CI workflows are manual-only;
hosted CI is currently deferred and no remote result is claimed by these records.

The current authoritative record is
`foundation-984594e48a0c-20260831T172037Z`, collected from clean source revision
`984594e48a0c773a67108432a74ae38e6f50c17d`. It records 15 passing outcomes, one explicitly
inconclusive coverage-status outcome, a `pending` overall result, both PGM-01 validators, the exact
IR validator checkout and digests, and 69/69 verified checksums.

Run `make verify-evidence` with the exact packages in `requirements-evidence.txt` to verify the
committed `evidence/ANCHORS` record-set boundary. The verifier checks the complete file/checksum
census, the recursively anchored historical and remote trees, envelope and manifest schemas plus
formats, every manifest artifact digest and size, local envelope input/output digests,
source-identity agreement, external PGM-01 schema identity, and independently re-derived outcome
values, result status, summary, and limitations.

Directories below `evidence/historical/` are quarantined development history and are not
authoritative. In particular, `historical/untrusted-pre-exit-status/` contains records whose
historical conclusive claims predate retained numeric exit statuses.
