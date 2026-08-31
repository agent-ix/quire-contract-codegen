"""Tests for foundation evidence assembly and local JSON Schema validation."""

from __future__ import annotations

import importlib.util
import json
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


def load_builder():
    spec = importlib.util.spec_from_file_location("foundation_evidence_builder", BUILDER_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load foundation evidence builder")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


builder = load_builder()


class FoundationEvidenceBuilderTests(unittest.TestCase):
    def test_evidence_tools_have_measurement_plan_ownership(self) -> None:
        for path in (BUILDER_PATH, VALIDATOR_PATH):
            self.assertIn("# Implements: MP-001", path.read_text(encoding="utf-8"), path.name)

    def test_dependency_pins_match_vendored_schema_and_planning(self) -> None:
        self.assertEqual(
            builder.verified_pgm01_schema_digest(),
            builder.PGM01_ENVELOPE_SCHEMA_DIGEST,
        )
        pgm_text = (ROOT / "planning/pgm-01-reconciliation.md").read_text(encoding="utf-8")
        pins_text = (ROOT / "planning/draft-dependency-pins.md").read_text(encoding="utf-8")
        gap_text = (ROOT / "planning/foundation-gap-analysis.md").read_text(encoding="utf-8")
        for text, label in ((pgm_text, "PGM reconciliation"), (pins_text, "dependency pins")):
            self.assertIn(builder.PGM01_CANDIDATE_REVISION, text, label)
            self.assertIn(builder.PGM01_ENVELOPE_SCHEMA_DIGEST, text, label)
        for text, label in ((pins_text, "dependency pins"), (gap_text, "gap analysis")):
            self.assertIn(builder.RUNTIME_CANDIDATE_REVISION, text, label)

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
            self.assertEqual(dependencies["pgm01"]["candidateRevision"], builder.PGM01_CANDIDATE_REVISION)
            self.assertEqual(
                dependencies["pgm01"]["envelopeSchemaDigest"]["value"],
                builder.PGM01_ENVELOPE_SCHEMA_DIGEST,
            )
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

    def test_passed_status_cannot_contradict_retained_transcript(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory) / "foundation-fixture"
            evidence_dir.mkdir()
            self.write_fixture_inputs(evidence_dir)
            (evidence_dir / "test.stdout").write_text(
                "test result: FAILED. 0 passed; 7 failed\n", encoding="utf-8"
            )

            with self.assertRaisesRegex(ValueError, "contradicts retained transcript"):
                builder.build(evidence_dir)

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
            "cargo-version.txt": "cargo 1.94.1\n",
            "jsonschema-version.txt": "3.2.0\n",
            "python-version.txt": "Python 3.10.12\n",
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
