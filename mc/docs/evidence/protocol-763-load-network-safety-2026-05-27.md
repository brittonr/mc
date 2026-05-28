# Protocol-763 load/network safety envelope — 2026-05-27

## Scope

This checkpoint drains the production/load/network safety Cairn by adding a fail-closed safety envelope to the runner receipt surface and by keeping production/public/unbounded claims as explicit non-claims.

It does **not** prove public-server safety, production readiness, WAN safety, adversarial-network safety, unbounded soak, unbounded reconnect, packet-loss tolerance, or broad gameplay correctness.

## Authorization policy

A load or network experiment is launchable only when at least one is true:

- target scope is owned local loopback (`owned-local-loopback`), or
- explicit external authorization is present (`MC_COMPAT_EXTERNAL_LOAD_AUTHORIZED=1`).

The runner treats `MC_COMPAT_PUBLIC_TARGET=1` without explicit authorization as a preflight failure before starting traffic. This gives reviewers a deterministic negative fixture for public/unowned targets even though maintained rails default to owned local infrastructure.

## Bounded envelopes

| Field | Current bound | Source |
| --- | --- | --- |
| target scope | `owned-local-loopback` | runner safety envelope |
| max clients | `2` | maintained two-client rails |
| max duration | `600s` | maintained bounded soak cap |
| reconnect sessions | `1` normally, `2` for reconnect rails | scenario envelope |
| latency/jitter/loss | explicit string fields, default `0` | environment/receipt envelope |
| telemetry | required for promotion readiness | client/server receipt evidence |
| live receipt | required for promotion readiness | non-dry-run receipt |

## Runner receipt surface

Receipts now include a `load_network_safety` block with:

- target scope and authorization fields;
- client/duration/reconnect/network bounds;
- missing-field and bound-violation diagnostics;
- `preflight_passed` for launch safety;
- `promotion_ready` for evidence promotion;
- explicit false claims for public-server safety, production readiness, unbounded soak, unbounded reconnect, WAN safety, and adversarial-network safety.

## Fixtures and checks

Positive fixture:

- owned local target;
- planned clients at or below the bound;
- duration at or below the bound;
- telemetry present;
- live receipt present;
- `preflight_passed=true` and `promotion_ready=true`.

Negative fixture:

- public target without authorization;
- planned clients above the bound;
- duration above the bound;
- missing latency field;
- missing telemetry and no live receipt;
- `preflight_passed=false` and `promotion_ready=false` with explicit diagnostics.

## Evidence

- Live bounded receipt: `docs/evidence/protocol-763-load-network-safety-live-2026-05-27.receipt.json`.
  - BLAKE3: `b799ec666ddef41d06d57460b8829879c3815f864c580b8f350216ff9ac70436`.
  - Receipt fields: `status=pass`, `mode=run`, `dry_run=false`, `scenario.passed=true`, `load_network_safety.preflight_passed=true`, `load_network_safety.promotion_ready=true`, `load_network_safety.live_receipt=true`, `load_network_safety.telemetry_present=true`, `load_network_safety.bound_violations=[]`, `load_network_safety.missing_fields=[]`.
- Live run log: `docs/evidence/protocol-763-load-network-safety-live-2026-05-27.run.log`.
  - BLAKE3: `d294d50c139cdb8485b3deb6a2476160a6a3fa6d7a37b8bf6cd84d0b600f28a7`.
- Live BLAKE3 manifest: `docs/evidence/protocol-763-load-network-safety-live-2026-05-27.b3`.
- Runner tests: `nix develop --no-update-lock-file -c cargo test --manifest-path tools/mc-compat-runner/Cargo.toml` (`33 passed`).
- Formatting: `nix develop --no-update-lock-file -c cargo fmt --manifest-path tools/mc-compat-runner/Cargo.toml -- --check`.
- Freshness/checkpoint gate: `docs/evidence/protocol-763-load-network-safety-gate-2026-05-27.run.log`.

## Decision

- Question: Can production/load/network claims be prevented from being promoted without authorization, bounds, telemetry, and live receipts?
- Inspected evidence: runner safety envelope implementation, positive/negative unit fixtures, format/test output, and maintained current bundle non-claims.
- Decision: Yes. Bounded owned-local compatibility receipts can continue, but broader production/public/WAN/unbounded claims remain blocked until a future authorized live envelope produces reviewable telemetry and BLAKE3-backed evidence.
- Decision owner: agent.
- Next action: keep `prove-production-load-network-safety` archived; any future load/network expansion must add a new bounded envelope row and live evidence before changing non-claims.
