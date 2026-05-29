# Proposal: Add oracle property and mutation tests

## Why

The runner has many focused unit tests, but they are hand-selected examples. The scenario oracle should be proven against systematic mutations: every required milestone must matter, every forbidden marker must fail, and receipt validation should reject missing or inconsistent fields. Without this, a new scenario can accidentally pass because one marker is stale, duplicated, or not checked.

## What Changes

- Add deterministic property-style tests over the scenario catalog.
- For every scenario, prove that a complete fixture passes and removing each required client/server milestone fails with the expected missing field.
- For every scenario, prove that inserting each forbidden marker fails.
- Add mutation tests for ordered causality, receipt summary validation, and load/network safety fields.
- Keep the oracle tests in pure Rust functions with no process, network, Docker, Xvfb, or child-repo dependencies.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, optional scenario test fixtures, README/evidence docs.
- **Testing**: positive and negative oracle/property tests, receipt mutation tests, maintained dry-run checks.
- **Non-claims**: these tests harden harness logic only; they do not add live protocol coverage.
