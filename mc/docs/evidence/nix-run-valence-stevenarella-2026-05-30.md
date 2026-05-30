# Nix-run Valence/Stevenarella launch checkpoint — 2026-05-30

## Question

Can the root flake execute the real non-dry-run `nix run .#valence` and `nix run .#stevenarella` paths, not only the dry-run wrappers?

## Inspected evidence

- Launch log: `docs/evidence/nix-run-valence-stevenarella-2026-05-30.run.log`, BLAKE3 `542f5d0250493823cf87019d20f6ae99019c9de6dad5e076f2882bc8de00f8b0`.
- Flake app definitions: `flake.nix`, BLAKE3 `420bb3a249911a2b94b86e24a10da2ec29963e66ee06f6c3382435c58a6bed6d`.
- README command docs: `README.md`, BLAKE3 `49a9a8f31746aa8ba173b9a19c30c8155b77f39740fd25d00d92d95b47ecdb29`.
- Valence real path ran `cargo run --example ctf` through `nix run .#valence`, built the CTF example, reached `Running .../target/nix-run-valence/debug/examples/ctf`, then stopped by bounded timeout with `valence_exit_status=124`.
- Stevenarella real path ran `cargo run` through `nix run .#stevenarella`, auto-wrapped Xvfb because `DISPLAY` was absent, reached `Running .../target/nix-run-stevenarella/debug/stevenarella`, logged `Starting steven` and `Shader version: #version 150`, then stopped by bounded timeout with `stevenarella_exit_status=124`.

## Decision

The root flake app wiring is sufficient for launching both editable local checkouts. The timeout statuses are expected because both commands are long-running interactive processes; success is the observed transition from wrapper to real Cargo run plus Valence/Stevenarella startup markers.

## Owner

mc compatibility maintainer.

## Next action

Use `nix run .#valence` to keep the Valence CTF example running, and `nix run .#stevenarella` to start the client. Keep dry-run checks for cheap CI shape coverage and refresh this checkpoint when either app wrapper changes.
