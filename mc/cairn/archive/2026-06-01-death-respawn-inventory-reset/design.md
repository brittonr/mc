# Design: Death respawn inventory reset rail

## Context

The `death inventory reset` row is currently a non-claim or bounded-gap item in the compatibility evidence set. This change creates a row-scoped Cairn so future implementation cannot silently broaden existing evidence.

## Decisions

### 1. Keep the row narrow

**Choice:** Claim only one configured death event with pre-death inventory, death/drop or reset policy, respawn, and post-respawn inventory state.

**Rationale:** Narrow rows are reviewable and keep broad compatibility claims false until every required row has evidence.

### 2. Compare normalized metrics

**Choice:** Checkers compare pre-death inventory slots, death cause, drop/reset policy, dropped item ids/counts, respawn inventory slots, and server correlation milestones.

**Rationale:** Raw logs and pass/fail alone do not prove the intended compatibility claim.

### 3. Fail closed on evidence gaps

**Choice:** Reject missing pre/post inventory metrics, wrong drop/reset state, missing respawn, unexpected score/capture side effect, or full lifecycle overclaim.

**Rationale:** Missing reference, telemetry, parser, or correlation data must block promotion instead of becoming implicit coverage.

### 4. Preserve adjacent non-claims

**Choice:** Docs and receipts keep these non-claims explicit: all death causes, all inventory policies, XP drops, item despawn timing, full death/respawn lifecycle, full CTF correctness, and production readiness.

**Rationale:** Each Cairn reduces one named gap without smuggling in full protocol, gameplay, production, or security claims.

## Risks / Trade-offs

- CTF and survival inventory reset policies can differ; the row must name its fixture mode.
- Drops need deterministic cleanup to avoid stale pickup evidence.
