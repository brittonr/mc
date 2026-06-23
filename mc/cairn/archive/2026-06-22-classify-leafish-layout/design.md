# Design: Classify the Leafish layout boundary

## Context

The repository has explicit notes for core component trees and nested repos. Hyperion is documented as an independent nested repo. Stevenarella and Valence are parent-owned role components. Leafish currently appears as a root-level nested Git checkout without the same ownership boundary.

## Decisions

### 1. Make ownership explicit before moving files

**Choice:** First classify Leafish as owned component, reference/vendor input, or external checkout.

**Rationale:** The right path and validation depend on whether the code is actively maintained by this workspace.

### 2. Prefer role paths for owned components

**Choice:** If Leafish is an owned client, move or transition it under a role path such as `clients/leafish/` and update docs/checks accordingly.

**Rationale:** Role paths make repo scans and component ownership consistent.

### 3. Prefer reference paths for passive inputs

**Choice:** If Leafish is only a comparison/reference codebase, move or document it under a reference/vendor path and exclude it from normal compatibility gates unless explicitly selected.

**Rationale:** Reference code should not look like an active product component.

### 4. Keep nested Git exceptions documented

**Choice:** Any nested Git directory under `mc/` must be named in workspace docs with ownership, command boundary, and whether parent repo status should ignore it.

**Rationale:** Undocumented nested repos cause accidental cross-repo edits and misleading status output.

## Risks / Trade-offs

- Moving Leafish can create large diffs; mitigate with a transition path or documentation-only decision first.
- Treating it as reference-only can hide useful code from scans; mitigate with explicit opt-in docs.
- Treating it as owned increases maintenance scope; mitigate with local AGENTS notes and targeted checks.
