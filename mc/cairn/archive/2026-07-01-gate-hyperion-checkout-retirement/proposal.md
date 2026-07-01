# Proposal: Gate Hyperion checkout retirement

## Why

Selected Hyperion concepts have been promoted into Valence-owned integration surfaces, but the local `hyperion/` checkout still has independent repo ownership, accepted Cairn requirements, docs/config references, and recent focused evidence. Removing it without a retirement gate would destroy local state and make the deletion unreviewable from the parent repo because `mc/hyperion` is not parent-tracked.

## What Changes

- Record a Hyperion retirement readiness audit as promoted evidence.
- Define a required gate before any future physical deletion of the `hyperion/` checkout.
- Preserve the checkout in this change because live blockers remain.
- Clarify that a future deletion must prove accepted specs, docs/config, evidence, and nested repo state no longer depend on the checkout.

## Impact

- **Files**: Cairn change package, Valence/Hyperion integration spec delta, and reviewable audit evidence under `docs/evidence/`.
- **Testing**: focused audit log, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest validation.
- **Non-claims**: this does not delete `hyperion/`, port all Hyperion behavior into Valence, retire Hyperion evidence, or prove Hyperion compatibility through Valence.
