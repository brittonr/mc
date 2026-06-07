## Why

`sign-editor-open-update` remains fixture-bounded because existing sign/block-entity evidence does not automate or correlate the sign editor open/update interaction. A dedicated live rail can prove the exact open/update packet pair without borrowing coverage from sign persistence.

## What Changes

- Add an isolated sign editor live rail that opens one sign editor for one configured sign position and submits one four-line payload.
- Record client open/update milestones and backend accepted-update correlation under `docs/evidence/`.
- Validate the evidence with the targeted packet live-evidence checker before any promotion.
- Keep sign persistence, block-entity update breadth, arbitrary NBT, and broad sign-editing semantics as non-claims.

## Impact

- **Files**: `tools/mc-compat-runner/src/**`, sign scenario/evidence fixtures, targeted packet checker inputs, `docs/evidence/**`, acceptance matrix/current bundle/packet inventory docs.
- **Testing**: Baseline targeted packet/doc checks, sign editor rail checks or blocker evidence, checker positive/negative tests, matrix/bundle/inventory checks, evidence-manifest/task-evidence checks, Cairn gates and validation.
