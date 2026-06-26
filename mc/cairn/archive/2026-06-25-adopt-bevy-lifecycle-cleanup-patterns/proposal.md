# Proposal: Adopt Bevy lifecycle cleanup patterns

## Why

Valence and its examples use explicit cleanup systems for disconnected clients, despawned entities, open containers, GUI viewers, fixture state, and stale task/result state. Some cleanup is required because Valence uses explicit `Despawned` markers, but other cleanup can be made clearer with Bevy lifecycle patterns such as component ownership, removal/change detection, lifecycle-specific cleanup systems, and documented cleanup sets.

## What Changes

- Inventory selected cleanup paths, owners, triggers, stale-state risks, schedule phases, and evidence impact.
- Classify cleanup as component lifecycle, explicit `Despawned` marker cleanup, removal detection, resource/index cleanup, or external I/O cleanup.
- Migrate selected cleanup to Bevy lifecycle patterns or named cleanup sets where this preserves behavior.
- Keep cleanup that must stay explicit documented with rationale.
- Add positive cleanup tests and negative stale entity, duplicate cleanup, missing owner, reconnect, and plugin-disabled tests.

## Impact

- **Files**: selected cleanup systems in examples, `valence_inventory::gui`, server/client/entity cleanup modules, focused tests, and evidence docs.
- **Testing**: focused lifecycle tests, selected example/compatibility rails when behavior changes, schedule hygiene when cleanup sets change, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not change Valence's explicit despawn model, add automatic recursive cleanup everywhere, or broaden production safety claims.
