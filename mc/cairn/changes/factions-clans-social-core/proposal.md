# Proposal: Add factions/clans social core

## Why

A factions or clans mode needs a durable social core before territory, raiding, diplomacy, or economy rules can be safe. Players need clear group identity, membership lifecycle, role permissions, invite/kick/leave rules, chat/presence surfaces, and persistence boundaries that can be tested without a live server.

Hyperion already has ECS-driven game-specific event code under `events/bedwars` and reusable command, permission, stats, text, inventory, and scheduling crates. A factions/clans core should follow that Bevy-first event/plugin pattern while keeping social rules mode-local and avoiding changes to Bedwars, default Hyperion behavior, Valence, or compatibility rails.

## What Changes

- Introduce a Hyperion-owned factions/clans social core for clan identity, roster membership, invites, joins, leaves, kicks, transfers, disbands, roles, permissions, display names, tags, descriptions, chat routing, and presence summaries.
- Keep deterministic membership and permission decisions in pure cores with thin Bevy/command/network shells.
- Define persistence snapshots and recovery rules for faction identity, rosters, roles, pending invites, audit events, and corruption handling.
- Require positive and negative tests for valid lifecycle transitions and malformed, duplicate, unauthorized, stale, or persistence-corrupt cases.
- Preserve explicit non-claims around land claims, raiding, economy, diplomacy, public-server moderation, production persistence, and broad gameplay balance unless separate Cairns add them.

## Impact

- **Files**: new or extended Hyperion factions/clans event crate or plugin modules under `hyperion/events/`, possible shared pure-core modules under Hyperion-owned crates only if reusable seams are justified, focused tests, and `docs/evidence/` receipts when tasks are closed.
- **Testing**: baseline Hyperion checks before shared-core edits when applicable, pure social-core tests, Bevy shell/plugin tests, command/permission tests, persistence snapshot fixtures, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not implement territory claims, raids, diplomacy, economy, anti-cheat enforcement, public-server safety, production-scale persistence, Valence behavior, Bedwars behavior, or broad Minecraft compatibility.
