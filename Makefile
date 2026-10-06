# =============================================================================
# Quire Contract Codegen Makefile
# =============================================================================
#
# Native orchestration: every target calls the toolchain that owns the job.
# =============================================================================

TRUSTED_HOME := $(shell /usr/bin/python3 -c 'import os,pwd; print(pwd.getpwuid(os.getuid()).pw_dir)')
override BASH := /usr/bin/bash
override CARGO := $(TRUSTED_HOME)/.cargo/bin/cargo
override MSRV := 1.98.1
QUIRE ?= quire
# --locked only when no local patch is active: a patch rewrites the resolution.
LOCKED ?= $(if $(wildcard .cargo/config.toml),,--locked)


.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test"
	@echo "  make build            - Release build"
	@echo "  make kani             - Run the Kani obligation lane serially"
	@echo "  make tools            - Install the host tools the tests drive (llvm-tools, cargo-llvm-cov, Kani)"
	@echo "  make msrv             - Check the crate with Rust $(MSRV)"
	@echo "  make spec             - Quire-validate the specification"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - Run all configured cargo-deny checks"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
	@echo "  make rustdoc          - Build warning-free API documentation"
	@echo "  make use-local        - Patch first-party git deps to sibling checkouts (.cargo/config.toml)"
	@echo "  make use-remote       - Remove the local patch file and restore Cargo.lock; build from GitHub"
	@echo "  make ci               - All local CI gates"

# =============================================================================
# Format / Lint / Test
# =============================================================================

.PHONY: fmt
fmt:
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check:
	$(CARGO) fmt --all -- --check

.PHONY: lint
lint:
	$(CARGO) clippy $(LOCKED) --all-targets -- -D warnings

.PHONY: test
test:
	$(CARGO) test $(LOCKED)

# The Kani lane (FR-015, TC-025). Kani and CBMC are memory-heavy, so the lane
# holds a host-wide lock, runs one harness at a time, and builds in its own
# target directory.
#
# Integration tests live in the single `it` binary (`tests/it/main.rs`); the
# `kani_obligations`, `skeleton_spine`, `kani_witness_join`,
# `bounded_kani_corpus`, `kani_generation` and `kani_batching` filters select those modules'
# tests, and `--ignored` runs only their `#[ignore]`d real-prover tests: the
# real-Kani obligation runs, the skeleton spine (one Boolean clause through the
# real prover and native replay through QSL), the witness join (a real
# counterexample replayed natively), the bounded corpus cases, the
# generation adapter proofs and the harness batching runs (the launcher process
# count before and after, and a real mixed batch) run through `cargo kani`. The filters
# follow `--` because libtest accepts several; cargo's own positional filter
# takes one.
# The test suite drives real tools: the native-coverage tests run `cargo +$(MSRV)
# llvm-cov` and read llvm-cov/llvm-profdata from that toolchain's sysroot, and the Kani
# tests run `cargo kani`. Install them once per machine.
.PHONY: tools
tools:
	rustup component add llvm-tools-preview --toolchain $(MSRV)
	$(CARGO) install --locked cargo-llvm-cov
	$(CARGO) install --locked kani-verifier
	$(CARGO) kani setup

.PHONY: kani
kani:
	flock /tmp/agent-e-heavy-build.lock $(CARGO) +$(MSRV) test $(LOCKED) -j 4 \
		--test it --target-dir target-codex-backends \
		-- --ignored --test-threads=1 kani_obligations skeleton_spine kani_witness_join bounded_kani_corpus kani_generation kani_batching

.PHONY: build
build:
	$(CARGO) build $(LOCKED) --release

.PHONY: msrv
msrv:
	$(CARGO) +$(MSRV) test $(LOCKED)

.PHONY: spec
spec:
	$(QUIRE) validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'

.PHONY: clean
clean:
	$(CARGO) clean

# =============================================================================
# Supply chain & safety
# =============================================================================

# One copy of every agent-ix git crate in Cargo.lock; see the header of
# scripts/check_one_copy.awk.
.PHONY: deny
deny:
	$(CARGO) deny check
	awk -f scripts/check_one_copy.awk Cargo.lock

.PHONY: cargo-audit
cargo-audit:
	$(CARGO) audit --ignore RUSTSEC-2026-0009

# Self-test first (#119): scripts/check_unsafe_comments.sh had no coverage of
# its own, and a prior draft of it silently stopped flagging real unsafe
# blocks in the exact escape-adjacent shapes this exercises, with
# `audit-unsafe` staying green throughout (agent-ix/quire-contract-codegen#118).
# The fixture corpus lives outside every root the scanner scans, so it never
# makes this target red on the repository's own code.
.PHONY: audit-unsafe-selftest
audit-unsafe-selftest:
	$(BASH) scripts/test_check_unsafe_comments.sh

.PHONY: audit-unsafe
audit-unsafe: audit-unsafe-selftest
	$(BASH) scripts/check_unsafe_comments.sh

.PHONY: rustdoc
rustdoc:
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc $(LOCKED) --no-deps --document-private-items

# =============================================================================
# Local development against sibling checkouts
#
# `use-local` writes a gitignored .cargo/config.toml that patches each
# first-party git dependency to its working tree at $(SIBLINGS)/<repo>, uncommitted
# edits included. `use-local` first snapshots Cargo.lock to the gitignored
# .cargo/Cargo.lock.pre-local; `use-remote` deletes the config and restores the
# lock from that snapshot (and does nothing to the lock if there is none). So the
# lock returns to its state before the first `use-local`; lock changes made while
# a patch is active are discarded, and a `cargo update -p` made without a patch is kept.
# Format: <repo>:<crate>:<crate-dir>; entries are grouped by repo here, in any
# order, so each repo gets exactly one [patch] table.
# SIBLINGS is the directory holding the sibling clones: the parent of the main
# checkout, so it is also right from a linked worktree. Override to relocate.
# =============================================================================

SIBLINGS ?= $(abspath $(shell git rev-parse --path-format=absolute --git-common-dir)/../..)
LOCAL_PATCHES ?= quire-contract-ir:quire-contract-ir:. quire-contract-ir:quire-contract-model:crates/quire-contract-model \
	quire-contract-runtime:quire-contract-runtime:. \
	quire-verification-contracts:quire-verification-contracts:. \
	quire-spec-language:qsl-attrs:qsl-attrs quire-spec-language:qsl-cst:qsl-cst quire-spec-language:qsl-eval:qsl-eval \
	quire-spec-language:qsl-forms:qsl-forms quire-spec-language:qsl-foundation:qsl-foundation \
	quire-spec-language:qsl-package:qsl-package quire-spec-language:qsl-replay:qsl-replay \
	quire-spec-language:qsl-semantics:qsl-semantics \
	quire-exact:quire-exact:. quire-semantic-value:quire-semantic-value:.

.PHONY: use-local
use-local:
	@set -e; mkdir -p .cargo; \
	for spec in $(LOCAL_PATCHES); do \
	  if [ "$$(printf '%s' "$$spec" | tr -cd ':' | wc -c)" != 2 ] || printf '%s' "$$spec" | grep -q '::\|^:\|:$$'; then \
	    echo "use-local: malformed LOCAL_PATCHES entry '$$spec' (want repo:crate:dir)" >&2; exit 1; \
	  fi; \
	  repo=$${spec%%:*}; rest=$${spec#*:}; dir=$${rest#*:}; \
	  if [ ! -f "$(SIBLINGS)/$$repo/$$dir/Cargo.toml" ]; then \
	    echo "use-local: $(SIBLINGS)/$$repo is not cloned (no Cargo.toml at $(SIBLINGS)/$$repo/$$dir); clone agent-ix/$$repo next to this repo" >&2; exit 1; \
	  fi; \
	done; \
	[ -f .cargo/Cargo.lock.pre-local ] || cp Cargo.lock .cargo/Cargo.lock.pre-local; \
	: > .cargo/config.toml; \
	repos=$$(for spec in $(LOCAL_PATCHES); do printf '%s\n' "$${spec%%:*}"; done | awk '!seen[$$0]++'); \
	first=1; \
	for repo in $$repos; do \
	  [ "$$first" = 1 ] || printf '\n' >> .cargo/config.toml; first=0; \
	  printf '[patch."https://github.com/agent-ix/%s"]\n' "$$repo" >> .cargo/config.toml; \
	  for spec in $(LOCAL_PATCHES); do \
	    [ "$${spec%%:*}" = "$$repo" ] || continue; \
	    rest=$${spec#*:}; crate=$${rest%%:*}; dir=$${rest#*:}; \
	    printf '%s = { path = "%s/%s/%s" }\n' "$$crate" "$(SIBLINGS)" "$$repo" "$$dir" >> .cargo/config.toml; \
	  done; \
	done; echo "wrote .cargo/config.toml"; \
	meta=$$(mktemp); \
	if ! $(CARGO) metadata --format-version 1 >/dev/null 2>"$$meta"; then \
	  cat "$$meta" >&2; rm -f "$$meta" .cargo/config.toml; \
	  [ ! -f .cargo/Cargo.lock.pre-local ] || { cp .cargo/Cargo.lock.pre-local Cargo.lock; rm -f .cargo/Cargo.lock.pre-local; }; \
	  echo "use-local: cargo metadata failed under the patch" >&2; exit 1; \
	fi; \
	if grep -q 'patch .* was not used' "$$meta"; then \
	  cat "$$meta" >&2; rm -f "$$meta" .cargo/config.toml; \
	  [ ! -f .cargo/Cargo.lock.pre-local ] || { cp .cargo/Cargo.lock.pre-local Cargo.lock; rm -f .cargo/Cargo.lock.pre-local; }; \
	  echo "use-local: a patch was not used; the sibling's version does not satisfy the requirement" >&2; exit 1; \
	fi; \
	rm -f "$$meta"

.PHONY: use-remote
use-remote:
	rm -f .cargo/config.toml
	@if [ -f .cargo/Cargo.lock.pre-local ]; then mv .cargo/Cargo.lock.pre-local Cargo.lock; echo "restored Cargo.lock from the pre-local snapshot"; fi

# =============================================================================
# Composite
# =============================================================================

.NOTPARALLEL: ci
.PHONY: ci
ci: fmt-check spec lint msrv deny audit-unsafe rustdoc test
