# Proposal: Clean repository artifact boundaries

## Why

The workspace has durable review evidence under `docs/evidence/`, but it also contains root-level transient logs, `result-*` outputs, an empty `config/` directory, top-level `evidence/` notes, and ignored/generated artifacts. Mixed artifact locations make review evidence harder to audit and increase the chance that transient files are cited or committed accidentally.

## What Changes

- Define which artifact classes are durable evidence, generated checked-in outputs, transient build/run outputs, or local-only scratch files.
- Move or retire top-level `evidence/` notes and root live logs according to the durable-evidence contract.
- Remove or document empty/reserved directories such as root `config/`.
- Update `.gitignore` and checks so transient root artifacts are ignored while review-critical `docs/evidence/` files remain visible.
- Add a guard that prevents Cairn tasks from citing target-only or transient root artifacts.

## Impact

- **Files**: `.gitignore`, root transient artifacts, `docs/evidence/`, top-level `evidence/`, root `config/`, README/architecture docs, evidence manifest/task evidence tools if touched, and Cairn artifacts.
- **Testing**: evidence manifest refresh/check, task evidence gate, artifact-boundary guard, Cairn validation/gates, and no-op checks for unchanged durable evidence.
- **Non-claims**: this improves repository hygiene and evidence reviewability only; it does not change compatibility semantics or add live evidence coverage.
