#!/usr/bin/env python3
"""Build the PGM-01 codegen foundation evidence record."""

from __future__ import annotations

import datetime as dt
import hashlib
import json
import platform
import re
import sys
from pathlib import Path
from typing import Any

SCRIPTS = Path(__file__).resolve().parent
if str(SCRIPTS) not in sys.path:
    sys.path.insert(0, str(SCRIPTS))

from evidence_policy import (  # noqa: E402
    MINIMUM_PYTHON_TESTS,
    MINIMUM_RUST_TESTS,
    MINIMUM_TRANSCRIPT_BYTES,
)

ROOT = Path(__file__).resolve().parent.parent
PGM01_CANDIDATE_REVISION = "7dac9d8c19952412b56a0347387666e2ca81e01d"
PGM01_ENVELOPE_SCHEMA_DIGEST = (
    "0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256"
)
PGM01_ENVELOPE_SCHEMA = (
    ROOT / "schemas" / "pgm01-derivation-evidence-envelope-v1.schema.json"
)
IR_CANDIDATE_REVISION = "5c49ebfd1c87415f74420ad047392bd03b1bd202"
RUNTIME_CANDIDATE_REVISION = "e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3"
INPUT_SCHEMA = ROOT / "schemas" / "foundation-evidence-input-v1.schema.json"
MANIFEST_SCHEMA = ROOT / "schemas" / "foundation-evidence-manifest-v1.schema.json"
COLLECTOR = ROOT / "scripts" / "collect_foundation_evidence.sh"
EVIDENCE_REQUIREMENTS = ROOT / "requirements-evidence.txt"
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
    "quire-validate": ('"valid": false', "document(s) failed structural validation"),
    "fmt": ("Diff in ",),
    "clippy": ("error: could not compile",),
    "test": ("test result: FAILED", "error: test failed"),
    "msrv": ("error: could not compile",),
    "deny": ("error:", "FAILED"),
    "unsafe-audit": ("unsafe audit failed", "missing SAFETY comment near"),
    "metadata": ("\nerror:", "\nerror["),
    "rustdoc": ("error: could not document",),
    "coverage": ("COVERAGE_STATUS_CONTRADICTION",),
    "evidence-tool": ("FAILED (", "Traceback (most recent call last)"),
    "pgm01-pinned-schema": ('"valid": false',),
    "input-schema": ('"valid": false',),
    "manifest-schema": ('"valid": false',),
    "pgm01-schema": ('"valid": false',),
    "pgm01-envelope": ('"valid": false', "governance validation error:"),
}
PASS_CORROBORATION_MARKERS = {
    "quire-validate": ("QUIRE_VALIDATION_PASSED",),
    "fmt": ("FMT_CHECK_PASSED",),
    "clippy": ("Finished `dev` profile",),
    "test": ("test result: ok.",),
    "msrv": ("test result: ok.",),
    "deny": ("advisories ok, bans ok, licenses ok, sources ok",),
    "unsafe-audit": ("unsafe audit passed",),
    "metadata": ('"packages"',),
    "rustdoc": ("Generated ",),
    "coverage": ('"statusLies": 0',),
    "evidence-tool": ("executed ", " Python tests from "),
    "pgm01-pinned-schema": ('"valid": true',),
    "input-schema": ('"valid": true',),
    "manifest-schema": ('"valid": true',),
    "pgm01-schema": ('"valid": true',),
    "pgm01-envelope": ('"valid": true',),
}
INCONCLUSIVE_TRANSCRIPT_MARKERS = {
    "coverage": ("COVERAGE_STATUS_INCONCLUSIVE",),
}
COLLECTED_COMMANDS = {
    "quire-validate": "quire validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md' && echo QUIRE_VALIDATION_PASSED",
    "fmt": "cargo fmt --all -- --check && echo FMT_CHECK_PASSED",
    "clippy": "cargo clippy --locked --all-targets -- -D warnings",
    "test": "cargo test --locked",
    "msrv": "cargo +1.75.0 test --locked",
    "deny": "CARGO_HOME=<isolated> cargo deny --offline --locked check",
    "unsafe-audit": "bash scripts/check_unsafe_comments.sh",
    "metadata": "cargo metadata --locked --format-version 1",
    "rustdoc": "RUSTDOCFLAGS=-Dwarnings cargo doc --locked --no-deps",
    "coverage": "python3 scripts/check_coverage_status.py",
    "evidence-tool": "make evidence-tool",
    "pgm01-pinned-schema": "python3 scripts/validate_json_schema.py schemas/pgm01-derivation-evidence-envelope-v1.schema.json evidence-envelope.json",
    "input-schema": "python3 scripts/validate_json_schema.py schemas/foundation-evidence-input-v1.schema.json collection-input.json",
    "manifest-schema": "python3 scripts/validate_json_schema.py schemas/foundation-evidence-manifest-v1.schema.json evidence-manifest.json",
    "pgm01-schema": "python3 scripts/validate_json_schema.py <PGM01_SCHEMA> evidence-envelope.json",
    "pgm01-envelope": "python3 <PGM01_VALIDATOR> --fixture evidence-envelope.json",
}

COLLECTOR_COMMAND_FRAGMENTS = {
    "quire-validate": "run_and_retain quire-validate \\\n  \"$trusted_bash\" -c '\"$1\" validate --scope . '\\''spec/**/*.md'\\'' '\\''planning/**/*.md'\\'' '\\''plan/**/*.md'\\'' && echo QUIRE_VALIDATION_PASSED' _ \"$trusted_quire\"",
    "fmt": "run_and_retain fmt \\\n  \"$trusted_bash\" -c '\"$1\" fmt --all -- --check && echo FMT_CHECK_PASSED'",
    "clippy": 'run_and_retain clippy "$trusted_cargo" clippy --locked --all-targets -- -D warnings',
    "test": 'run_and_retain test "$trusted_cargo" test --locked',
    "msrv": 'run_and_retain msrv "$trusted_cargo" +1.75.0 test --locked',
    "deny": 'run_and_retain deny /usr/bin/env CARGO_HOME="$deny_cargo_home" "$trusted_cargo" deny --offline --locked check',
    "unsafe-audit": 'run_and_retain unsafe-audit "$trusted_bash" scripts/check_unsafe_comments.sh',
    "metadata": 'run_and_retain metadata "$trusted_cargo" metadata --locked --format-version 1',
    "rustdoc": 'run_and_retain rustdoc /usr/bin/env RUSTDOCFLAGS=-Dwarnings "$trusted_cargo" doc --locked --no-deps',
    "coverage": 'run_and_retain coverage "$trusted_python" scripts/check_coverage_status.py',
    "evidence-tool": "run_and_retain evidence-tool /usr/bin/make evidence-tool",
    "pgm01-pinned-schema": "run_and_retain pgm01-pinned-schema \\\n    \"$trusted_python\" scripts/validate_json_schema.py \\\n    schemas/pgm01-derivation-evidence-envelope-v1.schema.json \\\n    \"$evidence_dir/evidence-envelope.json\"",
    "input-schema": "run_and_retain input-schema \\\n    \"$trusted_python\" scripts/validate_json_schema.py \\\n    schemas/foundation-evidence-input-v1.schema.json \"$evidence_dir/collection-input.json\"",
    "manifest-schema": "run_and_retain manifest-schema \\\n    \"$trusted_python\" scripts/validate_json_schema.py \\\n    schemas/foundation-evidence-manifest-v1.schema.json \"$evidence_dir/evidence-manifest.json\"",
    "pgm01-schema": "run_and_retain pgm01-schema \\\n    \"$trusted_python\" scripts/validate_json_schema.py \\\n    \"$pgm01_schema_path\" \"$evidence_dir/evidence-envelope.json\"",
    "pgm01-envelope": "run_and_retain pgm01-envelope \\\n    \"$trusted_python\" \"$pgm01_validator_path\" --fixture \"$evidence_dir/evidence-envelope.json\"",
}


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def digest(value: str) -> dict[str, str]:
    return {"algorithm": "sha256", "value": value}


def write_json(path: Path, value: Any) -> None:
    path.write_text(
        json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


def verified_pgm01_schema_digest() -> str:
    """Return the vendored PGM-01 schema digest, failing on pin drift."""
    actual = sha256_file(PGM01_ENVELOPE_SCHEMA)
    if actual != PGM01_ENVELOPE_SCHEMA_DIGEST:
        raise ValueError(
            "vendored PGM-01 envelope schema digest mismatch: "
            f"expected {PGM01_ENVELOPE_SCHEMA_DIGEST}, got {actual}"
        )
    return actual


def transcript_is_corroborated(name: str, stdout: str, stderr: str) -> bool:
    """Require positive, command-specific evidence that a zero-exit gate did work."""
    combined = stdout + "\n" + stderr
    if len(stdout.encode()) + len(stderr.encode()) < MINIMUM_TRANSCRIPT_BYTES:
        return False
    markers = PASS_CORROBORATION_MARKERS.get(name)
    if markers is None or not all(marker in combined for marker in markers):
        return False
    if name in {"test", "msrv"}:
        passed = sum(
            int(count)
            for count in re.findall(r"test result: ok\. ([0-9]+) passed;", combined)
        )
        return passed >= MINIMUM_RUST_TESTS
    if name == "evidence-tool":
        match = re.search(
            r"executed ([0-9]+) Python tests from ([0-9]+) files", combined
        )
        return (
            match is not None
            and int(match.group(1)) >= MINIMUM_PYTHON_TESTS
            and int(match.group(2)) >= 1
            and "\nOK\n" in combined
        )
    if name == "metadata":
        try:
            value = json.loads(stdout)
        except json.JSONDecodeError:
            return False
        return isinstance(value, dict) and bool(value.get("packages"))
    if name == "coverage":
        try:
            value = json.loads(stdout.splitlines()[0])
        except (IndexError, json.JSONDecodeError):
            return False
        return value.get("statusLies") == 0 and isinstance(
            value.get("totals", {}).get("total"), int
        )
    if name in VALIDATOR_TRANSCRIPTS:
        try:
            value = json.loads(stdout)
        except json.JSONDecodeError:
            return False
        return value.get("valid") is True and value.get("errors") == []
    return True


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
        if availability is not None and availability not in {"passed", "failed"}:
            raise ValueError(f"invalid availability status in {availability_path}")
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
                    stdout, stderr = (
                        path.read_text(encoding="utf-8", errors="replace")
                        for path in transcript_paths
                    )
                    combined = stdout + "\n" + stderr
                    contradiction = next(
                        (
                            marker
                            for marker in PASS_CONTRADICTION_MARKERS.get(name, ())
                            if marker in combined
                        ),
                        None,
                    )
                    if contradiction is not None:
                        status = "failed"
                    elif not transcript_is_corroborated(name, stdout, stderr):
                        status = "inconclusive"
                    elif any(
                        marker in combined
                        for marker in INCONCLUSIVE_TRANSCRIPT_MARKERS.get(name, ())
                    ):
                        status = "inconclusive"
                    else:
                        status = "passed"
            else:
                status = "failed"
            expected_word = "passed" if exit_status == 0 else "failed"
            if availability is not None and availability != expected_word:
                status = "failed"
        outcomes.append({"name": name, "status": status})
    return outcomes


def summarize_outcomes(
    outcomes: list[dict[str, str]],
) -> tuple[str, str, list[str]]:
    """Derive a truthful terminal result and per-outcome limitations."""
    failed = sorted(item["name"] for item in outcomes if item["status"] == "failed")
    inconclusive = sorted(
        item["name"] for item in outcomes if item["status"] == "inconclusive"
    )
    skipped = sorted(
        item["name"] for item in outcomes if item["status"] == "skipped-unavailable"
    )
    limitations = [
        *(f"failed foundation outcome: {name}" for name in failed),
        *(f"inconclusive foundation outcome: {name}" for name in inconclusive),
        *(f"skipped-unavailable foundation outcome: {name}" for name in skipped),
    ]
    if failed:
        return (
            "rejected",
            f"{len(failed)} codegen foundation checks failed",
            limitations,
        )
    if inconclusive or skipped:
        parts = []
        if inconclusive:
            parts.append(f"{len(inconclusive)} inconclusive")
        if skipped:
            parts.append(f"{len(skipped)} skipped-unavailable")
        return (
            "pending",
            "foundation outcomes: " + ", ".join(parts),
            limitations,
        )
    return (
        "conclusive",
        "all retained codegen foundation checks passed; semantic claims are out of scope",
        limitations,
    )


def parameter_files() -> list[Path]:
    """Return the complete source-and-test set controlling foundation evidence."""
    fixed_paths = {
        ROOT / "CLAUDE.md",
        ROOT / "Cargo.toml",
        ROOT / "Cargo.lock",
        ROOT / "Makefile",
        ROOT / ".gitignore",
        ROOT / "clippy.toml",
        ROOT / "deny.toml",
        ROOT / "rustfmt.toml",
        ROOT / "rust-toolchain.toml",
        ROOT / "spec" / "test-matrix.md",
        ROOT / "spec" / "assurance" / "MP-001-codegen-measurements.md",
        EVIDENCE_REQUIREMENTS,
        INPUT_SCHEMA,
        MANIFEST_SCHEMA,
        PGM01_ENVELOPE_SCHEMA,
    }
    controlled_paths = {
        path
        for directory in (
            ROOT / "src",
            ROOT / "scripts",
            ROOT / "tools",
            ROOT / "schemas",
        )
        if directory.exists()
        for path in directory.rglob("*")
        if path.is_file() and "__pycache__" not in path.parts
    }
    test_paths = {
        path
        for path in (ROOT / "tests").rglob("*")
        if path.is_file() and "__pycache__" not in path.parts
    }
    return sorted(
        fixed_paths | controlled_paths | test_paths,
        key=lambda path: path.relative_to(ROOT).as_posix(),
    )


def hash_parameter_files() -> str:
    paths = parameter_files()
    state = hashlib.sha256()
    for path in paths:
        state.update(str(path.relative_to(ROOT)).encode("utf-8"))
        state.update(b"\0")
        state.update(path.read_bytes())
        state.update(b"\0")
    return state.hexdigest()


def gate_script_digests() -> dict[str, str]:
    """Bind every executable local gate implementation into the record."""
    return {
        path.relative_to(ROOT).as_posix(): sha256_file(path)
        for path in parameter_files()
        if path.is_relative_to(ROOT / "scripts") or path.is_relative_to(ROOT / "tools")
    }


def collected_commands() -> list[str]:
    """Return commands in transcript order after binding them to the collector."""
    configured = [transcript for _, transcript in COMMAND_TRANSCRIPTS]
    if set(configured) != set(COLLECTED_COMMANDS):
        raise ValueError("collected command census differs from transcript census")
    collector_text = COLLECTOR.read_text(encoding="utf-8")
    missing = [
        name
        for name, fragment in COLLECTOR_COMMAND_FRAGMENTS.items()
        if fragment not in collector_text
    ]
    if missing:
        raise ValueError(f"collector command implementation drift: {missing}")
    return [COLLECTED_COMMANDS[name] for name in configured]


def foundation_limitations(outcomes: list[dict[str, str]]) -> list[str]:
    _, _, outcome_limitations = summarize_outcomes(outcomes)
    return [
        "foundation evidence does not establish semantic code-generation conformance",
        "runtime source is merged; its human source-release decision remains pending",
        "hosted CI is intentionally deferred by operator direction",
        *outcome_limitations,
    ]


def expected_manifest_artifact_names(evidence_dir: Path) -> set[str]:
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
    return {
        path.name
        for path in evidence_dir.iterdir()
        if path.is_file() and path.name not in excluded
    }


# Implements: MP-001
def build(evidence_dir: Path) -> None:
    pgm01_schema_digest = verified_pgm01_schema_digest()
    evidence_dir = evidence_dir.resolve()
    recorded_pgm01_revision = (
        (evidence_dir / "ir-validator-revision.txt").read_text(encoding="utf-8").strip()
    )
    if recorded_pgm01_revision != IR_CANDIDATE_REVISION:
        raise ValueError(
            "PGM-01 validator revision mismatch: "
            f"expected {IR_CANDIDATE_REVISION}, got {recorded_pgm01_revision}"
        )
    recorded_pgm01_schema_digest = (
        (evidence_dir / "pgm01-schema-sha256.txt").read_text(encoding="utf-8").strip()
    )
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
    revision = (
        (evidence_dir / "source-revision.txt").read_text(encoding="utf-8").strip()
    )
    source_state = (
        (evidence_dir / "source-state.txt").read_text(encoding="utf-8").strip()
    )
    metadata = json.loads(
        (evidence_dir / "metadata.stdout").read_text(encoding="utf-8")
    )
    package = next(
        item
        for item in metadata["packages"]
        if item["name"] == "quire-contract-codegen"
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
        "gateScripts": gate_script_digests(),
        "commands": collected_commands(),
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
                "validatorRevision": (evidence_dir / "ir-validator-revision.txt")
                .read_text(encoding="utf-8")
                .strip(),
            },
            "irCorpus": f"agent-ix/quire-contract-ir@{IR_CANDIDATE_REVISION}",
            "runtimeCandidateRevision": RUNTIME_CANDIDATE_REVISION,
        },
    }
    input_path = evidence_dir / "collection-input.json"
    write_json(input_path, collection_input)

    artifact_names = expected_manifest_artifact_names(evidence_dir)
    entries = []
    for path in sorted(evidence_dir.iterdir(), key=lambda item: item.name):
        if path.name in artifact_names:
            entries.append(
                {
                    "path": path.name,
                    "sha256": sha256_file(path),
                    "size": path.stat().st_size,
                }
            )

    outcomes = command_outcomes(evidence_dir)
    result_status, result_summary, _ = summarize_outcomes(outcomes)
    manifest = {
        "schemaVersion": "quire.codegen-foundation-evidence-manifest/v1",
        "sourceRevision": revision,
        "collectedAt": recorded_at,
        "outcomes": outcomes,
        "artifacts": entries,
        "limitations": foundation_limitations(outcomes),
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
            "invocation": [
                "scripts/collect_foundation_evidence.sh",
                invocation_directory,
            ],
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
            "reviewers": [
                (evidence_dir / "reviewer-of-record.txt")
                .read_text(encoding="utf-8")
                .strip()
            ],
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
                "reviewerRole": "reviewer-of-record; not a GitHub approval",
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
