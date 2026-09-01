# =============================================================================
# Quire Contract Codegen Makefile
# =============================================================================

TRUSTED_HOME := $(shell /usr/bin/python3 -c 'import os,pwd; print(pwd.getpwuid(os.getuid()).pw_dir)')
override BASH := /usr/bin/bash
override CARGO := $(TRUSTED_HOME)/.cargo/bin/cargo
override MSRV := 1.75.0
override PYTHON := /usr/bin/python3
override QUIRE := $(TRUSTED_HOME)/.npm-global/bin/quire

ifneq ($(filter ci,$(MAKECMDGOALS)),)
codegen_ci_parse_status := $(shell /usr/bin/env MAKEFLAGS='$(MAKEFLAGS)' /usr/bin/python3 scripts/check_failure_propagation.py --makefile '$(firstword $(MAKEFILE_LIST))' --parse-time >/dev/null 2>&1; echo $$?)
ifneq ($(codegen_ci_parse_status),0)
$(error local CI parse-time integrity guard rejected this invocation)
endif
endif

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
	@echo "  make deny             - cargo deny check licenses"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
	@echo "  make evidence-tool    - Test the foundation evidence toolchain and pins"
	@echo "  make verify-evidence  - Re-verify every authoritative retained evidence record"
	@echo "  make coverage         - Report specification-to-test coverage"
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
	$(CARGO) clippy --all-targets -- -D warnings

.PHONY: test
test:
	$(CARGO) test

.PHONY: build
build:
	$(CARGO) build --release

.PHONY: msrv
msrv:
	$(CARGO) +$(MSRV) check --lib

.PHONY: spec
spec:
	$(QUIRE) validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md'

.PHONY: clean
clean:
	$(CARGO) clean

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	$(CARGO) deny check licenses

.PHONY: cargo-audit
cargo-audit:
	$(CARGO) audit

.PHONY: audit-unsafe
audit-unsafe:
	$(BASH) scripts/check_unsafe_comments.sh

.PHONY: evidence-tool
evidence-tool:
	$(PYTHON) -m py_compile scripts/build_foundation_envelope.py scripts/check_coverage_status.py scripts/check_failure_propagation.py scripts/evidence_policy.py scripts/run_python_tests.py scripts/update_evidence_anchors.py scripts/validate_json_schema.py scripts/verify_foundation_evidence.py
	$(PYTHON) scripts/run_python_tests.py

.PHONY: verify-evidence
verify-evidence:
	$(PYTHON) scripts/verify_foundation_evidence.py

.PHONY: coverage
coverage:
	$(PYTHON) scripts/check_coverage_status.py

.PHONY: rustdoc
rustdoc:
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc --no-deps

# =============================================================================
# Composite
# =============================================================================

.PHONY: ci-guard
ci-guard:
	/usr/bin/python3 scripts/check_failure_propagation.py

.NOTPARALLEL: ci
.PHONY: ci
ci: ci-guard fmt-check spec lint test msrv deny audit-unsafe rustdoc coverage evidence-tool verify-evidence

# This final guard deliberately follows every assignment and include opportunity.
ifneq ($(filter ci,$(MAKECMDGOALS)),)
codegen_ci_final_parse_status := $(shell /usr/bin/env MAKEFLAGS='$(MAKEFLAGS)' /usr/bin/python3 scripts/check_failure_propagation.py --makefile '$(firstword $(MAKEFILE_LIST))' --parse-time >/dev/null 2>&1; echo $$?)
ifneq ($(codegen_ci_final_parse_status),0)
$(error local CI final parse-time integrity guard rejected this invocation)
endif
endif
