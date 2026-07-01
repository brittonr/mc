# mc-compat verification

Use this page to choose focused validation commands after a change. The validation tier taxonomy in [check-tiers.md](check-tiers.md) is the source of truth; this page is an index and closeout guide. Checks here do not upgrade dry-run, docs, manifest, or generated-surface evidence into live gameplay compatibility evidence.

## Default rule

Use the smallest tier that covers the files and claim boundary you changed, then add any affected component-specific check named by a local `AGENTS.md`, a Cairn task, or the relevant focused doc.

```sh
nix flake check
```

Full flake checks are useful before broad review, but focused checks are preferred during a drain loop.

## Docs and layout

For Markdown-only README/docs/layout work, run the docs-layout tier and Cairn archive tier named in [check-tiers.md](check-tiers.md). Common focused checks include:

```sh
tools/check_layout_boundaries.rs --self-test
tools/check_layout_boundaries.rs --root .
tools/check_check_tiers.rs --self-test
tools/check_check_tiers.rs --root .
nix run .#cairn -- validate --root .
```

## Generated scenario surfaces

Scenario command docs, scenario indexes, wrapper metadata, and generated runner tables are tied to `compat/config/scenario-manifest.ncl` and `tools/check_scenario_manifest.rs`.

```sh
# Refresh generated scenario surfaces after manifest/checker changes.
tools/check_scenario_manifest.rs --write-generated-surfaces

# Verify generated surfaces and manifest contracts.
nix build .#checks.x86_64-linux.mc-compat-scenario-manifest --no-link -L
nix build .#checks.x86_64-linux.mc-compat-generated-harness-surfaces --no-link -L
```

The generated-surface check fails when `compat/runner/src/scenario_manifest_generated.rs`, `compat/config/generated/scenario-wrapper-metadata.nix`, `docs/evidence/mc-compat-scenario-index.generated.md`, or `docs/scenario-commands.generated.md` drift from the manifest/checker output.

## Evidence and task closeout

For evidence manifests, promoted receipts, or checked Cairn tasks, keep durable logs under `docs/evidence/` and ensure task-cited `.run.log` files include `exit_status=0`.

```sh
nix run .#evidence-manifest-refresh -- --check
nix build .#checks.x86_64-linux.mc-compat-evidence-manifest-refresh --no-link -L
nix build .#checks.x86_64-linux.mc-compat-cairn-task-evidence --no-link -L
```

Use `nix run .#evidence-manifest-refresh -- --refresh` only when the check reports stale digests and you intend to update the checked manifests.

## Component and dry-run checks

For wrapper command documentation or runner command-shape changes, run the affected dry-run rather than a live scenario unless the Cairn task explicitly requires live/manual evidence.

```sh
nix run .#stevenarella -- --dry-run
nix run .#valence -- --dry-run
nix run .#mc-compat-smoke -- --dry-run --server-backend valence --scenario smoke
nix build .#checks.x86_64-linux.mc-compat-maintained-dry-runs --no-link -L
```

Component source changes require the local subtree workflow from `clients/stevenarella/AGENTS.md` or `servers/valence/AGENTS.md` as applicable. Historical Hyperion-derived work must use reviewable source snapshots or archived evidence rather than a live local checkout.

## Cairn closeout

Run Cairn commands from the mc root with the repo-pinned app:

```sh
nix run .#cairn -- gate proposal <change> --root .
nix run .#cairn -- gate design <change> --root .
nix run .#cairn -- gate tasks <change> --root .
nix run .#cairn -- validate --root .
nix run .#cairn -- sync <change> --root .
nix run .#cairn -- archive <change> --root .
```

Inspect dry-run sync/archive plans before executing. Execute sync/archive only after implementation evidence is durable, tasks are checked off with verification, and validation is clean.
