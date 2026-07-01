# Design: Execute Hyperion preset plugin slots

## Context

`GamePreset` exposes replacement and custom plugin intent lists. `plan_game_preset` validates those lists, and `build_game_app_from_preset_and_proxy` disables replacement targets by folding replacements into disabled features. The shell has no typed plugin value to install for a replacement or custom plugin, so custom composition is only partially represented.

## Decisions

### 1. Split semantic planning from plugin installation

**Choice:** The pure planner validates mode, default gameplay inclusion, feature disables, replacement targets, custom plugin identities, dependency rules, and deterministic ordering. The shell receives typed plugin-slot installers and mutates the Bevy `App` only after the plan is valid.

**Rationale:** The core must remain testable without Bevy, while installing plugins is inherently imperative shell behavior.

### 2. Use typed plugin slots instead of names-only execution

**Choice:** Replacement and custom additions use typed slot descriptors that include a stable diagnostic name and a shell-owned installer for the Bevy plugin.

**Rationale:** A name alone is useful for receipts but cannot execute. A typed slot prevents the API from implying that arbitrary strings become plugins.

### 3. Define replacement ordering explicitly

**Choice:** Replacements disable the selected default feature and install the replacement at the same documented composition boundary or after the disabled feature anchor where Bevy permits it.

**Rationale:** Replacement behavior must be deterministic and reviewable, not dependent on anonymous tuple order.

### 4. Preserve compatibility wrappers

**Choice:** Existing default app-building functions remain wrappers around the default preset with no explicit slots.

**Rationale:** Existing callers should keep the same default behavior while new callers opt into executable custom composition.

## Risks / Trade-offs

- Bevy plugin groups are type-driven, so the shell may need a small trait or closure boundary for slot installers.
- Replacement ordering can be constrained by Bevy's plugin group APIs; unsupported replacement positions should fail during preflight rather than silently install late.
- Public API naming must make clear that dynamic runtime plugin loading is still a non-claim.
