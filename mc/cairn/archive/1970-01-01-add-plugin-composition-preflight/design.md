# Design: Add plugin composition preflight

## Context

Hyperion already has pure preset validation for several rules, but builder and direct plugin paths still rely on plugin-build assertions for some dependency or duplicate cases. The app builder should be able to reject invalid composition before adding Hyperion core, default gameplay, mode plugins, proxy resources, or custom plugins.

## Decisions

### 1. Validate all builder-controlled composition first

**Choice:** A preflight core validates explicit composition requests and returns either an ordered plan or typed diagnostics before the builder creates or mutates a Bevy `App`.

**Rationale:** Users should get a deterministic error and no partial app state for invalid builder requests.

### 2. Model diagnostics as data

**Choice:** Preflight errors name missing mode, duplicate mode, unsupported replacement, dependency gap, duplicate plugin/slot, and partial-app-prevention conditions as typed variants.

**Rationale:** Tests and future tooling can match diagnostics without parsing panic strings.

### 3. Keep direct plugin guards as deterministic fallback

**Choice:** Direct mode/feature plugin addition can still assert when Bevy requires infallible `Plugin::build`, but assertions should be narrow, named, and covered by tests.

**Rationale:** Bevy plugin APIs do not provide a universal fallible path, but diagnostics can still be clear.

### 4. Separate preflight from executable slot installation

**Choice:** Preflight validates semantic composition and slot metadata. Actual plugin installation remains a shell step after preflight succeeds.

**Rationale:** This keeps the pure core deterministic and avoids storing Bevy plugin values in planner data.

## Risks / Trade-offs

- Moving more checks before app creation may require reshaping existing builder APIs to return plan diagnostics earlier.
- Some resources like proxy bind and crypto are shell inputs; preflight should validate only composition, not network availability.
- Direct plugin misuse may still panic; document this boundary and ensure the builder path returns `Result`.
