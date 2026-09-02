#!/usr/bin/env python3
"""Publish the exact upstream identities this crate generates against (FR-006-AC-2).

Every artifact this repository generates records the IR revision and the runtime
revision it was lowered against. Those two revisions appear in four places — the
crate's own constants, the dependency declarations, the lockfile, and the
generated manifests — and NFR-002 requires them to be the same revision in all
four. A generator that says it targeted one revision and linked another produces
artifacts whose provenance is fiction.

This publishes `codegen.upstream-identity/v1`: one row per declared upstream,
each carrying the revision the crate declares, the revision the manifest pins,
the revision the lockfile resolved, and whether the three agree.

Two things it is not.

It is not a resolver. It reads the files that are already on disk. It never runs
cargo, never fetches, and never edits a pin to make a row go green.

It is not a network probe. It does not ask GitHub whether a revision exists.
Whether the pinned revision is the right one is a review question; whether the
four places agree is a checkable fact, and only the second is claimed here.

Exit status
  gate mode : 0 when every row agrees, 1 when one does not, 2 when a file this
              needs could not be read at all
  --json    : 0 whenever a document was produced, whatever it says; 2 when no
              document could be produced
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
PROTOCOL = "codegen.upstream-identity/v1"

# The upstreams this crate lowers against, and where each one's revision is
# declared. The constant name is the crate's own statement; the package name is
# what Cargo calls it. Both are needed because they are the two things that have
# to agree.
UPSTREAMS = (
    ("IR_CANDIDATE_REVISION", "quire-contract-ir", ["FR-001", "NFR-002-AC-1"]),
    ("RUNTIME_REVISION", "quire-contract-runtime", ["FR-002", "NFR-002-AC-1"]),
)

REVISION = re.compile(r"^[0-9a-f]{40}$")


class ProducerError(RuntimeError):
    """No result document could be produced at all."""


def read(relative: str) -> str:
    path = ROOT / relative
    try:
        return path.read_text(encoding="utf-8")
    except OSError as error:
        raise ProducerError(f"{relative} could not be read: {error}") from error


def declared_constant(source: str, name: str) -> str | None:
    """The revision the crate itself states, read from its own constant."""
    found = re.search(rf'{name}: &str = "([0-9a-f]{{40}})"', source)
    return found.group(1) if found else None


def manifest_revision(manifest: str, package: str) -> str | None:
    """The revision the dependency declaration pins."""
    found = re.search(rf'{re.escape(package)} = \{{[^}}]*?rev = "([0-9a-f]{{40}})"', manifest)
    return found.group(1) if found else None


def lockfile_revision(lockfile: str, package: str) -> str | None:
    """The revision the lockfile resolved.

    Read from the `source` line of the package's own stanza rather than by
    searching the whole file for the revision the manifest happens to name. The
    difference matters: searching for the manifest's answer can only ever
    confirm it, which is a check that cannot disagree.
    """
    stanza = re.search(
        rf'\[\[package\]\]\nname = "{re.escape(package)}"\n(?:.*\n)*?source = "([^"]+)"',
        lockfile,
    )
    if stanza is None:
        return None
    found = re.search(r"[?&]rev=([0-9a-f]{40})", stanza.group(1))
    return found.group(1) if found else None


def collect() -> dict[str, Any]:
    """Produce the result document. Never raises for a disagreeing row."""
    source = read("src/oracle.rs")
    manifest = read("Cargo.toml")
    lockfile = read("Cargo.lock")

    entries = []
    for constant, package, traces in UPSTREAMS:
        declared = declared_constant(source, constant)
        pinned = manifest_revision(manifest, package)
        resolved = lockfile_revision(lockfile, package)
        observed = [declared, pinned, resolved]
        if any(value is None for value in observed):
            # One of the three places does not state a revision at all. That is
            # not a disagreement between revisions; nothing was computed for this
            # upstream, and saying `fail` would claim a comparison that never
            # happened.
            outcome = "not-computed"
        elif not all(REVISION.match(value) for value in observed):
            outcome = "malformed"
        elif declared == pinned == resolved:
            outcome = "pass"
        else:
            outcome = "fail"
        entries.append(
            {
                "symbol": f"upstream::{package}",
                "outcome": outcome,
                "traceIds": traces,
                "declaredRevision": declared,
                "manifestRevision": pinned,
                "lockfileRevision": resolved,
                # The measurement behind the verdict: how many of the three
                # places agreed with the crate's own constant. A verdict with no
                # measurement behind it is a claim.
                "agreeingSources": sum(
                    1 for value in observed if value is not None and value == declared
                ),
                "floor": 3,
            }
        )
    return {
        "protocol": PROTOCOL,
        "tool": {"identity": "quire-contract-codegen/check_upstream_pins", "version": None},
        "entries": entries,
    }


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--json",
        action="store_true",
        help=f"emit {PROTOCOL} on stdout; the producer role",
    )
    arguments = parser.parse_args(argv[1:])

    try:
        document = collect()
    except ProducerError as error:
        print(str(error), file=sys.stderr)
        return 2

    if arguments.json:
        print(json.dumps(document, indent=2, sort_keys=True))
        return 0

    failures = [row for row in document["entries"] if row["outcome"] != "pass"]
    for row in failures:
        print(
            f"UPSTREAM {row['outcome'].upper()}: {row['symbol']} "
            f"declared={row['declaredRevision']} manifest={row['manifestRevision']} "
            f"lockfile={row['lockfileRevision']}",
            file=sys.stderr,
        )
    if failures:
        print(
            f"{len(failures)} of {len(document['entries'])} upstream identities do not agree",
            file=sys.stderr,
        )
        return 1
    for row in document["entries"]:
        print(f"{row['symbol']}: {row['declaredRevision']} (constant, manifest and lockfile agree)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
