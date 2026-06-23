# Design: Add a repository layout guard

## Context

The repo already has evidence and manifest guards. Layout issues are currently caught by human review or specialized checks. A dedicated layout guard should be small, deterministic, and based on the component registry when available.

## Decisions

### 1. Pure core, thin shell

**Choice:** The guard core receives an in-memory file tree/component registry model and returns diagnostics. The shell walks the filesystem and renders output.

**Rationale:** Layout rules can be tested without touching the real repo.

### 2. Start advisory or focused

**Choice:** Introduce the guard as a focused check, then make selected diagnostics required once false positives are resolved.

**Rationale:** The current tree has known transition states and active cleanup Cairns.

### 3. Guard layout, not semantics

**Choice:** The guard reports structural issues such as undocumented roots, nested Git exceptions, transient artifacts, missing local notes, and generated marker drift. It does not claim compatibility correctness.

**Rationale:** Scope must remain clear and fast.

### 4. Reuse registry and ignore contracts

**Choice:** When the component registry and artifact-boundary rules exist, use them as guard inputs instead of duplicating allowlists.

**Rationale:** One source of layout truth prevents drift.

## Risks / Trade-offs

- Guard can produce noise during migrations; mitigate with waivers tied to active Cairns.
- Too much filesystem policy can become brittle; mitigate with fixture-driven rules.
- Guard may overlap evidence checks; keep evidence correctness in evidence gates and layout location checks in this guard.
