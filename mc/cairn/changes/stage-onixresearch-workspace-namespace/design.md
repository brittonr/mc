## Context

The local research workspace currently stores many repositories as siblings under `/home/brittonr/git/`, including `cairn`, `valence`, `octet`, `mantle`, `trellis`, `mc`, and related Onix projects. That layout works with existing local automation, but it does not clearly separate OnixResearch-owned stack repositories from unrelated checkouts.

The desired target shape is a canonical namespace such as `/home/brittonr/git/OnixResearch/<repo>`. Existing consumers still reference legacy sibling paths directly, including `path:/home/brittonr/git/cairn#cairn`, repository-specific scripts, Pi skill guidance, lock files, docs, and evidence notes. Removing or moving those paths without a compatibility period would break validation commands and local workflows.

## Decisions

### 1. Treat the namespace move as a staged migration

**Choice:** The first implementation must inventory consumers and define the migration contract before moving repositories or updating broad path references.

**Rationale:** The risky behavior is not the directory name; it is hidden coupling to legacy paths. Inventory-first migration avoids breaking Cairn validation, Nix `path:` inputs, Pi skills, and cross-repo update scripts.

### 2. Canonical path plus compatibility symlink phase

**Choice:** The target canonical home is `~/git/OnixResearch/<repo>`, while legacy `~/git/<repo>` paths remain as temporary compatibility symlinks for migrated repositories until all active consumers are updated or explicitly waived.

**Rationale:** Compatibility symlinks let existing commands keep working while docs and automation move toward the canonical namespace. They also provide a reversible transition if a path consumer is missed.

### 3. Use a shared root variable for updated automation

**Choice:** New or updated scripts, docs, and skills should prefer a shared root setting such as `ONIX_RESEARCH_ROOT=$HOME/git/OnixResearch`, with explicit fallbacks or compatibility notes where legacy paths remain required.

**Rationale:** A single root variable makes later relocations and machine-specific setups less fragile than copying hard-coded absolute paths into every command.

### 4. Keep repo identity and remote semantics unchanged

**Choice:** The namespace migration must not change repository remotes, branch names, commit identity, package ownership, evidence semantics, or release claims unless a separate change explicitly covers those behaviors.

**Rationale:** This is a local workspace layout migration. It should not accidentally imply organizational, provenance, release, or semantic changes.

### 5. Prove both canonical and compatibility paths before archive

**Choice:** Closeout evidence must show selected commands work through the canonical namespace and through compatibility paths during the transition.

**Rationale:** A migration is only safe if both the new path and the compatibility layer are valid for the tools that still depend on legacy paths.

## Risks / Trade-offs

- Some tools may resolve symlinks and emit canonical paths in logs, which can create noisy evidence diffs. Validation should distinguish path spelling drift from semantic failure.
- Lock files or generated evidence may intentionally preserve historical legacy paths. The inventory should classify those as historical or generated instead of rewriting them blindly.
- Moving repositories may interact with nested Git worktrees, `.jj` metadata, local hooks, editor state, or untracked scratch files. The implementation should move clean repositories first or explicitly record blockers.
- Compatibility symlinks reduce migration pressure if left indefinitely. A follow-up retirement change should remove them only after active references are updated and validated.
