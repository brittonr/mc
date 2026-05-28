# Proposal: Valence combat-loop Steel arrow policy

## Summary

Extend the Steel runtime-configuration work from the compatibility runner evidence rail into the Valence CTF server combat loop. The projectile-probe arrow damage used by live Valence combat must come from a Rust-validated Steel policy snapshot after an atomic publish succeeds, with bad reloads preserving the previous decision path.

## Motivation

The archived runtime-configuration slice intentionally stopped short of Valence/server combat-loop integration. It proved Steel module loading, typed boundaries, apply plans, and runner-side projectile evidence, but active Valence gameplay still uses fixed Rust constants for projectile-probe damage. That leaves the representative gameplay policy split: the runner can predict a Steel-derived damage value, while the server path cannot yet consume it.

This change closes that gap without broadening the claim to all combat rules. Arrow damage remains the representative policy because it is already isolated by the compatibility projectile probe and has reviewable milestone logs.

## Scope

- Add a Valence-side Steel arrow-damage policy adapter with a pure Rust validation core and a thin runtime shell.
- Publish policy snapshots atomically before live combat can observe them.
- Replace the Valence CTF projectile-probe damage call sites with the published policy decision, while retaining the current constant as the default fallback before any operator override is accepted.
- Support explicit reload requests for the server policy path; no filesystem watcher is required in this change.
- Record policy provenance and redacted reload diagnostics in Valence milestone/evidence logs.
- Add positive and negative tests for default policy, edited policy, invalid policy, reload rollback, and both projectile-probe call sites.
- Add evidence that protocol-763 Valence projectile combat uses the Steel-published policy in live server code.

## Out of scope

- Generalizing every Valence combat rule to Steel.
- Remote config distribution or config UI.
- Allowing Steel direct access to Bevy/Valence world state, filesystem, network, wall clock, randomness, or process APIs.
- Changing vanilla parity claims; this remains a bounded compatibility/probe path.
- Replacing the previously archived runner-only Steel evidence slice.

## Impact

- **Files**: Valence CTF example combat code, Steel policy adapter/core, runtime config evidence/checker updates, tests, and protocol-763 evidence receipts.
- **Testing**: pure policy-core tests, Valence example tests or compile checks, negative reload tests, checker validation, Cairn gates, and a reviewable live or dry-run-plus-live-server receipt under `docs/evidence/`.
