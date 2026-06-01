# Survival biome/dimension runner rail — 2026-06-01

This checkpoint closes only the runner/client rail task for `survival-biome-dimension-state`.

Implemented seam:

- Stevenarella child commit `5a192fc` adds `MC_COMPAT_SURVIVAL_BIOME_DIMENSION_PROBE=1` and logs one client-side `survival_biome_dimension_state` milestone from the protocol-763 join-game dimension/world identifiers.
- Parent runner accepts `--scenario survival-biome-dimension-state`, sets the probe env, requires the client environment milestone, and records the scenario in the Nickel scenario manifest plus checked-in generated Rust table.

Non-claims: no Paper fixture, no Valence fixture, no paired reference parity receipt, no biome semantics beyond the join-game dimension/world identifier, no dimension travel, no world persistence, no matrix promotion, no full survival compatibility, and no production readiness.
