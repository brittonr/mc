# Proposal: Add Build Battle creative contest mode

## Why

Build Battle and creative contest modes broaden the roadmap beyond PvP. Hypixel lists Build Battle as a first-class casual game, and public server directories show a large Creative category. The mode is implementation-friendly because it centers on plot allocation, theme selection, phase timing, build permissions, voting, scoring, and cleanup rather than complex combat or persistent survival simulation.

A dedicated Cairn keeps contest permissions and voting fairness explicit and prevents creative-mode privileges from leaking into other modes.

## What Changes

- Add a Hyperion-owned Build Battle/creative contest plugin with lobby, theme selection, plot allocation, build phase, voting phase, score aggregation, winner presentation, and plot cleanup.
- Keep theme selection, plot assignment, build permission decisions, phase transitions, vote validation, score aggregation, and cleanup plans in pure deterministic cores with thin Bevy/world/command shells.
- Define named configuration for plot templates, theme pools, phase durations, allowed commands/items, vote scale, party/team entry, and cleanup ownership.
- Add positive and negative tests for valid contests, plot isolation, build permissions, vote counting, ties, cleanup, invalid themes, out-of-plot edits, duplicate votes, self-voting policy, disconnected players, stale phases, and creative privilege leaks.

## Impact

- **Files**: new or extended Hyperion creative-contest modules under `hyperion/events/`, plot/theme fixtures, vote/score fixtures, focused tests, and `docs/evidence/` receipts.
- **Testing**: baseline Hyperion checks before shared world/permission edits when applicable, pure contest/vote tests, Bevy shell/plugin tests, plot cleanup fixtures, permission tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not implement Hypixel Build Battle, production moderation, mature content filtering, WorldEdit parity, persistent creative plots, Valence behavior, Bedwars behavior, public-server safety, or broad Minecraft compatibility.
