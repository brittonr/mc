# Proposal: Protocol-763 compatibility acceptance matrix and gap checklist

## Why

Evidence now spans many receipts, but full compatibility is still unproven. A tracked matrix prevents redoing saturated soaks and makes remaining gaps inspectable before choosing more implementation seams.

## What Changes

- Create a checked-in matrix that maps every protocol/gameplay semantic seam to receipt path, BLAKE3, commands, commits, claims, and non-claims.
- Add a machine-readable checklist/fixture and a cheap gate that fails if required matrix rows or evidence links are missing.
- Document the remaining ROI order and boundaries without claiming full compatibility.

## Impact

- **Files**: `docs/evidence/`, `README.md`, optional `tools/mc-compat-runner` metadata/checker, and `flake.nix` for the cheap gate.
- **Testing**: `git diff --check`, matrix checker/dry-run gate, JSON/Markdown parse or grep assertions, no live server required.
