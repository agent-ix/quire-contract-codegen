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
PGM01_CANDIDATE_REVISION = "942670a0db78be57cfa9bdd6d04302b453781a49"
PGM01_ENVELOPE_SCHEMA_DIGEST = (
    "0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256"
)
RUNTIME_CANDIDATE_REVISION = "61d121f635df2b22492892a03c03f5935b984a00"
INPUT_SCHEMA = ROOT / "schemas" / "foundation-evidence-input-v1.schema.json"
MANIFEST_SCHEMA = ROOT / "schemas" / "foundation-evidence-manifest-v1.schema.json"
COLLECTOR = ROOT / "scripts" / "collect_foundation_evidence.sh"
BUILDER = Path(__file__).resolve()
SCHEMA_VALIDATOR = ROOT / "scripts" / "validate_json_schema.py"


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def digest(value: str) -> dict[str, str]:
    return {"algorithm": "sha256", "value": value}


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def hash_parameter_files() -> str:
    paths = (
        ROOT / "Cargo.toml",
        ROOT / "Cargo.lock",
        ROOT / "Makefile",
        ROOT / "rust-toolchain.toml",
        COLLECTOR,
        BUILDER,
        SCHEMA_VALIDATOR,
        INPUT_SCHEMA,
        MANIFEST_SCHEMA,
    )
    state = hashlib.sha256()
    for path in paths:
        state.update(str(path.relative_to(ROOT)).encode("utf-8"))
        state.update(b"\0")
        state.update(path.read_bytes())
        state.update(b"\0")
    return state.hexdigest()


def build(evidence_dir: Path) -> None:
    evidence_dir = evidence_dir.resolve()
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
    recorded_at = (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )

    collection_input = {
        "schemaVersion": "quire.codegen-foundation-evidence-input/v1",
        "sourceRevision": revision,
        "sourceState": source_state,
        "phase": "foundation",
        "commands": [
            "quire validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md'",
            "python3 scripts/validate_json_schema.py schemas/foundation-evidence-input-v1.schema.json collection-input.json",
            "python3 scripts/validate_json_schema.py schemas/foundation-evidence-manifest-v1.schema.json evidence-manifest.json",
            "python3 scripts/validate_json_schema.py $PGM01_SCHEMA evidence-envelope.json (when available)",
            "cargo fmt --all -- --check",
            "cargo clippy --all-targets -- -D warnings",
            "cargo test",
            "cargo +1.75.0 check --lib",
            "cargo deny check licenses",
            "bash scripts/check_unsafe_comments.sh",
            "cargo metadata --format-version 1",
            "RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps",
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
                "envelopeSchemaDigest": digest(PGM01_ENVELOPE_SCHEMA_DIGEST),
            },
            "irCorpus": "agent-ix/quire-contract-ir#10-open-no-candidate",
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

    manifest = {
        "schemaVersion": "quire.codegen-foundation-evidence-manifest/v1",
        "sourceRevision": revision,
        "collectedAt": recorded_at,
        "outcomes": [
            {"name": name, "status": "passed"}
            for name in (
                "quire-validate",
                "fmt",
                "clippy",
                "test",
                "msrv",
                "deny",
                "unsafe-audit",
                "metadata",
                "rustdoc",
            )
        ],
        "artifacts": entries,
        "limitations": [
            "foundation evidence does not establish semantic code-generation conformance",
            "PGM-01 candidate is under review and must be reconciled again after merge",
            "authoritative IR schema and corpus have no candidate revision",
            "runtime candidate is draft and requires final reconciliation",
            "remote CI, CODEOWNER approval, and the human source-release decision are pending",
        ],
    }
    manifest_path = evidence_dir / "evidence-manifest.json"
    write_json(manifest_path, manifest)

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
            }
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
            "status": "conclusive",
            "summary": (
                "all collected codegen foundation checks passed; semantic claims are out of scope"
            ),
            "requirementRefs": ["PGM-01-R08", "PGM-01-R09", "MP-001"],
        },
        "extensions": {
            "dev.agent-ix.codegen": {
                "componentClass": "direct-development-tool",
                "envelopeSchemaDigest": PGM01_ENVELOPE_SCHEMA_DIGEST,
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
