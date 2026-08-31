#!/usr/bin/env python3
"""Build the PGM-01 codegen foundation evidence record."""

from __future__ import annotations

import datetime as dt
import hashlib
import json
import platform
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parent.parent
PGM01_CANDIDATE_REVISION = "7dac9d8c19952412b56a0347387666e2ca81e01d"
PGM01_ENVELOPE_SCHEMA_DIGEST = (
    "0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256"
)
PGM01_ENVELOPE_SCHEMA = (
    ROOT / "schemas" / "pgm01-derivation-evidence-envelope-v1.schema.json"
)
IR_CANDIDATE_REVISION = "37eb00153d5c139ebc01622b6e12a4ab79256f88"
RUNTIME_CANDIDATE_REVISION = "e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3"
INPUT_SCHEMA = ROOT / "schemas" / "foundation-evidence-input-v1.schema.json"
MANIFEST_SCHEMA = ROOT / "schemas" / "foundation-evidence-manifest-v1.schema.json"
COLLECTOR = ROOT / "scripts" / "collect_foundation_evidence.sh"
BUILDER = Path(__file__).resolve()
SCHEMA_VALIDATOR = ROOT / "scripts" / "validate_json_schema.py"
EVIDENCE_VERIFIER = ROOT / "scripts" / "verify_foundation_evidence.py"
COMMAND_TRANSCRIPTS = (
    ("quire-validate", "quire-validate"),
    ("fmt", "fmt"),
    ("clippy", "clippy"),
    ("test", "test"),
    ("msrv", "msrv"),
    ("deny", "deny"),
    ("unsafe-audit", "unsafe-audit"),
    ("metadata", "metadata"),
    ("rustdoc", "rustdoc"),
    ("coverage", "coverage"),
    ("evidence-tool", "evidence-tool"),
    ("pgm01-pinned-schema", "pgm01-pinned-schema"),
    ("input-schema", "input-schema"),
    ("manifest-schema", "manifest-schema"),
    ("pgm01-schema", "pgm01-schema"),
    ("pgm01-envelope", "pgm01-envelope"),
)
VALIDATOR_TRANSCRIPTS = (
    "pgm01-pinned-schema",
    "input-schema",
    "manifest-schema",
    "pgm01-schema",
    "pgm01-envelope",
)
PASS_CONTRADICTION_MARKERS = {
    "quire-validate": ("\"valid\": false", "validation failed"),
    "fmt": ("Diff in ",),
    "clippy": ("error: could not compile",),
    "test": ("test result: FAILED", "error: test failed"),
    "msrv": ("error: could not compile",),
    "deny": ("error:", "FAILED"),
    "unsafe-audit": ("unsafe audit failed", "missing // SAFETY:"),
    "metadata": ("error:", "error["),
    "rustdoc": ("error: could not document",),
    "coverage": ("[complete-but-unbacked]",),
    "evidence-tool": ("FAILED (", "Traceback (most recent call last)"),
    "pgm01-pinned-schema": ('"valid": false',),
    "input-schema": ('"valid": false',),
    "manifest-schema": ('"valid": false',),
    "pgm01-schema": ('"valid": false',),
    "pgm01-envelope": ('"valid": false', "governance validation error:"),
}


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def digest(value: str) -> dict[str, str]:
    return {"algorithm": "sha256", "value": value}


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def verified_pgm01_schema_digest() -> str:
    """Return the vendored PGM-01 schema digest, failing on pin drift."""
    actual = sha256_file(PGM01_ENVELOPE_SCHEMA)
    if actual != PGM01_ENVELOPE_SCHEMA_DIGEST:
        raise ValueError(
            "vendored PGM-01 envelope schema digest mismatch: "
            f"expected {PGM01_ENVELOPE_SCHEMA_DIGEST}, got {actual}"
        )
    return actual


def command_outcomes(evidence_dir: Path) -> list[dict[str, str]]:
    """Derive outcomes from every retained numeric or availability status file."""
    outcomes = []
    numeric = {
        path.name.removesuffix(".status.txt")
        for path in evidence_dir.glob("*.status.txt")
    }
    availability = {
        path.name.removesuffix("-status.txt")
        for path in evidence_dir.glob("*-status.txt")
        if not path.name.endswith(".status.txt")
    }
    required = {transcript for _, transcript in COMMAND_TRANSCRIPTS}
    transcripts = sorted(required | numeric | availability)
    if not transcripts:
        raise ValueError(f"no retained command statuses in {evidence_dir}")
    for transcript in transcripts:
        name = transcript
        status_path = evidence_dir / f"{transcript}.status.txt"
        availability_path = evidence_dir / f"{transcript}-status.txt"
        availability = (
            availability_path.read_text(encoding="utf-8").strip()
            if availability_path.exists()
            else None
        )
        if availability == "skipped-unavailable":
            outcomes.append({"name": name, "status": availability})
            continue
        if not status_path.exists():
            status = "inconclusive"
        else:
            try:
                exit_status = int(status_path.read_text(encoding="utf-8").strip())
            except ValueError as error:
                raise ValueError(f"invalid exit status in {status_path}") from error
            if exit_status == 0:
                transcript_paths = (
                    evidence_dir / f"{transcript}.stdout",
                    evidence_dir / f"{transcript}.stderr",
                )
                if not all(path.exists() for path in transcript_paths):
                    status = "inconclusive"
                else:
                    combined = "\n".join(
                        path.read_text(encoding="utf-8", errors="replace")
                        for path in transcript_paths
                    )
                    contradiction = next(
                        (
                            marker
                            for marker in PASS_CONTRADICTION_MARKERS.get(name, ())
                            if marker in combined
                        ),
                        None,
                    )
                    if contradiction is not None:
                        raise ValueError(
                            f"passed status for {name} contradicts retained transcript: "
                            f"{contradiction}"
                        )
                    status = "passed"
            else:
                status = "failed"
        outcomes.append({"name": name, "status": status})
    return outcomes


def hash_parameter_files() -> str:
    paths = (
        ROOT / "Cargo.toml",
        ROOT / "Cargo.lock",
        ROOT / "Makefile",
        ROOT / "rust-toolchain.toml",
        COLLECTOR,
        BUILDER,
        SCHEMA_VALIDATOR,
        EVIDENCE_VERIFIER,
        INPUT_SCHEMA,
        MANIFEST_SCHEMA,
        PGM01_ENVELOPE_SCHEMA,
    )
    state = hashlib.sha256()
    for path in paths:
        state.update(str(path.relative_to(ROOT)).encode("utf-8"))
        state.update(b"\0")
        state.update(path.read_bytes())
        state.update(b"\0")
    return state.hexdigest()


# Implements: MP-001
def build(evidence_dir: Path) -> None:
    pgm01_schema_digest = verified_pgm01_schema_digest()
    evidence_dir = evidence_dir.resolve()
    recorded_pgm01_revision = (evidence_dir / "pgm01-revision.txt").read_text(
        encoding="utf-8"
    ).strip()
    if recorded_pgm01_revision != IR_CANDIDATE_REVISION:
        raise ValueError(
            "PGM-01 validator revision mismatch: "
            f"expected {IR_CANDIDATE_REVISION}, got {recorded_pgm01_revision}"
        )
    recorded_pgm01_schema_digest = (
        evidence_dir / "pgm01-schema-sha256.txt"
    ).read_text(encoding="utf-8").strip()
    if recorded_pgm01_schema_digest != pgm01_schema_digest:
        raise ValueError(
            "external PGM-01 schema digest mismatch: "
            f"expected {pgm01_schema_digest}, got {recorded_pgm01_schema_digest}"
        )
    invocation_directory = (
        str(evidence_dir.relative_to(ROOT))
        if evidence_dir.is_relative_to(ROOT)
        else str(evidence_dir)
    )
    revision = (evidence_dir / "source-revision.txt").read_text(encoding="utf-8").strip()
    source_state = (evidence_dir / "source-state.txt").read_text(encoding="utf-8").strip()
    metadata = json.loads((evidence_dir / "metadata.stdout").read_text(encoding="utf-8"))
    package = next(
        item for item in metadata["packages"] if item["name"] == "quire-contract-codegen"
    )
    recorded_at_path = evidence_dir / "recorded-at.txt"
    if recorded_at_path.exists():
        recorded_at = recorded_at_path.read_text(encoding="utf-8").strip()
    else:
        recorded_at = (
            dt.datetime.now(dt.timezone.utc)
            .replace(microsecond=0)
            .isoformat()
            .replace("+00:00", "Z")
        )
        recorded_at_path.write_text(recorded_at + "\n", encoding="utf-8")

    collection_input = {
        "schemaVersion": "quire.codegen-foundation-evidence-input/v1",
        "sourceRevision": revision,
        "sourceState": source_state,
        "phase": "foundation",
        "commands": [
            "quire validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md'",
            "python3 scripts/validate_json_schema.py schemas/foundation-evidence-input-v1.schema.json collection-input.json",
            "python3 scripts/validate_json_schema.py schemas/foundation-evidence-manifest-v1.schema.json evidence-manifest.json",
            "python3 scripts/validate_json_schema.py schemas/pgm01-derivation-evidence-envelope-v1.schema.json evidence-envelope.json",
            "python3 scripts/validate_json_schema.py $PGM01_SCHEMA evidence-envelope.json",
            "python3 $PGM01_VALIDATOR --fixture evidence-envelope.json",
            "cargo fmt --all -- --check",
            "cargo clippy --all-targets -- -D warnings",
            "cargo test",
            "cargo +1.75.0 check --lib",
            "cargo deny check licenses",
            "bash scripts/check_unsafe_comments.sh",
            "cargo metadata --format-version 1",
            "RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps",
            "quire coverage --scope .",
            "make evidence-tool",
        ],
        "tools": {
            "cargo": (evidence_dir / "cargo-version.txt")
            .read_text(encoding="utf-8")
            .splitlines()[0],
            "jsonschema": (evidence_dir / "jsonschema-version.txt")
            .read_text(encoding="utf-8")
            .strip(),
            "python": (evidence_dir / "python-version.txt")
            .read_text(encoding="utf-8")
            .strip(),
            "python-packages": (evidence_dir / "python-packages.txt")
            .read_text(encoding="utf-8")
            .strip(),
            "quire": json.loads(
                (evidence_dir / "quire-provenance.json").read_text(encoding="utf-8")
            )["cli"]["version"],
            "rustc": (evidence_dir / "rustc-version.txt")
            .read_text(encoding="utf-8")
            .splitlines()[0],
            "rust-msrv": (evidence_dir / "msrv-rustc-version.txt")
            .read_text(encoding="utf-8")
            .splitlines()[0],
        },
        "dependencies": {
            "pgm01": {
                "policy": "ix://agent-ix/quire-contract-ir/PGM-01",
                "candidateRevision": PGM01_CANDIDATE_REVISION,
                "envelopeSchema": "quire.derivation-evidence/v1",
                "envelopeSchemaDigest": digest(pgm01_schema_digest),
                "schemaPath": (evidence_dir / "pgm01-schema-path.txt")
                .read_text(encoding="utf-8")
                .strip(),
                "schemaDigest": digest(
                    (evidence_dir / "pgm01-schema-sha256.txt")
                    .read_text(encoding="utf-8")
                    .strip()
                ),
                "validatorPath": (evidence_dir / "pgm01-validator-path.txt")
                .read_text(encoding="utf-8")
                .strip(),
                "validatorDigest": digest(
                    (evidence_dir / "pgm01-validator-sha256.txt")
                    .read_text(encoding="utf-8")
                    .strip()
                ),
                "validatorRevision": (evidence_dir / "pgm01-revision.txt")
                .read_text(encoding="utf-8")
                .strip(),
            },
            "irCorpus": f"agent-ix/quire-contract-ir@{IR_CANDIDATE_REVISION}",
            "runtimeCandidateRevision": RUNTIME_CANDIDATE_REVISION,
        },
    }
    input_path = evidence_dir / "collection-input.json"
    write_json(input_path, collection_input)

    excluded = {
        "collection-input.json",
        "evidence-envelope.json",
        "evidence-manifest.json",
        "pgm01-envelope.stderr",
        "pgm01-envelope.stdout",
        "pgm01-envelope-status.txt",
        "sha256sums.txt",
    }
    for transcript in VALIDATOR_TRANSCRIPTS:
        excluded.update(
            {
                f"{transcript}.status.txt",
                f"{transcript}.stderr",
                f"{transcript}.stdout",
                f"{transcript}-status.txt",
            }
        )
    entries = []
    for path in sorted(evidence_dir.iterdir(), key=lambda item: item.name):
        if path.is_file() and path.name not in excluded:
            entries.append(
                {
                    "path": path.name,
                    "sha256": sha256_file(path),
                    "size": path.stat().st_size,
                }
            )

    outcomes = command_outcomes(evidence_dir)
    manifest = {
        "schemaVersion": "quire.codegen-foundation-evidence-manifest/v1",
        "sourceRevision": revision,
        "collectedAt": recorded_at,
        "outcomes": outcomes,
        "artifacts": entries,
        "limitations": [
            "foundation evidence does not establish semantic code-generation conformance",
            "authoritative IR schema and corpus candidate remains under review",
            "runtime source is merged; its human source-release decision remains pending",
            "hosted CI is intentionally deferred by operator direction",
        ],
    }
    manifest_path = evidence_dir / "evidence-manifest.json"
    write_json(manifest_path, manifest)

    failed = [item["name"] for item in outcomes if item["status"] == "failed"]
    inconclusive = [
        item["name"] for item in outcomes if item["status"] == "inconclusive"
    ]
    if failed:
        result_status = "inconclusive"
        result_summary = f"{len(failed)} codegen foundation checks failed"
    elif inconclusive:
        result_status = "pending"
        result_summary = f"{len(inconclusive)} foundation check outcomes are inconclusive"
    else:
        result_status = "conclusive"
        result_summary = (
            "all executed codegen foundation checks passed; semantic claims are out of scope"
        )
    envelope = {
        "schemaVersion": "quire.derivation-evidence/v1",
        "recordId": evidence_dir.name,
        "recordedAt": recorded_at,
        "producer": {
            "name": "quire-contract-codegen-foundation-collector",
            "version": package["version"],
            "sourceRevision": revision,
            "executableDigest": digest(sha256_file(COLLECTOR)),
            "invocation": ["scripts/collect_foundation_evidence.sh", invocation_directory],
        },
        "inputs": [
            {
                "role": "foundation-evidence-collection-input",
                "uri": "collection-input.json",
                "mediaType": "application/json",
                "schema": {
                    "id": "quire.codegen-foundation-evidence-input",
                    "version": "v1",
                    "digest": digest(sha256_file(INPUT_SCHEMA)),
                },
                "contentDigest": digest(sha256_file(input_path)),
            },
            {
                "role": "pgm01-envelope-schema",
                "uri": f"https://github.com/agent-ix/quire-contract-ir/blob/{PGM01_CANDIDATE_REVISION}/schemas/derivation-evidence-envelope-v1.schema.json",
                "mediaType": "application/schema+json",
                "schema": {
                    "id": "quire.derivation-evidence",
                    "version": "v1",
                    "digest": digest(pgm01_schema_digest),
                },
                "contentDigest": digest(
                    (evidence_dir / "pgm01-schema-sha256.txt")
                    .read_text(encoding="utf-8")
                    .strip()
                ),
            },
            {
                "role": "pgm01-governance-validator",
                "uri": f"https://github.com/agent-ix/quire-contract-ir/blob/{IR_CANDIDATE_REVISION}/scripts/validate_governance.py",
                "mediaType": "text/x-python",
                "schema": {
                    "id": "python-source",
                    "version": "v3",
                    "digest": digest(
                        (evidence_dir / "pgm01-validator-sha256.txt")
                        .read_text(encoding="utf-8")
                        .strip()
                    ),
                },
                "contentDigest": digest(
                    (evidence_dir / "pgm01-validator-sha256.txt")
                    .read_text(encoding="utf-8")
                    .strip()
                ),
            },
        ],
        "backend": {
            "kind": "none",
            "reason": (
                "deterministic foundation evidence packaging; invoked tools are identified "
                "in the input and manifest"
            ),
        },
        "outputs": [
            {
                "role": "codegen-foundation-evidence-manifest",
                "uri": "evidence-manifest.json",
                "mediaType": "application/json",
                "schema": {
                    "id": "quire.codegen-foundation-evidence-manifest",
                    "version": "v1",
                    "digest": digest(sha256_file(MANIFEST_SCHEMA)),
                },
                "contentDigest": digest(sha256_file(manifest_path)),
            }
        ],
        "parametersDigest": digest(hash_parameter_files()),
        "environment": {
            "targetTriple": next(
                line.split(": ", 1)[1]
                for line in (evidence_dir / "rustc-version.txt")
                .read_text(encoding="utf-8")
                .splitlines()
                if line.startswith("host: ")
            ),
            "operatingSystem": platform.platform(),
            "toolchain": collection_input["tools"]["rustc"],
            "dependenciesDigest": digest(sha256_file(ROOT / "Cargo.lock")),
        },
        "provenance": {
            "repository": "https://github.com/agent-ix/quire-contract-codegen",
            "sourceRevision": revision,
            "candidateRevision": revision,
            "contributionMethod": "agent-assisted",
            "reviewers": ["@kreneskyp"],
        },
        "result": {
            "status": result_status,
            "summary": result_summary,
            "requirementRefs": ["PGM-01-R08", "PGM-01-R09", "MP-001"],
        },
        "extensions": {
            "dev.agent-ix.codegen": {
                "componentClass": "direct-development-tool",
                "envelopeSchemaDigest": pgm01_schema_digest,
                "phase": "foundation",
                "pgm01CandidateRevision": PGM01_CANDIDATE_REVISION,
                "reviewState": "pending",
                "sourceState": source_state,
            }
        },
    }
    write_json(evidence_dir / "evidence-envelope.json", envelope)


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: build_foundation_envelope.py EVIDENCE_DIR", file=sys.stderr)
        return 2
    evidence_dir = Path(sys.argv[1])
    build(evidence_dir)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
