# Design: Scenario family modules

## Context

The runner already has typed Nickel scenario manifests and generated tables, but hand-authored scenario behavior remains centralized. The split should make adding or reviewing one family local while keeping static validation strong enough to catch drift.

## Decisions

### 1. Split by scenario family

**Choice:** Move behavior constants, run strategy helpers, env intents, and validation helpers into family modules for CTF, inventory, survival, combat/projectile/equipment, negative rails, MCP, and targeted-packet live capabilities.

**Rationale:** Scenario families have different owners and evidence boundaries. Focused modules reduce accidental cross-family edits.

### 2. Keep public scenario API stable

**Choice:** Preserve the current `Scenario`, `ScenarioSpec`, `ScenarioBehaviorKind`, lookup helpers, and generated table consumers as compatibility façades while extracting implementation details.

**Rationale:** Runner, receipt, planning, and tests should not need a broad API rewrite just to split files.

### 3. Generate or validate duplicated rows

**Choice:** Prefer generated rows from `compat/config/scenario-manifest.ncl`; where human-authored behavior remains, require parity checks against generated manifest data.

**Rationale:** Scenario names, aliases, milestones, and wrapper metadata are already duplicated across many surfaces.

### 4. Preserve live capability contracts

**Choice:** Keep targeted packet/live capability contracts in a focused module with explicit non-claim and blocker validation.

**Rationale:** Those rows are evidence-sensitive and should remain fail-closed.

## Risks / Trade-offs

- Over-splitting can obscure the complete scenario inventory; keep a central façade/table that reviewers can inspect.
- Generated and hand-authored data may drift during the split; generated-surface checks are required.
- Scenario family extraction can affect dry-run output indirectly through behavior helpers; representative dry-runs should cover each touched family.
