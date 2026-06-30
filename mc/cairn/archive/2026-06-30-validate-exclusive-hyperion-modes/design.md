# Design: Validate exclusive Hyperion modes

## Context

`add_game_type_plugins` currently chooses one mode from `GameType`, but direct Bevy usage could add more than one mode plugin. Composability should support feature plugins, not ambiguous global mode state.

## Decisions

### 1. Explicit exclusivity model

**Choice:** Represent selected world mode as an exclusive mode identity and distinguish it from additive gameplay feature plugins.

**Rationale:** A global world-mode resource cannot safely represent multiple active modes without a larger arena/multi-world design.

### 2. Pure validation first

**Choice:** Implement duplicate-mode checks as pure validation over selected mode identities and feature classifications, then call that from plugins/builders.

**Rationale:** The rules are small and should be testable without Bevy side effects.

### 3. Fail before ambiguous mutation

**Choice:** Preset builders should reject invalid mode combinations before mutating `App`; direct plugin addition should provide deterministic diagnostics or a guarded startup check.

**Rationale:** Silent last-write-wins mode state is hard to debug.

### 4. Additive features remain allowed

**Choice:** Shared gameplay plugins, utility plugins, and user-provided feature plugins remain composable with one exclusive mode.

**Rationale:** The goal is safe composability, not blocking customization.

## Risks / Trade-offs

- Bevy's plugin API does not naturally return errors from all install paths; direct plugin diagnostics may need assertions, resources, or startup validation.
- Future multi-world or multi-arena support may need a different model; this Cairn documents that as a non-claim.
- Tests must cover both builder validation and direct plugin misuse where practical.
