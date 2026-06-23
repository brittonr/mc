# Proposal: Clarify Cairn policy ownership and path

## Why

`cairn-policy/` is a top-level generated-policy area separate from `cairn/`. That may be necessary for the pinned Cairn binary, but its ownership and path contract should be explicit. Without documentation, future cleanup may move or regenerate policy files in a way that breaks validation.

## What Changes

- Document the owner, generation command, schema compatibility constraints, and path reason for `cairn-policy/`.
- Decide whether the policy should remain top-level or move under a clearer `cairn/` subpath after confirming pinned-binary compatibility.
- Add a freshness/schema check for generated policy files.
- Keep current path stable unless compatibility evidence proves a move is safe.

## Impact

- **Files**: `cairn-policy/`, Cairn policy generation docs, README/architecture notes, flake checks if policy paths are embedded, generated policy freshness checks, and Cairn artifacts.
- **Testing**: Cairn validation, policy export/check if available, generated policy freshness/schema compatibility, and Cairn gates.
- **Non-claims**: this clarifies lifecycle-policy ownership only; it does not change compatibility scenarios, runner behavior, or evidence claims.
