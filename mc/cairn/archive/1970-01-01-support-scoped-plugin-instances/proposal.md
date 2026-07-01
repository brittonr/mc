# Proposal: Support scoped plugin instances

## Why

Valence gameplay examples now carry scope concepts, but many paths still assume one static primary arena or one global plugin contract per plugin name. That is enough for current compatibility fixtures, but it limits multiple arenas, multiple instances of the same mode, or same-app combinations where events and state must not cross scopes.

## What Changes

- Define an explicit plugin-instance identity model for gameplay plugins and compatibility fixtures.
- Move selected per-mode/per-arena state toward arena-, layer-, or instance-owned state handles instead of single global resources where coexistence matters.
- Include instance identity in events, diagnostics, milestones, and contract metadata when the same semantic can occur in more than one scope.
- Preserve current single-primary fixture behavior through compatibility adapters.

## Impact

- **Files**: Valence gameplay contract helpers, CTF/survival/terrain scope wiring, selected event or milestone payloads, tests, and evidence docs.
- **Testing**: positive multiple-instance or multi-mode scope tests, negative wrong-scope/stale-scope/missing-scope tests, compatibility fixture checks, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests.
- **Non-claims**: this does not make every Valence example production-multi-tenant, add default gameplay, or promise vanilla parity; it establishes scoped instance seams for selected plugins.
