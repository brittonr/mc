# Proposal: Add SkyBlock and OneBlock island mode

## Why

SkyBlock is one of the largest persistent Minecraft server categories, and OneBlock is a compact variant with a clear progression loop. Public server directories show thousands of SkyBlock-tagged servers and hundreds of OneBlock-tagged servers. The mode complements the current survival compatibility work by focusing on isolated islands, controlled block generation, persistence, quests, economy hooks, and safe reset boundaries rather than broad vanilla world simulation.

A dedicated Cairn is needed because persistent island ownership and generator progression have different risks than arena modes: storage integrity, grief boundaries, visitor permissions, chunk ownership, and recovery from corrupt snapshots.

## What Changes

- Add a Hyperion-owned island-mode plugin supporting SkyBlock-style private islands and a configurable OneBlock generator profile under one bounded island-mode contract.
- Define island lifecycle, spawn/home, ownership, membership, visitor permissions, generator progression, starter inventory, void/fall recovery, reset/delete, and snapshot persistence boundaries.
- Keep island allocation, permission decisions, generator outputs, progression transitions, quest/reward decisions, and snapshot validation in pure deterministic cores with thin Bevy/world/storage shells.
- Add positive and negative tests for island create/join/visit, generator progression, block placement/breaking, snapshot restore, reset cleanup, invalid generator state, unauthorized edits, duplicate ownership, stale members, corrupt snapshots, and cross-island leaks.

## Impact

- **Files**: new or extended Hyperion island-mode modules under `hyperion/events/`, island/generator metadata fixtures, storage snapshot fixtures, focused tests, and `docs/evidence/` receipts.
- **Testing**: baseline Hyperion checks before shared world/storage edits when applicable, pure island/generator tests, Bevy shell/plugin tests, persistence recovery fixtures, permission tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not implement a production economy, marketplace, anti-grief guarantee, full SkyBlock network, full OneBlock content catalog, vanilla survival parity, Valence behavior, public-server safety, or broad Minecraft compatibility.
