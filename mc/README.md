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

Paper remains available as a fallback/control backend:

```sh
nix run .#mc-compat-smoke -- --run --server-backend paper
```

## Verification

```sh
nix flake check
```
