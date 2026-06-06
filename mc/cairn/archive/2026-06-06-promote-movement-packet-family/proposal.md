# Proposal: Promote movement packet-family evidence

## Why

Player movement packet rows are high-volume and currently unpromoted as a packet family. A deterministic local movement correlation row would cover an important serverbound seam while avoiding anti-cheat, physics, or security claims.

## What Changes

- Add one bounded movement packet-family row for a configured position/look/on-ground transition.
- Require client movement action milestones and Valence server correlation for the normalized movement fields.
- Promote only the configured movement packet row or rows, keeping all movement physics, anti-cheat, collision, latency tolerance, malicious-client resilience, full protocol coverage, and production readiness as non-claims.

## Impact

- **Files**: Stevenarella movement probe, Valence fixture instrumentation, runner scenario metadata, packet inventory/current bundle docs, checker, evidence artifacts, and Cairn specs/tasks.
- **Testing**: positive/negative checker fixtures, focused scenario tests, packet inventory/current-bundle checks, evidence manifests, task-evidence gate, and Cairn validation.
