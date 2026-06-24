# Design: Clean repository artifact boundaries

## Context

Repo-local instructions already require Cairn evidence to be copied into `docs/evidence/` and not left only under `target/`. The current tree still has several root-level transient or ambiguous artifacts. The cleanup should strengthen the contract without deleting review-critical files.

## Decisions

### 1. Classify artifacts before deleting or moving

**Choice:** Define artifact classes: durable evidence, generated checked-in output, transient run/build output, and local scratch.

**Rationale:** This prevents accidentally removing evidence that a Cairn task still needs.

### 2. Keep durable review evidence under `docs/evidence/`

**Choice:** Evidence cited by Cairn tasks, specs, or review notes must live under `docs/evidence/` with BLAKE3 manifests when required.

**Rationale:** Nix/source-closure evidence checks can see tracked parent files there, and reviewers know where to look.

### 3. Ignore or remove root transient outputs

**Choice:** Root `target-*live.log`, `result-*`, pycache, and similar generated artifacts should be ignored, moved to `target/`, or promoted into `docs/evidence/` before citation.

**Rationale:** The root should communicate source layout, not local run state.

### 4. Preserve visibility of evidence problems

**Choice:** Ignore rules must not hide missing/stale durable evidence manifests or task-cited logs.

**Rationale:** Artifact cleanup should make evidence gates stricter, not quieter.

## Risks / Trade-offs

- Removing ambiguous files can break undocumented workflows; mitigate by documenting replacements and checking references before deletion.
- Ignoring too broadly can hide important evidence; mitigate with targeted patterns and evidence-manifest checks.
- Moving old notes can stale BLAKE3 manifests; mitigate with manifest refresh/check before archive.
