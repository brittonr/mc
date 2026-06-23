# Proposal: Migrate CTF rule rows to typed-event pass/fail

## Why

The CTF rule evidence set has many bounded live rows for scoring, combat, projectiles, reconnect, invalid actions, score limits, races, and spawn/team reset. Most of those rows still advertise `substring-fallback`, leaving a large review surface where legacy text could satisfy pass/fail even when typed client/server milestones are incomplete or misordered.

A CTF typed-event migration wave burns down that fallback cluster in row families while preserving all existing CTF non-claims.

## What Changes

- Partition maintained CTF rows into typed-event migration families: scoring/soak, combat/projectile, reconnect/lifecycle, invalid-action containment, score-limit/race, and spawn/team reset.
- Mark migrated CTF rows as `typed-event-ready` in the scenario manifest and generated surfaces only after row-family gates cover client milestones, Valence server milestones, forbidden surfaces, and ordering.
- Add positive and negative fixtures for missing evidence, wrong actor/victim correlation, wrong flag/team state, and misordered server/client phases.
- Add manifest readiness fixtures and documentation updates for the typed-event-ready scenario set.
- Preserve wrappers, receipt schema, dry-run shapes, current-bundle rows, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, runner typed-event validation, CTF row checkers, README/evidence docs, and Cairn lifecycle files.
- **Testing**: row-family typed-event fixtures, scenario-manifest checks, generated-surface freshness, CTF dry-run wrapper checks, evidence manifest validation, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only validation basis for bounded CTF rows. It does not claim full CTF correctness, all races, all invalid actions, adversarial security, latency tolerance, public-server safety, production readiness, broad Minecraft compatibility, or vanilla/reference parity.
