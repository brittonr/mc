# mc-compat scenario commands

This document owns the detailed command reference for maintained mc-compat scenarios. The root README stays a quickstart and navigation index. This page is command-shape documentation only: it does not claim live success, semantic equivalence, public-server safety, production readiness, broad Minecraft compatibility, full CTF correctness, or full survival correctness.

## Machine-owned command index

The generated command table lives in [scenario-commands.generated.md](scenario-commands.generated.md). It is produced from `compat/config/scenario-manifest.ncl` by `tools/check_scenario_manifest.rs --write-generated-surfaces` and checked by `mc-compat-generated-harness-surfaces`.

The generated reviewer index in [evidence/mc-compat-scenario-index.generated.md](evidence/mc-compat-scenario-index.generated.md) records harness wiring, wrapper names, dry-run checks, migration state, and receipt expectation labels.

## Router forms

Choose a typed scenario with the router form `scenario run <scenario>`, the legacy `--scenario` flag, or `MC_COMPAT_SCENARIO`. Maintained flake aliases keep their public app names and route internally through the typed router; dry-run output includes a `typed scenario route` line without changing scenario semantics or compatibility claims.

```sh
# Baseline login/status/render smoke through the typed router.
nix run .#mc-compat-smoke -- scenario run smoke --dry-run \
  --receipt target/mc-compat-smoke.json

# Legacy flag form remains supported for compatibility.
nix run .#mc-compat-smoke -- --dry-run --scenario smoke \
  --receipt target/mc-compat-smoke.json
```

## Core smoke commands

Dry-run the plan without starting the server or client:

```sh
nix run .#mc-compat-smoke -- --dry-run
# or
scripts/mc-compat-smoke.sh --dry-run
```

Run the bounded headless smoke:

```sh
CLIENT_TIMEOUT=8 nix run .#mc-compat-smoke -- --run
# or
CLIENT_TIMEOUT=8 scripts/mc-compat-smoke.sh --run
```

Write a machine-readable smoke receipt for Cairn/Octet evidence flows:

```sh
SMOKE_RECEIPT=target/mc-compat-smoke.json CLIENT_TIMEOUT=8 nix run .#mc-compat-smoke -- --run
# or
nix run .#mc-compat-smoke -- --dry-run --server-backend paper --receipt target/mc-compat-smoke.json
```

## Maintained scenario examples

```sh
# Bounded one-client Valence compat-bot probe: protocol/login/render milestones only.
CLIENT_TIMEOUT=30 nix run .#mc-compat-smoke -- --run \
  --server-backend valence \
  --scenario valence-compat-bot-probe \
  --receipt target/mc-compat-bot-probe.json

# Single-client semantic repeat scoring: protocol/login/render/team/flag/two-score milestones.
CLIENT_TIMEOUT=60 nix run .#mc-compat-smoke -- --run \
  --server-backend valence \
  --scenario flag-score-repeat \
  --receipt target/mc-compat-flag-score-repeat.json

# Mirrored BLUE-team scoring path: BLUE portal, RED flag pickup, BLUE score milestone.
CLIENT_TIMEOUT=180 nix run .#mc-compat-smoke -- --run \
  --server-backend valence \
  --scenario blue-flag-score \
  --receipt target/mc-compat-blue-flag-score.json

# Reconnect-aware gameplay receipt, with optional status/proxy/capture fixture metadata.
nix run .#mc-compat-smoke -- --dry-run \
  --server-backend valence \
  --scenario reconnect-flag-score \
  --expect-status-description "compat fixture" \
  --expect-status-version "compat-version" \
  --expect-status-sample compatbot,observer \
  --packet-capture-summary \
  --proxy-route velocity-local \
  --proxy-forwarding-mode modern \
  --receipt target/mc-compat-open-cairns.json

# Two-client load-ish score scenario with server-side correlation.
CLIENT_TIMEOUT=60 nix run .#mc-compat-smoke -- --run \
  --server-backend valence \
  --scenario multi-client-load-score \
  --receipt target/mc-compat-multi-client-load-score.json

# Maintained protocol-763 Valence CTF 600s bounded soak receipt.
nix run .#mc-compat-valence-ctf-600s-soak
# deterministic, non-side-effecting fixture for the same soak command shape:
nix run .#mc-compat-valence-ctf-600s-soak -- --dry-run

# Maintained protocol-763 Valence CTF BLUE-team 600s bounded soak receipt.
nix run .#mc-compat-valence-ctf-blue-600s-soak
# deterministic, non-side-effecting fixture for the same BLUE soak command shape:
nix run .#mc-compat-valence-ctf-blue-600s-soak -- --dry-run

# Maintained protocol-763 Valence CTF inventory/drop/pickup/click/open-container/block-place interaction receipt.
# Requires client inventory/drop/pickup/click/open-container/container-click/block-place milestones plus Valence hotbar, drop-item, pickup, click-slot, open-container, container-click, and block-place server correlation.
nix run .#mc-compat-valence-ctf-inventory-interaction
# deterministic, non-side-effecting fixture for the same inventory/drop/pickup/click/open-container/block-place command shape:
nix run .#mc-compat-valence-ctf-inventory-interaction -- --dry-run

# Maintained protocol-763 Valence CTF inventory stack split/merge receipt.
# Requires one compatbot RedWool 64->32/32 split and merge-back with client state-id progression plus Valence ClickSlot split/merge correlation; not a broad inventory semantics claim.
nix run .#mc-compat-valence-inventory-stack-split-merge
# deterministic, non-side-effecting fixture for the same stack split/merge command shape:
nix run .#mc-compat-valence-inventory-stack-split-merge -- --dry-run

# Maintained protocol-763 Valence CTF inventory drag transaction receipt.
# Requires one compatbot RedWool x64 drag from slot 37 into slots 38 and 39 with a 32/32 final distribution plus Valence ClickSlot drag correlation; not a broad inventory semantics claim.
nix run .#mc-compat-valence-inventory-drag-transactions
# deterministic, non-side-effecting fixture for the same drag transaction command shape:
nix run .#mc-compat-valence-inventory-drag-transactions -- --dry-run

# Maintained protocol-763 Valence survival break/place/pickup receipt.
# Requires dedicated Valence survival_compat fixture, Stevenarella fixed-coordinate break/place probe, and Valence survival join/break/pickup/place server correlation.
nix run .#mc-compat-valence-survival-break-place-pickup
# deterministic, non-side-effecting fixture for the same survival command shape:
nix run .#mc-compat-valence-survival-break-place-pickup -- --dry-run

# Maintained protocol-763 Valence survival crafting-table receipt.
# Requires paired client/server crafting-table open/input/result/collect milestones; remains a row receipt, not aggregate survival parity.
nix run .#mc-compat-valence-survival-crafting-table
# deterministic, non-side-effecting fixture for the same crafting-table command shape:
nix run .#mc-compat-valence-survival-crafting-table -- --dry-run

# Maintained protocol-763 Valence survival crafting recipe breadth receipt.
# Requires paired shaped, shapeless, and invalid-recipe rejection milestones; remains a bounded row receipt, not aggregate survival parity.
nix run .#mc-compat-valence-survival-crafting-recipe-breadth
# deterministic, non-side-effecting fixture for the same crafting breadth command shape:
nix run .#mc-compat-valence-survival-crafting-recipe-breadth -- --dry-run

# Maintained protocol-763 Valence survival furnace persistence receipt.
# Requires paired client/server open/input/fuel/burn/output/collect/reconnect/state milestones; remains a row receipt, not aggregate survival parity.
nix run .#mc-compat-valence-survival-furnace-persistence
# deterministic, non-side-effecting fixture for the same furnace command shape:
nix run .#mc-compat-valence-survival-furnace-persistence -- --dry-run

# Maintained protocol-763 Valence survival furnace smelting breadth receipt.
# Requires paired raw-iron/coal smelt and invalid-fuel rejection milestones; remains a bounded row receipt, not aggregate furnace parity.
nix run .#mc-compat-valence-survival-furnace-smelting-breadth
# deterministic, non-side-effecting fixture for the same furnace breadth command shape:
nix run .#mc-compat-valence-survival-furnace-smelting-breadth -- --dry-run

# Maintained protocol-763 Valence survival hunger/health-cycle receipt.
# Requires isolated health, food, saturation, recovery, and inventory checkpoints; remains a bounded row receipt, not aggregate hunger parity.
nix run .#mc-compat-valence-survival-hunger-health-cycle
# deterministic, non-side-effecting fixture for the same hunger/health-cycle command shape:
nix run .#mc-compat-valence-survival-hunger-health-cycle -- --dry-run

# Deterministic MCP-controlled Stevenarella dry-run contract.
# Records stdio handshake/tool/outcome contract, child revision status, explicit non-claims, and fail-closed frame artifact placeholders; live --run remains blocked until owned-local capture evidence lands.
nix run .#mc-compat-mcp-controlled-smoke -- --dry-run

# Maintained protocol-763 Valence CTF two-client combat/damage receipt.
# Requires both clients to join/select opposing teams, Stevenarella attack + victim health-update milestones, and Valence combat_damage server correlation.
nix run .#mc-compat-valence-ctf-combat-damage
# deterministic, non-side-effecting fixture for the same combat command shape:
nix run .#mc-compat-valence-ctf-combat-damage -- --dry-run

# Maintained protocol-763 Valence CTF two-client combat/knockback receipt.
# Extends the combat rail with victim-side non-zero EntityVelocity evidence plus Valence combat_knockback server correlation.
nix run .#mc-compat-valence-ctf-combat-knockback
# deterministic, non-side-effecting fixture for the same combat/knockback command shape:
nix run .#mc-compat-valence-ctf-combat-knockback -- --dry-run

# Maintained protocol-763 Valence CTF armor mitigation receipt.
nix run .#mc-compat-valence-ctf-armor-equipment-mitigation
# deterministic, non-side-effecting fixture for the same armor command shape:
nix run .#mc-compat-valence-ctf-armor-equipment-mitigation -- --dry-run

# Bounded Paper-reference armor combat parity row; run both backends and compare normalized KV inputs.
nix run .#mc-compat-smoke -- --run --server-backend paper \
  --scenario vanilla-combat-armor-reference-parity \
  --receipt target/vanilla-combat-armor-reference-paper.json
nix run .#mc-compat-smoke -- --run --server-backend valence \
  --scenario vanilla-combat-armor-reference-parity \
  --receipt target/vanilla-combat-armor-reference-valence.json

# Maintained protocol-763 Valence CTF equipment update observation receipt.
nix run .#mc-compat-valence-ctf-equipment-update-observation
# deterministic, non-side-effecting fixture for the same equipment update command shape:
nix run .#mc-compat-valence-ctf-equipment-update-observation -- --dry-run

# Maintained protocol-763 Valence CTF projectile use/loadout receipt.
nix run .#mc-compat-valence-ctf-projectile-hit
# deterministic, non-side-effecting fixture for the same projectile hit command shape:
nix run .#mc-compat-valence-ctf-projectile-hit -- --dry-run

# Maintained protocol-763 Valence CTF projectile damage attribution receipt.
nix run .#mc-compat-valence-ctf-projectile-damage-attribution
# deterministic, non-side-effecting fixture for the same projectile damage command shape:
nix run .#mc-compat-valence-ctf-projectile-damage-attribution -- --dry-run

# Maintained protocol-763 Valence CTF flag-carrier death/return receipt.
# Requires two clients, flag pickup, client-observed death/respawn health restore, Valence flag_carrier_death + flag_return correlation, and no accidental score/capture patterns.
nix run .#mc-compat-valence-ctf-flag-carrier-death-return
# deterministic, non-side-effecting fixture for the same flag-carrier death/return command shape:
nix run .#mc-compat-valence-ctf-flag-carrier-death-return -- --dry-run

# Maintained protocol-763 Valence CTF reconnect flag-state receipt.
# Requires one continuous Valence server, same-username reconnect, first-session flag pickup, disconnect flag return, and post-reconnect coherent flag state.
nix run .#mc-compat-valence-ctf-reconnect-flag-state
# deterministic, non-side-effecting fixture for the same reconnect flag-state command shape:
nix run .#mc-compat-valence-ctf-reconnect-flag-state -- --dry-run

# Maintained protocol-763 Valence CTF invalid pickup ownership receipt.
# Requires one client attempting an own-flag pickup, client containment, Valence invalid_flag_pickup_rejected correlation, and no owner transfer/score/capture patterns.
nix run .#mc-compat-valence-ctf-invalid-pickup-ownership
# deterministic, non-side-effecting fixture for the same invalid pickup ownership command shape:
nix run .#mc-compat-valence-ctf-invalid-pickup-ownership -- --dry-run

# Maintained protocol-763 Valence CTF invalid return/drop receipt.
# Requires one client attempting an own-base return/drop without carrier ownership, client containment, Valence invalid_flag_return_drop_rejected correlation, and no state mutation/score/capture patterns.
nix run .#mc-compat-valence-ctf-invalid-return-drop
# deterministic, non-side-effecting fixture for the same invalid return/drop command shape:
nix run .#mc-compat-valence-ctf-invalid-return-drop -- --dry-run

# Maintained protocol-763 Valence CTF invalid opponent-base return/drop receipt.
# Requires one red client attempting a blue opponent-base return/drop without carrier ownership, client containment, Valence invalid_opponent_base_return_drop_rejected correlation, and no state mutation/score/capture patterns.
nix run .#mc-compat-valence-ctf-invalid-opponent-base-return-drop
# deterministic, non-side-effecting fixture for the same opponent-base invalid return/drop command shape:
nix run .#mc-compat-valence-ctf-invalid-opponent-base-return-drop -- --dry-run

# Maintained protocol-763 Valence CTF score-limit win-condition receipt.
# Requires one near-limit capture, configured score-limit telemetry, exactly one win/end milestone, and no duplicate-win/post-win score mutation patterns.
nix run .#mc-compat-valence-ctf-score-limit-win-condition
# deterministic, non-side-effecting fixture for the same score-limit command shape:
nix run .#mc-compat-valence-ctf-score-limit-win-condition -- --dry-run

# Maintained protocol-763 Valence CTF simultaneous pickup/capture race receipt.
# Requires two same-team contenders, one accepted pickup/capture, one rejected duplicate pickup, bounded race-window telemetry, and no double-accept/duplicate-score patterns.
nix run .#mc-compat-valence-ctf-simultaneous-pickup-capture-race
# deterministic, non-side-effecting fixture for the same race command shape:
nix run .#mc-compat-valence-ctf-simultaneous-pickup-capture-race -- --dry-run

# Maintained protocol-763 Valence CTF spawn/team balance/resource reset receipt.
# Requires two clients assigned to opposite teams, spawn/resource assignment telemetry, post-score reset-state correlation, and no imbalance/stale-resource patterns.
nix run .#mc-compat-valence-ctf-spawn-team-balance-reset
# deterministic, non-side-effecting fixture for the same spawn reset command shape:
nix run .#mc-compat-valence-ctf-spawn-team-balance-reset -- --dry-run

# Maintained protocol-763 Valence CTF bounded latency/jitter receipt over the inventory semantic rail.
# Records bounded perturbation parameters, owned-local WAN telemetry fields, full inventory/open-container/block-place milestone oracle evidence, and no privileged network mutation.
nix run .#mc-compat-valence-ctf-latency-jitter-inventory
# deterministic, non-side-effecting fixture for the same latency/jitter command shape:
nix run .#mc-compat-valence-ctf-latency-jitter-inventory -- --dry-run
```

## Generated and evidence checks

```sh
# Build every maintained dry-run receipt/check plus the evidence indexes:
nix build .#checks.x86_64-linux.mc-compat-maintained-dry-runs --no-link -L

# Check deterministic dry-run receipt shapes for historical maintained rows:
# flag-score-repeat, survival-chest-persistence, survival-hunger-food,
# survival-hunger-health-cycle, survival-mob-drop, survival-redstone-toggle,
# survival-world-persistence-restart,
# survival-crash-recovery-parity, survival-block-entity-persistence-parity,
# and survival-biome-dimension-state. This is harness-shape coverage only;
# live/reference parity remains tied to promoted evidence rows.
nix build .#checks.x86_64-linux.mc-compat-historical-scenario-dry-runs --no-link -L

# Check only the survival break/place/pickup dry-run receipt shape:
nix build .#checks.x86_64-linux.mc-compat-valence-survival-break-place-pickup-dry-run --no-link -L

# Check only the MCP-controlled dry-run receipt/checker shape:
nix build .#checks.x86_64-linux.mc-compat-mcp-controlled-smoke-dry-run --no-link -L

# Check the current protocol-763 evidence bundle:
nix build .#checks.x86_64-linux.mc-compat-current-evidence-bundle --no-link -L
```
