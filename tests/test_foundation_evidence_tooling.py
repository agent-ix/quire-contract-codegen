"""Tests for foundation evidence assembly and local JSON Schema validation."""

from __future__ import annotations

import importlib.util
import hashlib
import io
import json
import os
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
FAILURE_PROPAGATION_PATH = ROOT / "scripts" / "check_failure_propagation.py"
RUNNER_PATH = ROOT / "scripts" / "run_python_tests.py"
MEASUREMENT_PLAN = ROOT / "spec" / "assurance" / "MP-001-codegen-measurements.md"


def load_builder():
    spec = importlib.util.spec_from_file_location(
        "foundation_evidence_builder", BUILDER_PATH
    )
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load foundation evidence builder")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


builder = load_builder()


def load_verifier():
    spec = importlib.util.spec_from_file_location(
        "foundation_evidence_verifier", VERIFIER_PATH
    )
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load foundation evidence verifier")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


verifier = load_verifier()
LIVE_TOOL_FILES = verifier.live_tool_files()


def load_runner():
    spec = importlib.util.spec_from_file_location(
        "recursive_python_test_runner", RUNNER_PATH
    )
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load recursive Python test runner")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


runner = load_runner()


def load_coverage_checker():
    spec = importlib.util.spec_from_file_location(
        "coverage_status_checker", COVERAGE_PATH
    )
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load coverage status checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


coverage_checker = load_coverage_checker()


def load_failure_propagation_checker():
    spec = importlib.util.spec_from_file_location(
        "failure_propagation_checker", FAILURE_PROPAGATION_PATH
    )
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load failure propagation checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


failure_checker = load_failure_propagation_checker()


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
        pgm_text = (ROOT / "planning/pgm-01-reconciliation.md").read_text(
            encoding="utf-8"
        )
        pins_text = (ROOT / "planning/draft-dependency-pins.md").read_text(
            encoding="utf-8"
        )
        gap_text = (ROOT / "planning/foundation-gap-analysis.md").read_text(
            encoding="utf-8"
        )
        cac_text = (ROOT / "spec/assurance/CAC-001-codegen-contract.md").read_text(
            encoding="utf-8"
        )
        for text, label in (
            (pgm_text, "PGM reconciliation"),
            (pins_text, "dependency pins"),
        ):
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

    def test_parameter_digest_includes_python_tests(self) -> None:
        self.assertIn(
            ROOT / "tests" / "test_foundation_evidence_tooling.py",
            builder.parameter_files(),
        )
        for path in (
            ROOT / "src" / "lib.rs",
            ROOT / "scripts" / "unsafe_comment_baseline.txt",
            ROOT / "schemas" / "foundation-evidence-input-v1.schema.json",
            ROOT / ".gitignore",
            ROOT / "clippy.toml",
            ROOT / "rustfmt.toml",
        ):
            self.assertIn(path, builder.parameter_files())

    def test_shared_evidence_floors_have_reviewed_headroom(self) -> None:
        self.assertEqual(builder.MINIMUM_RUST_TESTS, 2)
        self.assertEqual(builder.MINIMUM_PYTHON_TESTS, 60)
        self.assertEqual(failure_checker.MINIMUM_PYTHON_TESTS, 60)
        self.assertEqual(builder.MINIMUM_TRANSCRIPT_BYTES, 8)
        self.assertFalse(builder.transcript_is_corroborated("fmt", "1234567", ""))
        self.assertTrue(
            builder.transcript_is_corroborated("fmt", "FMT_CHECK_PASSED\n", "")
        )

    def test_pgm01_pin_mismatch_fails_closed(self) -> None:
        with mock.patch.object(builder, "PGM01_ENVELOPE_SCHEMA_DIGEST", "0" * 64):
            with self.assertRaisesRegex(ValueError, "schema digest mismatch"):
                builder.verified_pgm01_schema_digest()

    def test_build_preserves_dependency_identity_roles_digests_and_extensions(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory) / "foundation-fixture"
            evidence_dir.mkdir()
            self.write_fixture_inputs(evidence_dir)

            builder.build(evidence_dir)

            collection_input = self.read_json(evidence_dir / "collection-input.json")
            manifest = self.read_json(evidence_dir / "evidence-manifest.json")
            envelope = self.read_json(evidence_dir / "evidence-envelope.json")

            dependencies = collection_input["dependencies"]
            self.assertEqual(
                dependencies["runtimeCandidateRevision"],
                builder.RUNTIME_CANDIDATE_REVISION,
            )
            self.assertEqual(
                dependencies["irCorpus"],
                f"agent-ix/quire-contract-ir@{builder.IR_CANDIDATE_REVISION}",
            )
            self.assertEqual(
                dependencies["pgm01"]["candidateRevision"],
                builder.PGM01_CANDIDATE_REVISION,
            )
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
            fixed = {
                ROOT / name
                for name in (
                    ".gitignore",
                    "CLAUDE.md",
                    "Cargo.lock",
                    "Cargo.toml",
                    "Makefile",
                    "clippy.toml",
                    "deny.toml",
                    "requirements-evidence.txt",
                    "rust-toolchain.toml",
                    "rustfmt.toml",
                    "schemas/foundation-evidence-input-v1.schema.json",
                    "schemas/foundation-evidence-manifest-v1.schema.json",
                    "schemas/pgm01-derivation-evidence-envelope-v1.schema.json",
                    "spec/assurance/MP-001-codegen-measurements.md",
                    "spec/test-matrix.md",
                )
            }
            recursive = {
                path
                for directory_name in ("src", "scripts", "tools", "schemas", "tests")
                for path in (ROOT / directory_name).rglob("*")
                if path.is_file() and "__pycache__" not in path.parts
            }
            expected_paths = sorted(
                fixed | recursive, key=lambda path: path.relative_to(ROOT).as_posix()
            )
            state = hashlib.sha256()
            for path in expected_paths:
                state.update(path.relative_to(ROOT).as_posix().encode())
                state.update(b"\0")
                state.update(path.read_bytes())
                state.update(b"\0")
            self.assertEqual(envelope["parametersDigest"]["value"], state.hexdigest())
            expected_gate_scripts = {
                path.relative_to(ROOT).as_posix(): hashlib.sha256(
                    path.read_bytes()
                ).hexdigest()
                for path in expected_paths
                if path.is_relative_to(ROOT / "scripts")
                or path.is_relative_to(ROOT / "tools")
            }
            self.assertEqual(collection_input["gateScripts"], expected_gate_scripts)

    def test_verifier_rederives_gate_script_digests(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = self.make_sealed_record(Path(directory))
            collection_input = self.read_json(record / "collection-input.json")
            first = next(iter(collection_input["gateScripts"]))
            collection_input["gateScripts"][first] = "0" * 64
            self.write_json(record / "collection-input.json", collection_input)
            envelope = self.read_json(record / "evidence-envelope.json")
            envelope["inputs"][0]["contentDigest"]["value"] = builder.sha256_file(
                record / "collection-input.json"
            )
            self.write_json(record / "evidence-envelope.json", envelope)
            self.relink_and_seal(record)
            with self.assertRaisesRegex(
                verifier.EvidenceError, "gate script digest mismatch"
            ):
                verifier.verify_record(record)

    def test_verifier_rederives_collector_commands(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = self.make_sealed_record(Path(directory))
            collection_input = self.read_json(record / "collection-input.json")
            collection_input["commands"][0] = "true"
            self.write_json(record / "collection-input.json", collection_input)
            envelope = self.read_json(record / "evidence-envelope.json")
            envelope["inputs"][0]["contentDigest"]["value"] = builder.sha256_file(
                record / "collection-input.json"
            )
            self.write_json(record / "evidence-envelope.json", envelope)
            self.relink_and_seal(record)
            with self.assertRaisesRegex(
                verifier.EvidenceError, "command declaration mismatch"
            ):
                verifier.verify_record(record)

    def test_verifier_rederives_non_python_tool_identity(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = self.make_sealed_record(Path(directory))
            (record / "cargo-version.txt").write_text(
                "cargo 0.0.0\n", encoding="utf-8"
            )
            builder.build(record)
            self.relink_and_seal(record)
            with self.assertRaisesRegex(verifier.EvidenceError, "cargo-version.txt"):
                verifier.verify_record(record)

    def test_build_records_failed_and_missing_commands_without_a_pass_claim(
        self,
    ) -> None:
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

    def test_metadata_json_error_type_is_not_a_false_contradiction(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory) / "foundation-fixture"
            evidence_dir.mkdir()
            self.write_fixture_inputs(evidence_dir)
            (evidence_dir / "metadata.stdout").write_text(
                json.dumps(
                    {
                        "packages": [
                            {
                                "name": "quire-contract-codegen",
                                "version": "0.1.0",
                                "description": "uses std::error::Error",
                            }
                        ]
                    }
                ),
                encoding="utf-8",
            )

            builder.build(evidence_dir)

            manifest = self.read_json(evidence_dir / "evidence-manifest.json")
            outcomes = {item["name"]: item["status"] for item in manifest["outcomes"]}
            self.assertEqual(outcomes["metadata"], "passed")

            (evidence_dir / "metadata.stderr").write_text(
                "error: metadata resolution failed\n", encoding="utf-8"
            )
            builder.build(evidence_dir)
            manifest = self.read_json(evidence_dir / "evidence-manifest.json")
            outcomes = {item["name"]: item["status"] for item in manifest["outcomes"]}
            self.assertEqual(outcomes["metadata"], "failed")

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
            self.assertIn({"name": "audit", "status": "failed"}, manifest["outcomes"])
            self.assertEqual(envelope["result"]["status"], "rejected")

    def test_collector_and_declared_command_sets_agree(self) -> None:
        collector = COLLECTOR_PATH.read_text(encoding="utf-8").split(
            'provenance --pretty >"$evidence_dir/quire-provenance.json"', 1
        )[1]
        collected = set(
            re.findall(r"(?m)^\s*run_and_retain ([a-z0-9-]+)(?: |$)", collector)
        )
        declared = {transcript for _, transcript in builder.COMMAND_TRANSCRIPTS}
        self.assertEqual(collected, declared)
        self.assertEqual(
            builder.collected_commands(),
            [
                builder.COLLECTED_COMMANDS[name]
                for _, name in builder.COMMAND_TRANSCRIPTS
            ],
        )

    def test_collected_command_declaration_rejects_collector_drift(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            collector = Path(directory) / "collector.sh"
            collector.write_text("no commands\n", encoding="utf-8")
            with mock.patch.object(builder, "COLLECTOR", collector):
                with self.assertRaisesRegex(ValueError, "implementation drift"):
                    builder.collected_commands()

    def test_collector_runs_locked_complete_rust_gates(self) -> None:
        collector = COLLECTOR_PATH.read_text(encoding="utf-8")
        for command in (
            'run_and_retain clippy "$trusted_cargo" clippy --locked --all-targets -- -D warnings',
            'run_and_retain test "$trusted_cargo" test --locked',
            'run_and_retain msrv "$trusted_cargo" +1.75.0 test --locked',
            'run_and_retain deny /usr/bin/env CARGO_HOME="$deny_cargo_home" "$trusted_cargo" deny --offline --locked check',
            'run_and_retain metadata "$trusted_cargo" metadata --locked --format-version 1',
            'run_and_retain rustdoc /usr/bin/env RUSTDOCFLAGS=-Dwarnings "$trusted_cargo" doc --locked --no-deps',
        ):
            self.assertIn(command, collector)
        self.assertIn('staging_root="$(mktemp -d)"', collector)
        self.assertIn('cp -a "$default_cargo_home/advisory-dbs"', collector)

    def test_collector_runs_locked_complete_rust_gates(self) -> None:
        collector = COLLECTOR_PATH.read_text(encoding="utf-8")
        for command in (
            'run_and_retain clippy "$trusted_cargo" clippy --locked --all-targets -- -D warnings',
            'run_and_retain test "$trusted_cargo" test --locked',
            'run_and_retain msrv "$trusted_cargo" +1.75.0 test --locked',
            'run_and_retain deny /usr/bin/env CARGO_HOME="$deny_cargo_home" "$trusted_cargo" deny --offline --locked check',
            'run_and_retain metadata "$trusted_cargo" metadata --locked --format-version 1',
            'run_and_retain rustdoc /usr/bin/env RUSTDOCFLAGS=-Dwarnings "$trusted_cargo" doc --locked --no-deps',
        ):
            self.assertIn(command, collector)
        self.assertIn('staging_root="$(mktemp -d)"', collector)
        self.assertIn("retain_collection", collector)
        self.assertIn('cp -a "$default_cargo_home/advisory-dbs"', collector)

    def test_every_declared_command_has_contradiction_markers(self) -> None:
        declared = {name for name, _ in builder.COMMAND_TRANSCRIPTS}
        self.assertEqual(declared, set(builder.PASS_CONTRADICTION_MARKERS))

    def test_every_declared_command_has_positive_corroboration(self) -> None:
        declared = {name for name, _ in builder.COMMAND_TRANSCRIPTS}
        self.assertEqual(declared, set(builder.PASS_CORROBORATION_MARKERS))

    def test_empty_zero_exit_transcripts_cannot_reseal_conclusive(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory)
            self.write_fixture_inputs(evidence_dir)
            for _, transcript in builder.COMMAND_TRANSCRIPTS:
                (evidence_dir / f"{transcript}.stdout").write_text("", encoding="utf-8")
                (evidence_dir / f"{transcript}.stderr").write_text("", encoding="utf-8")
            outcomes = builder.command_outcomes(evidence_dir)
            status, _, _ = builder.summarize_outcomes(outcomes)
            self.assertNotEqual(status, "conclusive")
            self.assertTrue(all(item["status"] != "passed" for item in outcomes))

    def test_zero_or_reduced_rust_test_census_is_inconclusive(self) -> None:
        for passed in (0, builder.MINIMUM_RUST_TESTS - 1):
            with self.subTest(
                passed=passed
            ), tempfile.TemporaryDirectory() as directory:
                evidence_dir = Path(directory)
                self.write_fixture_inputs(evidence_dir)
                (evidence_dir / "test.stdout").write_text(
                    f"test result: ok. {passed} passed; 0 failed; 0 ignored\n",
                    encoding="utf-8",
                )
                outcomes = {
                    item["name"]: item["status"]
                    for item in builder.command_outcomes(evidence_dir)
                }
                self.assertEqual(outcomes["test"], "inconclusive")

    def test_all_matrix_rows_remain_planned_until_upstream_fix(self) -> None:
        matrix = (ROOT / "spec" / "test-matrix.md").read_text(encoding="utf-8")
        rows = coverage_checker.matrix_rows(matrix)
        self.assertGreater(len(rows), 0)
        for row in rows:
            self.assertEqual(row[-1], "🚧 Planned", row)

    def test_coverage_checker_rejects_unminted_ids_and_every_diagnostic_reason(
        self,
    ) -> None:
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
        self.assertEqual(
            coverage_checker.missing_matrix_ids(
                report,
                "| FR-001 | FR-001-AC-1 | TC-001 |",
            ),
            [],
        )
        self.assertEqual(
            coverage_checker.missing_matrix_ids(report, "| FR-001 | FR-001-AC-1 |"),
            ["TC-001"],
        )
        matrix = (ROOT / "spec" / "test-matrix.md").read_text(encoding="utf-8")
        self.assertEqual(coverage_checker.matrix_row_errors(matrix), [])
        substituted = matrix.replace(
            "| FR-001 | FR-001-AC-2 | TC-002 |", "| FR-001 | FR-001-AC-2 | TC-007 |"
        )
        self.assertNotEqual(coverage_checker.matrix_row_errors(substituted), [])
        blank = matrix.replace(
            "| FR-001 | FR-001-AC-2 | TC-002 |", "| FR-001 | FR-001-AC-2 |  |"
        )
        self.assertIn(
            "matrix contains an empty verification cell",
            coverage_checker.matrix_row_errors(blank),
        )

    def test_ignored_trace_detector_has_no_fixed_line_window(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "tests" / "ignored.rs"
            source.parent.mkdir(parents=True)
            source.write_text(
                "// Verifies: TC-001\n"
                + ("\n" * 20)
                + "#[ignore]\n#[test]\nfn ignored() {}\n",
                encoding="utf-8",
            )
            findings = coverage_checker.ignored_trace_tests(root)
            self.assertEqual(len(findings), 1)
            self.assertIn("TC-001", findings[0])
            source.write_text(
                "// Verifies: TC-001\n#[cfg_attr(all(), ignore)]\n#[test]\nfn ignored() {}\n",
                encoding="utf-8",
            )
            findings = coverage_checker.ignored_trace_tests(root)
            self.assertEqual(len(findings), 1)
            self.assertIn("TC-001", findings[0])

    def test_cfg_controlled_and_included_trace_tests_are_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            configured = root / "tests" / "configured.rs"
            configured.parent.mkdir(parents=True)
            configured.write_text(
                "// Verifies: TC-001\n#[cfg(any())]\n#[test]\nfn hidden() {}\n",
                encoding="utf-8",
            )
            self.assertEqual(len(coverage_checker.configured_trace_tests(root)), 1)
            included = root / "notes" / "body.rs"
            included.parent.mkdir()
            included.write_text(
                "// Verifies: TC-002\n#[ignore]\n#[test]\nfn hidden() {}\n",
                encoding="utf-8",
            )
            findings = coverage_checker.ignored_trace_tests(root)
            self.assertTrue(any("notes/body.rs" in finding for finding in findings))

    def test_coverage_scanner_handles_inner_attrs_nested_dirs_and_cfg_test(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            inner = root / "src" / "inner.rs"
            nested = root / "src" / "target" / "evidence" / "nested.rs"
            allowed = root / "tests" / "allowed.rs"
            inner.parent.mkdir(parents=True)
            nested.parent.mkdir(parents=True)
            allowed.parent.mkdir(parents=True)
            inner.write_text("#![cfg(any())]\n// Verifies: TC-001\n", encoding="utf-8")
            nested.write_text(
                "#[cfg(any())]\nfn hidden() { /* Verifies: TC-002 */ }\n",
                encoding="utf-8",
            )
            allowed.write_text(
                "#[cfg(test)]\nmod tests {\n    // Verifies: TC-003\n"
                "    #[test]\n    fn traced() {}\n}\n",
                encoding="utf-8",
            )
            findings = coverage_checker.configured_trace_tests(root)
            self.assertEqual(len(findings), 2, findings)
            self.assertTrue(any("src/inner.rs" in finding for finding in findings))
            self.assertTrue(
                any("src/target/evidence/nested.rs" in finding for finding in findings)
            )

    def test_cfg_controlled_external_module_uses_child_trace_context(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            parent = root / "src" / "lib.rs"
            child = root / "src" / "hidden.rs"
            parent.parent.mkdir(parents=True)
            parent.write_text("#[cfg(any())]\nmod hidden;\n", encoding="utf-8")
            child.write_text("// Verifies: TC-004\n", encoding="utf-8")
            findings = coverage_checker.configured_trace_tests(root)
            self.assertEqual(len(findings), 1, findings)
            self.assertIn("TC-004", findings[0])

    def test_coverage_main_rejects_completed_matrix_status_behaviorally(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "spec").mkdir()
            matrix = "\n".join(
                "| " + " | ".join(row) + " |"
                for row in coverage_checker.EXPECTED_MATRIX_ROWS
            ).replace("🚧 Planned", "✅ Complete", 1)
            (root / "spec/test-matrix.md").write_text(matrix, encoding="utf-8")
            minted = sorted(set(coverage_checker.TRACE_ID.findall(matrix)))
            completed = subprocess.CompletedProcess(
                ["quire"],
                0,
                json.dumps(
                    {
                        "minted_targets": [{"id": value} for value in minted],
                        "diagnostics": [],
                        "status_lies": [],
                        "totals": {"total": len(minted)},
                    }
                ),
                "",
            )
            with (
                mock.patch.object(coverage_checker, "ROOT", root),
                mock.patch.object(
                    coverage_checker.subprocess, "run", return_value=completed
                ) as run,
                mock.patch.object(coverage_checker.sys, "argv", [str(COVERAGE_PATH)]),
                mock.patch.object(coverage_checker.sys, "stdout", io.StringIO()),
                mock.patch.object(coverage_checker.sys, "stderr", io.StringIO()),
            ):
                self.assertEqual(coverage_checker.main(), 1)
            self.assertEqual(
                run.call_args.args[0][0], str(coverage_checker.trusted_quire())
            )

    def test_production_rust_ownership_includes_build_script(self) -> None:
        self.assertEqual(coverage_checker.unowned_production_rust(ROOT), [])
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "src").mkdir()
            (root / "build.rs").write_text("fn main() {}\n", encoding="utf-8")
            (root / "src" / "lib.rs").write_text(
                "// Implements: FR-001\n", encoding="utf-8"
            )
            self.assertEqual(
                coverage_checker.unowned_production_rust(root), ["build.rs"]
            )

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

    def test_verifier_accepts_complete_fixture_and_rejects_mutated_outcome(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = self.make_sealed_record(Path(directory))
            verifier.verify_record(record)
            manifest = self.read_json(record / "evidence-manifest.json")
            manifest["outcomes"][0]["status"] = "failed"
            self.write_json(record / "evidence-manifest.json", manifest)
            self.relink_and_seal(record)
            with self.assertRaisesRegex(
                verifier.EvidenceError, "outcome value mismatch"
            ):
                verifier.verify_record(record)

    def test_verifier_enforces_collection_input_envelope_link(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = self.make_sealed_record(Path(directory))
            with (record / "collection-input.json").open(
                "a", encoding="utf-8"
            ) as stream:
                stream.write("\n")
            self.relink_and_seal(record)
            with self.assertRaisesRegex(
                verifier.EvidenceError, "envelope digest mismatch"
            ):
                verifier.verify_record(record)

    def test_record_directory_prefix_and_positive_censuses_are_enforced(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = self.make_sealed_record(Path(directory))
            renamed = record.with_name("foundation-deadbeefcafe-20260831T999999Z")
            record.rename(renamed)
            with self.assertRaisesRegex(
                verifier.EvidenceError, "directory revision prefix"
            ):
                verifier.verify_record(renamed)
        with tempfile.TemporaryDirectory() as directory:
            record = self.make_sealed_record(Path(directory))
            with mock.patch.object(verifier, "verify_checksums", return_value=0):
                with self.assertRaisesRegex(
                    verifier.EvidenceError, "empty checksum census"
                ):
                    verifier.verify_record(record)
            with mock.patch.object(verifier, "verify_artifacts", return_value=0):
                with self.assertRaisesRegex(
                    verifier.EvidenceError, "empty manifest artifact"
                ):
                    verifier.verify_record(record)

    def test_python_runtime_identity_is_rederived(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = self.make_sealed_record(Path(directory))
            (record / "python-version.txt").write_text(
                "Python 0.0.0\n", encoding="utf-8"
            )
            builder.build(record)
            self.relink_and_seal(record)
            with self.assertRaisesRegex(verifier.EvidenceError, "python-version.txt"):
                verifier.verify_record(record)

    def test_verifier_rejects_mutated_result_parameters_limitations_and_artifacts(
        self,
    ) -> None:
        mutations = (
            ("result", "result status mismatch"),
            ("parameters", "parameters digest mismatch"),
            ("limitations", "manifest limitations mismatch"),
            ("artifacts", "manifest artifact census mismatch"),
        )
        for mutation, message in mutations:
            with self.subTest(
                mutation=mutation
            ), tempfile.TemporaryDirectory() as directory:
                record = self.make_sealed_record(Path(directory))
                envelope = self.read_json(record / "evidence-envelope.json")
                manifest = self.read_json(record / "evidence-manifest.json")
                if mutation == "result":
                    envelope["result"]["status"] = "conclusive"
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

    def test_verifier_rejects_mutated_outcome_census_and_result_summary(self) -> None:
        for mutation, message in (
            ("outcome-census", "outcome census mismatch"),
            ("result-summary", "result summary mismatch"),
        ):
            with self.subTest(
                mutation=mutation
            ), tempfile.TemporaryDirectory() as directory:
                record = self.make_sealed_record(Path(directory))
                manifest = self.read_json(record / "evidence-manifest.json")
                envelope = self.read_json(record / "evidence-envelope.json")
                if mutation == "outcome-census":
                    manifest["outcomes"] = manifest["outcomes"][:-1]
                else:
                    envelope["result"]["summary"] = "fabricated summary"
                self.write_json(record / "evidence-manifest.json", manifest)
                self.write_json(record / "evidence-envelope.json", envelope)
                self.relink_and_seal(record)
                with self.assertRaisesRegex(verifier.EvidenceError, message):
                    verifier.verify_record(record)

    def test_verifier_rejects_symlinked_record_artifact(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = self.make_sealed_record(Path(directory))
            transcript = record / "test.stdout"
            target = record.parent / "outside.txt"
            target.write_text("substituted\n", encoding="utf-8")
            transcript.unlink()
            transcript.symlink_to(target)
            with self.assertRaisesRegex(verifier.EvidenceError, "symlink"):
                verifier.verify_checksums(record)

    def test_availability_status_enum_rejects_unknown_values(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = Path(directory)
            self.write_fixture_inputs(record)
            (record / "pgm01-envelope-status.txt").write_text(
                "fabricated\n", encoding="utf-8"
            )
            with self.assertRaisesRegex(ValueError, "invalid availability status"):
                builder.command_outcomes(record)

    def test_word_status_disagreement_and_real_failure_markers_fail(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = Path(directory)
            self.write_fixture_inputs(record)
            (record / "pgm01-schema-status.txt").write_text(
                "failed\n", encoding="utf-8"
            )
            (record / "quire-validate.stdout").write_text(
                "2 document(s) failed structural validation\n", encoding="utf-8"
            )
            (record / "unsafe-audit.stderr").write_text(
                "missing SAFETY comment near line 1\n", encoding="utf-8"
            )
            outcomes = {
                item["name"]: item["status"]
                for item in builder.command_outcomes(record)
            }
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
        recipes = [
            line[1:].lstrip() for line in makefile.splitlines() if line.startswith("\t")
        ]
        ignored = [
            recipe
            for recipe in recipes
            if recipe.startswith("-") or re.search(r"\|\|\s*(?::|true)(?:\s|$)", recipe)
        ]
        self.assertEqual(ignored, [])
        self.assertIn(
            "ci: ci-guard fmt-check spec lint test msrv deny audit-unsafe rustdoc coverage evidence-tool verify-evidence",
            makefile,
        )
        for arguments in (("-i", "-n", "ci"), ("--eval=.IGNORE:", "-n", "ci")):
            completed = subprocess.run(
                ["make", *arguments],
                cwd=ROOT,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(completed.returncode, 0, arguments)
            self.assertIn(
                "local CI parse-time integrity guard rejected", completed.stderr
            )
        with tempfile.TemporaryDirectory() as directory:
            cargo = Path(directory) / "cargo"
            cargo.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
            cargo.chmod(0o755)
            environment = dict(os.environ)
            for variable in ("MAKEFLAGS", "MFLAGS", "MAKELEVEL"):
                environment.pop(variable, None)
            environment["PATH"] = f"{directory}:{environment['PATH']}"
            completed = subprocess.run(
                ["make", "ci"],
                cwd=ROOT,
                check=False,
                capture_output=True,
                text=True,
                env=environment,
            )
            self.assertNotEqual(completed.returncode, 0)
            self.assertIn(
                "local CI parse-time integrity guard rejected", completed.stderr
            )

    def test_parallel_makeflags_are_allowed_but_execution_suppression_is_not(
        self,
    ) -> None:
        for value in (
            "-j4",
            "--jobs=4 --load-average=2.5",
            "-j --jobserver-auth=3,4",
        ):
            self.assertEqual(failure_checker.makeflags_errors(value), [], value)
        for value in ("-t", "--touch", "-n", "--just-print", "-i", "--ignore-errors"):
            self.assertNotEqual(failure_checker.makeflags_errors(value), [], value)

    def test_prefixed_and_late_makeflags_assignments_are_rejected(self) -> None:
        original = (ROOT / "Makefile").read_text(encoding="utf-8")
        assignments = (
            "export MAKEFLAGS = -i",
            "override MAKEFLAGS := -i",
            "export override MAKEFLAGS = -i",
        )
        for assignment in assignments:
            with self.subTest(
                assignment=assignment
            ), tempfile.TemporaryDirectory() as directory:
                makefile = Path(directory) / "Makefile"
                makefile.write_text(original + "\n" + assignment + "\n", encoding="utf-8")
                errors = failure_checker.inspect_makefile(makefile)
                self.assertTrue(any("unsafe MAKEFLAGS token" in error for error in errors))
                completed = subprocess.run(
                    ["/usr/bin/make", "-f", str(makefile), "-n", "ci"],
                    cwd=ROOT,
                    check=False,
                    capture_output=True,
                    text=True,
                )
                self.assertNotEqual(completed.returncode, 0)

    def test_command_line_makeflags_override_is_rejected(self) -> None:
        completed = subprocess.run(
            ["/usr/bin/make", "-n", "ci", "MAKEFLAGS=-i"],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertNotEqual(completed.returncode, 0)
        self.assertIn("parse-time integrity guard rejected", completed.stderr)

    def test_makefile_includes_are_rejected(self) -> None:
        original = (ROOT / "Makefile").read_text(encoding="utf-8")
        for directive in ("include local.mk", "-include local.mk", "sinclude local.mk"):
            with self.subTest(
                directive=directive
            ), tempfile.TemporaryDirectory() as directory:
                makefile = Path(directory) / "Makefile"
                makefile.write_text(original + "\n" + directive + "\n", encoding="utf-8")
                errors = failure_checker.inspect_makefile(makefile)
                self.assertTrue(any("includes unreviewed" in error for error in errors))

    def test_static_recipe_guard_rejects_suppression_and_early_exit(self) -> None:
        original = (ROOT / "Makefile").read_text(encoding="utf-8")
        mutations = (
            original.replace(
                "\t$(CARGO) fmt --all -- --check", "\t-$(CARGO) fmt --all -- --check"
            ),
            original.replace(
                "\t$(BASH) scripts/check_unsafe_comments.sh",
                "\texit 0; $(BASH) scripts/check_unsafe_comments.sh",
            ),
        )
        for mutation in mutations:
            with self.subTest(), tempfile.TemporaryDirectory() as directory:
                makefile = Path(directory) / "Makefile"
                makefile.write_text(mutation, encoding="utf-8")
                self.assertNotEqual(failure_checker.inspect_makefile(makefile), [])

    def test_unsafe_audit_rejects_a_known_bad_tree(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "src").mkdir()
            (root / "src" / "lib.rs").write_text(
                "fn bad() { unsafe { core::hint::unreachable_unchecked() } }\n",
                encoding="utf-8",
            )
            completed = subprocess.run(
                ["/usr/bin/bash", str(ROOT / "scripts" / "check_unsafe_comments.sh")],
                cwd=root,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(completed.returncode, 0)
            self.assertIn("missing unsafe comment baseline", completed.stderr)

    def test_empty_neutered_python_gate_outputs_are_rejected(self) -> None:
        completed = subprocess.CompletedProcess(["gate"], 0, "", "")
        with mock.patch.object(
            failure_checker.subprocess, "run", return_value=completed
        ):
            errors = failure_checker.inspect_gate_outputs()
        self.assertEqual(len(errors), 3)

    def test_tool_shadow_pairs_are_rejected_before_local_ci_runs(self) -> None:
        for names in (("cargo", "rustup"), ("python3", "quire")):
            with self.subTest(names=names), tempfile.TemporaryDirectory() as directory:
                for name in names:
                    executable = Path(directory) / name
                    executable.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
                    executable.chmod(0o755)
                environment = dict(os.environ)
                for variable in ("MAKEFLAGS", "MFLAGS", "MAKELEVEL"):
                    environment.pop(variable, None)
                environment["PATH"] = f"{directory}:{environment['PATH']}"
                completed = subprocess.run(
                    ["make", "ci"],
                    cwd=ROOT,
                    check=False,
                    capture_output=True,
                    text=True,
                    env=environment,
                )
                self.assertNotEqual(completed.returncode, 0)
                self.assertIn(
                    "local CI parse-time integrity guard rejected", completed.stderr
                )

    def test_collector_leaves_anchor_update_as_separate_boundary(self) -> None:
        collector = COLLECTOR_PATH.read_text(encoding="utf-8")
        self.assertNotIn("scripts/update_evidence_anchors.py", collector)
        self.assertIn(
            "update evidence/ANCHORS as a separate review-boundary step", collector
        )

    def test_recursive_runner_executes_nested_testcases_without_main(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            nested = root / "tests" / "nested" / "test_hidden.py"
            nested.parent.mkdir(parents=True)
            nested.write_text(
                "import unittest\n"
                "class Hidden(unittest.TestCase):\n"
                "    def test_failure(self):\n"
                "        self.assertEqual(1, 2)\n",
                encoding="utf-8",
            )
            self.assertEqual(runner.discover_test_files(root), [nested])
            output = io.StringIO()
            self.assertEqual(runner.run_tests(root, output), 1)
            self.assertIn("FAILED", output.getvalue())
            self.assertNotIn("executed 1 Python tests", output.getvalue())
            nested.write_text(
                "import unittest\n"
                "class Hidden(unittest.TestCase):\n"
                "    def test_success(self):\n"
                "        self.assertEqual(1, 1)\n",
                encoding="utf-8",
            )
            self.assertEqual(runner.run_tests(root, io.StringIO()), 0)

    def test_recursive_runner_rejects_zero_executed_tests(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            empty = root / "tests" / "test_empty.py"
            empty.parent.mkdir(parents=True)
            empty.write_text("# no tests\n", encoding="utf-8")
            output = io.StringIO()
            self.assertEqual(runner.run_tests(root, output), 1)
            self.assertIn("no Python tests executed", output.getvalue())

    def test_runner_main_propagates_the_suite_status(self) -> None:
        with mock.patch.object(runner, "run_tests", return_value=1):
            self.assertEqual(runner.main(), 1)

    def test_builder_and_coverage_entry_points_do_substantive_work(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory)
            self.write_fixture_inputs(evidence_dir)
            with mock.patch.object(
                builder.sys, "argv", [str(BUILDER_PATH), str(evidence_dir)]
            ):
                self.assertEqual(builder.main(), 0)
            self.assertTrue((evidence_dir / "evidence-envelope.json").is_file())
        completed = subprocess.run(
            ["/usr/bin/python3", str(COVERAGE_PATH)],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(json.loads(completed.stdout.splitlines()[0])["statusLies"], 0)

    def test_documented_nonexistent_revision_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_root = Path(directory)
            (evidence_root / "historical").mkdir()
            (evidence_root / "README.md").write_text("deadbeef" * 5, encoding="utf-8")
            with mock.patch.object(verifier, "EVIDENCE_ROOT", evidence_root):
                with self.assertRaisesRegex(verifier.EvidenceError, "does not exist"):
                    verifier.verify_documented_revisions([])

    def test_documented_revision_check_is_unavailable_without_git(self) -> None:
        failed = subprocess.CompletedProcess(["git"], 128, "", "not a repository")
        with mock.patch.object(verifier.subprocess, "run", return_value=failed):
            with self.assertRaisesRegex(verifier.VerificationUnavailable, "Git object"):
                verifier.verify_documented_revisions([])

    def test_nonlocal_documented_revisions_are_explicitly_allowlisted(self) -> None:
        documents = [
            ROOT / "README.md",
            ROOT / "CLAUDE.md",
            *sorted((ROOT / "planning").rglob("*.md")),
            *sorted((ROOT / "plan").rglob("*.md")),
            *sorted((ROOT / "spec").rglob("*.md")),
        ]
        for document in (path for path in documents if path.is_file()):
            for revision in verifier.REVISION.findall(
                document.read_text(encoding="utf-8")
            ):
                resolved = subprocess.run(
                    ["git", "cat-file", "-e", f"{revision}^{{commit}}"],
                    cwd=ROOT,
                    check=False,
                    capture_output=True,
                )
                if resolved.returncode != 0:
                    self.assertIn(revision, verifier.EXTERNAL_REVISIONS, document)

    def test_verifier_main_formats_plain_builder_value_errors(self) -> None:
        output = io.StringIO()
        with (
            mock.patch.object(
                verifier,
                "verify_authoritative_records",
                side_effect=ValueError("invalid availability status"),
            ),
            mock.patch.object(verifier.sys, "stderr", output),
        ):
            self.assertEqual(verifier.main(), 1)
        self.assertIn(
            "verification failed: invalid availability status", output.getvalue()
        )
        self.assertNotIn("Traceback", output.getvalue())

    def test_anchor_verifier_rejects_rename_deletion_addition_and_digest_drift(
        self,
    ) -> None:
        for mutation in ("rename", "delete", "addition", "digest-drift"):
            with self.subTest(
                mutation=mutation
            ), tempfile.TemporaryDirectory() as directory:
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
                        (evidence_root / "added.txt").write_text(
                            "new\n", encoding="utf-8"
                        )
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

    def test_duplicate_authoritative_record_is_rejected(self) -> None:
        with mock.patch.object(
            verifier,
            "verify_anchors",
            return_value=[Path("foundation-one"), Path("foundation-two")],
        ):
            with self.assertRaisesRegex(verifier.EvidenceError, "exactly one"):
                verifier.verify_authoritative_records()

    def test_anchor_file_symlink_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            evidence_root = root / "evidence"
            evidence_root.mkdir()
            target = root / "outside-anchors"
            target.write_text("", encoding="utf-8")
            anchors = evidence_root / "ANCHORS"
            anchors.symlink_to(target)
            with (
                mock.patch.object(verifier, "ROOT", root),
                mock.patch.object(verifier, "EVIDENCE_ROOT", evidence_root),
                mock.patch.object(verifier, "ANCHORS", anchors),
            ):
                with self.assertRaisesRegex(verifier.EvidenceError, "symlink"):
                    verifier.verify_anchors()

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
                {
                    "extensions": {
                        "dev.agent-ix.codegen": {"historicalDisposition": disposition}
                    }
                },
            )
            with (
                mock.patch.object(verifier, "ROOT", root),
                mock.patch.object(verifier, "EVIDENCE_ROOT", root / "evidence"),
            ):
                verifier.verify_historical_dispositions()
                disposition["status"] = "conclusive"
                self.write_json(
                    envelope_path,
                    {
                        "extensions": {
                            "dev.agent-ix.codegen": {
                                "historicalDisposition": disposition
                            }
                        }
                    },
                )
                with self.assertRaisesRegex(verifier.EvidenceError, "disposition"):
                    verifier.verify_historical_dispositions()

    def test_historical_readme_census_is_bidirectional(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            historical = root / "historical"
            record = historical / "foundation-abcdef123456-20260831T010203Z"
            record.mkdir(parents=True)
            (record / "evidence-envelope.json").write_text("{}\n", encoding="utf-8")
            readme = historical / "README.md"
            readme.write_text(f"- `{record.name}`\n", encoding="utf-8")
            with mock.patch.object(verifier, "EVIDENCE_ROOT", root):
                verifier.verify_historical_index()
                readme.write_text("no records\n", encoding="utf-8")
                with self.assertRaisesRegex(verifier.EvidenceError, "census mismatch"):
                    verifier.verify_historical_index()

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
        self.assertIn(
            "foundation collector fail-closed self-test passed", completed.stdout
        )

    @staticmethod
    def read_json(path: Path):
        return json.loads(path.read_text(encoding="utf-8"))

    @staticmethod
    def write_json(path: Path, value: object) -> None:
        path.write_text(
            json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )

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
            "cargo-version.txt": LIVE_TOOL_FILES["cargo-version.txt"] + "\n",
            "jsonschema-version.txt": LIVE_TOOL_FILES["jsonschema-version.txt"] + "\n",
            "python-version.txt": LIVE_TOOL_FILES["python-version.txt"] + "\n",
            "python-packages.txt": LIVE_TOOL_FILES["python-packages.txt"] + "\n",
            "pgm01-schema-path.txt": "/tmp/quire-contract-ir/schemas/derivation-evidence-envelope-v1.schema.json\n",
            "pgm01-schema-sha256.txt": builder.PGM01_ENVELOPE_SCHEMA_DIGEST + "\n",
            "pgm01-validator-path.txt": "/tmp/quire-contract-ir/scripts/validate_governance.py\n",
            "pgm01-validator-sha256.txt": "b" * 64 + "\n",
            "ir-validator-revision.txt": builder.IR_CANDIDATE_REVISION + "\n",
            "rustc-version.txt": LIVE_TOOL_FILES["rustc-version.txt"] + "\n",
            "msrv-rustc-version.txt": LIVE_TOOL_FILES["msrv-rustc-version.txt"] + "\n",
        }
        for name, value in values.items():
            (evidence_dir / name).write_text(value, encoding="utf-8")
        for _, transcript in builder.COMMAND_TRANSCRIPTS:
            (evidence_dir / f"{transcript}.status.txt").write_text(
                "0\n", encoding="utf-8"
            )
            (evidence_dir / f"{transcript}.stdout").write_text("", encoding="utf-8")
            (evidence_dir / f"{transcript}.stderr").write_text("", encoding="utf-8")
        positive = {
            "quire-validate.stdout": "QUIRE_VALIDATION_PASSED\n",
            "fmt.stdout": "FMT_CHECK_PASSED\n",
            "clippy.stderr": "Finished `dev` profile [unoptimized + debuginfo]\n",
            "test.stdout": "test result: ok. 10 passed; 0 failed; 0 ignored\n",
            "msrv.stdout": "test result: ok. 10 passed; 0 failed; 0 ignored\n",
            "deny.stdout": "advisories ok, bans ok, licenses ok, sources ok\n",
            "unsafe-audit.stdout": "unsafe audit passed\n",
            "rustdoc.stderr": "Generated /tmp/doc/quire_contract_codegen/index.html\n",
            "coverage.stdout": json.dumps({"statusLies": 0, "totals": {"total": 28}})
            + "\n",
            "coverage.stderr": "COVERAGE_STATUS_INCONCLUSIVE: unavailable\n",
            "evidence-tool.stdout": "executed 60 Python tests from 1 files\n",
            "evidence-tool.stderr": "Ran 60 tests\n\nOK\n",
        }
        valid = json.dumps({"errors": [], "valid": True}) + "\n"
        for transcript in builder.VALIDATOR_TRANSCRIPTS:
            positive[f"{transcript}.stdout"] = valid
        for name, value in positive.items():
            (evidence_dir / name).write_text(value, encoding="utf-8")
        (evidence_dir / "metadata.stdout").write_text(
            json.dumps(
                {"packages": [{"name": "quire-contract-codegen", "version": "0.1.0"}]}
            ),
            encoding="utf-8",
        )
        (evidence_dir / "quire-provenance.json").write_text(
            LIVE_TOOL_FILES["quire-provenance.json"] + "\n", encoding="utf-8"
        )


class SchemaValidatorTests(unittest.TestCase):
    def test_foundation_schemas_reject_empty_objects(self) -> None:
        for schema in (builder.INPUT_SCHEMA, builder.MANIFEST_SCHEMA):
            with self.subTest(schema=schema.name):
                with self.assertRaisesRegex(verifier.EvidenceError, "schema violation"):
                    verifier.validate_json({}, schema, "negative fixture")

    def test_requirements_file_must_exactly_match_executable_pins(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            requirements = Path(directory) / "requirements.txt"
            requirements.write_text("jsonschema==3.2.0\n", encoding="utf-8")
            spec = importlib.util.spec_from_file_location(
                "schema_validator", VALIDATOR_PATH
            )
            if spec is None or spec.loader is None:
                self.fail("could not load schema validator")
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            with self.assertRaisesRegex(
                module.EvidenceRequirementsError, "does not match"
            ):
                module.pinned_requirements(requirements)

    def test_required_formats_and_checksum_syntax_are_not_neuterable(self) -> None:
        spec = importlib.util.spec_from_file_location(
            "schema_validator_formats", VALIDATOR_PATH
        )
        if spec is None or spec.loader is None:
            self.fail("could not load schema validator")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        self.assertEqual(module.REQUIRED_FORMATS, {"date-time", "uri", "uri-reference"})
        checker = module.checked_format_checker()
        self.assertTrue(module.REQUIRED_FORMATS.issubset(checker.checkers))
        self.assertIsNone(verifier.CHECKSUM_LINE.fullmatch("0" * 63 + "  ./file"))
        self.assertIsNotNone(verifier.CHECKSUM_LINE.fullmatch("0" * 64 + "  ./file"))

    def test_malformed_requirements_are_evidence_failure_not_unavailability(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            requirements = root / "requirements.txt"
            requirements.write_text("# malformed repository input\n", encoding="utf-8")
            schema = root / "schema.json"
            schema.write_text('{"type":"object"}\n', encoding="utf-8")
            requirements_module = sys.modules[
                verifier.checked_format_checker.__module__
            ]
            with mock.patch.object(requirements_module, "REQUIREMENTS", requirements):
                with self.assertRaises(verifier.EvidenceError) as caught:
                    verifier.validate_json({}, schema, "fixture")
            self.assertNotIsInstance(caught.exception, verifier.VerificationUnavailable)

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
            instance_path.write_text(
                '{"recordedAt":"NOT-A-TIMESTAMP"}', encoding="utf-8"
            )
            rejected = self.run_validator(schema_path, instance_path)
            self.assertEqual(rejected.returncode, 1, rejected.stderr)

    @staticmethod
    def run_validator(
        schema_path: Path, instance_path: Path
    ) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(VALIDATOR_PATH), str(schema_path), str(instance_path)],
            check=False,
            capture_output=True,
            text=True,
        )


if __name__ == "__main__":
    unittest.main()
