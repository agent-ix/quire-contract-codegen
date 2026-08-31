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
RUNNER_PATH = ROOT / "scripts" / "run_python_tests.py"
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


def load_runner():
    spec = importlib.util.spec_from_file_location("recursive_python_test_runner", RUNNER_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load recursive Python test runner")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


runner = load_runner()


def load_coverage_checker():
    spec = importlib.util.spec_from_file_location("coverage_status_checker", COVERAGE_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load coverage status checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


coverage_checker = load_coverage_checker()


class FoundationEvidenceBuilderTests(unittest.TestCase):
    def test_evidence_tools_have_measurement_plan_ownership(self) -> None:
        tools = sorted(
            path
            for directory in (ROOT / "scripts", ROOT / "tools")
            if directory.exists()
            for path in directory.rglob("*")
            if path.is_file() and path.suffix in {".py", ".sh"}
        )
        self.assertGreaterEqual(len(tools), 5)
        plan = MEASUREMENT_PLAN.read_text(encoding="utf-8")
        for path in tools:
            self.assertRegex(
                path.read_text(encoding="utf-8"),
                r"(?m)^# Implements: [A-Za-z0-9-]+$",
                path.name,
            )
            self.assertIn(f"`{path.relative_to(ROOT)}`", plan, path.name)

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
            self.assertEqual(envelope["result"]["status"], "rejected")
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
            self.assertEqual(envelope["result"]["status"], "rejected")

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
            self.assertEqual(envelope["result"]["status"], "rejected")

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

    def test_coverage_checker_rejects_unminted_ids_and_every_diagnostic_reason(self) -> None:
        report = {
            "minted_targets": [{"id": "TC-001"}, {"id": "FR-001-AC-1"}],
            "diagnostics": [{"reason": "future-open-vocabulary-reason"}],
        }
        self.assertEqual(
            coverage_checker.undeclared_matrix_ids(
                report,
                "| FR-001 | FR-001-AC-1 | TC-001 |\n"
                "| FR-999 | FR-999-AC-1 | TC-999 |",
            ),
            ["FR-999-AC-1", "TC-999"],
        )
        self.assertEqual(
            coverage_checker.diagnostic_reasons(report),
            ["future-open-vocabulary-reason"],
        )

    def test_ignored_trace_detector_has_no_fixed_line_window(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "tests" / "ignored.rs"
            source.parent.mkdir(parents=True)
            source.write_text(
                "// Verifies: TC-001\n" + ("\n" * 20) + "#[ignore]\n#[test]\nfn ignored() {}\n",
                encoding="utf-8",
            )
            findings = coverage_checker.ignored_trace_tests(root)
            self.assertEqual(len(findings), 1)
            self.assertIn("TC-001", findings[0])

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

    def test_verifier_accepts_complete_fixture_and_rejects_mutated_outcome(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = self.make_sealed_record(Path(directory))
            verifier.verify_record(record)
            manifest = self.read_json(record / "evidence-manifest.json")
            manifest["outcomes"][0]["status"] = "failed"
            self.write_json(record / "evidence-manifest.json", manifest)
            self.relink_and_seal(record)
            with self.assertRaisesRegex(verifier.EvidenceError, "outcome value mismatch"):
                verifier.verify_record(record)

    def test_verifier_rejects_mutated_result_parameters_limitations_and_artifacts(self) -> None:
        mutations = (
            ("result", "result status mismatch"),
            ("parameters", "parameters digest mismatch"),
            ("limitations", "manifest limitations mismatch"),
            ("artifacts", "manifest artifact census mismatch"),
        )
        for mutation, message in mutations:
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory() as directory:
                record = self.make_sealed_record(Path(directory))
                envelope = self.read_json(record / "evidence-envelope.json")
                manifest = self.read_json(record / "evidence-manifest.json")
                if mutation == "result":
                    envelope["result"]["status"] = "pending"
                elif mutation == "parameters":
                    envelope["parametersDigest"]["value"] = "0" * 64
                elif mutation == "limitations":
                    manifest["limitations"] = manifest["limitations"][:-1]
                else:
                    manifest["artifacts"] = manifest["artifacts"][:-1]
                self.write_json(record / "evidence-manifest.json", manifest)
                self.write_json(record / "evidence-envelope.json", envelope)
                self.relink_and_seal(record)
                with self.assertRaisesRegex(verifier.EvidenceError, message):
                    verifier.verify_record(record)

    def test_word_status_disagreement_and_real_failure_markers_fail(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = Path(directory)
            self.write_fixture_inputs(record)
            (record / "pgm01-schema-status.txt").write_text("failed\n", encoding="utf-8")
            (record / "quire-validate.stdout").write_text(
                "2 document(s) failed structural validation\n", encoding="utf-8"
            )
            (record / "unsafe-audit.stderr").write_text(
                "missing SAFETY comment near line 1\n", encoding="utf-8"
            )
            outcomes = {item["name"]: item["status"] for item in builder.command_outcomes(record)}
            self.assertEqual(outcomes["pgm01-schema"], "failed")
            self.assertEqual(outcomes["quire-validate"], "failed")
            self.assertEqual(outcomes["unsafe-audit"], "failed")

    def test_combined_pending_summary_counts_both_categories(self) -> None:
        status, summary, _ = builder.summarize_outcomes(
            [
                {"name": "coverage", "status": "inconclusive"},
                {"name": "validator", "status": "skipped-unavailable"},
            ]
        )
        self.assertEqual(status, "pending")
        self.assertIn("1 inconclusive", summary)
        self.assertIn("1 skipped-unavailable", summary)

    def test_make_recipes_cannot_ignore_gate_failures(self) -> None:
        makefile = (ROOT / "Makefile").read_text(encoding="utf-8")
        ignored = [line for line in makefile.splitlines() if line.startswith("\t-")]
        self.assertEqual(ignored, [])
        self.assertIn(
            "ci: fmt-check spec lint test msrv deny audit-unsafe rustdoc coverage evidence-tool verify-evidence",
            makefile,
        )

    def test_recursive_runner_discovers_nested_tests(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            nested = root / "tests" / "nested" / "test_hidden.py"
            nested.parent.mkdir(parents=True)
            nested.write_text("raise SystemExit(0)\n", encoding="utf-8")
            self.assertEqual(runner.discover_test_files(root), [nested])

    def test_documented_nonexistent_revision_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_root = Path(directory)
            (evidence_root / "historical").mkdir()
            (evidence_root / "README.md").write_text("deadbeef" * 5, encoding="utf-8")
            with mock.patch.object(verifier, "EVIDENCE_ROOT", evidence_root):
                with self.assertRaisesRegex(verifier.EvidenceError, "does not exist"):
                    verifier.verify_documented_revisions([])

    def test_anchor_verifier_rejects_rename_deletion_addition_and_digest_drift(self) -> None:
        for mutation in ("rename", "delete", "addition", "digest-drift"):
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                evidence_root = root / "evidence"
                evidence_root.mkdir()
                readme = evidence_root / "README.md"
                readme.write_text("anchored\n", encoding="utf-8")
                anchors = evidence_root / "ANCHORS"
                anchors.write_text(
                    f"{verifier.sha256_file(readme)}  evidence/README.md\n",
                    encoding="utf-8",
                )
                with (
                    mock.patch.object(verifier, "ROOT", root),
                    mock.patch.object(verifier, "EVIDENCE_ROOT", evidence_root),
                    mock.patch.object(verifier, "ANCHORS", anchors),
                ):
                    self.assertEqual(verifier.verify_anchors(), [])
                    if mutation == "rename":
                        readme.rename(evidence_root / "RENAMED.md")
                    elif mutation == "delete":
                        readme.unlink()
                    elif mutation == "addition":
                        (evidence_root / "added.txt").write_text("new\n", encoding="utf-8")
                    else:
                        readme.write_text("changed\n", encoding="utf-8")
                    with self.assertRaises(verifier.EvidenceError):
                        verifier.verify_anchors()

    def test_missing_or_empty_authoritative_anchor_set_is_unavailable(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            evidence_root = root / "evidence"
            evidence_root.mkdir()
            anchors = evidence_root / "ANCHORS"
            with (
                mock.patch.object(verifier, "ROOT", root),
                mock.patch.object(verifier, "EVIDENCE_ROOT", evidence_root),
                mock.patch.object(verifier, "ANCHORS", anchors),
            ):
                with self.assertRaises(verifier.VerificationUnavailable):
                    verifier.verify_anchors()
                anchors.write_text("", encoding="utf-8")
                with self.assertRaises(verifier.VerificationUnavailable):
                    verifier.verify_authoritative_records()

    def test_every_historical_envelope_requires_in_band_retraction(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            historical = root / "evidence" / "historical" / "record"
            historical.mkdir(parents=True)
            envelope_path = historical / "evidence-envelope.json"
            disposition = {
                "authoritative": False,
                "status": "retracted",
                "reason": "superseded historical foundation record",
            }
            self.write_json(
                envelope_path,
                {"extensions": {"dev.agent-ix.codegen": {"historicalDisposition": disposition}}},
            )
            with (
                mock.patch.object(verifier, "ROOT", root),
                mock.patch.object(verifier, "EVIDENCE_ROOT", root / "evidence"),
            ):
                verifier.verify_historical_dispositions()
                disposition["status"] = "conclusive"
                self.write_json(
                    envelope_path,
                    {"extensions": {"dev.agent-ix.codegen": {"historicalDisposition": disposition}}},
                )
                with self.assertRaisesRegex(verifier.EvidenceError, "disposition"):
                    verifier.verify_historical_dispositions()

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
    def write_json(path: Path, value: object) -> None:
        path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    @classmethod
    def make_sealed_record(cls, root: Path) -> Path:
        record = root / "foundation-fixture"
        record.mkdir()
        cls.write_fixture_inputs(record)
        builder.build(record)
        cls.relink_and_seal(record)
        return record

    @classmethod
    def relink_and_seal(cls, record: Path) -> None:
        envelope = cls.read_json(record / "evidence-envelope.json")
        envelope["outputs"][0]["contentDigest"]["value"] = builder.sha256_file(
            record / "evidence-manifest.json"
        )
        cls.write_json(record / "evidence-envelope.json", envelope)
        lines = []
        for path in sorted(record.iterdir(), key=lambda item: item.name):
            if path.is_file() and path.name != "sha256sums.txt":
                lines.append(f"{builder.sha256_file(path)}  ./{path.name}\n")
        (record / "sha256sums.txt").write_text("".join(lines), encoding="utf-8")

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
    def test_requirements_file_must_exactly_match_executable_pins(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            requirements = Path(directory) / "requirements.txt"
            requirements.write_text("jsonschema==3.2.0\n", encoding="utf-8")
            spec = importlib.util.spec_from_file_location("schema_validator", VALIDATOR_PATH)
            if spec is None or spec.loader is None:
                self.fail("could not load schema validator")
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            with self.assertRaisesRegex(RuntimeError, "does not match"):
                module.pinned_requirements(requirements)

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
