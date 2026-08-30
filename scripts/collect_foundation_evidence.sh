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
{
  echo "schema=contract-evidence-envelope-v1"
  echo "phase=foundation"
  echo "tool=quire-contract-codegen/scripts/collect_foundation_evidence.sh"
  echo "input.source_revision=$(git rev-parse HEAD)"
  echo "input.source_state=$source_state"
  echo "input.governance=agent-ix/quire-contract-ir#3-open"
  echo "input.ir_corpus=agent-ix/quire-contract-ir#10-open-no-candidate"
  echo "input.runtime=agent-ix/quire-contract-runtime#5-head-4392a2385f95defdeef2ee883fcc8024cab1d168"
  echo "output.identity=sha256sums.txt"
} >"$evidence_dir/evidence-envelope.txt"

run_and_retain quire-validate quire validate --scope . 'spec/**/*.md'
run_and_retain fmt cargo fmt --all -- --check
run_and_retain clippy cargo clippy --all-targets -- -D warnings
run_and_retain test cargo test
run_and_retain deny cargo deny check licenses
run_and_retain unsafe-audit bash scripts/check_unsafe_comments.sh
run_and_retain metadata cargo metadata --format-version 1
run_and_retain rustdoc env RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps

(
  cd "$evidence_dir"
  find . -maxdepth 1 -type f ! -name sha256sums.txt -print0 \
    | sort -z \
    | xargs -0 sha256sum >sha256sums.txt
)
