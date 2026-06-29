# Runner config source map baseline

Captured before refactoring `compat/runner/src/main.rs` parsing behavior for Cairn change `introduce-runner-config-patches`.

## Current source order

1. Root selection: `MC_COMPAT_ROOT`, then `ROOT`, then current directory.
2. Defaults from the resolved repo layout and hard-coded runner constants.
3. Selected Nickel-exported JSON config from `MC_COMPAT_CONFIG` or the last CLI `--config`/`--config=...` path, applied before environment overrides.
4. Selected restricted Steel config from `MC_COMPAT_STEEL_CONFIG` or the last CLI `--steel-config`/`--steel-config=...` path, applied before environment overrides.
5. Environment overrides: client/server paths and revisions, backend, target, server identity, protocol/port, client identity/timeout/success patterns, scenario, status expectations, packet-capture flag, proxy settings, receipt paths, failure bundle, and safety booleans.
6. CLI arguments in order. CLI `--config` and `--steel-config` are applied again at their argument position, preserving the current high-precedence behavior of explicit CLI config flags.
7. Final server port normalization: if no source explicitly set `server_port`, the port defaults from the final selected backend.
8. Cross-field mode check: `--run-matrix` rejects a single receipt path.

## Default values

- Backend: Valence.
- Server version/protocol/port: `1.18.2` / `758` / Valence default port `25565`.
- Valence revision/example: `8ad9c85` / `terrain`.
- Client username/timeout: `compatbot` / `20s`.
- Mode/scenario: dry-run / smoke.
- Success patterns: detected protocol, dimension type, received chat message.
- Receipt, receipt-dir, failure bundle, compare receipts, config paths, proxy settings, status expectations, and Paper plugin jar default to unset.

## Baseline evidence

Focused pre-change config tests passed in:

- `docs/evidence/run-logs/2026-06-28/introduce-runner-config-patches.baseline-config-tests.run.log`

The log contains `exit_status=0` and covers default resolution, CLI/env/file precedence, Nickel-exported JSON, restricted Steel config, mode parsing, receipt paths, backend/scenario selection, invalid backend rejection, missing client diagnostics, and run-matrix receipt conflict rejection.

## Non-claims

This baseline is configuration architecture evidence only. It does not claim broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, or full CTF/survival correctness.
