# Creative inventory live rail — 2026-06-07

## Scope

This evidence supports Cairn change `add-creative-inventory-action-live-rail`. The change defines and validates a bounded creative-inventory live contract for row `creative-inventory-action` but does not promote the row beyond its existing fixture-bounded status.

## Contract

The pure runner contract names exactly one candidate mutation:

- scenario: `inventory-interaction`;
- actor: `compatbot`;
- game-mode precondition: `creative`;
- semantic slot: `hotbar_0`;
- wire slot: `36`;
- item: `minecraft:stone`;
- item count: `64`;
- packet row: `play/serverbound/0x2b CreativeInventoryActionC2SPacket`;
- expected server correlation: `creative_slot_mutation_accepted`.

`tools/mc-compat-runner/src/scenario_core.rs` keeps this contract in pure data and validates it with positive and negative tests. The scenario capability registry now points the creative row at `deterministic-creative-fixture-contract` and keeps the client path as `stevenarella-creative-action-driver-missing`.

## Blocker decision

No maintained Stevenarella live driver currently performs the configured creative-mode mutation, so the row remains fixture-bounded. The blocker is recorded in:

- `docs/evidence/creative-inventory-live-rail-2026-06-07.kv`;
- `docs/evidence/creative-inventory-live-rail-2026-06-07.receipt.json`.

The targeted packet live-evidence checker rejects the blocker KV as expected because `live.promotion.status=blocked`, `live.evidence.mode=fixture-bounded-blocker`, and live metrics are not `ok`. This is intentional fail-closed behavior and prevents matrix, bundle, or packet-inventory promotion.

## Validation evidence

- Baseline targeted packet/matrix/bundle checks: `docs/evidence/creative-inventory-live-rail-baseline-2026-06-07.run.log` (`exit_status=0`).
- Implementation checks: `docs/evidence/creative-inventory-live-rail-checks-2026-06-07.run.log` (`exit_status=0`).
- Blocker live-checker assertion: `docs/evidence/creative-inventory-live-rail-blocker-checker-2026-06-07.run.log` (`exit_status=0`).
- Non-promotion matrix/bundle/targeted checks: `docs/evidence/creative-inventory-live-rail-nonpromotion-checks-2026-06-07.run.log` (`exit_status=0`).
- Cairn gates: `docs/evidence/creative-inventory-live-rail-cairn-gates-2026-06-07.run.log` (`exit_status=0`).
- Evidence manifest refresh/checks and task-evidence gate: `docs/evidence/creative-inventory-live-rail-evidence-manifest-refresh-2026-06-07.run.log` and `docs/evidence/creative-inventory-live-rail-evidence-checks-2026-06-07.run.log` (`exit_status=0`).
- Sync/archive/post-archive validation: `docs/evidence/creative-inventory-live-rail-sync-2026-06-07.run.log`, `docs/evidence/creative-inventory-live-rail-post-sync-validate-2026-06-07.run.log`, `docs/evidence/creative-inventory-live-rail-archive-2026-06-07.run.log`, `docs/evidence/creative-inventory-live-rail-final-manifest-refresh-2026-06-07.run.log`, and `docs/evidence/creative-inventory-live-rail-post-archive-checks-2026-06-07.run.log` (`exit_status=0`).

## Non-claims

This change does not claim live creative inventory parity, all creative inventory semantics, all slots, all items, game-mode transitions, pick-block behavior, public-server safety, production readiness, broad Minecraft compatibility, or full protocol 763 compatibility.
