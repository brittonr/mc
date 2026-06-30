# Proposal: Add Parkour time-trial mode

## Why

Parkour is a popular casual mode with a large server-directory footprint and a comparatively small gameplay kernel. The repository already has a Valence `parkour.rs` example, but a Hyperion-owned Cairn can define the mode without copying Valence example behavior or changing compatibility rails. This gives the project a fast non-PvP win that exercises movement checkpoints, timers, leaderboards, reset handling, and course validation.

A dedicated Cairn also keeps the mode testable as pure route/checkpoint/timing rules with thin Bevy and packet shells.

## What Changes

- Add a Hyperion-owned Parkour event plugin with course registration, lobby/start, checkpoint progression, fall/reset handling, finish detection, timing, leaderboard projection, and cleanup.
- Treat any Valence parkour example as reference-only unless a separate integration Cairn classifies concepts for adoption or porting.
- Keep course validation, checkpoint transitions, timer decisions, fall reset plans, and leaderboard ranking in pure deterministic cores.
- Add positive and negative tests for valid course completion, checkpoint resume, fall reset, leaderboard update, invalid course metadata, skipped checkpoints, stale timers, wrong-mode movement, duplicate finishes, and cleanup leaks.

## Impact

- **Files**: new or extended Hyperion parkour modules under `hyperion/events/`, course metadata fixtures, optional reference notes for Valence example comparison, focused tests, and `docs/evidence/` receipts.
- **Testing**: baseline Hyperion checks before movement/shared state edits when applicable, course validator tests, pure timing/checkpoint tests, Bevy shell/plugin tests, leaderboard fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not port Valence parkour code by default, implement production leaderboards, claim movement vanilla parity, change Bedwars or survival behavior, change Valence behavior, provide public-server safety, or prove broad Minecraft compatibility.
