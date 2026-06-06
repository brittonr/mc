# Design: Promote movement packet-family evidence

## Context

Movement packets are central to client/server compatibility but broad movement correctness is too large. This row should validate one configured movement packet path and server correlation only.

## Decisions

### 1. Use one deterministic movement transition

**Choice:** Move `compatbot` from a configured start position to one configured target position/look/on-ground state.

**Rationale:** Exact coordinates make the row reviewable and avoid physics breadth.

### 2. Promote only observed packet variants

**Choice:** Promote only the movement packet variant actually sent and correlated, such as position-only or full position/look.

**Rationale:** Protocol 763 has multiple movement packet shapes; they should not inherit coverage from one another.

### 3. Exclude anti-cheat and physics

**Choice:** Validate packet parsing/correlation, not collision, speed limits, anti-cheat, latency, or malicious behavior.

**Rationale:** Those require separate rows and safety contracts.

## Risks / Trade-offs

- Movement can be noisy; the fixture must isolate exactly one expected transition.
- Float tolerance must be named in the contract if coordinates are compared.
- Existing gameplay rows may already move clients, but implicit movement should not be promoted without normalized metrics.
