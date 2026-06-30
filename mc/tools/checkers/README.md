# mc-compat checker crate

This crate owns Rust evidence-checker binaries that are being migrated out of loose `tools/*.rs` scripts.

## Binary and wrapper contract

- Library core: `src/key_value.rs` contains pure key-value parsing, diagnostics, clean-revision checks, overclaim rejection, and fixture helpers. It has no filesystem, process, network, clock, or environment access.
- Imperative shell: `src/cli.rs` is the thin command wrapper for `--self-test` and `<evidence.kv>` file reads.
- Binaries: `src/bin/check_*.rs` preserve the existing checker command names with hyphenated installed binary names.
- Compatibility wrappers: migrated `tools/check_*.rs` files remain as small Rust entrypoints that delegate to this crate, so direct script-shaped command surfaces and flake check names do not change.
- Flake checks: existing check names remain stable; migrated checks may build these binaries through the crate package or through the compatibility wrappers.

## Migrated Rust checker family

| Checker | Crate binary | Legacy wrapper | Status |
| --- | --- | --- | --- |
| Block entity sign packet family | `check-block-entity-sign-packet-family` | `tools/check_block_entity_sign_packet_family.rs` | Migrated |
| Inventory stack split/merge evidence | `check-inventory-stack-split-merge-evidence` | `tools/check_inventory_stack_split_merge_evidence.rs` | Migrated |
| Inventory drag transactions evidence | `check-inventory-drag-transactions-evidence` | `tools/check_inventory_drag_transactions_evidence.rs` | Migrated |
| Scoreboard team packet family | `check-scoreboard-team-packet-family` | `tools/check_scoreboard_team_packet_family.rs` | Migrated in 2026-06-30 wave |

Each migrated checker has a positive fixture and negative fail-closed fixtures in its module self-test and unit tests. These tests verify valid evidence passes and invalid evidence reports specific diagnostics.

## 2026-06-30 migration wave inventory

Selected checker: `tools/check_scoreboard_team_packet_family.rs` because it was already framework-backed, uses one `docs/evidence/scoreboard-team-packet-family-2026-06-06.kv` input, has self-test positive and negative fixtures, and is wired by the stable flake check snippets that compile `tools/check_scoreboard_team_packet_family.rs` directly. The compatibility wrapper keeps that direct command surface stable while the crate binary provides the reusable checker core.

Remaining framework-backed loose Rust checker: `tools/check_movement_packet_family.rs`. It remains standalone debt because it also validates packet-inventory TSV rows and should migrate in a separate wave with modeled TSV inputs and wrapper parity fixtures.

Other loose Rust gates under `tools/check_*.rs` remain unchanged by this wave. Migration priority is: migrate touched evidence gates first, then framework-backed reusable key-value gates, then gates that need modeled filesystem or manifest inputs, and leave legacy Python gates for explicit Rust/Steel migration work before behavior changes.

## Legacy Python gate inventory

Owner for all untouched debt rows: mc-compat maintainers. Next action for all untouched debt rows: migrate to Rust before changing validation behavior, or add a Cairn waiver that records owner, reason, non-claim impact, and next action.

| Python gate | Status | Reason | Non-claim impact |
| --- | --- | --- | --- |
| `tools/check_armor_modifier_matrix.py` | Untouched debt | Not changed by this crate bootstrap | No new armor, combat, or parity claim |
| `tools/check_ctf_rule_ledger.py` | Untouched debt | Not changed by this crate bootstrap | No new CTF correctness claim |
| `tools/check_death_respawn_lifecycle.py` | Untouched debt | Not changed by this crate bootstrap | No new lifecycle correctness claim |
| `tools/check_equipment_slot_item_matrix.py` | Untouched debt | Not changed by this crate bootstrap | No new equipment breadth claim |
| `tools/check_inventory_semantics_matrix.py` | Untouched debt | Not changed by this crate bootstrap | No new inventory semantics claim |
| `tools/check_load_network_safety.py` | Untouched debt | Not changed by this crate bootstrap | No new load or public-server safety claim |
| `tools/check_projectile_travel_collision.py` | Untouched debt | Not changed by this crate bootstrap | No new projectile physics or combat claim |
| `tools/check_protocol_coverage_ledger.py` | Untouched debt | Not changed by this crate bootstrap | No new protocol coverage claim |
| `tools/check_survival_reference_parity.py` | Untouched debt | Not changed by this crate bootstrap | No new survival or vanilla parity claim |
| `tools/check_vanilla_combat_parity.py` | Untouched debt | Not changed by this crate bootstrap | No new vanilla combat parity claim |

## Non-claims

This crate is a maintainability boundary for evidence checkers. It does not create live compatibility evidence, semantic equivalence, production readiness, public-server safety, full CTF correctness, full survival correctness, broad Minecraft compatibility, or vanilla/reference parity claims.
