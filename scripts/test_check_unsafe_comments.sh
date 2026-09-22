#!/usr/bin/env bash
# Regression test for scripts/check_unsafe_comments.sh (the unsafe-audit gate,
# `make audit-unsafe`). The gate itself had no test coverage: its behaviour
# was verified only by mutations run by hand at review time (evidence in old
# PR bodies, not in the repo). That already bit once -- in #118 the first
# draft of a scanner fix silently stopped flagging real, uncommented `unsafe`
# blocks that followed a char literal, an escaped quote, a raw string, or a
# multi-line string, and `make audit-unsafe` stayed green throughout.
# Implements: NFR-002. Closes: #119.
#
# This harness runs the scanner against a fixture corpus that lives outside
# every root it scans (src/tests/benches/examples at the repo root), so the
# fixture's deliberately uncommented `unsafe` blocks never make the real
# `make audit-unsafe` red. See scripts/fixtures/unsafe_audit/README.md.
#
# Usage: scripts/test_check_unsafe_comments.sh [path-to-scanner]
# The scanner path defaults to scripts/check_unsafe_comments.sh (resolved
# from the repository root). Passing an alternate path lets this same
# fixture and expectation run against a mutated copy of the scanner, which is
# how this harness's own ability to fail is demonstrated and verified.
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
fixture_dir="$repo_root/scripts/fixtures/unsafe_audit"
expected_file="$fixture_dir/expected_missing.txt"
scanner=${1:-"$repo_root/scripts/check_unsafe_comments.sh"}

case "$scanner" in
  /*) : ;;
  *) scanner="$repo_root/$scanner" ;;
esac

if [[ ! -f "$scanner" ]]; then
  echo "unsafe-audit selftest: no such scanner: ${scanner}" >&2
  exit 2
fi

tmp_stderr=$(mktemp)
trap 'rm -f "$tmp_stderr"' EXIT

# An empty scripts/unsafe_comment_baseline.txt lives under the fixture dir so
# the scanner takes its normal "baseline exists" path and prints every
# flagged line to stderr as "missing SAFETY comment near <file>:<line>",
# rather than the separate "missing baseline file" error it emits when no
# baseline is present at all.
set +e
(cd "$fixture_dir" && bash "$scanner") 2>"$tmp_stderr" 1>/dev/null
status=$?
set -e

if (( status != 0 && status != 1 )); then
  echo "unsafe-audit selftest: scanner exited ${status} (expected 0 or 1)" >&2
  echo "--- scanner stderr ---" >&2
  cat "$tmp_stderr" >&2
  exit 1
fi

actual=$(sed -n 's/^missing SAFETY comment near //p' "$tmp_stderr" | sort -u)
expected=$(sort -u "$expected_file")

if [[ "$actual" != "$expected" ]]; then
  echo "unsafe-audit selftest FAILED against ${scanner}" >&2
  echo "--- expected flagged lines ---" >&2
  echo "$expected" >&2
  echo "--- actual flagged lines ---" >&2
  echo "$actual" >&2
  echo "--- diff (expected vs actual) ---" >&2
  diff <(echo "$expected") <(echo "$actual") >&2 || true
  exit 1
fi

flagged_count=$(printf '%s\n' "$expected" | grep -c .)
echo "unsafe-audit selftest passed against ${scanner}: ${flagged_count} fixture lines flagged exactly as expected"
