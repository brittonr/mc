# Design: Add a profile and skin cache with rate limiting

## Context

Hyperion's `MojangClient` provides provider-specific URLs, rate limiting, and profile parsing, while `LocalDb` stores skins. Valence already performs authentication/profile work during login. A Valence integration should make provider and cache policy explicit and test parsing without network access.

## Decisions

### 1. Configuration owns provider policy

**Choice:** API provider endpoints, request budgets, time windows, cache storage paths, TTLs, and fallback behavior are typed configuration values.

**Rationale:** Hard-coded provider decisions and paths are unsuitable for a general framework.

### 2. Pure parsing and cache decisions

**Choice:** JSON/profile parsing, cache hit/miss decisions, staleness decisions, and rate-limit admission are pure cores over explicit inputs. HTTP clients and storage backends are shells.

**Rationale:** Network-independent fixtures can cover provider edge cases.

### 3. Cache backend is pluggable

**Choice:** Start with an in-memory or simple local adapter behind a trait/config boundary. Heed or another backend can be optional.

**Rationale:** Server operators may already own persistence.

### 4. Privacy and retention are documented

**Choice:** Docs must describe stored identifiers, skin/profile properties, retention controls, and deletion or disable behavior.

**Rationale:** Profile caches persist player-related data.

## Risks / Trade-offs

- Provider policies may change; keep diagnostics explicit and configs updateable.
- Cache staleness can show old skins; document TTL/fallback behavior.
- Storage corruption can affect login; fail closed or fall back according to configured policy.
