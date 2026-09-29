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
override QUIRE := $(TRUSTED_HOME)/.npm-global/bin/quire


.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test"
	@echo "  make build            - Release build"
	@echo "  make kani             - Run the Kani obligation lane serially"
	@echo "  make msrv             - Check the crate with Rust $(MSRV)"
	@echo "  make spec             - Quire-validate the specification"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - Run all configured cargo-deny checks"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
	@echo "  make conformance      - Run the bounded generation conformance corpus"
	@echo "  make rustdoc          - Build warning-free API documentation"
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
	$(CARGO) clippy --locked --all-targets -- -D warnings

.PHONY: test
test:
	$(CARGO) test --locked

# The Kani lane (FR-015, TC-025). Kani and CBMC are memory-heavy, so the lane
# holds a host-wide lock, runs one harness at a time, and builds in its own
# target directory.
#
# Integration tests live in the single `it` binary (`tests/it/main.rs`); the
# `kani_obligations` and `skeleton_spine` filters select those modules' tests,
# and `--ignored` runs only their `#[ignore]`d real-prover tests. The skeleton
# spine runs one Boolean clause through the real prover and native replay
# through QSL. Both filters follow `--` because libtest accepts several; cargo's
# own positional filter takes one.
.PHONY: kani
kani:
	flock /tmp/agent-e-heavy-build.lock $(CARGO) +$(MSRV) test --locked -j 4 \
		--test it --target-dir target-codex-backends \
		-- --ignored --test-threads=1 kani_obligations skeleton_spine

.PHONY: build
build:
	$(CARGO) build --locked --release

.PHONY: msrv
msrv:
	$(CARGO) +$(MSRV) test --locked

.PHONY: spec
spec:
	$(QUIRE) validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md' \
		'reviews/**/*.md'

.PHONY: clean
clean:
	$(CARGO) clean

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	$(CARGO) deny check

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
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc --locked --no-deps

# =============================================================================
# Generation conformance
# =============================================================================

.PHONY: conformance
conformance:
	$(CARGO) run --quiet --example generation_conformance

# =============================================================================
# Composite
# =============================================================================

.NOTPARALLEL: ci
.PHONY: ci
ci: fmt-check spec lint msrv deny audit-unsafe rustdoc conformance test
