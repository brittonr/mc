# Proposal: Bounded armor combat reference parity

## Why

The current evidence bundle has one paired Paper-reference combat row for an iron-sword, no-armor hit, while armor mitigation remains either Valence-only or documented as an exact-vanilla non-claim. A narrow armor row is the next useful reduction in that gap: compare one diamond-chestplate, no-enchantment, no-status melee hit against the same Paper-reference harness before claiming any broader combat correctness.

## What Changes

- Add `vanilla-combat-armor-reference-parity` as a distinct runner scenario, manifest row, and receipt/bundle marker.
- Extend Paper and Valence fixtures so the bounded reference interaction equips `compatbotb` with a diamond chestplate and emits normalized `vanilla_combat_reference_*` metrics under the new row id.
- Extend the Rust comparator contract to accept both the existing no-armor row and the new diamond-chestplate row, while rejecting missing reference evidence, Valence-only evidence, stale revisions, row mismatches, unarmored armor rows, and mismatched damage/health metrics.
- Produce paired Paper-reference and Valence receipts/logs, normalized KV comparator inputs, and BLAKE3 manifests under `docs/evidence/`.
- Update matrix/current-bundle docs to promote only this one armor row and keep exact Mojang vanilla parity, all armor breadth, enchantments/status effects, modifier stacking, broad combat balancing, full CTF correctness, broad Minecraft compatibility, and production readiness as explicit non-claims.

## Impact

- **Area**: combat reference parity evidence.
- **Files**: `tools/mc-compat-runner`, `tools/paper-survival-fixture`, `valence/examples/ctf.rs`, `tools/check_vanilla_combat_reference_parity.rs`, scenario manifest/generated table, evidence docs/manifests, acceptance matrix/current bundle, Cairn artifacts.
- **Testing**: comparator positive/negative fixtures, runner unit tests, Valence example tests, scenario manifest drift/typecheck, paired live comparator, evidence manifest/task-evidence gates, Cairn gates, and Cairn validation.
- **Non-claims**: exact Mojang vanilla parity, all combat balancing, all weapons, all armor loadouts, enchantments/status effects, modifier stacking, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, public-server safety, and production readiness.
