## Candidate decision

Selected: `stage-onixresearch-workspace-namespace`

Why now: the Onix repositories are increasingly treated as one research stack, but many local skills, scripts, Nix inputs, flake locks, and evidence notes still assume sibling paths such as `/home/brittonr/git/cairn` and `path:/home/brittonr/git/cairn#cairn`. A direct move would break shell/Git consumers and Nix `path:` consumers differently. A staged namespace plan can make `~/git/OnixResearch/` the canonical home while retaining temporary shell/Git compatibility links, migrating literal Nix path inputs to canonical paths, and recording reviewable validation.

Prerequisites satisfied: current sibling repositories are already co-located under `~/git/`; the mc repository has an accepted `repository-layout` specification for path ownership, inventories, compatibility decisions, docs, and validation; prior cross-repo work identified hard-coded path consumers that need a compatibility phase.

Deferred alternatives: moving every repository immediately without symlinks, rewriting all scripts before any move, introducing a global package manager workspace, changing remotes, changing repository ownership, and removing legacy paths are deferred until inventories and validation evidence show they are safe.

Non-claims: this change does not move repositories by itself, does not rename GitHub organizations or remotes, does not change repo contents, does not prove every third-party script is path-independent, does not remove compatibility links, and does not make `~/git/OnixResearch/` mandatory until a follow-up execution change accepts the migration evidence.

## Why

The current workspace shape keeps many OnixResearch repositories as direct siblings under `~/git/`. That is convenient for existing commands, but it hides the stack boundary and makes cross-repo automation rely on ad hoc path knowledge. Moving everything at once would be risky because path references exist in Pi skills, Nix `path:` inputs, flake locks, scripts, docs, evidence logs, and local operator muscle memory.

A Cairn change should capture the migration contract before any filesystem mutation: what gets inventoried, which paths become canonical, which legacy paths remain as compatibility symlinks, which references get updated, and which checks prove the transition is safe enough to archive.

## What Changes

- Add repository-layout requirements for an OnixResearch workspace namespace inventory.
- Define `~/git/OnixResearch/<repo>` as the proposed canonical namespace only after inventories and compatibility checks are recorded.
- Require temporary compatibility symlinks from legacy `~/git/<repo>` paths during the staged migration for shell/Git path consumers.
- Require active docs, scripts, skills, and literal Nix `path:` inputs to move toward a shared root variable instead of hard-coded legacy paths, because Nix rejects symlinked flake roots.
- Require validation evidence before archive and before any later removal of compatibility symlinks.

## Impact

- **Files**: this package adds Cairn proposal/design/tasks/spec delta. A later implementation may update workspace docs, scripts, Pi skills, Nix inputs/locks, flake references, and local symlink setup notes.
- **Testing**: planned checks include path-reference inventory, symlink/canonical-path dry-runs, selected canonical Nix path-input checks, expected rejection checks for literal symlinked Nix `path:` inputs, Cairn gates, repository validation, and focused command smoke tests through canonical paths and shell/Git compatibility paths.
- **Non-claims**: no repository is moved by this package; no remote, branch, commit, package lock, or release evidence claim changes without follow-up implementation evidence.
