# Proposal: Retire Hyperion checkout

## Why

The workspace no longer needs a live local Hyperion checkout after the selected Valence-owned integration surfaces and retirement gate were promoted. Keeping the nested checkout in the root layout creates stale ownership guidance, layout exceptions, and local-state risk. Because the parent repository does not track `mc/hyperion`, removal must be treated as a local workspace cleanup backed by durable evidence rather than a parent-repo deletion diff.

## What Changes

- Back up the nested Hyperion working-copy state into durable evidence before removal.
- Remove the Hyperion row from the component registry and layout summaries.
- Update agent and verification guidance so Hyperion is no longer described as an active local component root.
- Retire accepted Hyperion-local mode obligations into historical/reference status while preserving Valence-owned integration/non-claim language.
- Physically remove the untracked local Hyperion checkout after backup and live-reference cleanup.

## Impact

- **Files**: component registry, layout/docs/agent guidance, accepted Cairn specs, retirement evidence, and Cairn archive package.
- **Testing**: backup log, deletion log, layout validation, Cairn gates/validation, task-evidence validation, and evidence-manifest validation.
- **Non-claims**: this does not push or delete upstream Hyperion history, does not prove full Hyperion behavior is ported to Valence, does not remove historical archive/evidence references, and does not make Valence Hyperion-compatible.
