# Protocol-763 Stevenarella ⇄ Valence CTF acceptance matrix

## Scope

This matrix indexes landed, bounded protocol-763 compatibility evidence for the local Stevenarella client against the local Valence CTF example. Rows point to maintained commands, receipt paths, BLAKE3 hashes, and scoped claims. The matrix is not a claim of full Minecraft compatibility, full CTF correctness, production readiness, or unbounded load/reconnect/latency safety.

## Landed evidence rows

| Seam | Maintained command | Receipt | Evidence doc | BLAKE3 | Landed commits | Scoped claim | Explicit non-claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| RED/BLUE scoring soak | `nix run .#mc-compat-valence-ctf-600s-soak`; `nix run .#mc-compat-valence-ctf-blue-600s-soak` | `target/mc-compat-blue-soak/blue-flag-score-600s.json` | `docs/evidence/valence-ctf-blue-600s-soak.md` | `b7c861f27ef7ceaf94705a74a5459d3f9df625dada4b14d8715ba8e9c5d921de` | parent `13e3b8c` | Bounded owned Valence CTF score path evidence, including BLUE mirror coverage. | No full CTF correctness, no production load, no unbounded soak. |
| Inventory/drop | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `target/mc-compat-inventory-drop/inventory-drop-server.json` | `docs/evidence/valence-ctf-inventory-drop-server.md` | `4aeb08172b35edd03d57169c63a4942ca149c783fbc51539702922ac246a0e46` | parent `5fff0a6` | Client inventory/hotbar/drop evidence correlated with Valence drop-item server milestone. | No complete inventory protocol coverage. |
| Block placement / use-item-on-block | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `target/mc-compat-block-place/block-place.json` | `docs/evidence/valence-ctf-block-place.md` | `9feec3b967b3fd5cb011139eda524c32c73123323823b3ebef7bd93062e1c122` | parent `09d62fc`, Valence `9e21a61`, Stevenarella `ca45a2e` | Bounded block placement/use-item-on-block client event and Valence server correlation. | No broad block interaction matrix. |
| Pickup semantics | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `target/mc-compat-pickup/pickup.json` | `docs/evidence/valence-ctf-pickup.md` | `bcac4aab63857cf0d3b6dd148455324e7f0368dd3e57cfd26841ae7fc1b5ffe8` | parent `f98b782`, Valence `699097f`, Stevenarella `ec9b661` | Client-observed collect/pickup animation plus server `inventory_pickup_item` correlation. | No slot-restoration or full item lifecycle claim. |
| Player-inventory click/container click | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `target/mc-compat-click/click.json` | `docs/evidence/valence-ctf-click.md` | `c75381feed1d98cd33d584ab9b8efdfe849d85eb3d1bb6cc23a23578cc8d7f7d` | parent `7cc4d4c`, Valence `7d2afee`, Stevenarella `72ab57e` | Bounded `ClickSlotEvent` correlation for player inventory/container click path. | No exhaustive window transaction semantics. |
| Open-container semantics | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `target/mc-compat-open-container/open-container.json` | `docs/evidence/valence-ctf-open-container.md` | `b7913ddd1f000981f411f7f14331b67820761c1d317c528fbf8a5070c139d3f3` | parent `3550bf3`, Valence `f82abd3`, Stevenarella `b7a48ab` | Bounded client-observed open-container plus Valence server correlation. | No all-container/all-menu compatibility claim. |
| Two-client combat/damage | `nix run .#mc-compat-valence-ctf-combat-damage` | `target/mc-compat-combat/combat-damage.json` | `docs/evidence/valence-ctf-combat-damage.md` | `b67962dd5d4fe7242b69fd7c879390e80e13528475d55d7feb5305289f762ac8` | parent `5d82476`, Valence `3835dab`, Stevenarella `2447d8e` | Two deterministic clients on opposing teams with attack, health-update, and Valence `combat_damage` correlation. | No projectile/armor/knockback or full combat balancing claim. |
| Flag-carrier death/return | `nix run .#mc-compat-valence-ctf-flag-carrier-death-return` | `target/mc-compat-flag-carrier-death/flag-carrier-death-return.json` | `docs/evidence/valence-ctf-flag-carrier-death-return.md` | `d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4` | parent `a4903a4`, Valence `0441d88`, Stevenarella `dd1e1d2` | Flag pickup, carrier death/drop, flag return/reset, respawn evidence, and no unexpected score. | No all death/drop/recovery permutations. |
| Reconnect flag-state | `nix run .#mc-compat-valence-ctf-reconnect-flag-state` | `target/mc-compat-reconnect-flag-state/reconnect-flag-state.json` | `docs/evidence/valence-ctf-reconnect-flag-state.md` | `4d848af56b25ad4b3c466863bac5b2052adbbc1c59e2b2164bfb2a696c225cb3` | parent `35e6000`, Valence `a5c98df` | One continuous Valence server, same-username reconnect, first-session flag pickup, disconnect flag return, and second-session coherent flag state. | No unbounded reconnect safety or persistence semantics beyond this fixture. |

## Remaining gaps and non-claims

| Gap | Status | Why it remains | Next ROI |
| --- | --- | --- | --- |
| Acceptance matrix freshness | This document is now machine-checked for required fields. | Future evidence rows can still stale if receipts are replaced without updating hashes. | Keep checker in CI/local gates. |
| Latency/jitter tolerance | Open Cairn `roi-04-latency-jitter-tolerance`. | Current receipts run on local low-jitter loopback without bounded network impairment. | Add maintained bounded jitter/latency wrapper and receipt. |
| Projectile/armor/knockback combat | Open Cairn `roi-05-projectile-armor-knockback-combat`. | Current combat covers melee damage/health and flag death; it does not cover projectiles, armor mitigation, or knockback bounds. | Add richer combat rail. |
| Broad protocol coverage | Non-claim. | Evidence targets protocol-763 Valence CTF flows, not all packets/states/features. | Add targeted seams only when ROI justifies. |
| Production load / multiplayer scale | Non-claim. | Existing soaks are bounded local compatibility receipts, not production load tests. | Separate load-test design if needed. |
| Full CTF correctness | Non-claim. | Scoring, death/return, reconnect, inventory, and combat seams are covered separately but do not prove all game rules. | Continue seam-by-seam evidence. |

## ROI order

Original ROI order from the compatibility brainstorm was:

1. `roi-01-flag-carrier-death-return` — drained at parent `a4903a4`.
2. `roi-02-reconnect-flag-state` — drained at parent `35e6000`.
3. `roi-03-acceptance-matrix-gap-checklist` — this matrix/checker slice.
4. `roi-04-latency-jitter-tolerance` — next open implementation slice.
5. `roi-05-projectile-armor-knockback-combat` — follows latency/jitter unless live state changes.

Current first-drain recommendation after this slice: drain `roi-04-latency-jitter-tolerance`, because it adds transport tolerance evidence without duplicating already-covered gameplay semantics.
