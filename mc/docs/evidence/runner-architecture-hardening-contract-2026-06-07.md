# Runner architecture hardening contract (2026-06-07)

## Selected seam

`tools/check_scenario_manifest.rs` validates that `tools/mc-compat-runner/src/scenario_core.rs` exposes the live capability registry tokens required by current fixture-bounded blocker rows. The existing seam is in-memory and deterministic, but diagnostics are produced directly from token scanning. This pass narrows it into a pure registry evaluation core plus a thin diagnostic shell.

## Public output invariants

Must remain unchanged:
- `nix build .#checks.x86_64-linux.mc-compat-scenario-manifest` succeeds for the current repository.
- `tools/check_scenario_manifest.rs --self-test` prints `scenario manifest self-test passed: positive and negative fixtures exercised` on success.
- Repository check success summary remains `scenario manifest check passed: <N> rows validated`.
- Missing live capability tokens continue to fail closed with diagnostics anchored to `tools/mc-compat-runner/src/scenario_core.rs`.

## Test scope

Baseline and parity checks cover:
- Positive registry surface with all expected tokens.
- Negative missing-token surface.
- Negative unknown/malformed registry surface text.
- Existing manifest negative fixtures for duplicate rows, missing alias, missing milestone, invalid dry-run wrapper, and unsupported migration state.

## Non-claims

This architecture hardening pass does not add gameplay, protocol, live evidence, public-server, production-readiness, semantic-equivalence, receipt-schema, scenario-name, milestone, backend, packet-row, or acceptance-matrix coverage. It preserves current scenario/checker semantics only.
