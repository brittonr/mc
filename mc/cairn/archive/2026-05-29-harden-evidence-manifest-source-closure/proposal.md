## Why

The local evidence manifest checker can pass while the Nix flake check fails, because some manifests cite files that exist only in mutable local paths: nested child repos or `target/` outputs. Reviewers and Nix builders cannot verify those paths from the parent repo source closure.

## What Changes

- Replace manifest entries that cite nested child repos or `target/` outputs with durable `docs/evidence/` copies.
- Preserve the same BLAKE3 content identity for the copied artifacts.
- Add a spec rule that promoted manifests must stay inside the parent repo source closure unless an oracle document explicitly records external evidence.
- Record validation showing the Nix `mc-compat-evidence-manifests` check passes.

## Impact

- **Files**: `docs/evidence/*.b3`, copied source/jar artifacts under `docs/evidence/`, `cairn/specs/mc-compatibility/spec.md`, archive/task evidence.
- **Testing**: local manifest checker self-test/full scan, Nix `mc-compat-evidence-manifests`, Cairn validation/gates.
