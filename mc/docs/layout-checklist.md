# Repository layout checklist

Use this checklist for layout-only reviews. Run `tools/check_layout_boundaries.rs --self-test` and `tools/check_layout_boundaries.rs --root .` through the script's pinned Nix shell to verify the current documented nested Git boundaries.

## Major component roots

| Path | Role | Ownership | Local notes | Default gates |
| --- | --- | --- | --- | --- |
| `clients/stevenarella/` | Core client | Parent repository owned | Pending `clients/stevenarella/AGENTS.md` | Included through mc-compat checks when selected by scenario |
| `servers/valence/` | Core server | Parent repository owned | `servers/valence/AGENTS.md` | Included through mc-compat and Valence checks when selected |
| `compat/` | Compatibility harness/config/fixtures | Parent repository owned | Root `AGENTS.md` plus `docs/architecture.md` | Included by runner, generated-surface, and evidence checks |
| `hyperion/` | Independent engine/proxy repository | Nested repo, separate jj/git workflow | `hyperion/AGENTS.md` and `hyperion/.agent/napkin.md` | Excluded from parent default gates unless explicitly selected |
| `Leafish/` | Reference client checkout | Reference-only nested Git checkout | Waived here because the nested checkout is not parent-owned | Excluded from default gates unless explicitly selected |

## Review rules

- Major owned component roots SHOULD have a local `AGENTS.md` or an explicit waiver in this checklist.
- Reference-only or external checkouts MUST state owner, command boundary, parent-repo interaction, and default-gate participation.
- Root-level nested Git directories MUST be listed above before they are treated as intentional.
- Default compatibility gates MUST NOT require reference-only checkouts unless a command explicitly opts into that reference.
- New component roots SHOULD update `AGENTS.md`, `README.md`, `docs/architecture.md`, and this checklist in the same change.

## Current nested Git exceptions

- `hyperion/`: independent repository with separate jj/git workflow. Run Hyperion commands from inside `hyperion/`; do not use parent repo status as Hyperion status.
- `Leafish/`: reference-only client checkout retained for comparison and historical investigation. Do not include it in default compatibility gates, source-tree revision evidence, or parent-owned component scans unless a future Cairn explicitly reclassifies it.
