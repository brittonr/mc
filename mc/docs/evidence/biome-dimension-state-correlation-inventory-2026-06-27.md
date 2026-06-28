# Biome/dimension state correlation inventory — 2026-06-27

This checkpoint inventories only `survival-biome-dimension-state` before moving the row from substring fallback to typed join-state correlation.

## Current row

- Scenario: `survival-biome-dimension-state`.
- Manifest state before this change: `substring-fallback` with receipt expectations `schema`, `survival_biome_dimension`, `client_environment_observation`, and `non-claims`.
- Wrapper and dry-run check: `mc-compat-smoke` via `mc-compat-historical-scenario-dry-runs`.
- Current bundle row: `Survival biome/dimension join state`.

## Fixture and evidence sources

- Client probe toggle: `MC_COMPAT_SURVIVAL_BIOME_DIMENSION_PROBE`.
- Server fixture toggle: `MC_COMPAT_SURVIVAL_BIOME_DIMENSION_FIXTURE`.
- Client observation marker: `survival_biome_dimension_state spawn_environment=minecraft:overworld environment_identifier=minecraft:overworld client_environment_update=minecraft:overworld normalized_identifier=minecraft:overworld`.
- Server configured-state marker: `survival_biome_dimension_state username=compatbot spawn_environment=minecraft:overworld environment_identifier=minecraft:overworld server_environment_state=minecraft:overworld normalized_identifier=minecraft:overworld`.
- Protocol context source: runner receipt `server.protocol`.

## Correlation contract selected

The migrated receipt block records scenario identity, protocol context, client-observed join-state fields, server-configured fixture fields, correlation diagnostics, and non-claim labels. The pure validator rejects client-only evidence, server/client state mismatches, missing protocol context, and overbroad claim labels.

## Non-claims

This row remains a bounded configured join-state observation only. It does not claim dimension travel, portal behavior, all biome semantics, all dimensions, respawn rules, world persistence, full survival compatibility, broad vanilla parity, public-server safety, or production readiness.
