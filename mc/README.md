# Minecraft Rust compatibility smoke

This workspace contains local Minecraft compatibility experiments. The hardened smoke harness checks a Rust client against a Rust server:

- client: `stevenarella`
- server: Valence pinned to Minecraft `1.18.2` / protocol `758`
- runner: `tools/mc-compat-runner`, packaged by the root flake

The legacy shell entrypoint is intentionally only a thin compatibility shim around the flake app.

## Commands

Launch the editable local server/client checkouts through the root flake environment:

```sh
nix run .#valence -- --dry-run
nix run .#stevenarella -- --dry-run

# Omit --dry-run to start the Valence CTF example or Stevenarella client.
# Stevenarella auto-wraps with Xvfb when DISPLAY is unset.
```

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

The runner forces the GUI client through Xvfb/X11 with software GL and removes inherited Wayland/niri socket environment before launch. A bounded timeout is considered success only when the client log contains connection/render evidence such as detected protocol or loaded dimension data.

Write a machine-readable smoke receipt for Cairn/Octet evidence flows:

```sh
SMOKE_RECEIPT=target/mc-compat-smoke.json CLIENT_TIMEOUT=8 nix run .#mc-compat-smoke -- --run
# or
nix run .#mc-compat-smoke -- --dry-run --server-backend paper --receipt target/mc-compat-smoke.json
```

The current receipt schema is `mc.compat.scenario.receipt.v2`; receipts also retain the legacy marker `mc.compat.smoke.receipt.v1` for older consumers. A receipt records server/client inputs, the headless-isolation contract (`wayland_socket_inherited=false`), typed scenario milestones, server-side correlation when available, and explicit non-claims (`claims_correctness=false`, `claims_semantic_equivalence=false`) for downstream Cairn/Octet review. It is evidence that the bounded scenario ran under the specified local fixture, not a claim of full semantic equivalence.

Choose a typed scenario with `--scenario` or `MC_COMPAT_SCENARIO`:

```sh
# Baseline login/status/render smoke.
nix run .#mc-compat-smoke -- --dry-run --scenario smoke \
  --receipt target/mc-compat-smoke.json

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

# Maintained protocol-763 Valence survival break/place/pickup receipt.
# Requires dedicated Valence survival_compat fixture, Stevenarella fixed-coordinate break/place probe, and Valence survival join/break/pickup/place server correlation.
nix run .#mc-compat-valence-survival-break-place-pickup
# deterministic, non-side-effecting fixture for the same survival command shape:
nix run .#mc-compat-valence-survival-break-place-pickup -- --dry-run

# Maintained protocol-763 Valence survival furnace persistence receipt.
# Requires paired client/server open/input/fuel/burn/output/collect/reconnect/state milestones; remains a row receipt, not aggregate survival parity.
nix run .#mc-compat-valence-survival-furnace-persistence
# deterministic, non-side-effecting fixture for the same furnace command shape:
nix run .#mc-compat-valence-survival-furnace-persistence -- --dry-run

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

# Maintained protocol-763 Valence CTF bounded latency/jitter receipt over the inventory semantic rail.
# Records bounded perturbation parameters, owned-local WAN telemetry fields, full inventory/open-container/block-place milestone oracle evidence, and no privileged network mutation.
nix run .#mc-compat-valence-ctf-latency-jitter-inventory
# deterministic, non-side-effecting fixture for the same latency/jitter command shape:
nix run .#mc-compat-valence-ctf-latency-jitter-inventory -- --dry-run

# build every maintained dry-run receipt/check plus the evidence indexes:
nix build .#checks.x86_64-linux.mc-compat-maintained-dry-runs --no-link -L

# check only the survival break/place/pickup dry-run receipt shape:
nix build .#checks.x86_64-linux.mc-compat-valence-survival-break-place-pickup-dry-run --no-link -L

# check only the MCP-controlled dry-run receipt/checker shape:
nix build .#checks.x86_64-linux.mc-compat-mcp-controlled-smoke-dry-run --no-link -L

# check the current protocol-763 evidence bundle:
nix build .#checks.x86_64-linux.mc-compat-current-evidence-bundle --no-link -L
```

`valence-compat-bot-probe` receipts add a `compat_bot_probe` block that records the owned local target, bounded one-client limit, non-public-stress-tool guard, and explicit `external_server_load_authorized=false` non-claim. `reconnect-flag-score` extends gameplay evidence with an explicit reconnect milestone.

Receipts also include bounded blocks for the remaining compatibility seams:

- `status_response_resource`: configured/default status description, version, and player sample expectations used by the status probe.
- `packet_capture_oracle`: headless/redacted packet-summary metadata; raw payloads are not durable evidence by default.
- `typed_event_oracle`: typed event schema/migration metadata. Dry-run and failure receipts mark `migration_status="substring-fallback"`; successful live receipts can write a `.typed-events.log` sidecar derived from client/server milestone evidence and record `event_log_path`, normalized `timeline_blake3`, `event_count`, `contributes_to_pass_fail`, and `raw_payloads_recorded=false`. Smoke and inventory-interaction are the first typed-graph pass/fail rails; unmigrated rails stay explicitly marked as substring fallback.
- `negative_live_rail`: dry-run/live envelope metadata for bounded invalid-action scenarios. It records the selected rail, invalid action, expected containment/disconnect outcome vocabulary, observed outcome plus client postcondition milestone when live telemetry exists, owned-local/public authorization fields, client/time bounds, required evidence fields, and explicit non-claims for broad invalid-input, adversarial-security, production, inventory, plugin-message, and CTF semantics.
- `public_server_authorized_safety`: deterministic authorization fixture metadata. It records owner, authorization artifact, non-loopback fixture scope, client/duration/traffic bounds, redaction policy, checkpoint decision, `live_traffic_enabled=false`, and explicit non-claims for live public-server safety, third-party targets without authorization, production readiness, adversarial safety, WAN tolerance, load safety beyond configured bounds, and unbounded public testing.
- `proxy_compat_seam`: direct/proxied route, forwarding mode, owned-local-proxy guard, and non-claims such as `mtls_ported=false` and `credentials_recorded=false`.
- `gameplay_oracles`: Hyperion-derived milestone vocabulary, correlated-evidence requirement, and explicit non-claims for full CTF correctness, broad compatibility, and unbounded soak.

For `flag-score-repeat`, `reconnect-flag-score`, and `multi-client-load-score`, Valence receipts include `server.required_milestones`, `server.observed_milestones`, `server.missing_milestones`, `server.forbidden_matches`, and `server.client_server_correlation`. Multi-client receipts also include `client.usernames` and `client.log_paths` for per-client inspection. All scenario receipts include a `triage` block with first missing client/server milestones, first forbidden pattern/source, relevant client/server log paths, and a `suggested_boundary` such as `client-probe`, `server-correlation`, `protocol-runtime`, or `preflight-or-server-startup`. The nested `triage.enriched` block adds bounded/redacted context (`last_client_event`, `last_server_event`, `correlation_ids`, `timeline_excerpt`, and `boundary_confidence`) for debugging only; failure triage is not compatibility coverage.

## Nickel-backed config

The scenario manifest source of truth is `config/mc-compat/scenario-manifest.ncl`. Update it before adding or changing a maintained scenario, then run `nix build .#checks.x86_64-linux.mc-compat-scenario-manifest --no-link -L` to typecheck Nickel, run positive/negative manifest fixtures, and check drift against runner tables, flake dry-run checks, README command listings, and current evidence bundle rows. Runtime code consumes checked-in Rust tables in `tools/mc-compat-runner/src/scenario_manifest_generated.rs`; it does not evaluate Nickel at startup.

Evidence promotion plans use the typed shape in `config/mc-compat/evidence-promotion-plan.ncl` and the Rust tool `tools/promote_evidence.rs`. Safe workflow: run `nix build .#checks.x86_64-linux.mc-compat-evidence-promotion --no-link -L`, inspect the dry-run plan, apply only to an explicit output directory, then run acceptance matrix, current bundle, evidence manifest, and Cairn validation before claiming a row. The tool never force-adds broad directories; it copies only planned artifacts and writes `promotion-plan.md`.

Cairn task closeout evidence is checked by `tools/check_cairn_task_evidence.rs` and the flake check `mc-compat-cairn-task-evidence`. Before marking an active Cairn task complete, cite copied `docs/evidence/` command output such as a `.run.log` plus either its `.b3` manifest or an inline BLAKE3 digest; missing files, target-only receipts, and checked tasks without verification output fail the gate.

The checked-in default config is Nickel-authored at `config/mc-compat/default.ncl` and exported to `config/mc-compat/generated/default.json`. The runner consumes exported JSON, not Nickel at runtime:

```sh
nix shell nixpkgs#nickel -c nickel export \
  config/mc-compat/default.ncl \
  > config/mc-compat/generated/default.json

nix run .#mc-compat-smoke -- \
  --config config/mc-compat/generated/default.json \
  --dry-run
```

Config provides defaults; environment variables and later CLI flags override it. You can also set `MC_COMPAT_CONFIG=config/mc-compat/generated/default.json`.

Run both fallback/control Paper and intended/default Valence receipts, then compare them in one local gate:

```sh
CLIENT_TIMEOUT=8 nix run .#mc-compat-smoke -- \
  --run-matrix \
  --receipt-dir target/matrix-smoke
```

For a non-side-effecting fixture of the same matrix shape, put `--dry-run` after `--run-matrix`:

```sh
nix run .#mc-compat-smoke -- \
  --run-matrix --dry-run \
  --receipt-dir target/matrix-smoke-dry-run
```

Compare existing fallback/control Paper and intended/default Valence receipts:

```sh
nix run .#mc-compat-smoke -- --compare-receipts \
  target/mc-compat-smoke.json \
  target/mc-compat-smoke-valence.json
```

Matrix and comparison checks require one `paper` receipt and one `valence` receipt, both passing, both protocol `758`, expected backend ports, successful client evidence, and niri-safe Xvfb/X11/software-GL isolation.

## Editable Stevenarella checkout

Stevenarella is intentionally a local sibling checkout so it can be patched while debugging the client side of the compatibility seam. By default the runner expects `./stevenarella` to be an editable Stevenarella repository root containing `Cargo.toml`.

Use another checkout without moving files:

```sh
nix run .#mc-compat-smoke -- --dry-run --client-dir /path/to/stevenarella
# or
CLIENT_DIR=/path/to/stevenarella nix run .#mc-compat-smoke -- --dry-run
```

If the checkout is missing or does not look like the repository root, the runner fails before starting the smoke and tells you whether to clone Stevenarella or pass `--client-dir` / `CLIENT_DIR`.

## Editable Valence checkout

Valence is intentionally a local sibling checkout so it can be patched while debugging the compatibility seam. By default the runner expects:

- `./valence` to be an editable Valence git checkout
- `VALENCE_REV=c86b828^` to exist in that checkout; this is the compatible Minecraft `1.18.2` / protocol `758` revision
- `VALENCE_WORKTREE=/tmp/valence-compat-758` to be a disposable detached worktree created from that checkout

Use another checkout without moving files:

```sh
nix run .#mc-compat-smoke -- --dry-run --valence-repo /path/to/valence
# or
VALENCE_REPO=/path/to/valence nix run .#mc-compat-smoke -- --dry-run
```

If the checkout or revision is missing, the runner fails before starting the smoke and tells you whether to clone/fetch Valence or pass `--valence-repo` / `VALENCE_REPO`.

Paper remains available as a fallback/control backend:

```sh
nix run .#mc-compat-smoke -- --run --server-backend paper
```

## OnixResearch tool inputs

The flake pins Cairn and Octet through the canonical GitHub inputs:

- `cairn`: `github:onixresearch/cairn` (`https://github.com/onixresearch/cairn`)
- `octet`: `github:onixresearch/octet` (`https://github.com/onixresearch/octet`)

The dev shell exposes `cairn` and `cargo-octet` alongside the smoke runner:

```sh
nix develop
cairn --help
cargo-octet --help
```

The packages are also available as `.#cairn`, `.#cargo-octet`, and `.#octet`.

## Verification

```sh
nix flake check
```

The flake includes focused checks for the runner binary, Nickel config freshness/export consumption, scenario manifest type/drift validation, evidence promotion dry-run/apply fixtures, active Cairn task evidence closeout, baseline dry-run receipt emission, `valence-compat-bot-probe` bounded probe receipt shape, `multi-client-load-score` scenario dry-run receipt shape, `mc-compat-open-cairns-dry-run` receipt coverage for status resources, packet-capture summaries, proxy seams, and gameplay-oracle non-claims, Paper/Valence matrix dry-run receipts, Paper/Valence receipt comparison fixtures, evidence BLAKE3 manifest/stale-marker validation, full survival compatibility aggregate gating (`mc-compat-full-survival-gate`), aggregate CTF/protocol/production claim gates (`mc-compat-aggregate-claim-gates`), missing-checkout diagnostics, help text, Cairn CLI availability, and Octet fingerprint smoke over the receipt producer surface (`mc-compat-receipt-contract`).
