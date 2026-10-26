See CLAUDE.md.

## DO NOT RUN the installed IR-917 V2 clause Kani proof

`tc_025_real_kani_falsifies_both_same_clause_addition_mutants` exhausts the fixed
16 GiB Kani ceiling and returns `Inconclusive MemoryExhausted`. It proves nothing
and blocked feature work for a week. The IR planner reports an operator hold
until all feature work is done. `make kani` and `make kani-gate` explicitly skip
it and print `skipped-by-order`; do not force it manually. FR-015-AC-41/119 and
TC-025 remain Planned. The test refuses before CBMC unless a maintainer sets
its explicit risk opt-in variable for a later authorized run.

## Hash / digest / pin antipattern: do not introduce

Hashes, digests, SHAs, pins, checksum catalogs and records that track files, versions or
tools are an antipattern and have been removed from this repository. Do not introduce
any new use of them. If you find one, remove it as part of the change. The only hash
that stays is a canonical identity digest that binds a proof to the exact content it
proved. Package versions live in Cargo.toml / package.json and their lockfiles only;
reports name the app version they ran.
