# Survival biome/dimension fixture instrumentation — 2026-06-01

This checkpoint closes only the Paper/Valence fixture-instrumentation task for `survival-biome-dimension-state`.

Implemented seam:

- Valence child commit `88c5722` adds `MC_COMPAT_SURVIVAL_BIOME_DIMENSION_FIXTURE=1` and emits `survival_biome_dimension_state` with `spawn_environment`, `environment_identifier`, `server_environment_state`, and `normalized_identifier` for the existing overworld survival fixture.
- Parent Paper fixture source adds the same env-gated milestone with matching normalized fields.

Non-claims: no paired Paper/Valence live receipts, no biome semantics beyond the fixture's overworld identifier, no dimension travel, no world persistence, no matrix promotion, no full survival compatibility, and no production readiness.
