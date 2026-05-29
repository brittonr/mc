# Protocol-763 survival chest persistence contract — 2026-05-28

## Scope

This contract defines the bounded `survival-chest-persistence` row before runner or fixture work starts.

Covered claim, once paired evidence exists: one protocol-763 client stores one item stack (`Dirt` count `1`) in one chest slot inside one chest block, closes the chest, reconnects once, reopens the same chest, and observes the same stack still present in that slot on both Paper and Valence.

## Fixed fixture envelope

| Field | Value |
| --- | --- |
| Scenario | `survival-chest-persistence` |
| Reference backend | Paper `1.20.1`, protocol `763` |
| Implementation backend | Valence `survival_compat`, protocol `763` |
| Client | `compatbot` |
| Chest position | `8,64,0` |
| Chest slot | `0` |
| Stored item | `Dirt` |
| Stored count | `1` |
| Session cycle | close, disconnect/reconnect once, reopen |

## Required normalized metrics

| Metric | Meaning |
| --- | --- |
| `scenario.name` | Receipt scenario is `survival-chest-persistence`. |
| `server.protocol` | Backend reports protocol `763`. |
| `server.backend` | Receipt backend is `paper` for reference and `valence` for implementation. |
| `client.username` | Client identity for correlation. |
| `client.missing_milestones.empty` | Client required milestone list has no misses. |
| `client.forbidden_matches.empty` | Client forbidden-pattern list has no hits. |
| `server.missing_milestones.empty` | Server required milestone list has no misses. |
| `server.forbidden_matches.empty` | Server forbidden-pattern list has no hits. |
| `client.milestone.protocol_detected` | Client detected protocol. |
| `client.milestone.join_game` | Client entered play state. |
| `client.milestone.render_tick` | Client rendered with player state. |
| `client.milestone.survival_chest_open_seen` | Client saw chest open window. |
| `client.milestone.survival_chest_store_sent` | Client sent store/click action. |
| `client.milestone.survival_chest_close_sent` | Client closed the chest window. |
| `client.milestone.survival_chest_reconnect_sent` | Client completed reconnect cycle. |
| `client.milestone.survival_chest_reopen_seen` | Client saw reopened chest window. |
| `client.milestone.survival_chest_persisted_seen` | Client observed stored stack after reconnect. |
| `server.milestone.server_survival_chest_open` | Server observed chest open. |
| `server.milestone.server_survival_chest_store` | Server observed store/click update. |
| `server.milestone.server_survival_chest_close` | Server observed chest close. |
| `server.milestone.server_survival_chest_reopen` | Server observed reopen after reconnect. |
| `server.milestone.server_survival_chest_persisted` | Server observed persisted stored stack. |
| `client.chest.open.window` | Client chest window id on first open. |
| `client.chest.open.position` | Client target chest position. |
| `client.chest.store.window` | Client store action window id. |
| `client.chest.store.slot` | Client store action slot. |
| `client.chest.store.item` | Client store action item. |
| `client.chest.store.count` | Client store action count. |
| `client.chest.close.window` | Client close action window id. |
| `client.chest.reconnect.session` | Client reconnect session number. |
| `client.chest.reopen.window` | Client reopened chest window id. |
| `client.chest.reopen.position` | Client reopened chest position. |
| `client.chest.persisted.window` | Client persisted-observation window id. |
| `client.chest.persisted.slot` | Client persisted-observation slot. |
| `client.chest.persisted.item` | Client persisted-observation item. |
| `client.chest.persisted.count` | Client persisted-observation count. |
| `server.chest.open.position` | Server chest open position. |
| `server.chest.open.window` | Server chest open window id. |
| `server.chest.store.window` | Server store window id. |
| `server.chest.store.slot` | Server store slot. |
| `server.chest.store.item` | Server store item. |
| `server.chest.store.count` | Server store count. |
| `server.chest.close.window` | Server close window id. |
| `server.chest.reopen.position` | Server reopen position. |
| `server.chest.reopen.window` | Server reopen window id. |
| `server.chest.persisted.slot` | Server persisted slot. |
| `server.chest.persisted.item` | Server persisted item. |
| `server.chest.persisted.count` | Server persisted count. |

## Checker contract

`tools/check_survival_chest_persistence.rs` is the Rust promotion gate for this row. It must pass positive fixtures and reject:

- `missing_reference` / Valence-only evidence;
- `missing_metric` for any required metric above;
- `mismatched_metric:*.slot` when chest slot differs;
- `mismatched_metric:*.item` when item differs;
- `mismatched_metric:*.count` when count differs;
- `wrong_backend` for a backend that is not the expected Paper or Valence side.

## Non-claims

This contract does not claim full survival compatibility, all-container behavior, double chest behavior, hopper behavior, redstone inventory behavior, item NBT preservation, server restart persistence, world persistence, broad Minecraft survival behavior, production readiness, or broader vanilla parity.
