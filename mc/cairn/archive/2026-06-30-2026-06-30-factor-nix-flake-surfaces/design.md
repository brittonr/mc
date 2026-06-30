# Design: Root Nix flake surface modules

## Context

The root flake is the command and validation entrypoint for the mc workspace. It must keep stable output names for existing packages, apps, checks, and devshells while making implementation files smaller and more domain-owned.

## Decisions

### 1. Split checks by validation tier/domain

**Choice:** Move check definitions into focused modules aligned with docs/layout, generated surfaces, evidence, component behavior, Octet, and checker-framework families.

**Rationale:** Reviewers can map changes to `docs/check-tiers.md` and avoid scanning unrelated checks.

### 2. Preserve output names through a central export

**Choice:** Keep `nix/checks.nix`, `nix/packages.nix`, and `nix/apps.nix` as compatibility aggregators while importing helper modules.

**Rationale:** Existing `.#checks.*`, `.#packages.*`, and `.#apps.*` names must remain stable.

### 3. Introduce helper functions for repeated wrapper patterns

**Choice:** Factor common `runCommand` log-copying and scenario wrapper boilerplate into local Nix helpers.

**Rationale:** The current repetition hides meaningful differences and increases drift risk.

### 4. Bind generated scenario metadata deliberately

**Choice:** Use `compat/config/generated/scenario-wrapper-metadata.nix` only where it keeps output parity checkable and avoids reducing review clarity.

**Rationale:** Generated metadata is already a checked surface, but critical Nix outputs still need stable names and clear non-claims.

## Risks / Trade-offs

- Nix refactors can silently alter output names; flake output inventory parity must run before and after.
- More files can make evaluation dependencies less obvious; keep aggregators small and documented.
- Helper abstractions should not obscure check-specific evidence copied into `$out`.
