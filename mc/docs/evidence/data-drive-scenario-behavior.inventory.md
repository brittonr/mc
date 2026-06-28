# data-drive-scenario-behavior inventory

## Question
Which scenario behavior facts were split across runner surfaces before the metadata slice?

## Inspected evidence
- `compat/runner/src/scenario_core.rs`: canonical `SCENARIO_SPECS`, behavior kinds, static validation, live-capability non-claim registries.
- `compat/runner/src/main.rs`: run strategy, client/server env wiring, reconnect/session hooks, negative-live-rail hooks, MCP/projectile/combat selectors.
- `compat/runner/src/evidence_core.rs`: typed-event pass/fail selector, required MCP events, typed-event graph edges, evidence evaluators.
- `compat/runner/src/planning.rs`: route planning, log-path strategy, session-count derivation, route non-claims.

## Decision
Owner subtree is `compat/runner/`. The change keeps specialized shell hooks for command/env side effects, but adds a pure metadata core for deterministic behavior facts: run strategy, env-intent IDs, typed-event graph edges, evidence selectors, non-claims, and named handler hooks. The public typed-event edge consumer now resolves edges from that metadata surface; the runner shell delegates shared selector/run-strategy logic to pure core helpers.

## Requirement mapping
- `r[mc_compatibility.runner_modularity.scenario_metadata]`: `ScenarioBehaviorMetadata` exposes deterministic scenario facts and `validate_scenario_behavior_metadata` rejects malformed rows.
- `r[mc_compatibility.runner_modularity.scenario_extension_path]`: each scenario has a bounded catalog row plus named handler metadata; representative consumers query metadata instead of recovering facts with independent selector matches.
- `r[mc_compatibility.runner_modularity.scenario_metadata_positive_tests]`: representative single-client, reconnect, multi-client, projectile, inventory, survival, CTF, and MCP metadata fixtures live in `scenario_core_colocated_tests.rs`.
- `r[mc_compatibility.runner_modularity.scenario_metadata_negative_tests]`: negative metadata fixtures cover missing run strategy, unknown env intent, invalid graph edge, duplicate alias, and unsupported handler.
- `r[mc_compatibility.runner_modularity.scenario_metadata_validation]`: validation logs are promoted under `docs/evidence/` and covered by `.b3` manifests during closeout.

## Non-claims preserved
This metadata refactor does not add broad Minecraft compatibility, semantic equivalence, public-server safety, production readiness, full CTF correctness, or full survival correctness claims.
