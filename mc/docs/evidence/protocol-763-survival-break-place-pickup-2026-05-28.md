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
| Receipt | `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.receipt.json` | `66c0bedb1e06a791cb8519aeb5b6f817392dfad773a5b6f25107b93b4e242bca` |
| Runner log | `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.run.log` | `9a699cbde117d99611cb52cdc0c43c914937ce8516c58eca7593366ce347e272` |
| Client log | `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.client-compatbot.log` | `c3b89880c6cf9dbeca69e1c621b941447d0509f7e411d1a764017e07e25286e0` |
| Valence log | `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.valence.log` | `c4f25d1efe2a57a7f3e01bd9e8d6f706314fdf06b345f460fc28f45a4a63d775` |
| Child revision oracle | `docs/evidence/protocol-763-survival-child-revision-oracle-2026-05-28.md` | `9e42efea3971c907bff5da392ca4bdd34570904ce8cdd93ae693fcd165f12fcd` |
| Manifest | `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.b3` | records all artifact hashes above |

Payload commits at run time: parent `455fa5c`, Valence `1fac05a`, Stevenarella `9921e68`.

The receipt now machine-records child revisions: `client.git_rev=9921e686f56270cb5810c1f6187d19b051ecc236`, `client.git_status=clean`, `valence.git_rev_requested=1fac05a`, `valence.git_rev_resolved=1fac05a6d012f27b83d88d83c59e5ab320a78164`, and `valence.git_status=clean`.

Legacy child-revision oracle checkpoint retained for review history: `docs/evidence/protocol-763-survival-child-revision-oracle-2026-05-28.md`.

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
