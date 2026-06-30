# Design: Add Hyperion gameplay composition tests

## Context

Composable Bevy plugins need tests at the composition boundary, not only mechanics-level tests. The test matrix should assert which plugin combinations are valid, which are invalid, and which behaviors are non-claims.

## Decisions

### 1. Central composition matrix

**Choice:** Add focused tests for default presets, mode-only plugins, custom feature toggles, replacement hooks, additive custom plugins, and exclusive-mode failures.

**Rationale:** A matrix makes it easier to see which composition contracts are intentional.

### 2. Minimal Bevy apps

**Choice:** Use minimal `App` fixtures and pure planner fixtures where possible rather than full server/proxy startup.

**Rationale:** Composition regressions should fail quickly without standing up the world.

### 3. Positive and negative cases are paired

**Choice:** Every supported composition gets a nearby invalid or disabled configuration test.

**Rationale:** Composability is about failure modes as much as happy paths.

### 4. Evidence is reusable

**Choice:** Promote focused test logs and BLAKE3 manifests for archive review.

**Rationale:** The matrix should become durable evidence for later API changes.

## Risks / Trade-offs

- Tests that inspect Bevy internals can be brittle; prefer public resources/events/helpers where possible.
- Full default app builders may require proxy/crypto setup; use test-friendly binds or pure plans when appropriate.
- Some negative cases may depend on other Cairns landing first; gate them behind explicit dependency notes or helper assertions.
