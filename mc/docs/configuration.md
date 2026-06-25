# mc-compat configuration

This page owns configuration and source-tree reference details for the mc compatibility workspace. It is configuration documentation only; it does not claim scenario success, semantic equivalence, public-server safety, production readiness, or broad Minecraft compatibility.

## Nickel-backed config

The scenario manifest source of truth is `compat/config/scenario-manifest.ncl`. Update it before adding or changing a maintained scenario, then run:

```sh
nix build .#checks.x86_64-linux.mc-compat-scenario-manifest --no-link -L
```

That check typechecks Nickel, runs positive/negative manifest fixtures, and checks drift against runner tables, flake dry-run checks, scenario command docs, and current evidence bundle rows. Runtime code consumes checked-in Rust tables in `compat/runner/src/scenario_manifest_generated.rs`; it does not evaluate Nickel at startup.

The checked-in default config is Nickel-authored at `compat/config/default.ncl` and exported to `compat/config/generated/default.json`. The runner consumes exported JSON, not Nickel at runtime:

```sh
nix shell nixpkgs#nickel -c nickel export \
  compat/config/default.ncl \
  > compat/config/generated/default.json

nix run .#mc-compat-smoke -- \
  --config compat/config/generated/default.json \
  --dry-run
```

Config provides defaults; environment variables and later CLI flags override it. You can also set `MC_COMPAT_CONFIG=compat/config/generated/default.json`.

## Matrix and comparison workflows

Run both fallback/control Paper and intended/default Valence receipts, then compare them in one local gate:

```sh
CLIENT_TIMEOUT=8 nix run .#mc-compat-smoke -- \
  --run-matrix \
  --receipt-dir target/matrix-smoke
```

For a non-side-effecting fixture of the same matrix shape, put `--dry-run` after `--run-matrix`:

```sh
nix run .#mc-compat-smoke -- \
  --run-matrix --dry-run \
  --receipt-dir target/matrix-smoke-dry-run
```

Compare existing fallback/control Paper and intended/default Valence receipts:

```sh
nix run .#mc-compat-smoke -- --compare-receipts \
  target/mc-compat-smoke.json \
  target/mc-compat-smoke-valence.json
```

Matrix and comparison checks require one `paper` receipt and one `valence` receipt, both passing, both protocol `758`, expected backend ports, successful client evidence, and niri-safe Xvfb/X11/software-GL isolation.

## Core Stevenarella client source

Stevenarella is tracked directly in this repository so the client side of the compatibility seam can be patched with the harness. By default the runner expects `./clients/stevenarella` to be the core Stevenarella source root containing `Cargo.toml`.

Use another source tree without moving files:

```sh
nix run .#mc-compat-smoke -- --dry-run --client-dir /path/to/stevenarella
# or
CLIENT_DIR=/path/to/stevenarella nix run .#mc-compat-smoke -- --dry-run
```

If the source tree is missing or does not look like the Stevenarella root, the runner fails before starting the smoke and tells you whether to restore the core client tree or pass `--client-dir` / `CLIENT_DIR`.

## Core Valence server source

Valence is tracked directly in this repository so server fixtures can be patched with the harness. By default the runner expects:

- `./servers/valence` to be the core Valence server source tree
- `VALENCE_REV=8ad9c85` to exist in the parent repository history for the compatible Minecraft `1.18.2` / protocol `758` default
- `VALENCE_WORKTREE=/tmp/valence-compat-758` to be a disposable detached worktree created from that history

Protocol `763` rails usually set `VALENCE_REV=main`; those worktrees check out the monorepo root, and the runner automatically runs Cargo from the nested `mc/servers/valence` source directory.

Use another source tree without moving files:

```sh
nix run .#mc-compat-smoke -- --dry-run --valence-repo /path/to/valence
# or
VALENCE_REPO=/path/to/valence nix run .#mc-compat-smoke -- --dry-run
```

If the source tree or revision is missing, the runner fails before starting the smoke and tells you whether to restore/fetch the parent history or pass `--valence-repo` / `VALENCE_REPO`.

Paper remains available as a fallback/control backend:

```sh
nix run .#mc-compat-smoke -- --run --server-backend paper
```

## OnixResearch tool inputs

The flake pins Cairn and Octet through the canonical GitHub inputs:

- `cairn`: `github:onixresearch/cairn` (`https://github.com/onixresearch/cairn`)
- `octet`: `github:onixresearch/octet` (`https://github.com/onixresearch/octet`)

The dev shell exposes `cairn` and `cargo-octet` alongside the smoke runner:

```sh
nix develop
cairn --help
cargo-octet --help
```

The packages are also available as `.#cairn`, `.#cargo-octet`, and `.#octet`.

## Octet monorepo enforcement

The repo-owned aggregate checker derives the lint inventory from the pinned Octet input and verifies the owned Rust workspaces:

- `compat/runner`
- `clients/stevenarella`
- `servers/valence`

It requires workspace `[workspace.metadata.octet]`, consumer-owned `dylint.toml` files with every current Octet lint at `deny`, and reviewed stable-ID baselines under `compat/octet-baselines/`. The dynamic mode runs the repo-pinned `path:$PWD#cargo-octet` gate for each workspace and fails on any new unaccepted stable ID.

```sh
rustc --edition=2021 tools/check_octet_monorepo.rs -o target/check-octet-monorepo
OCTET_SOURCE_DIR=$(nix eval --raw --impure --expr '(builtins.getFlake (toString ./.)).inputs.octet.outPath')
target/check-octet-monorepo --root . --octet-source "$OCTET_SOURCE_DIR" --run-octet
```

Run `target/check-octet-monorepo --self-test` for positive and negative fixtures covering lint drift, missing config, and new unaccepted findings.

## Cairn policy ownership

Cairn lifecycle policy is owned by the pinned Cairn toolchain but checked into this workspace for deterministic validation. The source is Nickel under `cairn-policy/default.ncl` with contracts in `cairn-policy/contracts.ncl`; runtime Cairn commands consume the generated JSON at `cairn-policy/generated/cairn-policy.json` and do not evaluate Nickel.

Keep `cairn-policy/` top-level so the repo-pinned `nix run .#cairn -- policy export --check` default paths continue to work. Regenerate with:

```sh
nix run .#cairn -- policy export
nix run .#cairn -- policy export --check
# or
nix build .#checks.x86_64-linux.mc-cairn-policy-fresh --no-link -L
```

`.#checks.x86_64-linux.mc-cairn-policy-stale-detects-drift` is the negative fixture proving source drift is rejected.
