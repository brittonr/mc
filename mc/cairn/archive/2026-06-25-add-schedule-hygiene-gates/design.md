# Design: Add Bevy schedule hygiene gates

## Context

Bevy schedule bugs can be hard to spot from code review alone. Valence already exposes named sets and a schedule dump tool, so schedule evidence can be a small, focused artifact rather than a broad manual inspection.

## Decisions

### 1. Gate only schedule-impacting changes

**Choice:** Require schedule evidence for changes that add plugins, schedules, system sets, ordering constraints, event-loop phases, or default plugin membership.

**Rationale:** Routine non-schedule changes should not inherit heavy graph evidence.

### 2. Prefer focused schedule receipts

**Choice:** Evidence should name the schedule, plugin configuration, command, and expected sets/systems. Full SVG output is optional unless reviewers need it.

**Rationale:** Reviewers need stable facts, not necessarily large graph artifacts.

### 3. Compare optional/default behavior

**Choice:** Optional plugin changes should include plugin-enabled and plugin-disabled schedule/resource assertions when disabled behavior is part of the contract.

**Rationale:** Many Bevy improvements rely on opt-in behavior staying opt-in.

### 4. Ambiguity is explicit

**Choice:** Ambiguities are either removed, allowed with documented rationale, or captured as non-blocking when Bevy cannot infer safe parallelism.

**Rationale:** Schedule graphs should not imply determinism that the system does not provide.

## Risks / Trade-offs

- Schedule graph output can be noisy; keep checks focused on selected labels and sets.
- Some ambiguity is acceptable in ECS; do not require artificial ordering without a correctness need.
- Nix evidence checks see tracked files; promoted schedule artifacts must live under reviewable tracked paths when cited.
