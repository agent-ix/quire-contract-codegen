#!/usr/bin/env python3
"""Regenerate the complete committed codegen evidence anchor census."""

from __future__ import annotations

import hashlib
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
EVIDENCE_ROOT = ROOT / "evidence"
ANCHORS = EVIDENCE_ROOT / "ANCHORS"


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def tree_digest(root: Path) -> str:
    state = hashlib.sha256()
    for path in sorted(root.rglob("*"), key=lambda item: item.as_posix()):
        if path.is_symlink():
            raise ValueError(f"symlink is not allowed in retained evidence: {path}")
        relative = path.relative_to(root).as_posix()
        if path.is_dir():
            kind = b"d"
        elif path.is_file():
            kind = b"f"
        else:
            raise ValueError(f"unsupported retained-evidence entry: {path}")
        state.update(kind + b"\0" + relative.encode("utf-8") + b"\0")
        if path.is_file():
            state.update(bytes.fromhex(sha256_file(path)))
        state.update(b"\0")
    return state.hexdigest()


def rendered_anchors() -> str:
    entries: list[tuple[Path, str]] = []
    for path in EVIDENCE_ROOT.iterdir():
        if path == ANCHORS:
            continue
        if path.is_symlink():
            raise ValueError(f"symlink is not allowed in retained evidence: {path}")
        if (
            path.is_dir()
            and path.name.startswith("foundation-")
            and path.name != "foundation-remote"
        ):
            target = path / "sha256sums.txt"
            if not (path / "evidence-envelope.json").is_file() or not target.is_file():
                raise ValueError(f"incomplete authoritative evidence directory: {path}")
            entries.append((target, sha256_file(target)))
        elif path.is_dir():
            entries.append((path, tree_digest(path)))
        elif path.is_file():
            entries.append((path, sha256_file(path)))
        else:
            raise ValueError(f"unsupported evidence entry: {path}")
    lines = ["# Complete SHA-256 census for committed codegen evidence."]
    lines.extend(
        f"{digest}  {path.relative_to(ROOT).as_posix()}"
        for path, digest in sorted(entries, key=lambda item: item[0].as_posix())
    )
    return "\n".join(lines) + "\n"


# Implements: MP-001
def main() -> int:
    ANCHORS.write_text(rendered_anchors(), encoding="utf-8")
    print(f"updated {ANCHORS.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
