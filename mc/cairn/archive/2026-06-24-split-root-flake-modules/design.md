# Design: Split the root flake into focused local modules

## Context

The root flake currently centralizes all package/app/check/dev-shell definitions. It is reviewable but oversized. A split should remain idiomatic for this repository and avoid importing upstream NixOS module/overlay patterns that do not match the local Onix/Nix workflow.

## Decisions

### 1. Keep a thin public façade

**Choice:** `flake.nix` remains the only public flake entrypoint and assembles outputs from local imported files.

**Rationale:** Users and CI keep using the same `nix run` and `nix build` commands.

### 2. Split by output responsibility

**Choice:** Move packages, apps, checks, dev shells, and shared helper data into separate local files with explicit arguments.

**Rationale:** Reviewers can inspect one responsibility at a time, and repeated wrapper patterns become easier to generate or consolidate later.

### 3. Preserve output names first

**Choice:** The first split is behavior-preserving. Renames, generated wrappers, and check deprecations must be handled by separate Cairns.

**Rationale:** Output-name parity is the simplest way to prove the split is safe.

### 4. Validate with output inventory

**Choice:** Add a small inventory or check that compares expected app/check/package/dev-shell names before and after the split.

**Rationale:** A structural refactor should fail closed on missing public outputs.

## Risks / Trade-offs

- Nix imports can make variable flow less visible; mitigate with explicit argument sets and small files.
- Splitting before generation may move some repetition rather than remove it; mitigate by sequencing this with the manifest-derived surfaces change.
- Nix checks can become slow if validation is too broad; mitigate with focused dry-runs plus existing aggregate gates.
