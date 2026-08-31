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
`foundation-cf40e894221c-20260831T163108Z`, collected from clean source revision
`cf40e894221c1028b70e01cbaff56bf64512b809`. It records 16/16 passing outcomes, both
PGM-01 validators, the exact IR validator checkout and digests, and 68/68 verified checksums.

Run `make verify-evidence` to verify every direct `evidence/foundation-*` record. The verifier checks
the complete file/checksum census, envelope and manifest schemas, every manifest artifact digest and
size, local envelope input/output digests, source-identity agreement, and the outcome/status census.

Directories below `evidence/historical/` are quarantined development history and are not
authoritative. In particular, `historical/untrusted-pre-exit-status/` contains records whose
historical conclusive claims predate retained numeric exit statuses.
