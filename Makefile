# =============================================================================
# Quire Contract Codegen Makefile
# =============================================================================
#
# Native orchestration. Every target calls the toolchain that owns the job: cargo
# for the crate, the generation corpus for conformance, quire for static export,
# quoin for evidence. Nothing here computes a verdict, attests to its own
# correctness, or retains evidence of its own.
#
# This file is not a trust root and no longer tries to be one. The parse-time
# guards that used to police Make's own execution controls, and the 334-line
# recipe-failure prover behind them, went with the collector they were protecting.
#
# READ THIS BEFORE TRUSTING A GREEN `make ci`.
#
# `.IGNORE:` added to this file, a `-` prefix on a recipe line, or an assignment
# to SHELL makes every recipe report success without its exit status being
# consulted. Measured on this repository, not assumed: with a rustfmt violation, a
# failing test assertion and an unknown upstream constant all present, the control
# tree exits 2 at fmt-check -- the first of eleven `ci` prerequisites, so the other
# ten never run -- and the same tree with `.IGNORE:` prepended exits 0, runs all
# eleven, and fails seven of them (fmt-check, spec, lint, msrv, upstream-identity,
# test, assurance-chain). Every one printed its diagnostic. None failed the build.
#
# What that does and does not reach. Quoin binds retained inputs by digest and
# scripts/assurance_chain.py derives every attested result from the producer's own
# bytes, so a Makefile that lies about running a producer yields an absent or
# unreadable input and the chain errors rather than passing -- in the measurement
# above the chain did exactly that and returned 1, and Make discarded it. The
# gates that feed nothing into the chain (fmt-check, lint, deny, audit-unsafe,
# rustdoc) are simply neutered.
#
# tests/shared_assurance.rs asserts this file declares no such directive, which
# protects a reviewer reading a diff. It does not make this file's exit code
# trustworthy on a tree where it has been edited, because under `.IGNORE:` that
# test also runs, also fails, and is also swallowed. Recorded rather than closed,
# and tracked as agent-ix/quire-contract-codegen#14.
# =============================================================================

TRUSTED_HOME := $(shell /usr/bin/python3 -c 'import os,pwd; print(pwd.getpwuid(os.getuid()).pw_dir)')
override BASH := /usr/bin/bash
override CARGO := $(TRUSTED_HOME)/.cargo/bin/cargo
override MSRV := 1.75.0
override PYTHON := /usr/bin/python3
override QUIRE := $(TRUSTED_HOME)/.npm-global/bin/quire
override QUOIN := $(TRUSTED_HOME)/.npm-global/bin/quoin


.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test"
	@echo "  make build            - Release build"
	@echo "  make msrv             - Check the crate with Rust $(MSRV)"
	@echo "  make spec             - Quire-validate the specification"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - Run all configured cargo-deny checks"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
	@echo "  make conformance      - Run the bounded generation conformance corpus"
	@echo "  make upstream-identity- Check the IR and runtime revisions agree everywhere"
	@echo "  make assurance-env    - Create the pinned shared-assurance interpreter"
	@echo "  make assurance-inputs - Run the producers and write their structured results"
	@echo "  make pins             - Classify the toolchain through the shared matrix"
	@echo "  make compat-view      - Read retained evidence through the shared mapping"
	@echo "  make assurance-chain  - Seal, retain, and verify through Quoin"
	@echo "  make assurance        - pins + compat-view + assurance-chain"
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

# The traced tests invoke the assurance gates, so the producers must already have
# run. They are a prerequisite rather than something a test creates for itself: a
# test that can produce its own inputs can produce a green run out of nothing.
.PHONY: test
test: assurance-inputs
	$(CARGO) test --locked

.PHONY: build
build:
	$(CARGO) build --locked --release

# Same prerequisite as `test`, and for the same reason plus one. The MSRV lane
# runs the same traced tests, so it needs the same producer output; and because
# `msrv` precedes `test` in the `ci` list, a run without this prerequisite reads
# whatever `target/assurance/` happens to hold from an earlier run. That is not a
# hypothetical: it was observed reading a producer document left behind by a
# deliberately broken measurement run, and reporting `not_computed` about a tree
# that was fine.
.PHONY: msrv
msrv: assurance-inputs
	$(CARGO) +$(MSRV) test --locked

.PHONY: spec
spec:
	$(QUIRE) validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md' \
		'reviews/**/*.md'

.PHONY: clean
clean:
	$(CARGO) clean
	rm -rf $(ASSURANCE_VENV)

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	$(CARGO) deny check

.PHONY: cargo-audit
cargo-audit:
	$(CARGO) audit --ignore RUSTSEC-2026-0009

.PHONY: audit-unsafe
audit-unsafe:
	$(BASH) scripts/check_unsafe_comments.sh

.PHONY: rustdoc
rustdoc:
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc --locked --no-deps

# =============================================================================
# Shared assurance (FR-006)
#
# The shared-assurance lane runs in its own interpreter. engineering-assurance
# declares jsonschema>=4.23 and this repository's Draft 7 lane pins 3.2.0; both
# are right for their own job, so they get one environment each.
# =============================================================================

ASSURANCE_VENV ?= .venv-assurance
ASSURANCE_PYTHON ?= $(ASSURANCE_VENV)/bin/python

ASSURANCE_DIR := target/assurance
CONFORMANCE_RESULT := $(ASSURANCE_DIR)/generation-conformance.jsonl
UPSTREAM_RESULT := $(ASSURANCE_DIR)/upstream-identity.json
QUIRE_EXPORT := $(ASSURANCE_DIR)/quire-static-export.json
COMPAT_RESULT := $(ASSURANCE_DIR)/legacy-compatibility.json
MSRV_RESULT := $(ASSURANCE_DIR)/msrv.jsonl
REVISION ?= $(shell git rev-parse HEAD)

$(ASSURANCE_PYTHON):
	$(PYTHON) -m venv $(ASSURANCE_VENV)
	$(ASSURANCE_VENV)/bin/pip install --quiet --disable-pip-version-check \
		-r requirements-assurance.txt

.PHONY: assurance-env
assurance-env: $(ASSURANCE_PYTHON)

# The only target that runs a producer. Everything downstream consumes these
# files and refuses to create them. Each command below is the exact argv the
# corresponding proof obligation declares in assurance/change-assurance.json; a
# declared command that is not the executed command is a lie in a sealed
# attestation, and tests/shared_assurance.rs asks Make rather than taking this
# comment's word for it.
.PHONY: assurance-inputs
assurance-inputs: assurance-env
	mkdir -p $(ASSURANCE_DIR)
	$(CARGO) run --quiet --example generation_conformance > $(CONFORMANCE_RESULT)
	$(PYTHON) scripts/check_upstream_pins.py --json > $(UPSTREAM_RESULT)
	$(QUIRE) coverage --scope . --json > $(QUIRE_EXPORT)
	$(ASSURANCE_PYTHON) scripts/legacy_evidence_view.py --json > $(COMPAT_RESULT)
	rustup run $(MSRV) $(CARGO) check --locked --all-targets \
		--message-format=json > $(MSRV_RESULT)

.PHONY: conformance
conformance:
	$(CARGO) run --quiet --example generation_conformance

.PHONY: upstream-identity
upstream-identity:
	$(PYTHON) scripts/check_upstream_pins.py

.PHONY: pins
pins: assurance-env
	$(ASSURANCE_PYTHON) scripts/check_shared_pins.py

.PHONY: compat-view
compat-view: assurance-env
	$(ASSURANCE_PYTHON) scripts/legacy_evidence_view.py
	$(ASSURANCE_PYTHON) scripts/legacy_evidence_view.py --mutation-probes

.PHONY: assurance-chain
assurance-chain: assurance-inputs
	$(PYTHON) scripts/assurance_chain.py --candidate-revision $(REVISION)

.PHONY: assurance
assurance: pins compat-view assurance-chain

# An operator target, not a CI gate. It writes into a Quoin evidence store, which
# is a reviewed change rather than something a gate should do on every run.
.PHONY: assurance-record
assurance-record: assurance-inputs
	$(PYTHON) scripts/assurance_chain.py --adapt $(CONFORMANCE_RESULT) \
		> $(ASSURANCE_DIR)/entries.json
	$(QUOIN) evidence record \
		--repo . \
		--suite SUITE-001 \
		--commit $(REVISION) \
		--tool "quire-contract-codegen-generation-conformance 0.1.0" \
		--adapter entries \
		--kind Integration \
		--results $(ASSURANCE_DIR)/entries.json

# =============================================================================
# Composite
# =============================================================================

.NOTPARALLEL: ci
.PHONY: ci
ci: fmt-check spec lint msrv deny audit-unsafe rustdoc upstream-identity conformance test assurance

# This final guard deliberately follows every assignment and include opportunity.
