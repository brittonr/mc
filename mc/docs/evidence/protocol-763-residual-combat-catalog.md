# Protocol-763 residual combat catalog

## Scope

This catalog records combat semantics that remain outside the maintained Stevenarella ⇄ Valence CTF protocol-763 evidence rails after the landed `combat-damage` and `combat-knockback` slices.

## Covered combat rails

| Rail | Maintained command | Receipt | Evidence |
| --- | --- | --- | --- |
| Melee damage / health update | `nix run .#mc-compat-valence-ctf-combat-damage` | `target/mc-compat-combat/combat-damage.json` | `docs/evidence/stevenarella-valence-763-combat-damage-2026-05-25.md` |
| Melee knockback / velocity update | `nix run .#mc-compat-valence-ctf-combat-knockback` | `target/mc-compat-knockback/combat-knockback.json` | `docs/evidence/valence-ctf-combat-knockback.md` |

## Residual non-claims

The current protocol-763 compatibility evidence still does **not** prove:

- projectile spawning, travel, collision, or hit attribution,
- bow/crossbow/trident use semantics,
- armor mitigation or armor slot/equipment update semantics,
- enchantment effects or potion/status-effect combat modifiers,
- exact vanilla knockback balancing,
- full death/respawn correctness outside the bounded maintained rails,
- broad Minecraft combat correctness outside local Valence `ctf`.

## Next independently drainable combat seams

1. **Armor mitigation rail** — likely lower runtime complexity than projectiles if Valence CTF can equip deterministic armor and log before/after damage. Needs client-visible equipment/slot evidence plus Valence mitigation correlation.
2. **Projectile hit rail** — higher user-visible value but likely broader protocol surface: projectile spawn, velocity, hit/damage attribution, and client observation all need scoped pass/fail milestones.
3. **Equipment update rail** — useful prerequisite if armor proof first needs explicit client-observed armor/equipment state rather than server-only damage deltas.

Any future slice should remain a fresh Cairn package with a live receipt, BLAKE3 hash, dry-run gate, and explicit non-claims rather than broadening existing combat receipts silently.
