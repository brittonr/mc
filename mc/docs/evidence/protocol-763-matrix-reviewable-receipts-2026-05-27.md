# Protocol-763 matrix reviewable receipt copies — 2026-05-27

## Purpose

This checkpoint hardens the maintained acceptance matrix by copying review-critical receipts that previously pointed only at mutable `target/` paths into tracked `docs/evidence/` files. It keeps existing scoped claims and BLAKE3 hashes unchanged for the copied receipts.

## Copied receipts

| Seam | Reviewable receipt | BLAKE3 |
| --- | --- | --- |
| Inventory/drop | `docs/evidence/protocol-763-inventory-drop.matrix.receipt.json` | `4aeb08172b35edd03d57169c63a4942ca149c783fbc51539702922ac246a0e46` |
| Block placement / use-item-on-block | `docs/evidence/protocol-763-block-place.matrix.receipt.json` | `9feec3b967b3fd5cb011139eda524c32c73123323823b3ebef7bd93062e1c122` |
| Pickup semantics | `docs/evidence/protocol-763-pickup.matrix.receipt.json` | `bcac4aab63857cf0d3b6dd148455324e7f0368dd3e57cfd26841ae7fc1b5ffe8` |
| Player-inventory click/container click | `docs/evidence/protocol-763-click.matrix.receipt.json` | `c75381feed1d98cd33d584ab9b8efdfe849d85eb3d1bb6cc23a23578cc8d7f7d` |
| Open-container semantics | `docs/evidence/protocol-763-open-container.matrix.receipt.json` | `b7913ddd1f000981f411f7f14331b67820761c1d317c528fbf8a5070c139d3f3` |
| Two-client combat/damage | `docs/evidence/protocol-763-combat-damage.matrix.receipt.json` | `b67962dd5d4fe7242b69fd7c879390e80e13528475d55d7feb5305289f762ac8` |
| Flag-carrier death/return | `docs/evidence/protocol-763-flag-carrier-death-return.matrix.receipt.json` | `d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4` |
| Reconnect flag-state | `docs/evidence/protocol-763-reconnect-flag-state.matrix.receipt.json` | `4d848af56b25ad4b3c466863bac5b2052adbbc1c59e2b2164bfb2a696c225cb3` |
| Latency/jitter tolerance | `docs/evidence/protocol-763-latency-jitter-inventory.matrix.receipt.json` | `a4a407fb1ac3aceae06faeacb794891ff8411c8ac86470c651c89b37b6c7f33d` |
| Combat knockback | `docs/evidence/protocol-763-combat-knockback.matrix.receipt.json` | `a5d0ba5ea6155a99b58f245a03195da05b4925d7bd151b5b3f67503ae7a4cf09` |

## Historical exception

The RED/BLUE scoring soak row still points at its historical live receipt path because the mutable `target/mc-compat-blue-soak/blue-flag-score-600s.json` file has since been overwritten by a dry-run fixture. The reviewable evidence for that row is `docs/evidence/stevenarella-valence-763-blue-flag-600s-soak-2026-05-25.md`, which records the live receipt hash `b7c861f27ef7ceaf94705a74a5459d3f9df625dada4b14d8715ba8e9c5d921de`. The hardened checker allows this as an explicit historical/oracle decision rather than treating the current `target/` file as evidence.

## Verification

- Manifest: `docs/evidence/protocol-763-matrix-reviewable-receipts-2026-05-27.b3`.
- Freshness gate log: `docs/evidence/protocol-763-evidence-freshness-gate-2026-05-27.run.log`.
- Decision owner: agent.
- Next action: future row additions or receipt replacements should copy review-critical artifacts under `docs/evidence/` and update the matrix, current bundle, and BLAKE3 manifest together.
