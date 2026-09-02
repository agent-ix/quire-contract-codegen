#!/usr/bin/env python3
"""Drive the official change-assurance chain over already-produced results (FR-006).

Four things this file deliberately is not.

It is not a producer. It never runs a generator, a test, a compiler, or a
solver. Every input it reads was written by `make assurance-inputs`, and if one
is absent it says so and names that target. A driver that can produce its own
inputs is a driver that can produce a green run out of nothing.

It is not an envelope. Quoin's packaged FR-063 record, FR-064 attestation and
FR-065 receipt schemas are the shapes. This file projects
`assurance/change-assurance.json` into the record body Quoin requires and derives
nothing beyond the digests that file's own `derived_fields` names.

It is not a verdict. It runs `quoin` and reports what `quoin` said. Where a
scenario expects a refusal, the refusal is the expected result and the run is
green because the tool refused, not because the tool agreed.

It is not a retention store. Nothing is written under `evidence/`, nothing is
committed, and the Quoin store it uses lives under `target/`, which is ignored.

Exit status: 0 when every scenario, control and probe matched, 1 when one did
not, 2 on a usage or environment error — which is a different fact from a
mismatch and gets its own code.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
DECLARATION = ROOT / "assurance" / "change-assurance.json"
ASSURANCE_DIR = ROOT / "target" / "assurance"
STORE = ROOT / "target" / "assurance-store"

# The domain stream this repository transcribes into Quoin's evidence store. It
# is the crate's headline verification: the bounded generation corpus over the
# oracle, harness and strategy slices, with its rejection cases.
CONFORMANCE_PROTOCOL = "codegen.generation-conformance/v1"
UPSTREAM_PROTOCOL = "codegen.upstream-identity/v1"
PRIMARY_PROOF = "PROOF-generation-conformance"

# Every proof obligation's retained result, and the media type its producer
# declares. Stated rather than sniffed, because a producer's content type is
# part of what it produced.
INPUTS = {
    "PROOF-generation-conformance": ("generation-conformance.jsonl", "application/x-ndjson"),
    "PROOF-upstream-identity": ("upstream-identity.json", "application/json"),
    "PROOF-quire-static-export": ("quire-static-export.json", "application/json"),
    "PROOF-msrv": ("msrv.jsonl", "application/x-ndjson"),
}

# The producers whose documents are a newline-delimited row stream rather than a
# single object with an `entries` array. Named rather than detected, because
# sniffing the shape is how a truncated document gets read as the other kind.
STREAM_DOCUMENTS = ("PROOF-generation-conformance",)
ENTRY_DOCUMENTS = ("PROOF-upstream-identity",)

# The outcome vocabulary a producer's rows may use, and the attestation result
# each maps to. Every value is listed; an unlisted one is refused.
#
# The attestation schema's `result` field has exactly four values, so the
# domain's twelve states cannot all survive here — and they are not meant to.
# What survives here is the coarse result Quoin retains. The domain's own state
# stays in the retained bytes, which Quoin binds by digest and does not rewrite,
# and in the receipt reasons that tell the four apart.
ROW_RESULTS = {
    "pass": "passed",
    "fail": "failed",
    "malformed": "failed",
    "unavailable": "unavailable",
    "not-computed": "not_computed",
    "vacuous": "not_computed",
}

# Precedence when a stream carries more than one outcome. A single failure
# outranks any number of passes, and an unavailable outranks a not-computed,
# because the strongest thing observed is what the run has to be reported as.
RESULT_PRECEDENCE = ("failed", "unavailable", "not_computed", "passed")

# The fields `quire coverage --json` declares. A document missing any of them is
# not a coverage export, whatever else it is. Named rather than counted, because
# "a populated object" was the earlier rule and `{"junk":[1]}` satisfied it.
QUIRE_EXPORT_FIELDS = ("totals", "metrics", "engine", "status_lies", "unbacked_rows")


class ChainError(RuntimeError):
    """The chain could not be driven. Distinct from a scenario that did not match."""


def digest_of(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def quoin(*arguments: str, stdin: str | None = None) -> subprocess.CompletedProcess[str]:
    """Invoke the pinned Quoin CLI. It is the only command this file runs."""
    if shutil.which("quoin") is None:
        raise ChainError("quoin is not on PATH; the pinned CLI is required")
    return subprocess.run(
        ["quoin", *arguments], input=stdin, capture_output=True, text=True, check=False
    )


def tool_version(argv: list[str]) -> str | None:
    """Observe a tool's version, or report that it could not be observed.

    `None` is the answer when the probe failed, and it is recorded as `null`
    rather than replaced with a plausible-looking default. A fabricated version
    in a sealed attestation's environment is worse than an absent one, because a
    reader cannot tell it apart from a real observation.
    """
    try:
        result = subprocess.run(argv, capture_output=True, text=True, check=False)
    except OSError:
        return None
    if result.returncode != 0:
        return None
    return result.stdout.strip() or None


SEMVER = re.compile(r"\b(\d+\.\d+\.\d+)\b")


def semantic_version(text: str | None) -> str | None:
    """Extract the version the attestation schema's `immutable_version` accepts."""
    if text is None:
        return None
    found = SEMVER.search(text)
    return found.group(1) if found else None


def observe_environment() -> dict[str, Any]:
    quire_version: str | None = None
    try:
        raw = subprocess.run(["quire", "provenance"], capture_output=True, text=True, check=False)
    except OSError:
        # An absent tool is an unobserved tool, recorded as null. It is not a
        # crash, and it is certainly not a version.
        raw = None
    if raw is not None and raw.returncode == 0:
        try:
            provenance = json.loads(raw.stdout)
            quire_version = (
                f"{provenance['cli']['version']} engine {provenance['engine']['version']}"
            )
        except (json.JSONDecodeError, KeyError, TypeError):
            quire_version = None
    return {
        "quoin": tool_version(["quoin", "--version"]),
        "quire": quire_version,
        "ix-flow": tool_version(["ix-flow", "--version"]),
        "rustc": tool_version(["rustc", "--version"]),
        "platform": sys.platform,
    }


# ---------------------------------------------------------------------------
# The native adapter
# ---------------------------------------------------------------------------

# The domain stream's outcome vocabulary, and the Quoin entry outcome each one
# transcribes to. Every value is listed. An outcome this table does not name is
# refused rather than defaulted, because a silently defaulted unknown state is
# how twelve states become two.
CONFORMANCE_OUTCOMES = {
    "pass": "pass",
    "fail": "fail",
    "unavailable": "skip",
    "not-computed": "skip",
    "malformed": "fail",
    "vacuous": "skip",
}


def adapt_conformance(raw: str) -> dict[str, Any]:
    """Transcribe the declared domain protocol into Quoin's normalized entries.

    This is the whole of the adapter. It reads a protocol it names, maps a state
    vocabulary it enumerates, and refuses anything else. It runs nothing, judges
    nothing, and never looks at a process's output stream to decide an outcome —
    the generator's own result was decided by the generator, and what arrives
    here is already a field.
    """
    entries = []
    lines = [line for line in raw.splitlines() if line.strip()]
    if not lines:
        raise ChainError("the conformance stream is empty; there is nothing to transcribe")
    for number, line in enumerate(lines, start=1):
        try:
            row = json.loads(line)
        except json.JSONDecodeError as error:
            raise ChainError(f"conformance stream line {number} is malformed: {error}") from error
        protocol = row.get("protocol")
        if protocol != CONFORMANCE_PROTOCOL:
            raise ChainError(
                f"conformance stream line {number} declares protocol {protocol!r}; "
                f"this adapter transcribes {CONFORMANCE_PROTOCOL} and refuses to guess"
            )
        outcome = row.get("outcome")
        if outcome not in CONFORMANCE_OUTCOMES:
            raise ChainError(
                f"conformance stream line {number} declares outcome {outcome!r}, "
                "which this adapter does not name"
            )
        entries.append(
            {
                "symbol": row["symbol"],
                "outcome": CONFORMANCE_OUTCOMES[outcome],
                "traceIds": list(row.get("traceIds", [])),
            }
        )
    return {"entries": entries}


# ---------------------------------------------------------------------------
# The chain
# ---------------------------------------------------------------------------


class Chain:
    """Seal, retain and verify, entirely through the pinned Quoin CLI."""

    @staticmethod
    def crate_version() -> str:
        """The crate's own version, for the tools this repository ships.

        Read from Cargo.toml rather than written twice.
        """
        for line in (ROOT / "Cargo.toml").read_text(encoding="utf-8").splitlines():
            if line.startswith("version = "):
                return line.split('"')[1]
        raise ChainError("Cargo.toml declares no package version")

    def observe_tool_versions(self) -> dict[str, str]:
        """One observed version per declared tool identity.

        A tool whose version cannot be observed raises. The alternative is a
        sealed attestation naming a version nobody measured, and an attestation
        is only worth its weakest field.
        """
        crate = self.crate_version()
        probes = {
            "cargo": lambda: semantic_version(tool_version(["cargo", "--version"])),
            "quire": lambda: semantic_version(
                (self.environment.get("quire") or "").split(" ")[0] or None
            ),
        }
        versions: dict[str, str] = {}
        for proof in self.declaration["record"]["definition"]["proof_obligations"]:
            identity = proof["tool_identity"]
            if identity in versions:
                continue
            observed = probes[identity]() if identity in probes else crate
            if observed is None:
                raise ChainError(
                    f"the version of {identity} could not be observed; an attestation "
                    "will not be sealed naming a version nobody measured"
                )
            versions[identity] = observed
        return versions

    def __init__(self, candidate_revision: str, store: Path) -> None:
        self.revision = candidate_revision
        self.store = store
        self.environment = observe_environment()
        self.declaration = json.loads(DECLARATION.read_text(encoding="utf-8"))
        self.observed_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        self.tool_versions = self.observe_tool_versions()

    # -- record ------------------------------------------------------------

    def record_body(self) -> dict[str, Any]:
        """Project the declaration into Quoin's record body, deriving only digests."""
        declared = json.loads(json.dumps(self.declaration["record"]))
        sources = self.declaration["sources"]
        declared["subject"]["base_revision"] = self.revision
        for connection in declared["source_connections"]:
            path = ROOT / sources[connection["source_id"]]
            if not path.is_file():
                raise ChainError(f"declared source {connection['source_id']} is missing at {path}")
            connection["revision"] = self.revision
            connection["digest"] = digest_of(path.read_bytes())
        for proof in declared["definition"]["proof_obligations"]:
            configuration = proof.pop("configuration")
            path = ROOT / configuration
            if not path.is_file():
                raise ChainError(
                    f"{proof['proof_id']} names configuration {configuration}, which is missing"
                )
            proof["configuration_digest"] = digest_of(path.read_bytes())
            proof["_configuration_path"] = configuration
        export = ASSURANCE_DIR / INPUTS["PROOF-quire-static-export"][0]
        if not export.is_file():
            raise ChainError(
                f"{export} is absent. Run `make assurance-inputs`; this driver does "
                "not run producers."
            )
        declared["impact_snapshot"]["revision"] = self.revision
        declared["impact_snapshot"]["digest"] = digest_of(export.read_bytes())
        return declared

    def seal_record(self) -> tuple[str, dict[str, Any]]:
        body = self.record_body()
        configurations = {
            proof["proof_id"]: proof.pop("_configuration_path")
            for proof in body["definition"]["proof_obligations"]
        }
        result = quoin(
            "change-assurance",
            "seal-record",
            "--repo",
            str(self.store),
            "--input",
            "-",
            "--json",
            stdin=json.dumps(body),
        )
        if result.returncode != 0:
            raise ChainError(f"quoin refused the change-assurance record: {result.stderr.strip()}")
        digest = json.loads(result.stdout)["digest"]
        self.configurations = configurations
        self.record = body
        return digest, body

    # -- attestation -------------------------------------------------------

    def attestation_body(
        self,
        record_digest: str,
        proof_id: str,
        result_state: str,
        *,
        candidate_revision: str | None = None,
    ) -> dict[str, Any]:
        proof = next(
            item
            for item in self.record["definition"]["proof_obligations"]
            if item["proof_id"] == proof_id
        )
        return {
            "schema_version": 1,
            "record_type": "proof_attestation",
            "attestation_id": f"{proof_id}:{result_state}",
            "record_digest": record_digest,
            "candidate_revision": candidate_revision or self.revision,
            "proof_id": proof_id,
            "command": proof["command"],
            "tool": {
                "identity": proof["tool_identity"],
                "version": self.tool_versions[proof["tool_identity"]],
                "configuration_digest": proof["configuration_digest"],
            },
            "environment": self.environment,
            "observed_at": self.observed_at,
            "result": result_state,
        }

    def seal_attestation(
        self, body: dict[str, Any], output: Path, media_type: str
    ) -> dict[str, Any]:
        result = quoin(
            "change-assurance",
            "seal-attestation",
            "--input",
            "-",
            "--output",
            str(output),
            "--media-type",
            media_type,
            "--json",
            stdin=json.dumps(body),
        )
        if result.returncode != 0:
            raise ChainError(f"quoin refused the proof attestation: {result.stderr.strip()}")
        return json.loads(result.stdout)

    def intake(
        self, attestation: dict[str, Any], output: Path
    ) -> subprocess.CompletedProcess[str]:
        return quoin(
            "change-assurance",
            "intake",
            "--repo",
            str(self.store),
            "--attestation",
            "-",
            "--output",
            str(output),
            "--json",
            stdin=json.dumps(attestation),
        )

    def receipt(
        self,
        record_digest: str,
        selections: dict[str, str],
        decisions: Path,
        *,
        candidate_revision: str | None = None,
        audits: Path | None = None,
    ) -> tuple[int, dict[str, Any]]:
        arguments = [
            "change-assurance",
            "receipt",
            "--repo",
            str(self.store),
            "--record",
            record_digest,
            "--candidate-revision",
            candidate_revision or self.revision,
            "--decisions",
            str(decisions),
            "--json",
        ]
        for proof_id, attestation_digest in selections.items():
            arguments.extend(["--select", f"{proof_id}={attestation_digest}"])
        if audits is not None:
            arguments.extend(["--audits", str(audits)])
        result = quoin(*arguments)
        if result.returncode == 2:
            raise ChainError(f"quoin refused to emit a receipt: {result.stderr.strip()}")
        return result.returncode, json.loads(result.stdout)

    def verify_receipt(self, receipt: dict[str, Any]) -> tuple[int, str]:
        result = quoin(
            "change-assurance",
            "verify-receipt",
            "--input",
            "-",
            "--json",
            stdin=json.dumps(receipt),
        )
        return result.returncode, (result.stdout or result.stderr).strip()


# ---------------------------------------------------------------------------
# Inputs
# ---------------------------------------------------------------------------


def input_census() -> dict[str, str]:
    """Digest everything in the producer-output directory.

    The claim "this driver never creates its own inputs" is asserted twice and
    the two assertions have different reach. `tests/shared_assurance.rs` shims the
    tools a driver would have to run, which catches a driver that runs a tool
    someone thought to shim. This catches a driver that produced an input by any
    means at all, including a tool nobody named.

    An adversarial review found the first assertion alone insufficient: it shimmed
    cargo, rustup and rustc, and the driver was then made to regenerate a deleted
    `quire-static-export.json` by running `quire coverage` itself, and a deleted
    `upstream-identity.json` by running `python3 scripts/check_upstream_pins.py`.
    Neither tool was shimmed, both runs finished, and every gate stayed green.
    """
    if not ASSURANCE_DIR.is_dir():
        return {}
    return {
        path.relative_to(ASSURANCE_DIR).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest()
        for path in sorted(ASSURANCE_DIR.rglob("*"))
        if path.is_file()
    }


def require_inputs() -> dict[str, Path]:
    paths = {}
    for proof_id, (name, _) in INPUTS.items():
        path = ASSURANCE_DIR / name
        if not path.is_file():
            raise ChainError(
                f"{path.relative_to(ROOT)} is absent. Run `make assurance-inputs`. "
                "This driver consumes producer output and never creates it, so an "
                "absent input is an error rather than a step it can quietly do itself."
            )
        paths[proof_id] = path
    return paths


def _worst(results: list[str]) -> str:
    for candidate in RESULT_PRECEDENCE:
        if candidate in results:
            return candidate
    raise ChainError("a producer result stream carried no outcome at all")


def _rows_result(rows: list[dict[str, Any]], where: str) -> str:
    if not rows:
        raise ChainError(
            f"{where} carries no rows. A producer that reported nothing is vacuous, "
            "and vacuous is not passed."
        )
    results = []
    for index, row in enumerate(rows):
        outcome = row.get("outcome")
        if outcome not in ROW_RESULTS:
            raise ChainError(
                f"{where} row {index} declares outcome {outcome!r}, which is not named"
            )
        results.append(ROW_RESULTS[outcome])
    return _worst(results)


# The measurement fields each declared protocol's rows must carry, keyed by
# protocol. A verdict is cheap to forge; a measurement is not, so the chain
# requires the measurement to be present and self-consistent before it will read
# the verdict.
#
# This is not the chain re-running a producer or second-guessing its judgement.
# It is the chain refusing a document that states an outcome without stating what
# the outcome was derived from — which is exactly the shape a hand-written
# forgery takes, because reproducing the numbers is the expensive part.
MEASURED_PROTOCOLS = {
    CONFORMANCE_PROTOCOL: ("checksDischarged", "floor"),
    UPSTREAM_PROTOCOL: ("agreeingSources", "floor"),
}


def require_measurements(protocol: str | None, rows: list[dict[str, Any]], where: str) -> None:
    """Refuse rows that claim an outcome without the measurement behind them."""
    required = MEASURED_PROTOCOLS.get(protocol or "")
    if required is None:
        return
    measured, floor_field = required
    for index, row in enumerate(rows):
        for field in required:
            if field not in row:
                raise ChainError(
                    f"{where} row {index} ({row.get('symbol')}) declares outcome "
                    f"{row.get('outcome')!r} without {field}. This protocol reports a "
                    "measurement, and a verdict with no measurement behind it is a claim."
                )
        if row.get("outcome") != "pass":
            continue
        discharged, floor = row[measured], row[floor_field]
        if not isinstance(discharged, int) or not isinstance(floor, int):
            raise ChainError(
                f"{where} row {index} ({row.get('symbol')}) passes with a non-numeric "
                "measurement"
            )
        if discharged < floor:
            raise ChainError(
                f"{where} row {index} ({row.get('symbol')}) passes having discharged "
                f"{discharged} against a declared floor of {floor}. A case below its "
                "floor is vacuous, and vacuous is not passed."
            )


def derive_result(proof_id: str, path: Path) -> str:
    """Read the producer's own structured verdict out of the bytes it wrote.

    This is the difference between an attestation that states what happened and
    one that states what the caller hoped. Nothing here parses a transcript for
    words: every producer this repository owns emits a declared structured
    result, and `cargo` emits its own JSON message stream, so the verdict is read
    from a field in every case.

    A producer whose output cannot be read at all raises rather than defaulting.
    An attestation that says `passed` because its input was unreadable is the
    single worst failure this file could have.
    """
    try:
        raw = path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError) as error:
        raise ChainError(
            f"{path.name} could not be read: {error}. A producer result that cannot be "
            "read is not a failing result and is certainly not a passing one; it is an "
            "input this driver could not use, which is its own exit code."
        ) from error
    try:
        return _derive(proof_id, raw, path)
    except (json.JSONDecodeError, KeyError, TypeError) as error:
        # An unreadable or wrongly shaped producer document must reach the
        # environment-error channel, not fall out as a traceback. A stack trace is
        # not one of the twelve states, and exiting 1 would put "I could not read
        # the input" in the same channel as "a scenario did not match".
        raise ChainError(
            f"{path.name} is not a readable {proof_id} result: "
            f"{type(error).__name__}: {error}"
        ) from error


def _derive(proof_id: str, raw: str, path: Path) -> str:
    if proof_id in STREAM_DOCUMENTS:
        rows = [json.loads(line) for line in raw.splitlines() if line.strip()]
        if not rows:
            raise ChainError(
                f"{path.name} carries no rows. A producer that reported nothing is "
                "vacuous, and vacuous is not passed."
            )
        protocols = {row.get("protocol") for row in rows}
        if protocols != {CONFORMANCE_PROTOCOL}:
            raise ChainError(
                f"{path.name} declares protocols {sorted(str(item) for item in protocols)}; "
                f"this driver reads {CONFORMANCE_PROTOCOL} and refuses to guess"
            )
        require_measurements(CONFORMANCE_PROTOCOL, rows, path.name)
        return _rows_result(rows, path.name)
    if proof_id in ENTRY_DOCUMENTS:
        document = json.loads(raw)
        rows = document["entries"]
        require_measurements(document.get("protocol"), rows, path.name)
        return _rows_result(rows, path.name)
    if proof_id == "PROOF-quire-static-export":
        export = json.loads(raw)
        # Quire's export is a static fact set, not a run, so it has no outcome
        # field. What it can be held to is that it is the document it claims to
        # be. "A populated object" was the earlier rule and an adversarial review
        # showed `{"junk":[1]}` satisfying it: the chain attested `passed` over a
        # two-token file while the impact snapshot said `complete`. So the fields
        # a coverage export actually declares are named, and a document that does
        # not carry them is `not_computed` — which is a different answer from a
        # clean export and must not read as one.
        #
        # This is a shape check and it is deliberately not a content check. That
        # the export names every requirement in this repository is TC-010's
        # assertion, because it is a claim about this repository rather than about
        # the document, and the chain is not the place that knows the difference.
        if not isinstance(export, dict) or not export:
            return "not_computed"
        for required in QUIRE_EXPORT_FIELDS:
            if required not in export:
                return "not_computed"
        if not isinstance(export.get("totals"), dict) or not export["totals"]:
            return "not_computed"
        populated = any(
            isinstance(export.get(key), (list, dict)) and export.get(key) for key in export
        )
        return "passed" if populated else "not_computed"
    if proof_id == "PROOF-msrv":
        # `cargo --message-format=json` emits one JSON object per line and ends
        # with `build-finished`. The verdict is that object's `success` field.
        messages = [json.loads(line) for line in raw.splitlines() if line.strip()]
        finished = [item for item in messages if item.get("reason") == "build-finished"]
        if not finished:
            # The build did not report finishing. That is not a failure and it is
            # certainly not a pass; it is a run whose result was not computed.
            return "not_computed"
        if any(
            item.get("reason") == "compiler-message"
            and item.get("message", {}).get("level") == "error"
            for item in messages
        ):
            return "failed"
        return "passed" if finished[-1].get("success") is True else "failed"
    raise ChainError(f"no result rule is declared for {proof_id}")


def derive_failed_stream(raw: str) -> str:
    """One named edit to the real conformance stream: the first passing case fails.

    The corpus is green and has to stay green, so the failing case is derived
    from the real run rather than invented. A `fail` state demonstrated by a
    document nobody produced is a state nobody has actually seen travel the chain.
    """
    lines = [line for line in raw.splitlines() if line.strip()]
    for index, line in enumerate(lines):
        row = json.loads(line)
        if row.get("outcome") == "pass":
            row["outcome"] = "fail"
            lines[index] = json.dumps(row)
            return "\n".join(lines) + "\n"
    raise ChainError("the conformance stream contains no passing row to derive a failure from")


def derive_malformed_stream(raw: str) -> str:
    """The same one named edit, but the first passing case reports `malformed`.

    `malformed` is a declared member of this adapter's producer vocabulary --
    ROW_RESULTS and CONFORMANCE_OUTCOMES both name it -- so a producer really
    reporting it is a state that travels the chain, not a refusal wearing a
    label. It is derived from the real run for the same reason the failing case
    is: a state demonstrated by a document nobody produced is a state nobody has
    actually seen travel the chain.

    This is the only demonstration of `malformed` in this repository. Until the
    pre-stable preservation release it was demonstrated by the compatibility
    census, against a retained record whose field really had the wrong type.
    Those records are deleted. Measured on the pre-deletion tree, the chain alone
    reached ten of the twelve states and the census supplied `unsupported` and
    `malformed`; without this scenario, deleting the census would have left
    TC-012 passing at ten rather than going red, which is a gate weakening
    silently rather than a claim being withdrawn deliberately.
    """
    lines = [line for line in raw.splitlines() if line.strip()]
    for index, line in enumerate(lines):
        row = json.loads(line)
        if row.get("outcome") == "pass":
            row["outcome"] = "malformed"
            lines[index] = json.dumps(row)
            return "\n".join(lines) + "\n"
    raise ChainError("the conformance stream contains no passing row to derive a malformed one from")


# ---------------------------------------------------------------------------
# Scenarios
# ---------------------------------------------------------------------------


def run_chain(candidate_revision: str, workspace: Path) -> dict[str, Any]:
    inputs = require_inputs()
    store = workspace / "store"
    store.mkdir(parents=True, exist_ok=True)
    scratch = workspace / "scratch"
    scratch.mkdir(parents=True, exist_ok=True)

    chain = Chain(candidate_revision, store)
    record_digest, _ = chain.seal_record()

    decisions = scratch / "decisions.json"
    decisions.write_text(
        json.dumps({"run_id": chain.record["review_workflow"]["run_id"], "events": []}),
        encoding="utf-8",
    )

    def audit_reports(path: Path) -> Path:
        """A clean FR-032 audit report per proof, naming that proof's own obligations.

        `healthy` has to name the obligations the audit actually evaluated. A
        report that names none is an audit that evaluated nothing, and Quoin says
        so rather than reading it as clean — which is the correct behaviour and
        the reason this helper builds one report per proof instead of one for all.
        """
        reports = []
        for proof in chain.record["definition"]["proof_obligations"]:
            report = {"findings": [], "healthy": list(proof["obligation_ids"]), "unevaluated": []}
            reports.append(
                {
                    "proof_id": proof["proof_id"],
                    "report_digest": digest_of(
                        json.dumps(report, sort_keys=True, separators=(",", ":")).encode("utf-8")
                    ),
                    "report": report,
                }
            )
        path.write_text(json.dumps(reports), encoding="utf-8")
        return path

    audits = audit_reports(scratch / "audits.json")

    def proof_rows(receipt: dict[str, Any]) -> dict[str, dict[str, Any]]:
        return {row["proof_id"]: row for row in receipt["proofs"]}

    scenarios: list[dict[str, Any]] = []
    controls: list[dict[str, Any]] = []

    def scenario(name: str, state: str | None, matched: bool, detail: Any) -> None:
        """Record a scenario. `state` is None when it demonstrates no outcome.

        A scenario that is not about one of the twelve outcomes must say so,
        rather than borrow a label and inflate the census.
        """
        scenarios.append(
            {"scenario": name, "state": state, "matched": bool(matched), "detail": detail}
        )

    def control(name: str, pairs_with: str, matched: bool, detail: Any) -> None:
        controls.append(
            {"control": name, "pairs_with": pairs_with, "matched": bool(matched), "detail": detail}
        )

    # -- 1. the honest path: seal, retain, and get the bytes back unchanged ---
    #
    # The attested result is read out of the bytes the producer wrote, never
    # assumed. A loop that sealed `passed` for everything would report a green
    # chain over a red repository, which is the failure this whole migration is
    # supposed to make impossible.
    selections: dict[str, str] = {}
    observed_results: dict[str, str] = {}
    for proof_id, path in inputs.items():
        media_type = INPUTS[proof_id][1]
        observed = derive_result(proof_id, path)
        observed_results[proof_id] = observed
        body = chain.attestation_body(record_digest, proof_id, observed)
        sealed = chain.seal_attestation(body, path, media_type)
        taken = chain.intake(sealed, path)
        if taken.returncode != 0:
            raise ChainError(
                f"{proof_id}: intake refused an unmodified producer output: {taken.stderr.strip()}"
            )
        detail = json.loads(taken.stdout)
        retained = Path(detail["directory"]) / "output.bin"
        identical = retained.read_bytes() == path.read_bytes()
        selections[proof_id] = sealed["digest"]
        if proof_id == PRIMARY_PROOF:
            scenario(
                "retain-producer-output",
                "pass",
                identical,
                {"retained": str(retained), "bytes": retained.stat().st_size},
            )
            # The control observes what intake *did*, not what the scenario
            # measured. Both used to read the same `identical` boolean, which made
            # the positive control a second printing of the scenario's own result
            # rather than an independent one.
            control(
                "intake-accepts-unchanged-bytes",
                "retained-bytes-changed-after-sealing",
                taken.returncode == 0,
                {"proof": proof_id, "intake_exit": taken.returncode},
            )

    # Every producer at this candidate revision reported success, and the
    # attestations say so because the bytes did, not because the loop assumed it.
    # If a producer had failed — or if its rows read `unavailable` — this scenario
    # is where the chain says so and goes red.
    scenario(
        "attested-results-are-read-from-producer-output",
        None,
        all(result == "passed" for result in observed_results.values()),
        observed_results,
    )

    # -- 2. the receipt, and re-verifying it ---------------------------------
    status, receipt = chain.receipt(record_digest, selections, decisions)
    verified_status, _ = chain.verify_receipt(receipt)
    # No ix-flow decision exists, so an `incomplete` receipt is the correct
    # answer, and the reason it gives must be the missing decision specifically.
    # Asserting only "not valid" would be satisfied by a receipt that was invalid
    # because every proof failed.
    scenario(
        "receipt-reports-the-absent-human-decision",
        "partial",
        status == 1
        and receipt["outcome"] == "incomplete"
        and "decision_missing" in receipt["reasons"]
        and receipt["checks"]["review"]["outcome"] == "incomplete"
        and receipt["decision_event"] is None,
        {
            "outcome": receipt["outcome"],
            "exit": status,
            "reasons": receipt["reasons"],
            "review": receipt["checks"]["review"],
        },
    )
    scenario(
        "re-verify-the-sealed-receipt",
        "pass",
        verified_status == status,
        {"verify_exit": verified_status, "receipt_exit": status},
    )
    control(
        "verify-accepts-an-unedited-receipt",
        "refuse-an-edited-receipt",
        verified_status != 2,
        {"exit": verified_status},
    )

    # -- 3. an edited receipt is refused -------------------------------------
    edited = json.loads(json.dumps(receipt))
    edited["outcome"] = "valid"
    edited_status, edited_detail = chain.verify_receipt(edited)
    scenario(
        "refuse-an-edited-receipt",
        "tampered",
        edited_status == 2,
        {"exit": edited_status, "message": edited_detail[:200]},
    )

    # -- 4. retained bytes changed after sealing -----------------------------
    moved = scratch / "moved.jsonl"
    moved.write_bytes(inputs[PRIMARY_PROOF].read_bytes())
    body = chain.attestation_body(record_digest, PRIMARY_PROOF, "passed")
    sealed_moved = chain.seal_attestation(body, moved, "application/x-ndjson")
    moved.write_bytes(moved.read_bytes() + b"\n")
    refused = chain.intake(sealed_moved, moved)
    scenario(
        "retained-bytes-changed-after-sealing",
        "tampered",
        refused.returncode != 0,
        {"exit": refused.returncode, "message": refused.stderr.strip()[:200]},
    )

    # -- 5. a stale candidate binding ----------------------------------------
    stale_status, stale_receipt = chain.receipt(
        record_digest, selections, decisions, candidate_revision="0" * 40, audits=audits
    )
    stale_reasons = set(proof_rows(stale_receipt)[PRIMARY_PROOF]["reasons"])
    scenario(
        "stale-candidate-binding",
        "stale",
        "candidate_revision_mismatch" in stale_reasons,
        {"outcome": stale_receipt["outcome"], "reasons": sorted(stale_reasons)},
    )

    # -- 6. attested non-success states, each named by its own reason ---------
    #
    # The receipt is asked with a clean audit for every proof, so that the only
    # thing distinguishing these runs is the attested result. Without that, every
    # proof row reads `audit_not_evaluated` and three different states collapse
    # into one indistinguishable answer, which is precisely the failure this
    # scenario exists to rule out.
    audited_status, audited = chain.receipt(record_digest, selections, decisions, audits=audits)
    passing_row = proof_rows(audited)[PRIMARY_PROOF]
    control(
        "an-audited-passing-proof-is-valid-and-reasonless",
        "attested-failed",
        passing_row["outcome"] == "valid" and not passing_row["reasons"],
        {"row": passing_row["outcome"], "reasons": passing_row["reasons"]},
    )
    control(
        "receipt-discharges-a-current-binding",
        "stale-candidate-binding",
        "candidate_revision_mismatch" not in passing_row["reasons"],
        {"reasons": passing_row["reasons"]},
    )

    failed_stream = scratch / "failed.jsonl"
    failed_stream.write_text(
        derive_failed_stream(inputs[PRIMARY_PROOF].read_text(encoding="utf-8")), encoding="utf-8"
    )
    expected_reason = {
        "failed": "result_failed",
        "unavailable": "result_unavailable",
        "not_computed": "result_not_computed",
    }
    state_name = {"failed": "fail", "unavailable": "unavailable", "not_computed": "not-computed"}
    observed_reasons: dict[str, set[str]] = {}
    for state, source in (
        ("failed", failed_stream),
        ("unavailable", inputs[PRIMARY_PROOF]),
        ("not_computed", inputs[PRIMARY_PROOF]),
    ):
        body = chain.attestation_body(record_digest, PRIMARY_PROOF, state)
        body["attestation_id"] = f"{PRIMARY_PROOF}:{state}"
        sealed_state = chain.seal_attestation(body, source, "application/x-ndjson")
        taken = chain.intake(sealed_state, source)
        if taken.returncode != 0:
            raise ChainError(f"intake refused a {state} attestation: {taken.stderr.strip()}")
        state_selections = dict(selections)
        state_selections[PRIMARY_PROOF] = sealed_state["digest"]
        _, state_receipt = chain.receipt(
            record_digest, state_selections, decisions, audits=audits
        )
        rows = proof_rows(state_receipt)
        reasons = set(rows[PRIMARY_PROOF]["reasons"])
        observed_reasons[state] = reasons
        scenario(
            f"attested-{state}",
            state_name[state],
            expected_reason[state] in reasons,
            {"reasons": sorted(reasons), "receipt_outcome": state_receipt["outcome"]},
        )
        if state == "failed":
            control(
                "passing-proof-is-not-reported-as-failing",
                "attested-failed",
                not set(rows["PROOF-upstream-identity"]["reasons"])
                & set(expected_reason.values()),
                {"upstream_reasons": rows["PROOF-upstream-identity"]["reasons"]},
            )

    # The three non-success states must be pairwise distinguishable. Each being
    # non-passing individually would still be satisfied by collapsing all three.
    distinct = len({frozenset(value) for value in observed_reasons.values()}) == 3
    scenario(
        "non-success-states-stay-distinguishable",
        # Not a state demonstration: this is the assertion that the three states
        # demonstrated above are not the same answer wearing three names.
        None,
        distinct,
        {state: sorted(value) for state, value in observed_reasons.items()},
    )

    # -- 6b. a producer reporting `malformed` is carried, not refused ----------
    #
    # `malformed` is in this adapter's declared producer vocabulary, so a stream
    # reporting it is transcribed rather than refused or defaulted. The coarse
    # attestation result is `failed` -- the schema has four values and the domain
    # state is not meant to survive into it -- and the domain's own `malformed`
    # survives in the retained bytes Quoin binds by digest. Both halves are
    # asserted, because "it became a failure" without "the word is still in the
    # bytes" is the collapse this whole migration is about.
    malformed_stream = scratch / "malformed.jsonl"
    malformed_raw = derive_malformed_stream(inputs[PRIMARY_PROOF].read_text(encoding="utf-8"))
    malformed_stream.write_text(malformed_raw, encoding="utf-8")
    malformed_entries = adapt_conformance(malformed_raw)
    transcribed = {entry["outcome"] for entry in malformed_entries["entries"]}
    malformed_body = chain.attestation_body(record_digest, PRIMARY_PROOF, "failed")
    malformed_body["attestation_id"] = f"{PRIMARY_PROOF}:malformed"
    sealed_malformed = chain.seal_attestation(malformed_body, malformed_stream, "application/x-ndjson")
    taken_malformed = chain.intake(sealed_malformed, malformed_stream)
    if taken_malformed.returncode != 0:
        raise ChainError(f"intake refused a malformed attestation: {taken_malformed.stderr.strip()}")
    malformed_selections = dict(selections)
    malformed_selections[PRIMARY_PROOF] = sealed_malformed["digest"]
    _, malformed_receipt = chain.receipt(
        record_digest, malformed_selections, decisions, audits=audits
    )
    malformed_reasons = set(proof_rows(malformed_receipt)[PRIMARY_PROOF]["reasons"])
    scenario(
        "attested-malformed",
        "malformed",
        "result_failed" in malformed_reasons
        and "fail" in transcribed
        and '"outcome": "malformed"' in json.dumps(json.loads(malformed_raw.splitlines()[0])),
        {
            "transcribed_outcomes": sorted(transcribed),
            "reasons": sorted(malformed_reasons),
            "receipt_outcome": malformed_receipt["outcome"],
            "why": (
                "the producer reported malformed, the adapter named it rather than "
                "refusing it, the coarse attestation result is failed, and the "
                "retained bytes still carry the domain state"
            ),
        },
    )
    control(
        "a-clean-producer-is-not-reported-as-malformed",
        "attested-malformed",
        "result_failed" not in set(passing_row["reasons"]),
        {"reasons": passing_row["reasons"]},
    )

    # -- 7. an unaudited proof is not-computed, not clean ---------------------
    unaudited_row = proof_rows(receipt)[PRIMARY_PROOF]
    scenario(
        "audited-clean-versus-unaudited",
        "not-computed",
        "audit_not_evaluated" in unaudited_row["reasons"]
        and "audit_not_evaluated" not in passing_row["reasons"],
        {
            "unaudited": unaudited_row["reasons"],
            "audited": passing_row["reasons"],
            "why": (
                "an audit with no findings and no audit at all are different facts; "
                "the absence is reported as not-computed rather than as clean"
            ),
        },
    )
    control(
        "an-audit-that-was-run-clears-not-computed",
        "audited-clean-versus-unaudited",
        audited_status in (0, 1) and "audit_not_evaluated" not in audited["reasons"],
        {"receipt_reasons": audited["reasons"]},
    )

    # -- 8. a proof with no attestation stays missing -------------------------
    partial_selections = {key: value for key, value in selections.items() if key != "PROOF-msrv"}
    _, partial = chain.receipt(record_digest, partial_selections, decisions, audits=audits)
    missing_row = proof_rows(partial).get("PROOF-msrv", {})
    scenario(
        "unattested-proof-stays-missing",
        "partial",
        partial["outcome"] != "valid"
        and "attestation_missing" in set(missing_row.get("reasons", [])),
        {"outcome": partial["outcome"], "msrv_reasons": missing_row.get("reasons")},
    )

    # -- 9. the open unknowns survive into the receipt ------------------------
    declared_unknowns = {
        item["id"]
        for item in chain.record["definition"]["unknowns"]
        if item["disposition"] == "open"
    }
    carried = {item["id"] for item in audited.get("unknowns", [])}
    scenario(
        "declared-unknowns-are-carried-not-dropped",
        "inconclusive",
        declared_unknowns <= carried and "unresolved_unknown" in audited["reasons"],
        {"declared_open": sorted(declared_unknowns), "carried": sorted(carried)},
    )

    # Every control must name a scenario that actually ran. A `pairs_with` naming
    # nothing is a control that pairs with nothing, and a test asserting the
    # dangling name would be satisfied by the typo rather than by the pairing.
    names = {item["scenario"] for item in scenarios}
    dangling = sorted(item["control"] for item in controls if item["pairs_with"] not in names)
    if dangling:
        raise ChainError(
            f"these controls name a scenario that does not exist: {dangling}. "
            f"Scenarios present: {sorted(names)}"
        )

    return {
        "record_digest": record_digest,
        "candidate_revision": candidate_revision,
        "impact_snapshot_digest": chain.record["impact_snapshot"]["digest"],
        "quire_export": str(
            (ASSURANCE_DIR / INPUTS["PROOF-quire-static-export"][0]).relative_to(ROOT)
        ),
        "attested_results": observed_results,
        "receipt_outcome": receipt["outcome"],
        "audited_receipt_outcome": audited["outcome"],
        "audited_receipt_reasons": audited["reasons"],
        "scenarios": scenarios,
        "controls": controls,
    }


# ---------------------------------------------------------------------------
# Adapter probes
# ---------------------------------------------------------------------------

# The exact sentence fragment the suspect-link probe rewords. Kept as a constant
# so that a specification edit which invalidates the probe fails loudly here
# rather than silently turning the probe into a no-op.
SUSPECT_PROBE_MARKER = "Repeated generation from identical inputs is byte-identical."


def adapter_probes(workspace: Path) -> list[dict[str, Any]]:
    """Exercise the native adapter and Quoin's evidence audit in a scratch tree.

    A copy of `spec/` is used so the suite registry, requirements and matrix are
    the real ones and a binding is a real binding, while nothing is written into
    this repository's own store.
    """
    inputs = require_inputs()
    probe_root = workspace / "adapter"
    if probe_root.exists():
        shutil.rmtree(probe_root)
    probe_root.mkdir(parents=True)
    shutil.copytree(ROOT / "spec", probe_root / "spec")

    stream = inputs[PRIMARY_PROOF].read_text(encoding="utf-8")
    commit = "0" * 40
    results = []

    def record(suite: str, payload: dict[str, Any], commit_sha: str) -> dict[str, Any]:
        path = probe_root / "run.json"
        path.write_text(json.dumps(payload), encoding="utf-8")
        outcome = quoin(
            "evidence",
            "record",
            "--repo",
            str(probe_root),
            "--suite",
            suite,
            "--commit",
            commit_sha,
            "--tool",
            "quire-contract-codegen-generation-conformance 0.1.0",
            "--adapter",
            "entries",
            # The kind SUITE-001 declares. Method conformance compares kind to
            # kind, so a value the suite registry does not use would make the
            # check stay silent instead of checking.
            "--kind",
            "Integration",
            "--results",
            str(path),
            "--json",
        )
        if outcome.returncode != 0:
            raise ChainError(f"quoin refused an evidence record: {outcome.stderr.strip()}")
        return json.loads(outcome.stdout)

    def audit_kinds() -> dict[str, int]:
        outcome = quoin("evidence", "audit", "--repo", str(probe_root), "--json")
        if outcome.returncode not in (0, 1):
            raise ChainError(f"quoin evidence audit failed: {outcome.stderr.strip()}")
        findings = json.loads(outcome.stdout)["findings"]
        counted: dict[str, int] = {}
        for finding in findings:
            counted[finding["kind"]] = counted.get(finding["kind"], 0) + 1
        return counted

    # Probe 1 (positive control): the real run binds real obligations.
    transcribed = adapt_conformance(stream)
    bound = record("SUITE-001", transcribed, commit)["bound"]
    results.append(
        {
            "probe": "accepts-the-real-run",
            "state": "pass",
            "matched": bool(bound),
            "detail": {"bound": len(bound), "entries": len(transcribed["entries"])},
        }
    )

    # Probe 2: the adapter must carry a non-success outcome through as a
    # non-success outcome. The stream is derived from the real one by renaming
    # every outcome to `unavailable` — the state a machine with no usable
    # generator really produces — and it is transcribed by the adapter rather
    # than hand-built, so an adapter that mapped everything to `pass` would be
    # caught here instead of quietly producing a clean run.
    downgraded_lines = []
    for line in stream.splitlines():
        if not line.strip():
            continue
        row = json.loads(line)
        row["outcome"] = "unavailable"
        downgraded_lines.append(json.dumps(row))
    downgraded = adapt_conformance("\n".join(downgraded_lines))
    preserved = all(entry["outcome"] != "pass" for entry in downgraded["entries"])
    results.append(
        {
            "probe": "adapter-preserves-non-success-outcomes",
            "state": "unavailable",
            "matched": preserved and bool(downgraded["entries"]),
            "detail": {
                "outcomes": sorted({entry["outcome"] for entry in downgraded["entries"]}),
                "entries": len(downgraded["entries"]),
            },
        }
    )

    # Probe 3: a run in which every bound symbol was skipped is vacuous, and
    # Quoin says so rather than reading the row as covered. The entries come from
    # the adapter's own transcription of the derived stream above.
    record("SUITE-001", downgraded, commit)
    kinds = audit_kinds()
    results.append(
        {
            "probe": "audit-reports-a-vacuous-run",
            "state": "vacuous",
            "matched": kinds.get("vacuous-evidence", 0) > 0,
            "detail": kinds,
        }
    )

    # Probe 4: a reworded statement makes its bound evidence suspect.
    record("SUITE-001", transcribed, commit)
    requirement = probe_root / "spec" / "functional" / "FR-001-deterministic-oracles.md"
    text = requirement.read_text(encoding="utf-8")
    if SUSPECT_PROBE_MARKER not in text:
        raise ChainError(
            "the probe's acceptance-criterion marker is no longer present in FR-001. "
            "The suspect-link probe rewords a criterion that the transcribed run "
            "actually binds; a marker that no longer exists would turn the probe "
            "into a no-op, so this is an error rather than a skip."
        )
    results_before = audit_kinds()
    requirement.write_text(
        text.replace(
            SUSPECT_PROBE_MARKER,
            SUSPECT_PROBE_MARKER.rstrip(".")
            + ", and each generated artifact records the digest of its own bytes.",
            1,
        ),
        encoding="utf-8",
    )
    kinds = audit_kinds()
    results.append(
        {
            "probe": "audit-reports-a-suspect-link",
            "state": "suspect",
            "matched": kinds.get("suspect-link", 0) > results_before.get("suspect-link", 0),
            "detail": {"before": results_before, "after": kinds},
        }
    )

    # Probe 5: a foreign protocol is refused by the adapter, not guessed at.
    foreign_lines = []
    for line in stream.splitlines():
        if not line.strip():
            continue
        row = json.loads(line)
        row["protocol"] = "some.other.protocol/v1"
        foreign_lines.append(json.dumps(row))
    refused = False
    try:
        adapt_conformance("\n".join(foreign_lines))
    except ChainError:
        refused = True
    results.append(
        {
            "probe": "refuses-a-foreign-protocol",
            # A refusal by the adapter is not one of the states travelling the
            # chain; it is the adapter declining to produce one. Labelling it
            # `unsupported` would let the census count a refusal as a
            # demonstration. `unsupported` used to be demonstrated by the
            # compatibility view, against a retained record that really carried
            # an unknown PGM-01 schema version. That view and those records are
            # deleted, and `unsupported` is not in this adapter's producer
            # vocabulary, so the state has no demonstration here and this probe
            # does not invent one for it. Recorded as a limitation in MP-001 and
            # asserted absent by TC-012, not papered over.
            "state": None,
            "matched": refused,
            "detail": {"protocol": "some.other.protocol/v1"},
        }
    )

    # Probe 6: an empty stream is refused rather than transcribed into a clean run.
    empty_refused = False
    try:
        adapt_conformance("")
    except ChainError:
        empty_refused = True
    results.append(
        {
            "probe": "refuses-an-empty-stream",
            # Same reason. `vacuous` is demonstrated by Quoin's own audit, three
            # probes above, against a run it read and found empty of outcomes.
            "state": None,
            "matched": empty_refused,
            "detail": {},
        }
    )

    # Probe 7: an outcome the adapter does not name is refused rather than
    # defaulted. A silently defaulted unknown state is how twelve states become
    # two, and this repository's whole migration is about not doing that.
    unknown_lines = [line for line in stream.splitlines() if line.strip()]
    first = json.loads(unknown_lines[0])
    first["outcome"] = "probably-fine"
    unknown_lines[0] = json.dumps(first)
    unknown_refused = False
    try:
        adapt_conformance("\n".join(unknown_lines))
    except ChainError:
        unknown_refused = True
    results.append(
        {
            "probe": "refuses-an-unnamed-outcome",
            # Same reason. `malformed` is demonstrated by `attested-malformed`,
            # against a producer stream that really declares that outcome, which
            # is a state travelling the chain rather than the adapter declining
            # to produce one.
            "state": None,
            "matched": unknown_refused,
            "detail": {"outcome": "probably-fine"},
        }
    )

    return results


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--candidate-revision")
    parser.add_argument("--json", action="store_true")
    parser.add_argument(
        "--keep-store",
        action="store_true",
        help="keep this run's Quoin store under target/assurance-store for inspection",
    )
    parser.add_argument(
        "--adapt",
        metavar="PATH",
        help=(
            "transcribe a domain conformance stream into Quoin's normalized entries "
            "and print them; this is the adapter on its own, with no chain around it"
        ),
    )
    arguments = parser.parse_args(argv[1:])

    # Taken here, before this file does anything else, because the census can only
    # speak for the interval it brackets. Placed lower, an input regenerated
    # earlier in `main` would appear in both halves and read as untouched — which
    # is exactly what an adversarial re-probe demonstrated when it sat just above
    # the try block.
    #
    # The two isolation checks are complementary and neither subsumes the other.
    # This one covers any tool, including one nobody thought to shim, but only
    # from this point onward. `tests/shared_assurance.rs` covers any point,
    # including before this line, but only for the tools it names.
    inputs_before = input_census()

    if arguments.adapt is not None:
        try:
            entries = adapt_conformance(Path(arguments.adapt).read_text(encoding="utf-8"))
        except (ChainError, OSError) as error:
            print(str(error), file=sys.stderr)
            return 2
        print(json.dumps(entries, indent=2, sort_keys=True))
        return 0

    if arguments.candidate_revision is None:
        print("--candidate-revision is required", file=sys.stderr)
        return 2

    # Each run gets its own store. Two runs sharing one directory is a race that
    # makes a green run depend on which finished first, and this driver is
    # invoked concurrently by the test suite.
    STORE.mkdir(parents=True, exist_ok=True)
    workspace = Path(tempfile.mkdtemp(prefix="run-", dir=STORE))

    try:
        chain = run_chain(arguments.candidate_revision, workspace)
        probes = adapter_probes(workspace)
    except ChainError as error:
        print(str(error), file=sys.stderr)
        return 2
    finally:
        if not arguments.keep_store:
            shutil.rmtree(workspace, ignore_errors=True)

    # Whatever tool it might have used, the driver did not touch the directory it
    # reads its inputs from. This is the total form of the claim `require_inputs`
    # makes in prose, and it is checked rather than asserted.
    inputs_after = input_census()
    touched = sorted(
        name
        for name in set(inputs_before) | set(inputs_after)
        if inputs_before.get(name) != inputs_after.get(name)
    )
    if touched:
        print(
            f"the driver wrote into its own input directory: {touched}. Every file "
            "under target/assurance/ is producer output and this driver consumes it. "
            "A driver that can produce its own inputs can produce a green run out of "
            "nothing.",
            file=sys.stderr,
        )
        return 2

    report = {
        "schemaVersion": "quire-contract-codegen.assurance-chain-report/v1",
        **chain,
        "adapter_probes": probes,
        # Only a case that ran and matched counts. A scenario that failed did not
        # demonstrate its state, and one that demonstrates no state says so with
        # a null rather than borrowing a label.
        "states_demonstrated": sorted(
            {
                item["state"]
                for group in (chain["scenarios"], probes)
                for item in group
                if item["matched"] and item.get("state") is not None
            }
        ),
        "matched": all(
            item["matched"]
            for group in (chain["scenarios"], chain["controls"], probes)
            for item in group
        ),
    }
    if arguments.json:
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        for item in chain["scenarios"]:
            print(
                f"scenario {item['scenario']} [{item['state']}]: "
                f"{'ok' if item['matched'] else 'MISMATCH'}"
            )
        for item in chain["controls"]:
            print(
                f"control  {item['control']} (pairs with {item['pairs_with']}): "
                f"{'ok' if item['matched'] else 'MISMATCH'}"
            )
        for item in probes:
            print(
                f"probe    {item['probe']} [{item['state']}]: "
                f"{'ok' if item['matched'] else 'MISMATCH'}"
            )
    if not report["matched"]:
        print("the assurance chain did not match its declared scenarios", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
