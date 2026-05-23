# Minecraft Rust compatibility smoke

This workspace contains local Minecraft compatibility experiments. The hardened smoke harness checks a Rust client against a Rust server:

- client: `stevenarella`
- server: Valence pinned to Minecraft `1.18.2` / protocol `758`
- runner: `tools/mc-compat-runner`, packaged by the root flake

The legacy shell entrypoint is intentionally only a thin compatibility shim around the flake app.

## Commands

Dry-run the plan without starting the server or client:

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

The runner forces the GUI client through Xvfb/X11 with software GL and removes inherited Wayland/niri socket environment before launch. A bounded timeout is considered success only when the client log contains connection/render evidence such as detected protocol or loaded dimension data.

Write a machine-readable smoke receipt for Cairn/Octet evidence flows:

```sh
SMOKE_RECEIPT=target/mc-compat-smoke.json CLIENT_TIMEOUT=8 nix run .#mc-compat-smoke -- --run
# or
nix run .#mc-compat-smoke -- --dry-run --server-backend paper --receipt target/mc-compat-smoke.json
```

The receipt schema is `mc.compat.smoke.receipt.v1`. It records the server/client inputs, the headless-isolation contract (`wayland_socket_inherited=false`), the matched client success pattern when present, and explicit non-claims (`claims_correctness=false`, `claims_semantic_equivalence=false`) for downstream Cairn/Octet review. It is evidence that the bounded smoke ran under the selected inputs, not a proof of Minecraft correctness or semantic equivalence.

Compare fallback/control Paper and intended/default Valence receipts:

```sh
nix run .#mc-compat-smoke -- --compare-receipts \
  target/mc-compat-smoke.json \
  target/mc-compat-smoke-valence.json
```

The comparison requires one `paper` receipt and one `valence` receipt, both passing, both protocol `758`, expected backend ports, successful client evidence, and niri-safe Xvfb/X11/software-GL isolation.

## Editable Stevenarella checkout

Stevenarella is intentionally a local sibling checkout so it can be patched while debugging the client side of the compatibility seam. By default the runner expects `./stevenarella` to be an editable Stevenarella repository root containing `Cargo.toml`.

Use another checkout without moving files:

```sh
nix run .#mc-compat-smoke -- --dry-run --client-dir /path/to/stevenarella
# or
CLIENT_DIR=/path/to/stevenarella nix run .#mc-compat-smoke -- --dry-run
```

If the checkout is missing or does not look like the repository root, the runner fails before starting the smoke and tells you whether to clone Stevenarella or pass `--client-dir` / `CLIENT_DIR`.

## Editable Valence checkout

Valence is intentionally a local sibling checkout so it can be patched while debugging the compatibility seam. By default the runner expects:

- `./valence` to be an editable Valence git checkout
- `VALENCE_REV=c86b828^` to exist in that checkout; this is the compatible Minecraft `1.18.2` / protocol `758` revision
- `VALENCE_WORKTREE=/tmp/valence-compat-758` to be a disposable detached worktree created from that checkout

Use another checkout without moving files:

```sh
nix run .#mc-compat-smoke -- --dry-run --valence-repo /path/to/valence
# or
VALENCE_REPO=/path/to/valence nix run .#mc-compat-smoke -- --dry-run
```

If the checkout or revision is missing, the runner fails before starting the smoke and tells you whether to clone/fetch Valence or pass `--valence-repo` / `VALENCE_REPO`.

Paper remains available as a fallback/control backend:

```sh
nix run .#mc-compat-smoke -- --run --server-backend paper
```

## OnixResearch tool inputs

The flake pins Cairn and Octet over SSH so private/internal remotes are fetched through the user's SSH agent:

- `cairn`: `git+ssh://git@github.com/onixresearch/cairn.git`
- `octet`: `git+ssh://git@github.com/onixresearch/octet.git`

The dev shell exposes `cairn` and `cargo-octet` alongside the smoke runner:

```sh
nix develop
cairn --help
cargo-octet --help
```

The packages are also available as `.#cairn`, `.#cargo-octet`, and `.#octet`.

## Verification

```sh
nix flake check
```

The flake includes focused checks for the runner binary, dry-run receipt emission, Paper/Valence receipt comparison fixtures, missing-checkout diagnostics, help text, Cairn CLI availability, and Octet fingerprint smoke over the receipt producer surface (`mc-compat-receipt-contract`).
