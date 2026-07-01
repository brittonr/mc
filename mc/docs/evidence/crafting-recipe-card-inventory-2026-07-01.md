# Crafting recipe behavior-card inventory — 2026-07-01

This inventory satisfies `r[vanilla_composable_plugins.crafting_recipe_card.inventory]` before drafting the crafting recipe behavior card. It explicitly records the roadmap row, accepted behavior-card policy, and unresolved target-version recipe JSON prerequisite.

## Roadmap crafting row

`docs/vanilla-composable-plugins-roadmap.md` maps the Minecraft Wiki crafting/recipe domain to `VanillaCraftingPlugins` with candidate `CraftingRecipeCorePlugin`, `CraftingTableShellPlugin`, and `RecipeBookSyncPlugin` slices. The roadmap target scope is Java Edition 1.20.1 / protocol 763 unless a later Cairn changes it. Its first bounded crafting slice is shaped/shapeless recipe matching for a small extracted recipe set, with explicit stop conditions before all-recipe breadth, data-pack loading, or automated crafter behavior.

## Accepted behavior-card, core/shell, and evidence policy

Accepted `cairn/specs/vanilla-composable-plugins/spec.md` requires wiki-guided plugin work to:

- inventory source pages, target edition/version/protocol, extracted-data sources, Valence surfaces, and compatibility rails before deriving implementation slices;
- use behavior cards for follow-on implementation work;
- keep vanilla rule decisions in pure deterministic cores and side effects in thin Bevy/ECS shells;
- include positive and negative tests; and
- require extracted-data checks or Paper/vanilla parity receipts before claiming target-version vanilla behavior.

The active change delta adds the dedicated crafting recipe behavior-card, validation, docs, and closeout requirements.

## Accepted crafting recipe-breadth compatibility requirements

Accepted `cairn/specs/mc-compatibility/spec.md` includes the bounded `survival-crafting-recipe-breadth-parity` contract and checker requirements. The accepted row fixes one shaped recipe, one shapeless recipe, one invalid or insufficient-input rejection, one configured collection mode, normalized recipe/input/result/inventory metrics, deterministic Paper/reference versus Valence comparison, reviewable receipts under `docs/evidence/`, and narrow promotion language that keeps broader crafting as a non-claim.

Accepted typed-event migration requirements for `survival_crafting_recipe_breadth_typed_event_migration` move the row to typed-event-ready only when structured recipe events cover the required client milestones, Valence recipe-breadth server milestones, forbidden surfaces, and ordered valid/invalid crafting phases.

## Archived predecessor evidence

- Archived `2026-06-20-survival-crafting-recipe-breadth-parity` provides paired Paper/reference and Valence receipts for the bounded row.
- `docs/evidence/survival-crafting-recipe-breadth-receipts-2026-06-20.md` records:
  - shaped `minecraft:chest` from eight `OakPlanks` inputs in slots `1,2,3,4,6,7,8,9`, result `Chest x1`, collected by primary click into inventory slot `36`;
  - shapeless `minecraft:oak_planks` from one `OakLog` input in slot `1`, result `OakPlanks x4`, collected by primary click into inventory slot `37`;
  - invalid `minecraft:stick_insufficient_input_rejection` with one `OakPlanks` input in slot `1`, empty result slot, and `no_result` outcome; and
  - non-claims for all recipes, recipe-book UI behavior, recipe discovery breadth, arbitrary collection modes, shift-click/drag/split semantics, full survival compatibility, broad vanilla parity, public-server safety, production readiness, and semantic equivalence.
- Archived `2026-06-22-survival-crafting-recipe-breadth-typed-event-migration` provides typed-event readiness evidence for the same row; the retained scope is observability/pass-fail structure, not a recipe-core implementation.

## Wiki source reads used as guidance

Retrieved on 2026-07-01 with `crw_scrape` as untrusted external guidance only:

- Crafting: https://minecraft.wiki/w/Crafting — current page describes manual crafting, 2×2 and 3×3 grids, shaped/shapeless/fixed vocabulary, recipe book, automated crafter, and recipe-system entry points. Target use is vocabulary and seam discovery for Java Edition 1.20.1 / protocol 763; version-drift risk is high because current pages include newer behavior and automated-crafter text beyond this first slice.
- Recipe (Java Edition): https://minecraft.wiki/w/Recipe_(Java_Edition) — current page describes Java recipe JSON, `minecraft:crafting_shaped`, `minecraft:crafting_shapeless`, result counts, ingredients, and data-pack replacement behavior. Target use is data-shape vocabulary; version-drift risk is high because current recipe fields and data-pack behavior can drift beyond 1.20.1.
- Java Edition 1.20.1: https://minecraft.wiki/w/Java_Edition_1.20.1 — target-version checkpoint; page describes 1.20.1 as a Java Edition hotfix compatible with 1.20 servers.
- Protocol version: https://minecraft.wiki/w/Protocol_version — protocol vocabulary and protocol `763` row for Java Edition 1.20.1; version table is mutable and must be cross-checked with local protocol constants before claims.

Wiki text is not authoritative evidence. Future implementation claims still require target-version extracted recipe rows, focused positive/negative tests, and Paper/reference or accepted vanilla-reference evidence.

## Target scope and unresolved prerequisites

Target scope for this behavior card is Java Edition 1.20.1 / protocol 763, bounded to the existing selected recipe matrix and primary-click collection mode. Before any recipe-core or Valence runtime claim is promoted, follow-on work must provide target-version extracted recipe JSON rows for `minecraft:chest`, `minecraft:oak_planks`, and the rejected stick-input case, plus named stack/count limits and malformed-data validation.

Unresolved prerequisites intentionally left for follow-on implementation:

- no reusable pure recipe-matching core exists yet;
- no Valence Bevy/ECS crafting shell boundary is implemented;
- no all-recipe extraction or data-pack loader exists for this slice;
- no recipe-book UI behavior is scoped;
- no automated crafter behavior is scoped;
- no arbitrary collection modes, shift-click, drag, or split semantics are scoped; and
- no default plugin membership change is planned by this behavior-card package.

## Boundary decision

The predecessor row evidence is useful selected-row vocabulary and compatibility receipt evidence. It is not evidence of a reusable pure core, a Valence runtime shell, all-recipe breadth, recipe-book behavior, data-pack loading, public-server safety, production readiness, broad Minecraft compatibility, broad vanilla parity, or semantic equivalence.
