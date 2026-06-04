# Design: Bounded armor combat reference parity

## Goal

Promote one narrow armor combat reference row only when Paper-reference and Valence evidence agree for the same deterministic melee interaction: `compatbota` hits `compatbotb` with an iron sword while `compatbotb` wears exactly a diamond chestplate, with no enchantments and no status effects. The design preserves the existing no-armor row and keeps all broad combat and exact Mojang parity language as non-claims.

## Evidence contract

The new row id is `vanilla-combat-armor-reference-parity`. A valid record must name:

- `row=vanilla-combat-armor-reference-parity`
- `backend=paper-reference` or `backend=valence`
- `reference_oracle=paper-1.20.1-reference-harness`
- `reference_version=minecraft-1.20.1-protocol-763`
- clean child revision metadata
- `attacker_identity=compatbota`
- `victim_identity=compatbotb`
- `weapon=iron_sword`
- `armor_state=diamond_chestplate`
- `pre_health=20.0`
- `post_health=15.3`
- `damage_delta=4.7`
- `knockback_metric` with the existing bounded tolerance

The row intentionally uses Paper as the reference harness, not a direct Mojang oracle. Exact Mojang vanilla combat parity remains a non-claim.

## Functional core

`tools/check_vanilla_combat_reference_parity.rs` keeps a pure deterministic comparator:

- `CombatParityContract` selects the allowed row and expected row metrics.
- `CombatParityRecord` normalizes one backend's KV input.
- `CombatParityPair` holds reference plus Valence evidence.
- `compare_combat_parity(pair, contract) -> CombatParityDecision` returns pass/fail diagnostics without I/O.

Positive fixtures cover valid no-armor and valid diamond-chestplate paired rows, plus within-tolerance knockback. Negative fixtures cover missing reference, Valence-only evidence, unknown row, wrong reference version, missing tolerance, stale revisions, mismatched weapon/armor, unarmored armor rows, and wrong armor damage metrics.

## Imperative shell

Runner and fixtures own side effects:

- `tools/mc-compat-runner` exposes `--scenario vanilla-combat-armor-reference-parity`, starts the selected backend, configures the armor probe environment, runs the two Stevenarella clients, and writes receipts.
- The Paper fixture equips only the reference victim with a diamond chestplate when the armor reference probe is enabled and emits the new row id in normalized milestones.
- The Valence CTF fixture equips the same victim, applies a pure vanilla-style diamond-chestplate armor formula only for the reference armor probe, and leaves the older Valence-only armor mitigation rail unchanged.
- Evidence promotion copies only reviewable artifacts to `docs/evidence/`, including live logs, receipts, server logs, typed events, normalized KV files, source snapshot/JAR evidence, comparator logs, and BLAKE3 manifests.

## Promotion rule

Matrix and bundle docs may mark only the `vanilla-combat-armor-reference-parity` diamond-chestplate interaction as covered after the paired comparator passes. Existing Valence-only armor rows remain separately scoped, and exact Mojang vanilla parity, all armor loadouts, enchantments/status effects, modifier stacking, all weapons, projectile physics, broad combat balancing, full CTF correctness, full Minecraft compatibility, and production readiness remain explicit non-claims.

## Validation

Closeout must record:

- comparator self-test/compile log;
- runner unit test log;
- Valence example test log;
- Paper fixture build/source/JAR evidence;
- paired Paper/Valence live receipts and comparator output;
- scenario manifest check;
- evidence manifest check;
- task-evidence gate;
- Cairn gates and validation.
