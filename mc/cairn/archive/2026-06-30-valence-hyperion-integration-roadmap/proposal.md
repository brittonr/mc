# Proposal: Define Valence/Hyperion integration roadmap

## Why

A Valence/Hyperion merge should be a sequenced convergence through explicit seams, not a direct collapse of one project into the other. The accepted `valence-hyperion-integration` spec already records safety boundaries and several archived tracks for packet compose, proxy broadcast, cached chunk egress, and related adapter work. What is missing is one reviewable roadmap that explains ownership, dependencies, stop conditions, and the next implementation slices.

Without that roadmap, future work can accidentally reopen forbidden core-merge paths, duplicate archived integration work, or make unsupported claims about Hyperion compatibility, production scale, vanilla parity, or Valence default behavior.

## What Changes

- Inventory the accepted specs, archived integration Cairns, active game-mode Cairns, Hyperion architecture notes, Valence architecture notes, and `docs/hyperion-integration-boundaries.md`.
- Produce a Valence/Hyperion convergence roadmap that identifies Valence-owned, Hyperion-owned, adapter-owned, and reference-only responsibilities.
- Define dependency ordering for ownership audit, minimal bridge slice, optional proxy/backend work, optional gameplay plugins, and later consolidation decisions.
- Add decision-record requirements for adopt, port, reference, and reject classifications before implementation work uses Hyperion code or concepts.
- Keep default Valence behavior, Bedwars behavior, Hyperion runtime behavior, and mc-compat claims unchanged until later Cairns produce evidence.

## Impact

- **Files**: Cairn change artifacts, a roadmap/evidence document under `docs/evidence/` or `docs/`, and optional diagrams/manifests if promoted as evidence.
- **Testing**: Cairn proposal/design/tasks gates, Cairn validation, evidence-manifest checks for promoted docs, and no component test requirement unless implementation tasks are added later.
- **Non-claims**: this does not implement bridge code, merge workspaces, replace runtimes, change Valence defaults, change Hyperion defaults, prove production scale, or claim Hyperion compatibility.
