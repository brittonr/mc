## Context

The accepted roadmap records crafting/recipes as `VanillaCraftingPlugins`, with candidate `CraftingRecipeCorePlugin`, `CraftingTableShellPlugin`, and `RecipeBookSyncPlugin` slices. It requires positive and negative recipe-core tests, target-version recipe data, selected Paper parity before claims, and explicit non-claims for all data packs, all recipes, and automated crafter behavior.

The compatibility side already has bounded evidence for `survival-crafting-recipe-breadth`: paired Paper/reference and Valence receipts for a shaped chest recipe, a shapeless oak-planks recipe, and an invalid insufficient stick-input rejection, plus typed-event readiness for that row. Those artifacts prove a harness row only. They do not define a reusable pure recipe-matching core, a Valence shell boundary, target-version data loading rules, or plugin ownership.

The accepted `vanilla-composable-plugins` spec already requires wiki-guided implementation work to use behavior cards, functional core / Bevy shell separation, positive and negative tests, and non-overclaiming evidence.

## Decisions

### 1. Start with a behavior card, not a recipe core

**Choice:** The next target is a behavior card and focused validator. It should not implement recipe matching, add a Valence plugin, register systems, change scenario rails, or update `DefaultPlugins`.

**Rationale:** Existing crafting receipts are useful predecessor evidence, but implementation needs a reviewable pure-core/shell contract first. A card lets reviewers agree on selected rows, data assumptions, error cases, schedule boundaries, and non-claims before code hardens those decisions.

### 2. Bound the selected recipe matrix to existing receipt rows

**Choice:** The card should use the existing crafting recipe-breadth row as predecessor vocabulary: one shaped chest recipe, one shapeless oak-planks recipe, one invalid insufficient-input stick rejection, and the configured primary-click collection mode.

**Rationale:** These rows already have paired receipt evidence and typed-event rails, so the card can be specific without requiring a new live run. The card must still state that receipt evidence does not equal a reusable core implementation or all-recipe breadth.

### 3. Keep recipe decisions in a pure core

**Choice:** The behavior card must define a pure deterministic recipe-matching core over in-memory crafting-grid values, selected target-version recipe rows, output-slot state, and a collection request. The core should return a match/result diagnostic, a rejected/no-result diagnostic, or a typed malformed-data error without filesystem, network, Bevy world, packet, logging, or wall-clock side effects.

**Rationale:** Recipe matching is testable without starting Valence or connecting a client. Keeping this as the card's implementation boundary preserves the repository-wide functional core / imperative shell rule.

### 4. Keep Bevy/ECS shell ownership thin and opt-in

**Choice:** The future shell boundary should own inventory snapshots, click/event adaptation, ECS mutation, schedule registration, packet/client-visible updates, and diagnostics, while delegating all recipe decisions to the pure core. The card should require explicit opt-in plugin membership and disabled-plugin behavior before runtime claims.

**Rationale:** Crafting interacts with inventory click phases and presentation updates. Schedule-sensitive mutation should be planned separately and not inferred from row receipts.

### 5. Fail closed on incomplete cards and overclaims

**Choice:** The validator should accept a complete behavior card and reject missing source scope, target Java/protocol scope, selected recipe matrix, pure-core boundary, shell boundary, positive tests, negative tests, evidence requirements, stop conditions, or required non-claims. It should also reject claims for all recipes, data-pack loading, recipe-book UI behavior, automated crafter behavior, DefaultPlugins membership, broad vanilla parity, public-server safety, or production readiness.

**Rationale:** Negative validation prevents a documentation-only card from becoming permission for broad crafting work.

## Risks / Trade-offs

- The existing crafting receipts may not contain enough target data provenance for future core implementation. This package stops at behavior-card scope and requires extracted target-version recipe rows before stronger vanilla behavior claims.
- A behavior-card checker validates review shape, not runtime behavior. Future recipe core and shell work still need their own focused positive/negative tests and evidence.
- Crafting-table inventory APIs and schedule phases may require additional Valence source inspection during implementation. The card records these as shell prerequisites rather than assuming exact runtime APIs.
- The selected matrix intentionally leaves recipe-book sync, data packs, automated crafter behavior, arbitrary collection modes, shift-click/drag/split semantics, and all-recipe breadth out of scope.
