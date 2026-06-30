# Design: Valence/Hyperion integration roadmap

## Context

Valence and Hyperion share Minecraft-server goals and Bevy/ECS vocabulary, but they optimize for different ownership models. Valence is a modular framework and protocol foundation. Hyperion is a high-scale game engine/proxy stack with event-specific gameplay. The safe merge path is to converge through bounded APIs, adapters, optional plugins, and evidence-backed behavior claims.

Existing Cairn history already covers important pieces: Hyperion integration boundaries, packet composition, proxy broadcast routing, cached chunk egress, paletted container optimization evaluation, byte-backed protocol planning, and optional gameplay/plugin composition. A roadmap should connect these pieces and prevent future work from treating archived slices as a license for wholesale runtime import.

## Decisions

### Preserve project ownership boundaries

**Choice:** Treat Valence public APIs and compatibility surfaces as Valence-owned, Hyperion runtime/proxy/game-mode internals as Hyperion-owned, and bridge code as adapter-owned until a later accepted change promotes a specific API.

**Rationale:** This preserves Valence's stable, modular expectations while letting Hyperion inform high-scale design without copying unsafe, nightly-only, or game-specific internals into core.

### Sequence convergence through evidence gates

**Choice:** The roadmap orders future work by prerequisites: inventory and ownership first, adapter contracts second, minimal bridge slice third, optional backend/gameplay expansion later, and workspace/package consolidation only after compatibility and maintenance evidence exists.

**Rationale:** A minimal bridge can prove that the projects interoperate without committing to a full repository or runtime merge.

### Use decision records for every inspected source

**Choice:** Each future implementation Cairn records adopt, port, reference, or reject decisions for inspected Hyperion sources and identifies the Valence target or explicit non-target.

**Rationale:** This aligns with `docs/hyperion-integration-boundaries.md` and keeps forbidden core imports visible during review.

### Keep roadmap artifacts reviewable

**Choice:** The roadmap should be checked into `docs/` or promoted under `docs/evidence/` with BLAKE3 manifests when tasks cite it as closeout evidence.

**Rationale:** Cairn task closeout must be reproducible without relying on transient notes or untracked target outputs.

## Risks / Trade-offs

- A roadmap delays code, but it reduces the risk of irreversible architectural coupling.
- Existing archived Cairns may be stale relative to current source; the roadmap must label stale evidence instead of silently reusing it as current proof.
- Calling the work a "merge" can imply full compatibility or runtime replacement. The roadmap should consistently use bounded convergence language unless a later accepted change proves broader claims.
