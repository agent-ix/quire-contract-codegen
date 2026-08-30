# =============================================================================
# Quire Contract Codegen Makefile
# =============================================================================

CARGO ?= cargo

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test"
	@echo "  make build            - Release build"
	@echo "  make spec             - Quire-validate the specification"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - cargo deny check licenses"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
	@echo "  make evidence-tool    - Syntax-check the PGM-01 evidence builder"
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

.PHONY: spec
spec:
	quire validate --scope . 'spec/**/*.md' 'planning/**/*.md'

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
	bash scripts/check_unsafe_comments.sh

.PHONY: evidence-tool
evidence-tool:
	python3 -m py_compile scripts/build_foundation_envelope.py scripts/validate_json_schema.py

# =============================================================================
# Composite
# =============================================================================

.PHONY: ci
ci: fmt-check spec lint test deny audit-unsafe evidence-tool
