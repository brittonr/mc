# Agent Notes

## Scope
- Valence is a Rust workspace for building Minecraft: Java Edition servers.
- Root crate lives in `src/`.
- Supporting crates live under `crates/`.
- Runnable examples live under `examples/`.
- Scratch reproduction harness lives under `tools/playground`.
- Protocol/data extraction lives under `extractor/`.
- Docs site lives under `website/`.

## Workflow
- Read `README.md`, `CONTRIBUTING.md`, and the relevant crate before editing.
- Run cargo from repo root; workspace members are declared in the top-level `Cargo.toml`.
- CI copies `tools/playground/src/playground.template.rs` to `tools/playground/src/playground.rs` before fmt, clippy, tests, and docs. Do same locally if `playground.rs` is missing.

## Checks
- `cargo +nightly fmt --all` — `rustfmt.toml` uses unstable settings.
- `cargo clippy --workspace --no-deps --all-features --all-targets -- -D warnings`
- `cargo test --workspace --all-features --all-targets`
- `cargo test --workspace --all-features --doc`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --document-private-items`
- CI also runs Miri on nightly, `cargo udeps --all --all-features`, and `cargo test -p valence_nbt --all-targets`.

## Linux dependencies
- Docs, clippy, and test jobs on Linux install: `libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev libclang-dev libgtk-3-dev`.
- Workspace depgraph checks require `cargo-depgraph` and `graphviz`; when workspace structure changes, refresh `assets/depgraph.svg`.

## Octet
- Starter rollout command: `scripts/octet-check.sh`.
- Flake gate: `nix build .#checks.x86_64-linux.octet --no-link -L --impure`.
- Strict starter scope is `valence_math`, `valence_lang`, `valence_ident`, `valence_text`, `valence_weather`, `valence_world_border`, `valence_boss_bar`, `valence_player_list`, `valence_scoreboard`, `valence_equipment`, and `valence_advancement`; broaden beyond these only after a dedicated cleanup pass.
- Workspace runner config lives in `[workspace.metadata.octet]` in `Cargo.toml`.
- Workspace lint tuning config lives in `[octet]` in `dylint.toml`.
- Keep `valence_server_common` out of the starter default for now; a no-arg legacy Dylint run that pulled its `valence_generated` dependency path hit a `dylint-driver` SIGSEGV on nightly-2026-03-21.

## Conventions from `CONTRIBUTING.md`
- Keep modules top-down: parent/public items above private helpers and child types.
- Getters should not use a `get_` prefix.
- Bevy event types should end with `Event`.
- New dependencies should use full semver strings.
- Unit tests should unwrap errors instead of returning `Result`.

## Examples and playground
- Quick smoke test: `cargo r -r --example parkour`.
- Repeatable fork-local smoke receipt: `scripts/smoke-parkour.sh` builds the `parkour` example in the Nix devshell, waits for `:25565`, then cleans up the server process. Pass `--receipt <path>` to write `valence.parkour-smoke.receipt.v1` JSON; `--dry-run --receipt <path>` is checked by `nix build .#checks.x86_64-linux.parkour-smoke-receipt --no-link`.
- `game_of_life`, `terrain`, and `cow_sphere` are called out in the README as good examples to try.
- `tools/playground` is preferred scratch area for reproductions and small experiments.

## Extractor
- `extractor/` is a Gradle/Fabric project.
- To regenerate extracted data:
  1. `cd extractor`
  2. `./gradlew runServer`
  3. `./copy_extractor_output.sh`
- CI builds extractor with Java 21.
- Minecraft version bumps also touch `gradle.properties`, `src/main/resources/fabric.mod.json`, and `crates/valence_protocol/src/lib.rs`, then require rerunning extractor and example smoke tests.

## Website
- `website/` uses Zola plus mdBook and nightly rustdoc.
- `website/build.sh` builds the full site into `website/public`.
- For local content work, use `zola serve` for site pages and `mdbook serve` for the book.
