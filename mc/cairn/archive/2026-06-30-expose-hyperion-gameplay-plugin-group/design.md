# Design: Expose Hyperion gameplay plugin group

## Context

`CommonGameplayPlugin` currently installs attack, block, bow, chat, damage, regeneration, skin, spawn, stats, vanish, command, map, item, permission, and proxy plugins from inside `events/bedwars/src/lib.rs`. It is useful composition, but private and mode-owned.

## Decisions

### 1. Inventory before exposing API

**Choice:** Record each current default gameplay plugin, its schedule/resource/event dependencies, and whether it is mode-neutral or Bedwars-specific before making it public.

**Rationale:** A public composition surface should not accidentally bless private ordering assumptions or Bedwars-only mechanics as reusable defaults.

### 2. Public default group, individual public feature handles

**Choice:** Add a public `DefaultGameplayPlugins`-style Bevy `PluginGroup` and stable re-exports for feature plugins that are safe to compose directly.

**Rationale:** Bevy users expect plugin groups to be configurable and individual plugins to be addressable for replacement or disabling.

### 3. Compatibility first

**Choice:** Existing app builders and mode plugins continue to install the same default behavior until the mode-only Cairn changes that ownership.

**Rationale:** This Cairn creates the public seam without changing runtime mode semantics.

### 4. Test the seam, not every mechanic

**Choice:** Focus tests on plugin group installation, public paths, disable/replace behavior, and dependency diagnostics; mechanics retain their existing tests.

**Rationale:** The risk is API composition drift, not full gameplay parity.

## Risks / Trade-offs

- Exposing too much may freeze internals prematurely; inventory should classify private features explicitly.
- `PluginGroup` ordering can hide dependencies; tests and documentation should state stable ordering boundaries.
- Public re-exports may need follow-up migration once common gameplay moves to a shared crate.
