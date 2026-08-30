#!/usr/bin/env bash
set -euo pipefail

evidence_dir="${1:-evidence/foundation-candidate}"
if git diff --quiet && git diff --cached --quiet; then
  source_state=clean
else
  source_state=modified
fi
mkdir -p "$evidence_dir"

run_and_retain() {
  local name="$1"
  shift
  "$@" >"$evidence_dir/$name.stdout" 2>"$evidence_dir/$name.stderr"
}

git rev-parse HEAD >"$evidence_dir/source-revision.txt"
echo "$source_state" >"$evidence_dir/source-state.txt"
rustc --version --verbose >"$evidence_dir/rustc-version.txt"
cargo --version --verbose >"$evidence_dir/cargo-version.txt"
quire provenance --pretty >"$evidence_dir/quire-provenance.json"
run_and_retain quire-validate quire validate --scope . 'spec/**/*.md'
run_and_retain fmt cargo fmt --all -- --check
run_and_retain clippy cargo clippy --all-targets -- -D warnings
run_and_retain test cargo test
run_and_retain deny cargo deny check licenses
run_and_retain unsafe-audit bash scripts/check_unsafe_comments.sh
run_and_retain metadata cargo metadata --format-version 1
run_and_retain rustdoc env RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps

python3 scripts/build_foundation_envelope.py "$evidence_dir"

if [[ -n "${PGM01_VALIDATOR:-}" ]]; then
  run_and_retain pgm01-envelope \
    python3 "$PGM01_VALIDATOR" --fixture "$evidence_dir/evidence-envelope.json"
  echo passed >"$evidence_dir/pgm01-envelope-status.txt"
else
  echo skipped-unavailable >"$evidence_dir/pgm01-envelope-status.txt"
fi

(
  cd "$evidence_dir"
  find . -maxdepth 1 -type f ! -name sha256sums.txt -print0 \
    | sort -z \
    | xargs -0 sha256sum >sha256sums.txt
)
