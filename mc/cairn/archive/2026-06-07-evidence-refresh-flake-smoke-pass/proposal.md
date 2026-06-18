## Why

The workspace has many maintained flake apps, dry-run rails, live-equivalent fixtures, run logs, and BLAKE3 manifests. A bounded evidence refresh and smoke pass can catch drift and keep review artifacts current without adding new compatibility claims.

## What Changes

- Select a small maintained smoke set covering Cairn validation, targeted packet checks, scenario manifest checks, and representative dry-run apps.
- Run the selected checks and record reviewable logs under `docs/evidence/`.
- Refresh BLAKE3 manifests only for tracked evidence files that changed during the smoke pass.
- Preserve all current compatibility classifications and non-claims unless a separate Cairn promotes a row.

## Impact

- **Files**: `docs/evidence/**`, BLAKE3 manifests, and possibly current evidence index notes if a check exposes stale metadata.
- **Testing**: Repo-pinned Cairn validation/gates, selected flake dry-runs/smokes, evidence-manifest/task-evidence checks, and explicit successful exit-status logs.
