# Proposal: Flag-carrier death and flag-return compatibility rail

## Why

Scoring, inventory/open-container, and combat/damage are already proven, but CTF correctness still lacks the fragile edge where a flag carrier dies before capture. This slice turns that edge into a maintained receipt instead of another saturated score soak.

## What Changes

- Add a bounded Valence CTF scenario where one client carries a flag and an opposing client kills or damages the carrier enough to force death/return semantics.
- Instrument Valence and Stevenarella milestones for carrier identity, held flag, combat/death observation, flag reset/return, respawn readiness, and absence of accidental score.
- Add a maintained runner scenario, flake dry-run check, live evidence doc, receipt BLAKE3, and log-hygiene checks.

## Impact

- **Files**: `valence/examples/ctf.rs`, `stevenarella/src/server/mod.rs`, protocol mappings if exposed, `tools/mc-compat-runner/src/main.rs`, `flake.nix`, `README.md`, and `docs/evidence/`.
- **Testing**: runner tests, Valence `cargo +nightly check --example ctf`, Stevenarella `cargo check`, dry-run Nix check, one live two-client receipt.
