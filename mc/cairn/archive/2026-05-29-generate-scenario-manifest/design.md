# Design: Generated scenario manifest

## Context

The runner currently encodes scenario facts directly in Rust functions and mirrors them in Nix/README/evidence docs. Native config for this workspace is Nickel-backed, so scenario metadata should be typed and reviewable before code generation or drift checks consume it.

## Decisions

### 1. Nickel manifest, Rust runtime tables

**Choice:** Author `config/mc-compat/scenarios.ncl` with typed contracts and generate checked-in Rust/JSON artifacts for runtime use.

**Rationale:** Nickel gives schema validation and merge semantics at authoring time; the runner stays a simple Rust binary with no runtime Nickel dependency.

### 2. Drift checker before full generation

**Choice:** First require a checker that compares the manifest against existing Rust tables, help text, flake checks, and docs. Code generation can follow once the checker stabilizes.

**Rationale:** A fail-closed checker catches drift without forcing a risky runner refactor in the first step.

### 3. Scenario facts only

**Choice:** The manifest owns scenario metadata, not live environment secrets, child repo paths, or runtime mutable policy.

**Rationale:** Scenario facts are reviewable fixtures; runtime values remain under the existing runtime-configuration contracts.

### 4. Explicit migration state

**Choice:** Each manifest row records whether Rust tables are generated, checked-only, or intentionally manual.

**Rationale:** Partial migration must be visible to reviewers and future agents.

## Risks / Trade-offs

- `flake.nix` has many wrapper checks; automated generation may be large. A drift checker can reduce risk first.
- Help text and README examples have human wording that should not be blindly generated unless templates are stable.
- Manifest row names must stay stable because receipts and evidence docs cite scenario names.
