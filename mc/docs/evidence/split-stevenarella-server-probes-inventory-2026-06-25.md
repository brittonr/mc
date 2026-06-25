# Split Stevenarella server probes inventory

## Question

Which parts of `clients/stevenarella/src/server/mod.rs` own general server protocol state, which parts are compatibility probe decisions, and which boundaries were used for the split?

## Owner subtree

`clients/stevenarella/` is the affected owner subtree. No Hyperion or Valence code was used as reference code for this refactor, so no Hyperion adopt/port/reference/reject classification was needed.

## Requirement IDs

- `r[mc_compatibility.stevenarella_server_probe_split.inventory]`
- `r[mc_compatibility.stevenarella_server_probe_split.boundaries]`
- `r[mc_compatibility.stevenarella_server_probe_split.pure_probes]`
- `r[mc_compatibility.stevenarella_server_probe_split.shell_wiring]`
- `r[mc_compatibility.stevenarella_server_probe_split.compatibility]`
- `r[mc_compatibility.stevenarella_server_probe_split.tests]`
- `r[mc_compatibility.stevenarella_server_probe_split.validation]`

## Inspected evidence

- `clients/stevenarella/src/server/mod.rs`
- `clients/stevenarella/src/server/plugin_messages.rs`
- `clients/stevenarella/src/server/scenario_contracts_generated.rs`
- `clients/stevenarella/AGENTS.md`
- `docs/check-tiers.md`
- Preflight gates: `docs/evidence/split-stevenarella-server-probes-preflight-gates.run.log`
- Baseline focused tests: `docs/evidence/split-stevenarella-server-probes-baseline-server-tests.run.log`

## Responsibility inventory

| Responsibility | Previous location | Boundary after split |
| --- | --- | --- |
| Packet decoding/encoding, connection mutation, ECS/world mutation, packet dispatch | `server/mod.rs` packet handlers and `Server` methods | Remains in `server/mod.rs` as the imperative shell |
| Environment variable reads and session/config defaults | `Server::new`, `*_from_env` helpers, and per-probe shell branches | Remains in `server/mod.rs`; pure cores accept explicit inputs |
| CTF score-limit and flag repeat decisions | Inline string/number helpers and chat predicates | `server/probes.rs` pure predicates/constants; shell logs milestones |
| Combat probe schedule | Inline tick/limit/position table in `apply_mc_compat_active_probe` | `server/probes.rs::next_combat_probe_decision`; shell applies movement/attack packets |
| Inventory stack split/merge and drag-transaction state machines | Top-level helpers in `server/mod.rs` | `server/probes.rs` pure state machines; shell writes clicks and logs existing milestones |
| Survival mob-drop action schedule | Inline tick/mob/target checks in `apply_mc_compat_survival_mob_drop_probe` | `server/probes.rs::next_survival_mob_drop_action`; shell writes movement/attack packets |
| Survival fixture predicates for crafting, furnace, hunger, redstone, world persistence, and mob drops | Top-level helpers in `server/mod.rs` | `server/probes.rs` pure predicates/builders; packet handlers remain shells |
| Sign/block-entity NBT extraction and payload matching | Top-level helpers in `server/mod.rs` | `server/probes.rs` pure extraction/payload helpers |
| Biome/dimension environment derivation | Top-level helpers in `server/mod.rs` | `server/probes.rs` pure normalization/derivation helpers |
| Milestone logging | Interleaved with actions in `server/mod.rs` | Remains in `server/mod.rs` to preserve observable text |

## Module-boundary decision

`clients/stevenarella/src/server/probes.rs` is the functional core for selected compatibility probes. It takes explicit scalar, item, packet-observation, sign/NBT, and position inputs and returns deterministic booleans, actions, or decisions. It does not read environment variables, write packets, mutate ECS/world state, access resources/rendering, or log milestones.

`clients/stevenarella/src/server/mod.rs` remains the imperative shell. It reads env/config, owns packet IO and ECS/world mutation, translates pure decisions into existing packet writes, and emits existing milestone text.

## Compatibility preservation

The refactor preserved existing env var names, packet action order, fixture constants, and milestone strings for the touched CTF, inventory, combat, survival mob-drop, sign/block-entity, and biome/dimension paths. Dry-run evidence remains harness-shape evidence only.

## Non-claims

This inventory and the linked dry-runs do not claim broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness.

## Next action

Use `docs/evidence/split-stevenarella-server-probes-focused-server-tests.run.log`, `docs/evidence/split-stevenarella-server-probes-fmt.run.log`, and `docs/evidence/split-stevenarella-server-probes-selected-dry-runs.run.log` as focused implementation evidence before task closeout and archive validation.
