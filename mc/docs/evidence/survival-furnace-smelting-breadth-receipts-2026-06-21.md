# Survival furnace smelting breadth paired receipts — 2026-06-21

This checkpoint closes the receipt task for `survival-furnace-smelting-breadth`.

Evidence produced:

- Paper live run: `docs/evidence/survival-furnace-smelting-breadth-paper-2026-06-21.receipt.json`, run log, client log, server log, typed events, and normalized key/value evidence.
- Valence live run: `docs/evidence/survival-furnace-smelting-breadth-valence-2026-06-21.receipt.json`, run log, client log, server log, typed events, and normalized key/value evidence.
- Paper fixture jar: `docs/evidence/paper-survival-fixture/paper-survival-fixture-0.1.0-survival-furnace-smelting-breadth-2026-06-21.jar` (`f536c42c81cc54716f99dff2ba6c6d383508bf83e3425409b67651abe4923b24`).
- Row comparator: `tools/check_survival_breadth_contracts.rs --row survival-furnace-smelting-breadth-parity --paper docs/evidence/survival-furnace-smelting-breadth-paper-2026-06-21.kv --valence docs/evidence/survival-furnace-smelting-breadth-valence-2026-06-21.kv` passed with `exit_status=0` in `docs/evidence/survival-furnace-smelting-breadth-contract-check-2026-06-21.run.log`.

Observed matching normalized metrics:

- Smelting recipe: `minecraft:iron_ingot` from `RawIron x1` input and `Coal x1` fuel.
- Furnace timing constants: `burn_ticks=1600` and `cook_ticks=200` for this bounded fixture row.
- Output: `IronIngot x1`, collected by primary click into inventory slot `36`.
- Invalid fuel rejection: one `RawIron` invalid-fuel attempt after output collection, with outcome `no_burn` and no broad fuel semantics claim.
- Both backends recorded client milestones for furnace open/input/fuel/progress/output/collect/inventory update/invalid-fuel attempt, plus server milestones for open/input/fuel/progress/output/collect/invalid-fuel rejection/breadth state.

Child revisions recorded in receipts/evidence:

- Valence: `53ec70c527796b158463d087fbbb9d0826bc52c5`.
- Stevenarella: `c6bafb754e2e4d713c819d6f78cdbb45a6082bd7`.
- Paper backend: `1.20.1` with fixture jar BLAKE3 above; the Paper receipt records clean revision status and server revision `8ad9c8587a3273ec59b0ec4edae0bf790bdf403b`.

Non-claims: this proves only the configured RawIron + Coal smelt, one IronIngot output collection path, and one RawIron invalid-fuel rejection in the owned local fixture. It does not claim all smelting recipes, all fuels, hopper automation, furnace minecarts, long-running timing parity, all furnace UI interactions, full survival compatibility, broad vanilla parity, public-server safety, production readiness, or semantic equivalence.
