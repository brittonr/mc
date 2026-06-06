# Proposal: Introduce checker framework traits

## Why

Many `tools/check_*.rs` programs repeat the same shape: parse args, handle `--self-test`, read repo evidence, parse key/value records, require exact fields, reject overclaims, run positive and negative fixtures, and print diagnostics. The duplication makes new evidence gates inconsistent and increases the risk that a checker omits negative tests or broad-claim rejection.

## What Changes

- Add a small shared checker framework for standalone evidence tools: a `Checker` trait, `KeyValueEvidence` access, common diagnostics, token requirements, and positive/negative fixture runners.
- Migrate a narrow exemplar pair of checkers first, preserving CLI output and failure semantics.
- Keep checker-specific validation rules in pure per-checker cores and keep filesystem/stdout handling in thin shells.
- Add positive and negative framework tests plus per-checker parity fixtures.

## Impact

- **Files**: shared Rust checker support under `tools/` or a small tools crate/module, plus selected `tools/check_*.rs` migrations.
- **Testing**: framework unit tests, migrated checker self-tests, negative malformed-record/overclaim fixtures, task-evidence checks, and Cairn validation/gates.