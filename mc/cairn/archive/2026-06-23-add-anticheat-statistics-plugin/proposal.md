# Proposal: Add an optional anti-cheat statistics plugin

## Why

Hyperion's statistics crate targets real-time anti-cheat measurements such as packet cadence, motion deltas, click timing, and rotation changes. Valence could benefit from the same concept as an optional plugin, but it should be stable, privacy-conscious, and clearly separated from enforcement policy.

## What Changes

- Audit Hyperion's statistics crate and Valence event surfaces that could feed measurements.
- Define a stable statistics core for rolling/counting metrics without importing nightly-only SIMD code directly.
- Add an optional Valence plugin that samples explicit event streams and emits observations or scores without default enforcement.
- Add positive and negative fixtures for normal movement, burst packets, invalid sample windows, empty samples, overflow boundaries, and disabled-plugin behavior.
- Document limitations, false-positive risks, data retention, and non-claims.

## Impact

- **Files**: optional Valence statistics/anti-cheat crate or plugin, event adapters, tests, docs, and Cairn artifacts.
- **Testing**: pure statistics tests, bad-input fixtures, plugin-off regressions, sampling smoke tests, and Cairn gates/validation.
- **Non-claims**: this does not provide a complete anti-cheat, does not ban players by default, and does not claim vanilla parity.
