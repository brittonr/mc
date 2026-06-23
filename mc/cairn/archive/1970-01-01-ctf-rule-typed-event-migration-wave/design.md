# Design: CTF rule typed-event migration wave

## Scope

This wave migrates maintained CTF rows from substring fallback to typed-event pass/fail in row families. It does not create new gameplay evidence, alter row scopes, or weaken full CTF correctness gates.

## Functional core

The pure validation core evaluates typed-event graphs from in-memory client/server milestone evidence. CTF validators are organized by family:

- scoring/soak: flag pickup, route/capture, score emission, no duplicate score, RED/BLUE coverage where present
- combat/projectile: attacker/victim identity, attack or projectile use, server damage or attribution, client health/velocity postcondition
- reconnect/lifecycle: pre-disconnect state, disconnect/reconnect, restored state, death/respawn phase ordering where applicable
- invalid-action containment: attempted invalid action, rejection event, unchanged flag/score/ownership state, forbidden mutation absence
- score-limit/race: pre-state, accepted transition, rejected duplicate or terminal transition, final state, no duplicate win/score
- spawn/team reset: assignments, balanced counts, resource reset, no imbalance or stale-resource mutation

Positive fixtures include complete client/server event graphs per family. Negative fixtures cover missing events, wrong actor or victim correlation, wrong flag/team state, and misordered server/client phases.

## Imperative shell

Wrappers, dry-run invocation, receipt paths, and typed-event sidecar writing remain unchanged. The shell maps row receipt data into typed event graphs and calls pure family validators.

## Validation strategy

- Record baseline runner and manifest checks before edits.
- Migrate row families in small commits with generated surfaces refreshed after each family.
- Run row-family typed-event positive and negative fixtures.
- Run scenario manifest self-test/check/generated-surface checks.
- Run focused CTF dry-run wrapper checks and evidence manifests.
- Run Cairn gates, task-evidence validation, archive, accepted-spec validation, and post-archive evidence checks.

## Non-claims

Rows remain bounded to their existing CTF evidence. This migration does not claim full CTF correctness, all concurrency, all invalid actions, adversarial security, latency tolerance, public-server safety, production readiness, broad Minecraft compatibility, or vanilla/reference parity.
