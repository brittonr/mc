# Proposal: Prototype a Valence/Hyperion bridge slice

## Why

The safest way to prove that Valence and Hyperion can converge is a small, observable bridge slice instead of a broad runtime merge. The first slice should cover only the minimum player-facing loop: player join planning, initial chunk delivery, movement state mapping, and chat or broadcast routing. That proves the adapter seams while keeping default Valence networking and Hyperion runtime behavior unchanged.

This slice depends on the roadmap and type-ownership audit. It should use Valence-owned public APIs as the canonical surface, Hyperion concepts as classified references or explicitly audited sources, and pure adapter cores for bridge decisions.

## What Changes

- Build an optional prototype bridge plugin, example, or fixture harness for player join, initial chunk delivery, movement state mapping, and chat/broadcast routing.
- Implement pure bridge planning cores over explicit summaries for join facts, chunk/view facts, movement facts, chat route requests, and broadcast intents.
- Keep Valence app wiring, Hyperion/proxy interaction, packet writes, ECS mutation, sockets, logging, clocks, and runtime scheduling in thin shells.
- Preserve default direct Valence behavior and avoid claiming full Hyperion compatibility, production scale, vanilla parity, or default behavior changes.
- Add positive and negative tests for valid bridge plans, missing join facts, stale sessions, invalid chunks/dimensions, malformed movement, unauthorized chat routes, closed clients, and plugin-disabled behavior.

## Impact

- **Files**: optional Valence bridge crate/plugin/example or `tools/playground` harness, adapter core modules, focused tests, docs/evidence receipts, and Cairn artifacts.
- **Testing**: baseline direct-mode Valence checks before implementation, adapter-core tests, shell/plugin tests, selected packet/chunk/chat dry runs, optional smoke harness, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests.
- **Non-claims**: this does not merge repositories, replace Valence networking by default, import Hyperion runtime wholesale, claim Bedwars compatibility, claim large-scale production readiness, or prove vanilla gameplay parity.
