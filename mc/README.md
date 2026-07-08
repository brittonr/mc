# Minecraft Rust compatibility smoke

This workspace contains local Minecraft compatibility experiments. The hardened smoke harness checks a Rust client against a Rust server:

- client: `clients/stevenarella`
- server: `servers/valence` pinned to Minecraft `1.18.2` / protocol `758` by the default rail
- runner: `compat/runner`, packaged by the root flake

The legacy shell entrypoint is intentionally only a thin compatibility shim around the flake app.

## Quickstart

Launch the core server/client source trees through the root flake environment:

```sh
nix run .#valence -- --dry-run
nix run .#stevenarella -- --dry-run
```

Dry-run the compatibility plan without starting the server or client:

```sh
nix run .#mc-compat-smoke -- --dry-run
# or
scripts/mc-compat-smoke.sh --dry-run
```

Run the bounded headless smoke:

```sh
CLIENT_TIMEOUT=8 nix run .#mc-compat-smoke -- --run
# or
CLIENT_TIMEOUT=8 scripts/mc-compat-smoke.sh --run
```

Choose a typed scenario with the router form, the legacy `--scenario` flag, or `MC_COMPAT_SCENARIO`:

```sh
nix run .#mc-compat-smoke -- scenario run smoke --dry-run \
  --receipt target/mc-compat-smoke.json

nix run .#mc-compat-smoke -- --dry-run --scenario smoke \
  --receipt target/mc-compat-smoke.json
```

The full maintained scenario command reference now lives in [docs/scenario-commands.md](docs/scenario-commands.md), with the machine-owned command index at [docs/scenario-commands.generated.md](docs/scenario-commands.generated.md).

## Repository layout

`clients/stevenarella/` and `servers/valence/` are core component source trees tracked directly by this repository, not submodules. They retain upstream ancestry from the local fork heads used by the compatibility rails, but ongoing harness, client, and server changes now evolve in one parent history.

`Leafish/` is classified as a reference-only nested Git checkout. It is retained for comparison and historical investigation, excluded from default compatibility gates, and may only participate through explicit opt-in work. Stevenarella-specific workflow lives in `clients/stevenarella/AGENTS.md`, Valence-specific workflow lives in `servers/valence/AGENTS.md`, and the current layout checklist lives in `docs/layout-checklist.md`.

Artifact classes are documented in [docs/architecture.md](docs/architecture.md) and [docs/layout-checklist.md](docs/layout-checklist.md). Durable review evidence lives under `docs/evidence/`; generated checked-in outputs stay in owner-specific generated paths; root `result`, `result-*`, `target/`, `target-*.log`, root `*.run.log`, and interpreter caches are transient; `.pi/` and untracked experiments are local scratch. Root `evidence/` is retired, and checked configuration belongs under `compat/config/` rather than root `config/`.

Some receipt fields keep their historical names for schema compatibility, including `client.git_rev`, `valence.git_rev_resolved`, and MCP `stevenarella_child_revision`. For core component trees those values are scoped source-tree evidence: the last Git commit affecting that subtree plus dirty checks limited to that subtree, not an independent nested-repo HEAD.

## Documentation map

- [docs/scenario-commands.md](docs/scenario-commands.md): scenario router forms, maintained wrapper examples, dry-run checks, and links to generated scenario indexes.
- [docs/evidence-workflow.md](docs/evidence-workflow.md): receipt schemas, failure bundles, BLAKE3 manifests, Cairn task citations, and non-claim boundaries.
- [docs/configuration.md](docs/configuration.md): Nickel-backed config, source-tree overrides, matrix/compare workflows, Paper fallback, and pinned Cairn/Octet inputs.
- [docs/verification.md](docs/verification.md): focused verification commands and closeout flow.
- [docs/check-tiers.md](docs/check-tiers.md): validation tier taxonomy for docs, generated surfaces, evidence, component code, live/manual rails, and Cairn archive closeout.
- [docs/onixresearch-workspace-namespace.md](docs/onixresearch-workspace-namespace.md): staged `ONIX_RESEARCH_ROOT` migration, compatibility symlink rules, path-reference classification, and non-claims.
- [docs/scenario-derived-surfaces.md](docs/scenario-derived-surfaces.md): scenario-derived source-of-truth and freshness ownership.

## Evidence caveats

The runner forces the GUI client through Xvfb/X11 with software GL and removes inherited Wayland/niri socket environment before launch. A bounded timeout is considered success only when the client log contains connection/render evidence such as detected protocol or loaded dimension data.

The current receipt schema is `mc.compat.scenario.receipt.v2`; receipts also retain the legacy marker `mc.compat.smoke.receipt.v1` for older consumers. Dry-run receipts are deterministic harness-shape evidence only. A receipt is evidence that the bounded scenario ran under the specified local fixture; it is not a broad Minecraft compatibility, semantic equivalence, public-server safety, production readiness, full CTF correctness, or full survival correctness claim. See [docs/evidence-workflow.md](docs/evidence-workflow.md) before promoting or citing evidence.

## Verification

Use [docs/check-tiers.md](docs/check-tiers.md) to choose the smallest sufficient validation tier for the files and claim boundary you changed. The focused closeout guide is in [docs/verification.md](docs/verification.md).

```sh
nix flake check
```

Aggregate Octet enforcement for owned Rust workspaces:

```sh
nix build .#checks.x86_64-linux.mc-octet-monorepo --no-link -L

rustc --edition=2021 tools/check_octet_monorepo.rs -o target/check-octet-monorepo
OCTET_SOURCE_DIR=$(nix eval --raw --impure --expr '(builtins.getFlake (toString ./.)).inputs.octet.outPath')
target/check-octet-monorepo --root . --octet-source "$OCTET_SOURCE_DIR" --run-octet
```

The flake check covers static lint-inventory/config/baseline drift. The direct checker command additionally runs pinned Octet against each owned Rust workspace and rejects new unaccepted stable IDs.
