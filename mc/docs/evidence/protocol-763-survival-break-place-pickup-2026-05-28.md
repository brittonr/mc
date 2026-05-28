# Protocol-763 survival break/place/pickup rail — 2026-05-28

## Scope

Bounded owned-local Stevenarella ⇄ Valence protocol-763 survival fixture evidence for one survival loop: join/render, fixed-coordinate block break, pickup/inventory observation, and block placement. This is not a claim of full survival compatibility, vanilla parity, broad protocol coverage, production readiness, or semantic equivalence.

## Command

```sh
VALENCE_WORKTREE=/tmp/valence-compat-survival-1fac05a \
VALENCE_TARGET_DIR=/tmp/valence-compat-survival-target-1fac05a \
VALENCE_REV=1fac05a \
CLIENT_TIMEOUT=180 \
MC_COMPAT_SURVIVAL_BREAK_PLACE_PICKUP_RECEIPT=target/mc-compat-survival-break-place-pickup/survival-break-place-pickup-live-committed.json \
nix run --no-update-lock-file .#mc-compat-valence-survival-break-place-pickup -- \
  --run --client-dir stevenarella --valence-repo valence
```

## Evidence

| Artifact | Path | BLAKE3 |
| --- | --- | --- |
| Receipt | `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.receipt.json` | `d8afdad721a83aa1c9a4a099f78e01ea96985f7294b0874396f77c73db58217d` |
| Runner log | `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.run.log` | `cc5c7ee8e81adcd13d5fa6ff54bf5920e72ff79c2749dd5a9613c7944a7d4f31` |
| Client log | `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.client-compatbot.log` | `ef668103c8879c70515965e342d615965144116efd0cc6205689997c6c6a6efc` |
| Valence log | `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.valence.log` | `c9ebb200a2280336b442f606a611d3fc968deeab9a5cdd3112d0f9159248702a` |
| Manifest | `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.b3` | records all artifact hashes above |

Payload commits at run time: parent `fff4386`, Valence `1fac05a`, Stevenarella `9921e68`.

## Result

Receipt status is `pass`, mode is `run`, `dry_run=false`, scenario `survival-break-place-pickup`, selected Valence example `survival_compat`, and server/client correlation passed.

Observed client milestones:

- `protocol_detected`
- `join_game`
- `render_tick`
- `survival_break_sent`
- `survival_break_update`
- `survival_pickup_seen`
- `survival_place_sent`
- `survival_place_update`

Observed server milestones:

- `server_username_seen`
- `server_survival_join`
- `server_survival_break`
- `server_survival_pickup`
- `server_survival_place`

The receipt has no missing client/server milestones, no forbidden matches, `triage.suggested_boundary=none`, `claims_correctness=false`, and `claims_semantic_equivalence=false`.

## Non-claims

This evidence does not claim full survival compatibility, vanilla parity, all block/item interactions, natural drop physics, crafting/furnace/chest/mob/redstone/biome/dimension behavior, broad protocol-763 coverage, public-server safety, production readiness, WAN/adversarial-network safety, or unbounded soak/reconnect behavior.
