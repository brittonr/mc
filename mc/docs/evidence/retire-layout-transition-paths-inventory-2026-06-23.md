# Retire layout transition paths inventory

## Question

Which legacy transition paths are still recognized or referenced before retiring transition-path support from the mc layout resolver?

## Inspected evidence

- `compat/runner/src/layout.rs` defined canonical role paths and transition paths for client, server, runner, config, and the Paper survival fixture.
- Targeted searches covered exact path strings: `stevenarella`, `valence`, `tools/mc-compat-runner`, `config/mc-compat`, and `tools/paper-survival-fixture`.
- Root guidance already names canonical active paths in `README.md`, `AGENTS.md`, `docs/architecture.md`, `docs/layout-checklist.md`, and `compat/config/component-registry.ncl`.

## Inventory

| Legacy transition path | Canonical role path | Classification | Migration action |
| --- | --- | --- | --- |
| `stevenarella/` | `clients/stevenarella/` | Active resolver fallback plus active runner diagnostics/help wording | Reject as a default root; tell users to restore/move the tree to `clients/stevenarella/` or pass `--client-dir` / `CLIENT_DIR` for an explicit alternate checkout. |
| `valence/` | `servers/valence/` | Active resolver fallback and Valence worktree fallback; historical evidence/archive mentions also exist | Reject as a default root; tell users to restore/move the tree to `servers/valence/` or pass `--valence-repo` / `VALENCE_REPO` for an explicit alternate checkout. |
| `tools/mc-compat-runner/` | `compat/runner/` | Active resolver fallback; historical receipts/specs mention the old producer surface | Reject as a default root; keep historical evidence/archive mentions when they describe the old producer surface. |
| `config/mc-compat/` | `compat/config/` | Active resolver fallback; historical specs/tasks mention old config source-of-truth paths | Reject as a default root; current docs and registry use `compat/config/`; leave archive history intact. |
| `tools/paper-survival-fixture/` | `compat/fixtures/paper-survival/` | Active resolver fallback; historical survival evidence references old fixture source paths | Reject as a default root; keep historical evidence/archive mentions where they document past fixture provenance. |

## Decision

Retire default transition-path selection in `compat/runner/src/layout.rs`. Keep historical references in archived Cairn/evidence artifacts unless they would be mistaken for active defaults. Update active help/architecture diagnostics to name canonical role paths only.

## Non-claims

This inventory is layout hygiene only. It does not change scenario semantics, receipt schemas, gameplay compatibility, semantic equivalence, public-server safety, production readiness, or full CTF/survival correctness claims.
