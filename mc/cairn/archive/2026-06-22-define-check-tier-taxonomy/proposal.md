# Proposal: Define repository check tiers

## Why

The repository has fast local checks, generated-surface checks, evidence gates, live/manual rails, and archive closeout requirements, but they are scattered across README, flake checks, Cairn tasks, and agent notes. A named check-tier taxonomy would make it clear which checks to run for which kind of change.

## What Changes

- Define check tiers such as fast local, generated freshness, evidence validation, component tests, live/manual evidence, and archive closeout.
- Map existing flake checks/apps and manual commands into those tiers.
- Add documentation and optional wrapper outputs for common tier entrypoints.
- Require Cairn tasks to cite the smallest relevant tier plus any affected component checks.

## Impact

- **Files**: README/docs verification pages, flake check/app wrappers, Cairn task templates/guidance, evidence docs, component AGENTS notes, and Cairn artifacts.
- **Testing**: selected tier wrapper dry-runs, flake evaluation, docs freshness, Cairn validation/gates.
- **Non-claims**: this improves validation ergonomics only; it does not change check semantics, evidence content, or compatibility claims.
