# Design: Clarify Cairn policy ownership and path

## Context

The workspace has native Cairn lifecycle content under `cairn/` and generated policy under `cairn-policy/`. Repo notes mention schema compatibility with a pinned Cairn binary. The layout should make this relationship explicit.

## Decisions

### 1. Document before moving

**Choice:** First document why the policy path exists, what generates it, and which binary/schema constraints apply.

**Rationale:** Moving policy paths without understanding pinned compatibility could break validation.

### 2. Treat generated policy as checked artifact

**Choice:** The generated policy JSON remains checked and must have a freshness/schema compatibility check.

**Rationale:** Cairn gates depend on deterministic policy behavior.

### 3. Move only with compatibility proof

**Choice:** If moving under `cairn/` is desirable, do it only after a dry-run and validation prove the pinned repo Cairn accepts the new path or configured path.

**Rationale:** Layout cleanup must not break lifecycle tooling.

### 4. Keep owner and regeneration command visible

**Choice:** Docs should name who owns the policy file, how to regenerate it, and what check proves freshness.

**Rationale:** Generated policy drift is otherwise hard to diagnose.

## Risks / Trade-offs

- Keeping top-level `cairn-policy/` preserves clutter; mitigate with documentation and ownership.
- Moving can break pinned tooling; mitigate with compatibility gates before path change.
- Policy generation may depend on external Cairn versions; mitigate by using repo-pinned commands.
