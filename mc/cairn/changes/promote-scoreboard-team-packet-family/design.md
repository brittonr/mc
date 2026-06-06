# Design: Promote scoreboard/team packet-family evidence

## Context

CTF gameplay rows provide bounded team and score transitions, but packet-family claims should be explicit. This change should map one packet row such as `TeamS2CPacket`, `ScoreboardObjectiveUpdateS2CPacket`, `ScoreboardDisplayS2CPacket`, or `ScoreboardPlayerUpdateS2CPacket` to deterministic evidence.

## Decisions

### 1. Use existing CTF context when sufficient

**Choice:** Prefer existing CTF team/score evidence if normalized packet metrics can be extracted; otherwise add a small fixture.

**Rationale:** Existing CTF rows already have server-side semantics and reviewable receipts.

### 2. Promote exact packet rows only

**Choice:** Mark only the packet row or rows with direct evidence.

**Rationale:** Scoreboard/team has many update shapes that should not inherit coverage.

### 3. Exclude UI and full CTF correctness

**Choice:** Validate packet observation/correlation, not scoreboard UI rendering or all game-rule semantics.

**Rationale:** Those are separate claims.

## Risks / Trade-offs

- Existing CTF receipts may not expose packet-level fields; a fixture may be needed.
- Multiple scoreboard packet shapes are easy to conflate; the contract must enumerate exact rows.
- UI parity should remain a non-claim unless screenshot/semantic checks are added.
