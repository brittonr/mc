# Protocol-763 residual combat catalog

## Scope

This catalog records combat semantics that remain outside the maintained Stevenarella ⇄ Valence CTF protocol-763 evidence rails after the landed combat-damage, combat-knockback, armor/equipment, equipment-update, projectile-use, and pinned projectile-damage slices.

## Covered combat rails

| Rail | Maintained command | Receipt | Evidence |
| --- | --- | --- | --- |
| Melee damage / health update | `nix run .#mc-compat-valence-ctf-combat-damage` | `target/mc-compat-combat/combat-damage.json` | `docs/evidence/stevenarella-valence-763-combat-damage-2026-05-25.md` |
| Melee knockback / velocity update | `nix run .#mc-compat-valence-ctf-combat-knockback` | `target/mc-compat-knockback/combat-knockback.json` | `docs/evidence/valence-ctf-combat-knockback.md` |
| Armor equipment mitigation | `nix run .#mc-compat-valence-ctf-armor-equipment-mitigation` | `target/mc-compat-armor-mitigation/armor-equipment-mitigation.json` | `docs/evidence/protocol-763-roi-01-03-drained-receipts-index.md` |
| Equipment update observation | `nix run .#mc-compat-valence-ctf-equipment-update-observation` | `target/mc-compat-equipment-update/equipment-update-observation.json` | `docs/evidence/protocol-763-roi-01-03-drained-receipts-index.md` |
| Projectile use/loadout rail | `nix run .#mc-compat-valence-ctf-projectile-hit` | `target/mc-compat-projectile-hit/projectile-hit.json` | `docs/evidence/protocol-763-roi-01-03-drained-receipts-index.md` |
| Projectile damage attribution | `nix run .#mc-compat-valence-ctf-projectile-damage-attribution` | `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.receipt.json` | `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.md` |

## Residual non-claims

The current protocol-763 compatibility evidence still does **not** prove:

- all equipment slots, all item types, or all equipment packet permutations,
- all armor loadouts, exact vanilla mitigation parity, enchantment effects, or potion/status-effect combat modifiers,
- projectile travel, collision simulation, exact vanilla damage parity, or all bow/crossbow/trident use semantics,
- exact vanilla knockback balancing,
- full death/respawn correctness outside the bounded maintained rails,
- broad Minecraft combat correctness outside local Valence `ctf`.

## Next independently drainable combat seams

1. **Projectile travel/collision simulation rail** — extends the indexed projectile use/loadout and damage attribution rails only if client-visible travel/collision evidence can be correlated with Valence server evidence.
2. **Armor loadout/enchantment matrix rail** — extends the indexed armor mitigation rail across more loadouts or modifiers without claiming full vanilla parity.
3. **Equipment slot/item matrix rail** — extends the indexed equipment update observation rail across additional slots and item types.

Any future slice should remain a fresh Cairn package with a live receipt, BLAKE3 hash, dry-run gate, and explicit non-claims rather than broadening existing combat receipts silently.
