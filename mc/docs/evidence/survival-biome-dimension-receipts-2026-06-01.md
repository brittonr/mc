# Survival biome/dimension paired receipts — 2026-06-01

This checkpoint closes the receipt task for `survival-biome-dimension-state`.

Evidence produced:

- Paper live run: `docs/evidence/survival-biome-dimension-paper-2026-06-01.receipt.json`, server log, typed events, and row evidence key/value file.
- Valence live run: `docs/evidence/survival-biome-dimension-valence-2026-06-01.receipt.json`, server log, typed events, and row evidence key/value file.
- Paper fixture jar: `docs/evidence/mc-compat-paper-survival-biome-dimension-fixture-2026-06-01.jar`.
- Row comparator: `tools/check_survival_row_parity.rs --row survival-biome-dimension-state` passed for Paper and Valence row evidence.

Observed matching normalized metrics:

- `spawn_environment=minecraft:overworld`
- `environment_identifier=minecraft:overworld`
- `client_environment_update=minecraft:overworld`
- `server_environment_state=minecraft:overworld`
- `normalized_identifier=minecraft:overworld`

Non-claims: this proves one join-game overworld identifier seam only. It does not claim biome lookup semantics, dimension travel, Nether/End behavior, world persistence, adjacent survival rows, full survival compatibility, public-server safety, load behavior, or production readiness.
