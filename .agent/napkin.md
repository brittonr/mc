# Napkin

## Corrections
| Date | Source | What Went Wrong | What To Do Instead |
|------|--------|----------------|-------------------|
| 2026-04-14 | self | Used `bash` with `sed` to read source file snippets during repo inspection | Prefer `read` for file contents; use `bash` only for file discovery or commands |

## User Preferences
- Keep replies terse and direct.

## Patterns That Work
- Read `AGENTS.md`, `README.md`, `CONTRIBUTING.md`, and top-level `Cargo.toml` before editing.
- Run cargo from `valence/` repo root.
- In this environment, `cargo` is not on PATH outside the devshell; use `nix develop --no-update-lock-file -c cargo ...`.
- Copy `tools/playground/src/playground.template.rs` to `tools/playground/src/playground.rs` before fmt/clippy/test/doc if `playground.rs` is missing.
- Octet starter rollout uses checked-in `scripts/octet-check.sh`, which runs `nix build .#checks.x86_64-linux.octet --no-link -L --impure`.
- Strict starter scope for the Octet burndown: `valence_math`, `valence_lang`, `valence_ident`, `valence_text`, `valence_weather`, `valence_world_border`, `valence_boss_bar`, `valence_player_list`, `valence_scoreboard`, `valence_equipment`, `valence_advancement`, `valence_anvil`, `valence_inventory`, `valence_registry`, `valence_build_utils`, `valence_ident_macros`, `valence_protocol_macros`, `valence_command_macros`.

## Patterns That Don't Work
- Treat `mc/` workspace folder as if it were the `valence/` repo root.
- Run `nix develop --no-update-lock-file -c cargo +nightly fmt ...`; the devshell cargo is not a rustup proxy and treats `+nightly` as a subcommand. Use `cargo fmt` inside the devshell or `rustfmt --check` for one file.

## Domain Notes
- `valence/` is a Rust workspace with root crate in `src/`, member crates in `crates/*`, and tools in `tools/*`.
- `cargo +nightly fmt --all` needed because `rustfmt.toml` uses unstable settings.
- Octet strict starter burndown cleaned `valence_math`, `valence_lang`, `valence_ident`, `valence_text`, `valence_weather`, `valence_world_border`, `valence_boss_bar`, `valence_player_list`, `valence_scoreboard`, `valence_equipment`, `valence_advancement`, `valence_anvil`, `valence_inventory`, `valence_registry`, `valence_build_utils`, `valence_ident_macros`, `valence_protocol_macros`, and `valence_command_macros`; next broadening candidates should stay below `valence_server_common` until the Dylint driver/source-map caveat is revisited.
- A no-arg legacy Dylint run that included `valence_server_common` tripped a `dylint-driver` SIGSEGV while compiling transitive `valence_generated` on nightly-2026-03-21; keep that crate out of the starter default until toolchain or Dylint behavior changes.
