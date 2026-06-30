# Proposal: Add Murder Mystery social-deduction mode

## Why

Murder Mystery is a flagship social minigame on large Minecraft networks, including Hypixel. It adds a distinct non-arena-combat experience: hidden roles, asymmetric information, detective tools, innocent survival, murderer pressure, timed win conditions, and strong UX requirements around secrecy.

A dedicated Cairn keeps role secrecy, victory conditions, item grants, and kill validation mode-local. It also avoids cloning protected server-specific presentation or leaking hidden role information through logs, scoreboards, or diagnostics.

## What Changes

- Add a Hyperion-owned Murder Mystery event plugin with lobby, role assignment, hidden-role visibility, innocents, detective-like investigation role, murderer role, item grants, kill/interact rules, timer, victory conditions, spectator handling, and cleanup.
- Use original presentation and configurable role names/items rather than copying branded names, maps, sounds, cosmetics, or UI from existing servers.
- Keep role assignment, visibility filtering, item eligibility, kill validation, victory decisions, and cleanup plans in pure deterministic cores with thin Bevy/network/command shells.
- Add positive and negative tests for valid role assignment, hidden information, detective item grant, murderer elimination, innocent survival win, invalid kills, role leaks, duplicate role grants, stale players, disconnects, spectators, and cleanup leaks.

## Impact

- **Files**: new or extended Hyperion social-deduction modules under `hyperion/events/`, role/map/item fixtures, visibility tests, focused tests, and `docs/evidence/` receipts.
- **Testing**: baseline Hyperion checks before shared visibility/player-state edits when applicable, pure role/victory tests, Bevy shell/plugin tests, hidden-info leak tests, cleanup fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not implement Hypixel Murder Mystery, copy protected presentation, provide production moderation, guarantee anti-stream-sniping, change Bedwars, change Valence, prove public-server safety, or establish broad Minecraft compatibility.
