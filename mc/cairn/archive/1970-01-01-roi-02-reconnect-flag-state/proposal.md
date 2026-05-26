# Proposal: Reconnect while holding or touching flag compatibility rail

## Why

Reconnect after scoring is proven, but reconnect under live CTF flag ownership is not. This slice targets stale ownership/session cleanup bugs that are likely to affect real gameplay compatibility.

## What Changes

- Add a continuous-server scenario where a Stevenarella client picks up or touches the flag and disconnects before normal capture.
- Verify a reconnect by the same username, and where useful a peer observer, sees coherent CTF state with no stale owner or phantom score.
- Record deterministic receipt fields, dry-run checks, live evidence, and explicit non-claims.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, `valence/examples/ctf.rs`, `stevenarella/src/server/mod.rs`, `flake.nix`, `README.md`, and `docs/evidence/`.
- **Testing**: runner tests, dry-run Nix check, Valence/Stevenarella checks, one live continuous-server reconnect receipt.
