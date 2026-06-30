# Design: Add Hyperion game preset builder

## Context

The desired public shape is a simple composition surface such as `GamePreset::bedwars().without::<BowPlugin>().with(MyPlugin)`, but the exact Rust API must fit Bevy plugin typing and Hyperion's existing app builder functions.

## Decisions

### 1. Pure planning core

**Choice:** Represent preset selection, mode choice, default gameplay choice, feature toggles, replacement intents, and validation diagnostics in plain data before mutating `App`.

**Rationale:** Invalid combinations should be tested deterministically without Bevy world side effects.

### 2. Thin Bevy shell

**Choice:** The public builder applies a validated plan to `App` by adding Hyperion core, plugin groups, feature plugins, mode plugins, resources, proxy bind, and crypto in a boring shell.

**Rationale:** Bevy plugin installation is side-effectful; validation logic should stay outside that shell.

### 3. Default wrappers remain

**Choice:** Existing `build_game_app*` and `init_game*` entrypoints delegate to the default preset path.

**Rationale:** Existing users keep the current simple API while advanced users get explicit composition.

### 4. Typed diagnostics over panics

**Choice:** Invalid preset plans return deterministic errors for missing mode, duplicate exclusive mode, incompatible feature replacement, missing dependency, or unsupported custom plugin shape.

**Rationale:** Composition mistakes should fail early and reviewably.

## Risks / Trade-offs

- Rust type erasure around Bevy plugins may constrain the exact `without::<T>()` syntax; the Cairn requires the capability, not a final fluent spelling.
- Builder ergonomics can hide ordering; diagnostics and docs should expose the final plugin plan.
- Runtime plugin loading remains explicitly out of scope.
