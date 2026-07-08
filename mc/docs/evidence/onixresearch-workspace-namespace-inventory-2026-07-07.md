# OnixResearch workspace namespace inventory — 2026-07-07

## Question

Can the clean OnixResearch sibling repositories move from `/home/brittonr/git/<repo>` to `/home/brittonr/git/OnixResearch/<repo>` without breaking active validation paths?

## Inspected evidence

- Pre-change Cairn and layout baseline: `docs/evidence/onixresearch-workspace-namespace-baseline-2026-07-07.run.log`.
- Cleanliness and move execution: `docs/evidence/onixresearch-workspace-namespace-move-2026-07-07.run.log`.
- Active contract: `docs/onixresearch-workspace-namespace.md`.
- Focused checker: `tools/check_onixresearch_workspace_namespace.rs`.

## Repository classification

| Repository | Classification | Migration action |
| --- | --- | --- |
| `cairn` | active | Moved to `/home/brittonr/git/OnixResearch/cairn`; legacy `/home/brittonr/git/cairn` is a temporary compatibility symlink. |
| `valence` | active | Moved to `/home/brittonr/git/OnixResearch/valence`; legacy `/home/brittonr/git/valence` is a temporary compatibility symlink. `jj status` was clean before move. |
| `octet` | active | Moved to `/home/brittonr/git/OnixResearch/octet`; legacy `/home/brittonr/git/octet` is a temporary compatibility symlink. |
| `mantle` | active | Moved to `/home/brittonr/git/OnixResearch/mantle`; legacy `/home/brittonr/git/mantle` is a temporary compatibility symlink. |
| `trellis` | active | Moved to `/home/brittonr/git/OnixResearch/trellis`; legacy `/home/brittonr/git/trellis` is a temporary compatibility symlink. |
| `mc` | blocked | Not moved. It is parent-owned under `/home/brittonr/git/mc` and had pre-existing dirty state unrelated to this migration. |
| `onix-research` | intentionally retained | Existing lowercase checkout is unrelated and was not touched. |

## Reference inventory

| Surface | Examples found | Classification | Migration action |
| --- | --- | --- | --- |
| Pi skills | `/home/brittonr/.pi/agent/skills/cairn-drain/SKILL.md` names `/home/brittonr/git/cairn` and `path:/home/brittonr/git/cairn#cairn`. | active | Update to prefer `ONIX_RESEARCH_ROOT=${ONIX_RESEARCH_ROOT:-$HOME/git/OnixResearch}` and keep legacy symlink fallback. |
| mc agent guidance | `AGENTS.md` warns that sibling `/home/brittonr/git/cairn` is not the mc validation authority. | active | Keep the warning but mention canonical `ONIX_RESEARCH_ROOT/cairn` plus legacy compatibility symlink. |
| mc docs and evidence notes | Historical receipts and notes under `docs/evidence/` contain `/home/brittonr/git/mc`, `/home/brittonr/git/mc/valence`, and older component path spellings. | historical | Do not rewrite. These paths are evidence payloads and should remain citation-stable. |
| Nix path inputs | Active local commands and scratch scripts use `path:/home/brittonr/git/cairn#cairn`; mc flake inputs use GitHub URLs for Cairn/Octet and flake locks record Git URLs/owners, not local canonical paths. Literal `path:` inputs whose root is a symlink are rejected by Nix. | active / generated / blocked | Update durable guidance to canonical `ONIX_RESEARCH_ROOT` paths, validate canonical Cairn `path:` inputs, and record the expected rejection of literal symlinked compatibility `path:` inputs; leave lock data generated. |
| flake locks | `flake.lock` records GitHub/Git URLs for `cairn`, `octet`, and `slotcar`. | generated | Do not hand-edit. Update only via Nix if input pins change. |
| validation commands | `/tmp/pi-verify-*.sh`, `/tmp/pi-update-*.sh`, and related scratch scripts contain `/home/brittonr/git/{cairn,valence,octet,mantle,trellis}`. | removable / active scratch | Leave in `/tmp`; compatibility symlinks preserve them for this session. Future durable scripts should use `ONIX_RESEARCH_ROOT`. |
| evidence notes | Historical JSON/Markdown receipts include absolute paths from old run environments. | historical | Do not rewrite; path spelling is part of historical evidence. |
| compatibility-path smokes | Closeout commands intentionally invoke shell/Git operations through `/home/brittonr/git/<repo>` symlinks and intentionally probe literal `path:/home/brittonr/git/cairn#cairn` as a negative Nix fixture. | intentionally retained | Keep shell/Git compatibility smokes and the Nix symlink-rejection fixture until durable consumers no longer need the legacy path. |
| docs needing future migration | Stevenarella and Valence docs under `mc/` still use `/home/brittonr/git/mc` because `mc` did not move. | blocked | Leave unchanged until an `mc`-specific migration exists. |

## Required classifications present

The inventory includes `active`, `historical`, `generated`, `blocked`, `removable`, and `intentionally retained` references so reviewers can distinguish safe updates from evidence-preserving or blocked paths.

## Decision

Proceed with the staged move for clean sibling repositories only. Keep `/home/brittonr/git/mc` in place, keep compatibility symlinks for migrated sibling repositories, update safe active guidance toward `ONIX_RESEARCH_ROOT`, require canonical paths for literal Nix `path:` inputs, and require a later Cairn before compatibility-link retirement.

## Owner

Agent-maintained local workspace migration evidence for the `stage-onixresearch-workspace-namespace` Cairn.

## Next action

Run focused checker, canonical path smokes, compatibility shell/Git path smokes, expected symlinked-Nix rejection checks, selected canonical Nix path-input validation, Cairn gates, Cairn validation, task-evidence validation, sync, archive, and post-archive validation.
