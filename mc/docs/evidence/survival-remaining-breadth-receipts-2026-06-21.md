# Survival remaining breadth paired receipts — 2026-06-21

## Scope

This note promotes only six bounded survival breadth rows: mob AI/loot, redstone circuit, biome/dimension travel, world multichunk durability, container block-entity breadth, and sign editing live parity. Each row has paired Paper/reference and Valence receipts, typed-event logs, extracted client/server milestone logs, normalized KV metrics, a row checker run, and a Paper/Valence receipt comparison log under `docs/evidence/`.

Child revisions are clean in the promoted receipts:

- Stevenarella client/Paper-side child revision: `7224a6d4b5d17754daa694fe8ce7ab27bb79733f`.
- Valence server fixture child revision: `5f6bd1f714b9fb0f083eff48613ce60d9d83c86b`.
- Paper fixture jar promoted for review: `docs/evidence/paper-survival-fixture/paper-survival-fixture-0.1.0-survival-remaining-breadth-2026-06-21.jar`.

## Paired rows

| Row | Valence receipt | Paper receipt | Comparator log | Normalized metrics | Checker log |
| --- | --- | --- | --- | --- | --- |
| Mob AI/loot breadth | `docs/evidence/survival-mob-ai-loot-breadth-valence-2026-06-21.receipt.json` | `docs/evidence/survival-mob-ai-loot-breadth-paper-2026-06-21.receipt.json` | `docs/evidence/survival-mob-ai-loot-breadth-paper-valence-compare-2026-06-21.run.log` | `docs/evidence/survival-mob-ai-loot-breadth-valence-2026-06-21.kv`, `docs/evidence/survival-mob-ai-loot-breadth-paper-2026-06-21.kv` | `docs/evidence/survival-mob-ai-loot-breadth-checker-2026-06-21.run.log` |
| Redstone circuit breadth | `docs/evidence/survival-redstone-circuit-breadth-valence-2026-06-21.receipt.json` | `docs/evidence/survival-redstone-circuit-breadth-paper-2026-06-21.receipt.json` | `docs/evidence/survival-redstone-circuit-breadth-paper-valence-compare-2026-06-21.run.log` | `docs/evidence/survival-redstone-circuit-breadth-valence-2026-06-21.kv`, `docs/evidence/survival-redstone-circuit-breadth-paper-2026-06-21.kv` | `docs/evidence/survival-redstone-circuit-breadth-checker-2026-06-21.run.log` |
| Biome/dimension travel | `docs/evidence/survival-biome-dimension-travel-valence-2026-06-21.receipt.json` | `docs/evidence/survival-biome-dimension-travel-paper-2026-06-21.receipt.json` | `docs/evidence/survival-biome-dimension-travel-paper-valence-compare-2026-06-21.run.log` | `docs/evidence/survival-biome-dimension-travel-valence-2026-06-21.kv`, `docs/evidence/survival-biome-dimension-travel-paper-2026-06-21.kv` | `docs/evidence/survival-biome-dimension-travel-checker-2026-06-21.run.log` |
| Container block-entity breadth | `docs/evidence/survival-container-block-entity-breadth-valence-2026-06-21.receipt.json` | `docs/evidence/survival-container-block-entity-breadth-paper-2026-06-21.receipt.json` | `docs/evidence/survival-container-block-entity-breadth-paper-valence-compare-2026-06-21.run.log` | `docs/evidence/survival-container-block-entity-breadth-valence-2026-06-21.kv`, `docs/evidence/survival-container-block-entity-breadth-paper-2026-06-21.kv` | `docs/evidence/survival-container-block-entity-breadth-checker-2026-06-21.run.log` |
| Sign editing live parity | `docs/evidence/survival-sign-editing-live-valence-2026-06-21.receipt.json` | `docs/evidence/survival-sign-editing-live-paper-2026-06-21.receipt.json` | `docs/evidence/survival-sign-editing-live-paper-valence-compare-2026-06-21.run.log` | `docs/evidence/survival-sign-editing-live-valence-2026-06-21.kv`, `docs/evidence/survival-sign-editing-live-paper-2026-06-21.kv` | `docs/evidence/survival-sign-editing-live-checker-2026-06-21.run.log` |
| World multichunk durability | `docs/evidence/survival-world-multichunk-durability-valence-2026-06-21.receipt.json` | `docs/evidence/survival-world-multichunk-durability-paper-2026-06-21.receipt.json` | `docs/evidence/survival-world-multichunk-durability-paper-valence-compare-2026-06-21.run.log` | `docs/evidence/survival-world-multichunk-durability-valence-2026-06-21.kv`, `docs/evidence/survival-world-multichunk-durability-paper-2026-06-21.kv` | `docs/evidence/survival-world-multichunk-durability-checker-2026-06-21.run.log` |

## Non-claims

These receipts are bounded row evidence only. They do not claim full survival compatibility, broad vanilla parity, all mobs, all loot tables, all redstone circuits or tick ordering, all dimensions or portals, long-term durability, arbitrary crash consistency, all containers, arbitrary block-entity NBT, all sign UI behavior, public-server safety, production readiness, or unbounded load/soak safety.
