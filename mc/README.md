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

## Verification

```sh
nix flake check
```
