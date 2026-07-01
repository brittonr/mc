# Design: Minecraft Wiki-guided composable plugin roadmap

## Context

The Minecraft Wiki front page exposes the same domain seams we need for composable server mechanics: blocks, items, mobs, biomes, effects, crafting, smelting, smithing, redstone, commands, and protocol documentation. Valence already exposes many low-level plugins and Bevy `PluginGroup` composition surfaces, while compatibility evidence in this repository proves bounded protocol-763 rows rather than broad Minecraft parity.

The roadmap should use the wiki as a discovery and vocabulary guide, then require extracted game data and Paper/vanilla receipts for claims. Current Valence compatibility work usually targets Java Edition `1.20.1` / protocol `763`, so wiki pages describing newer releases must be treated as candidates until version scoped.

## Decisions

### Use the wiki as an index, not as sole authority

**Choice:** The roadmap will treat Minecraft Wiki pages and categories as domain discovery inputs and behavior summaries, not as the final authority for implementation claims.

**Rationale:** Wiki pages can change, can describe newer releases, and are community-authored. Version-scoped extracted data and Paper/vanilla parity receipts are better claim evidence.

### Add a dedicated capability instead of overloading existing specs

**Choice:** Create a `vanilla-composable-plugins` spec for roadmap, behavior-card, and plugin-sequencing rules.

**Rationale:** `valence-bevy-ecs` owns schedule and ECS hygiene, while `mc-compatibility` owns evidence rails. Wiki-guided plugin planning crosses both but should have its own lifecycle contract.

### Use behavior cards as implementation handoff artifacts

**Choice:** Every follow-on plugin slice should start from a behavior card that records source pages, version target, dependency plugin set, pure core, shell systems, tests, schedule impact, parity evidence, and non-claims.

**Rationale:** Behavior cards keep future changes small, reviewable, and resistant to accidental broad vanilla-compatibility claims.

### Preserve functional core / imperative shell boundaries

**Choice:** Wiki-derived rules should be implemented as pure deterministic cores first, with Bevy systems handling ECS state, I/O, packets, logs, and scheduling.

**Rationale:** Pure cores allow positive and negative tests without a live server, while thin shells keep plugin composition and schedule evidence easy to inspect.

### Sequence plugins by bounded proof value

**Choice:** The roadmap should prioritize bounded survival and CTF-compatible seams already near existing evidence rails before high-complexity domains such as full redstone or broad mob AI.

**Rationale:** Crafting, smelting, hunger/health, equipment, block interaction, block entities, and projectile/combat attribution can reuse existing client/server receipts. Redstone and broad AI need deeper architecture and should not block near-term composable plugin work.

## Risks / Trade-offs

- Wiki-guided planning can drift from the target game version unless every behavior card records edition/version scope.
- A roadmap adds up-front documentation, but it prevents expensive plugin boundary churn.
- Extracted data and Paper receipts increase validation cost, but they are necessary before claiming vanilla behavior.
- License/attribution rules may limit vendoring wiki-derived text or tables; roadmap artifacts should cite pages and summarize decisions rather than copy large page bodies.
