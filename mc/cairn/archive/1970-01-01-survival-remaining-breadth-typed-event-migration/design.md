# Design: remaining survival breadth typed-event migration

## Scope

This change migrates the six remaining survival breadth rows that share the latest breadth evidence cluster. It does not touch earlier survival rows covered by separate typed-event migrations and does not alter aggregate survival claim gates.

## Functional core

The pure validation core evaluates typed-event graphs from in-memory milestone evidence. This change extends the typed-event gate for the six row families:

- mob AI/loot: spawn, AI checkpoint, attack, death, drop spawn, pickup, inventory, final state
- redstone circuit: initial state, input, powered-on checkpoint, powered-off checkpoint, final state
- biome/dimension travel: origin, transition, destination, normalized environment state
- world multichunk durability: primary/secondary mutations, controlled restart, reconnect, post-restart observations
- container block-entity breadth: open, transfer, payload observation, metadata observation, reopen/persisted payload
- sign editing live: editor open, update submit, server acceptance, post-update text observation

Positive fixtures include complete client and server event graphs for each family. Negative fixtures remove required events and reorder representative phases to verify fail-closed diagnostics.

## Imperative shell

Wrappers, dry-run invocation, receipt paths, and typed-event sidecar writing remain unchanged. The shell reports typed-event contribution status only after pure validators accept the relevant row graphs.

## Validation strategy

- Record baseline runner and manifest checks before edits.
- Update manifest readiness and regenerate generated surfaces.
- Run typed-event focused tests and row-family negative fixtures.
- Run scenario manifest self-test/check/generated-surface checks.
- Run historical survival dry-run coverage and evidence manifest checks.
- Run Cairn gates, task-evidence validation, archive, accepted-spec validation, and post-archive evidence checks.

## Non-claims

Rows remain bounded to their configured fixtures and paired evidence. This migration does not claim full survival compatibility, broad vanilla parity, all mobs, general redstone parity, all dimensions, arbitrary durability, all containers, all sign UI behavior, public-server safety, production readiness, or semantic equivalence.
