# OnixResearch workspace namespace

This document records the staged local workspace migration for the OnixResearch stack. The canonical root is:

```sh
ONIX_RESEARCH_ROOT=${ONIX_RESEARCH_ROOT:-$HOME/git/OnixResearch}
```

On this machine that resolves to `/home/brittonr/git/OnixResearch`.

## Scope and repository set

Initial migrated repositories:

| Repository | Canonical path | Legacy compatibility path | Status |
| --- | --- | --- | --- |
| `cairn` | `/home/brittonr/git/OnixResearch/cairn` | `/home/brittonr/git/cairn` | moved with temporary compatibility symlink |
| `valence` | `/home/brittonr/git/OnixResearch/valence` | `/home/brittonr/git/valence` | moved with temporary compatibility symlink |
| `octet` | `/home/brittonr/git/OnixResearch/octet` | `/home/brittonr/git/octet` | moved with temporary compatibility symlink |
| `mantle` | `/home/brittonr/git/OnixResearch/mantle` | `/home/brittonr/git/mantle` | moved with temporary compatibility symlink |
| `trellis` | `/home/brittonr/git/OnixResearch/trellis` | `/home/brittonr/git/trellis` | moved with temporary compatibility symlink |
| `mc` | `/home/brittonr/git/mc` for this stage | `/home/brittonr/git/mc` | deferred because it is the parent-owned tracked subtree and had pre-existing dirty state |

The existing lowercase `/home/brittonr/git/onix-research` checkout is an unrelated repository and is intentionally not touched by this migration.

## Migration order and cleanliness

1. Inventory active and historical references before moving anything.
2. Move only repositories whose VCS status is clean or whose dirty state is explicitly recorded and accepted for the move.
3. Move clean sibling repositories into `ONIX_RESEARCH_ROOT`.
4. Create a temporary compatibility symlink at each migrated legacy path.
5. Validate canonical-path command smoke checks, compatibility-path shell/Git command smoke checks, and the expected rejection of literal symlinked Nix `path:` flake roots.
6. Update active docs, scripts, skills, and Nix path inputs toward `ONIX_RESEARCH_ROOT` when safe; literal Nix `path:` inputs must use the canonical path or resolve the symlink before calling Nix.
7. Leave generated, historical, blocked, or intentionally retained references in place with classification evidence.

`valence` uses Jujutsu locally; the move log records `jj status` before moving it. The other migrated siblings were checked with Git status before the move. `mc` remains at the legacy root in this stage because its parent repository status included unrelated pre-existing changes.

## Compatibility link rule

Migrated repositories MUST retain a temporary compatibility symlink from `/home/brittonr/git/<repo>` to `OnixResearch/<repo>` until active consumers are migrated or waived. legacy commands that use normal shell path resolution, such as `cd /home/brittonr/git/cairn` or `git -C /home/brittonr/git/cairn status`, remain valid during this phase through the symlink. Literal Nix `path:` inputs must use the canonical path, for example `nix run path:${ONIX_RESEARCH_ROOT:-$HOME/git/OnixResearch}/cairn#cairn -- ...`, because Nix rejects a flake root when the root path itself is a symlink.

Do not remove compatibility links while the path-reference inventory contains active or blocked legacy consumers. Removal requires a follow-up Cairn change with fresh inventory, canonical command validation, compatibility-retirement diagnostics, and rollback evidence.

Rollback for a migrated repository is intentionally simple while the compatibility link exists: remove the legacy symlink, move `ONIX_RESEARCH_ROOT/<repo>` back to `/home/brittonr/git/<repo>`, and rerun the same status and command-smoke checks. Do not rollback `mc` through this contract because it was not moved by this stage.

## Reference-update policy

New or touched active automation SHOULD prefer `ONIX_RESEARCH_ROOT` or an equivalent documented root setting. Hard-coded `/home/brittonr/git/<repo>` references are allowed only when classified as:

- `active`: currently executed and still covered by a compatibility symlink or explicit migration task.
- `historical`: old evidence, archive, or receipt text that should remain citation-stable.
- `generated`: produced by a tool or lockfile and updated only through its generator.
- `blocked`: unsafe to rewrite in this stage because it depends on another repo, dirty worktree, or external tool contract.
- `removable`: scratch or stale reference that can be deleted instead of migrated.
- `intentionally retained`: kept as a compatibility-path smoke, warning, or rollback example.

## Validation contract

Closeout validation for this stage records:

- path-reference inventory freshness;
- canonical-path command smoke checks;
- compatibility-path shell/Git command smoke checks;
- selected Nix path-input validation through canonical paths;
- expected rejection evidence for literal symlinked Nix `path:` compatibility inputs;
- Cairn proposal, design, and tasks gates;
- Cairn validation;
- task-evidence validation;
- BLAKE3 coverage for promoted logs and inventory evidence.

The focused checker is `tools/check_onixresearch_workspace_namespace.rs`. It validates the contract text, inventory classifications, migrated compatibility symlinks, deferred `mc` handling, and positive/negative self-test fixtures.

## Non-claims

This local path migration does not change remotes, does not rewrite history, does not prove release eligibility, does not prove behavioral correctness, does not prove whole-stack safety, and does not retire compatibility links. It also does not rename GitHub organizations, change package identity, change release evidence, or make the lowercase `onix-research` checkout part of this stack migration.
