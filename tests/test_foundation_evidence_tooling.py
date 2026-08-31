"""Tests for foundation evidence assembly and local JSON Schema validation."""

from __future__ import annotations

import importlib.util
import hashlib
import json
import re
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


ROOT = Path(__file__).resolve().parent.parent
BUILDER_PATH = ROOT / "scripts" / "build_foundation_envelope.py"
COLLECTOR_PATH = ROOT / "scripts" / "collect_foundation_evidence.sh"
VALIDATOR_PATH = ROOT / "scripts" / "validate_json_schema.py"
VERIFIER_PATH = ROOT / "scripts" / "verify_foundation_evidence.py"
COVERAGE_PATH = ROOT / "scripts" / "check_coverage_status.py"
MEASUREMENT_PLAN = ROOT / "spec" / "assurance" / "MP-001-codegen-measurements.md"


def load_builder():
    spec = importlib.util.spec_from_file_location("foundation_evidence_builder", BUILDER_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load foundation evidence builder")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


builder = load_builder()


def load_verifier():
    spec = importlib.util.spec_from_file_location("foundation_evidence_verifier", VERIFIER_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load foundation evidence verifier")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


verifier = load_verifier()


class FoundationEvidenceBuilderTests(unittest.TestCase):
    def test_evidence_tools_have_measurement_plan_ownership(self) -> None:
        tools = sorted((ROOT / "scripts").glob("*.py")) + sorted(
            (ROOT / "scripts").glob("*.sh")
        )
        self.assertGreaterEqual(len(tools), 5)
        plan = MEASUREMENT_PLAN.read_text(encoding="utf-8")
        for path in tools:
            self.assertRegex(
                path.read_text(encoding="utf-8"),
                r"(?m)^# Implements: [A-Za-z0-9-]+$",
                path.name,
            )
            self.assertIn(f"`scripts/{path.name}`", plan, path.name)

    def test_dependency_pins_match_vendored_schema_and_planning(self) -> None:
        self.assertEqual(
            builder.verified_pgm01_schema_digest(),
            builder.PGM01_ENVELOPE_SCHEMA_DIGEST,
        )
        pgm_text = (ROOT / "planning/pgm-01-reconciliation.md").read_text(encoding="utf-8")
        pins_text = (ROOT / "planning/draft-dependency-pins.md").read_text(encoding="utf-8")
        gap_text = (ROOT / "planning/foundation-gap-analysis.md").read_text(encoding="utf-8")
        cac_text = (ROOT / "spec/assurance/CAC-001-codegen-contract.md").read_text(
            encoding="utf-8"
        )
        for text, label in ((pgm_text, "PGM reconciliation"), (pins_text, "dependency pins")):
            self.assertIn(builder.PGM01_CANDIDATE_REVISION, text, label)
            self.assertIn(builder.PGM01_ENVELOPE_SCHEMA_DIGEST, text, label)
        for text, label in ((pins_text, "dependency pins"), (gap_text, "gap analysis")):
            self.assertIn(builder.RUNTIME_CANDIDATE_REVISION, text, label)
        for revision in (
            builder.PGM01_CANDIDATE_REVISION,
            builder.IR_CANDIDATE_REVISION,
            builder.RUNTIME_CANDIDATE_REVISION,
        ):
            self.assertIn(revision, cac_text)

    def test_pgm01_pin_mismatch_fails_closed(self) -> None:
        with mock.patch.object(builder, "PGM01_ENVELOPE_SCHEMA_DIGEST", "0" * 64):
            with self.assertRaisesRegex(ValueError, "schema digest mismatch"):
                builder.verified_pgm01_schema_digest()

    def test_build_preserves_dependency_identity_roles_digests_and_extensions(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory) / "foundation-fixture"
            evidence_dir.mkdir()
            self.write_fixture_inputs(evidence_dir)

            builder.build(evidence_dir)

            collection_input = self.read_json(evidence_dir / "collection-input.json")
            manifest = self.read_json(evidence_dir / "evidence-manifest.json")
            envelope = self.read_json(evidence_dir / "evidence-envelope.json")

            dependencies = collection_input["dependencies"]
            self.assertEqual(dependencies["runtimeCandidateRevision"], builder.RUNTIME_CANDIDATE_REVISION)
            self.assertEqual(
                dependencies["irCorpus"],
                f"agent-ix/quire-contract-ir@{builder.IR_CANDIDATE_REVISION}",
            )
            self.assertEqual(dependencies["pgm01"]["candidateRevision"], builder.PGM01_CANDIDATE_REVISION)
            self.assertEqual(
                dependencies["pgm01"]["envelopeSchemaDigest"]["value"],
                builder.PGM01_ENVELOPE_SCHEMA_DIGEST,
            )
            self.assertEqual(
                dependencies["pgm01"]["validatorRevision"],
                builder.IR_CANDIDATE_REVISION,
            )
            self.assertEqual(len(envelope["inputs"]), 3)
            self.assertEqual(manifest["sourceRevision"], "a" * 40)
            self.assertIn(
                {"name": "pgm01-pinned-schema", "status": "passed"},
                manifest["outcomes"],
            )
            self.assertEqual(
                envelope["inputs"][0]["role"], "foundation-evidence-collection-input"
            )
            self.assertEqual(
                envelope["outputs"][0]["role"], "codegen-foundation-evidence-manifest"
            )
            self.assertEqual(
                envelope["inputs"][0]["contentDigest"]["value"],
                builder.sha256_file(evidence_dir / "collection-input.json"),
            )
            self.assertEqual(
                envelope["outputs"][0]["contentDigest"]["value"],
                builder.sha256_file(evidence_dir / "evidence-manifest.json"),
            )
            extension = envelope["extensions"]["dev.agent-ix.codegen"]
            self.assertEqual(extension["componentClass"], "direct-development-tool")
            self.assertEqual(extension["phase"], "foundation")
            self.assertEqual(
                extension["pgm01CandidateRevision"], builder.PGM01_CANDIDATE_REVISION
            )
            self.assertEqual(
                extension["envelopeSchemaDigest"], builder.PGM01_ENVELOPE_SCHEMA_DIGEST
            )
            self.assertEqual(envelope["parametersDigest"]["value"], builder.hash_parameter_files())

    def test_build_records_failed_and_missing_commands_without_a_pass_claim(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory) / "foundation-fixture"
            evidence_dir.mkdir()
            self.write_fixture_inputs(evidence_dir)
            (evidence_dir / "clippy.status.txt").write_text("101\n", encoding="utf-8")
            (evidence_dir / "fmt.status.txt").unlink()

            builder.build(evidence_dir)

            manifest = self.read_json(evidence_dir / "evidence-manifest.json")
            envelope = self.read_json(evidence_dir / "evidence-envelope.json")
            outcomes = {item["name"]: item["status"] for item in manifest["outcomes"]}
            self.assertEqual(outcomes["clippy"], "failed")
            self.assertEqual(outcomes["fmt"], "inconclusive")
            self.assertEqual(envelope["result"]["status"], "inconclusive")
            self.assertNotIn("all executed", envelope["result"]["summary"])

    def test_passed_status_contradiction_is_retained_as_failure(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory) / "foundation-fixture"
            evidence_dir.mkdir()
            self.write_fixture_inputs(evidence_dir)
            (evidence_dir / "test.stdout").write_text(
                "test result: FAILED. 0 passed; 7 failed\n", encoding="utf-8"
            )

            builder.build(evidence_dir)

            manifest = self.read_json(evidence_dir / "evidence-manifest.json")
            envelope = self.read_json(evidence_dir / "evidence-envelope.json")
            outcomes = {item["name"]: item["status"] for item in manifest["outcomes"]}
            self.assertEqual(outcomes["test"], "failed")
            self.assertEqual(envelope["result"]["status"], "inconclusive")

    def test_new_retained_failure_is_included_by_outcome_census(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory) / "foundation-fixture"
            evidence_dir.mkdir()
            self.write_fixture_inputs(evidence_dir)
            (evidence_dir / "audit.status.txt").write_text("101\n", encoding="utf-8")
            (evidence_dir / "audit.stdout").write_text("", encoding="utf-8")
            (evidence_dir / "audit.stderr").write_text(
                "error: 3 vulnerabilities found\n", encoding="utf-8"
            )

            builder.build(evidence_dir)

            manifest = self.read_json(evidence_dir / "evidence-manifest.json")
            envelope = self.read_json(evidence_dir / "evidence-envelope.json")
            self.assertIn(
                {"name": "audit", "status": "failed"}, manifest["outcomes"]
            )
            self.assertEqual(envelope["result"]["status"], "inconclusive")

    def test_collector_and_declared_command_sets_agree(self) -> None:
        collector = COLLECTOR_PATH.read_text(encoding="utf-8").split(
            'quire provenance --pretty >"$evidence_dir/quire-provenance.json"', 1
        )[1]
        collected = set(
            re.findall(r"(?m)^\s*run_and_retain ([a-z0-9-]+)(?: |$)", collector)
        )
        declared = {transcript for _, transcript in builder.COMMAND_TRANSCRIPTS}
        self.assertEqual(collected, declared)

    def test_every_declared_command_has_contradiction_markers(self) -> None:
        declared = {name for name, _ in builder.COMMAND_TRANSCRIPTS}
        self.assertEqual(declared, set(builder.PASS_CONTRADICTION_MARKERS))

    def test_all_matrix_rows_remain_planned_until_upstream_fix(self) -> None:
        matrix = (ROOT / "spec" / "test-matrix.md").read_text(encoding="utf-8")
        rows = [
            line
            for line in matrix.splitlines()
            if line.startswith(("| FR-", "| StR-", "| TC-"))
        ]
        self.assertGreater(len(rows), 0)
        for row in rows:
            cells = [cell.strip() for cell in row.strip("|").split("|")]
            self.assertEqual(cells[-1], "🚧 Planned", row)

    def test_skipped_outcome_forces_pending_result_and_limitation(self) -> None:
        outcomes = [
            {"name": "test", "status": "passed"},
            {"name": "validator", "status": "skipped-unavailable"},
        ]
        status, summary, limitations = builder.summarize_outcomes(outcomes)
        self.assertEqual(status, "pending")
        self.assertIn("skipped", summary)
        self.assertEqual(
            limitations,
            ["skipped-unavailable foundation outcome: validator"],
        )

    def test_verifier_rederives_outcome_values(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory)
            self.write_fixture_inputs(evidence_dir)
            derived = builder.command_outcomes(evidence_dir)
            declared = [dict(item) for item in derived]
            declared[0]["status"] = "failed"
            self.assertNotEqual(derived, declared)

    def test_checksum_verifier_detects_transcript_tampering(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = Path(directory)
            transcript = record / "test.stdout"
            transcript.write_text("1 passed\n", encoding="utf-8")
            digest = hashlib.sha256(transcript.read_bytes()).hexdigest()
            (record / "sha256sums.txt").write_text(
                f"{digest}  ./test.stdout\n", encoding="utf-8"
            )
            self.assertEqual(verifier.verify_checksums(record), 1)
            transcript.write_text("999 passed\n", encoding="utf-8")
            with self.assertRaisesRegex(verifier.EvidenceError, "checksum mismatch"):
                verifier.verify_checksums(record)

    def test_validator_transcript_exclusions_are_explicitly_named(self) -> None:
        expected = {
            "pgm01-pinned-schema",
            "input-schema",
            "manifest-schema",
            "pgm01-schema",
            "pgm01-envelope",
        }
        self.assertEqual(set(builder.VALIDATOR_TRANSCRIPTS), expected)
        declared = {transcript for _, transcript in builder.COMMAND_TRANSCRIPTS}
        self.assertTrue(expected.issubset(declared))

    def test_collector_fail_closed_self_test(self) -> None:
        completed = subprocess.run(
            ["bash", str(COLLECTOR_PATH), "--self-test"],
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertIn("foundation collector fail-closed self-test passed", completed.stdout)

    @staticmethod
    def read_json(path: Path):
        return json.loads(path.read_text(encoding="utf-8"))

    @staticmethod
    def write_fixture_inputs(evidence_dir: Path) -> None:
        values = {
            "source-revision.txt": "a" * 40 + "\n",
            "source-state.txt": "clean\n",
            "reviewer-of-record.txt": "@kreneskyp\n",
            "cargo-version.txt": "cargo 1.94.1\n",
            "jsonschema-version.txt": "3.2.0\n",
            "python-version.txt": "Python 3.10.12\n",
            "python-packages.txt": "jsonschema==3.2.0\nrfc3339-validator==0.1.4\nrfc3986-validator==0.1.1\n",
            "pgm01-schema-path.txt": "/tmp/quire-contract-ir/schemas/derivation-evidence-envelope-v1.schema.json\n",
            "pgm01-schema-sha256.txt": builder.PGM01_ENVELOPE_SCHEMA_DIGEST + "\n",
            "pgm01-validator-path.txt": "/tmp/quire-contract-ir/scripts/validate_governance.py\n",
            "pgm01-validator-sha256.txt": "b" * 64 + "\n",
            "pgm01-revision.txt": builder.IR_CANDIDATE_REVISION + "\n",
            "rustc-version.txt": "rustc 1.94.1\nhost: x86_64-unknown-linux-gnu\n",
            "msrv-rustc-version.txt": "rustc 1.75.0\nhost: x86_64-unknown-linux-gnu\n",
        }
        for name, value in values.items():
            (evidence_dir / name).write_text(value, encoding="utf-8")
        for _, transcript in builder.COMMAND_TRANSCRIPTS:
            (evidence_dir / f"{transcript}.status.txt").write_text(
                "0\n", encoding="utf-8"
            )
            (evidence_dir / f"{transcript}.stdout").write_text("", encoding="utf-8")
            (evidence_dir / f"{transcript}.stderr").write_text("", encoding="utf-8")
        (evidence_dir / "metadata.stdout").write_text(
            json.dumps(
                {"packages": [{"name": "quire-contract-codegen", "version": "0.1.0"}]}
            ),
            encoding="utf-8",
        )
        (evidence_dir / "quire-provenance.json").write_text(
            json.dumps({"cli": {"version": "0.31.0"}}), encoding="utf-8"
        )


class SchemaValidatorTests(unittest.TestCase):
    def test_validator_accepts_valid_and_reports_invalid_path(self) -> None:
        schema = {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "additionalProperties": False,
            "required": ["value"],
            "properties": {"value": {"type": "integer"}},
        }
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "schema.json"
            instance_path = root / "instance.json"
            schema_path.write_text(json.dumps(schema), encoding="utf-8")

            instance_path.write_text('{"value": 1}', encoding="utf-8")
            accepted = self.run_validator(schema_path, instance_path)
            self.assertEqual(accepted.returncode, 0, accepted.stderr)
            self.assertEqual(json.loads(accepted.stdout), {"errors": [], "valid": True})

            instance_path.write_text('{"value": "wrong"}', encoding="utf-8")
            rejected = self.run_validator(schema_path, instance_path)
            self.assertEqual(rejected.returncode, 1, rejected.stderr)
            result = json.loads(rejected.stdout)
            self.assertFalse(result["valid"])
            self.assertEqual(result["errors"][0]["path"], "$.value")

    def test_validator_rejects_invalid_date_time_format(self) -> None:
        schema = {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "required": ["recordedAt"],
            "properties": {"recordedAt": {"type": "string", "format": "date-time"}},
        }
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "schema.json"
            instance_path = root / "instance.json"
            schema_path.write_text(json.dumps(schema), encoding="utf-8")
            instance_path.write_text('{"recordedAt":"NOT-A-TIMESTAMP"}', encoding="utf-8")
            rejected = self.run_validator(schema_path, instance_path)
            self.assertEqual(rejected.returncode, 1, rejected.stderr)

    @staticmethod
    def run_validator(schema_path: Path, instance_path: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(VALIDATOR_PATH), str(schema_path), str(instance_path)],
            check=False,
            capture_output=True,
            text=True,
        )


if __name__ == "__main__":
    unittest.main()
