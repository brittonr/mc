# Protocol-763 survival crafting table contract

## Scope

This contract defines only the bounded `survival-crafting-table` row. The row uses one deterministic crafting table at `4,64,0`, one configured recipe `minecraft:stick`, one configured input stack set (`OakPlanks` count `1` in crafting slots `1` and `4`), one result stack (`Stick` count `4` in result slot `0`), and exact inventory/result metrics after collecting into inventory slot `36`.

## Required evidence

Promotion requires paired Paper and Valence evidence:

- reference backend: `paper`
- Valence backend: `valence`
- protocol: `763`
- scenario: `survival-crafting-table`
- copied receipts under `docs/evidence/`
- copied client and server logs under `docs/evidence/`
- committed child-revision metadata in receipts or a reviewable oracle checkpoint whose value is exactly `docs/evidence/protocol-763-survival-crafting-table-revision-oracle-2026-05-30.md` and whose file exists under `docs/evidence/`
- BLAKE3 manifest entries for every promoted artifact

## Required metrics

The checker normalizes these metric names:

- `scenario.name`
- `server.protocol`
- `server.backend`
- `client.username`
- `client.git_rev`
- `client.git_status`
- `client.git_dirty`
- `valence.git_rev_requested` (Valence evidence only unless an oracle checkpoint is used)
- `valence.git_rev_resolved` (Valence evidence only unless an oracle checkpoint is used)
- `valence.git_status` (Valence evidence only unless an oracle checkpoint is used)
- `valence.git_dirty` (Valence evidence only unless an oracle checkpoint is used)
- `revision.oracle_checkpoint`
- `client.missing_milestones.empty`
- `client.forbidden_matches.empty`
- `server.missing_milestones.empty`
- `server.forbidden_matches.empty`
- `client.milestone.protocol_detected`
- `client.milestone.join_game`
- `client.milestone.render_tick`
- `client.milestone.survival_crafting_table_open_seen`
- `client.milestone.survival_crafting_input_a_sent`
- `client.milestone.survival_crafting_input_b_sent`
- `client.milestone.survival_crafting_result_seen`
- `client.milestone.survival_crafting_result_collected`
- `client.milestone.survival_crafting_inventory_updated`
- `server.milestone.server_survival_crafting_table_open`
- `server.milestone.server_survival_crafting_input_a`
- `server.milestone.server_survival_crafting_input_b`
- `server.milestone.server_survival_crafting_result`
- `server.milestone.server_survival_crafting_collect`
- `client.crafting.open.window`
- `client.crafting.open.position`
- `client.crafting.input_a.window`
- `client.crafting.input_a.slot`
- `client.crafting.input_a.item`
- `client.crafting.input_a.count`
- `client.crafting.input_b.window`
- `client.crafting.input_b.slot`
- `client.crafting.input_b.item`
- `client.crafting.input_b.count`
- `client.crafting.result.window`
- `client.crafting.result.slot`
- `client.crafting.result.item`
- `client.crafting.result.count`
- `client.crafting.result.recipe`
- `client.crafting.collect.window`
- `client.crafting.collect.slot`
- `client.crafting.collect.item`
- `client.crafting.collect.count`
- `client.crafting.inventory.slot`
- `client.crafting.inventory.item`
- `client.crafting.inventory.count`
- `server.crafting.open.position`
- `server.crafting.open.window`
- `server.crafting.input_a.window`
- `server.crafting.input_a.slot`
- `server.crafting.input_a.item`
- `server.crafting.input_a.count`
- `server.crafting.input_b.window`
- `server.crafting.input_b.slot`
- `server.crafting.input_b.item`
- `server.crafting.input_b.count`
- `server.crafting.result.window`
- `server.crafting.result.slot`
- `server.crafting.result.item`
- `server.crafting.result.count`
- `server.crafting.result.recipe`
- `server.crafting.collect.window`
- `server.crafting.collect.slot`
- `server.crafting.collect.item`
- `server.crafting.collect.count`
- `server.crafting.collect.inventory_slot`

## Fail-closed checks

The checker rejects:

- `missing_reference`
- `valence_only`
- `missing_metric`
- `mismatched_metric:*.slot`
- `mismatched_metric:*.item`
- `mismatched_metric:*.count`
- `wrong_backend`
- `stale_revision`
- `missing_revision_or_oracle`
- `invalid_oracle_checkpoint`
- `missing_oracle_checkpoint_file`
- wrong scenario, protocol, table position, recipe, input slots, result slot, inventory slot, item ids, or counts
- missing, dirty, dry-run, unavailable, or mismatched child revision metadata unless the exact reviewable oracle checkpoint is supplied

## Non-claims

This row does not claim full crafting coverage, all recipes, recipe-book behavior, shift-click matrices, all container transaction modes, full survival compatibility, broad vanilla parity, or production readiness.
