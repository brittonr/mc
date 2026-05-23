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
- Copy `tools/playground/src/playground.template.rs` to `tools/playground/src/playground.rs` before fmt/clippy/test/doc if `playground.rs` is missing.
- Octet starter rollout uses checked-in `scripts/octet-check.sh`, which runs `nix build .#checks.x86_64-linux.octet --no-link -L --impure`.
- Low-noise starter scope for first Octet pass: `valence_math`, `valence_lang`, `valence_ident`, `valence_text`.

## Patterns That Don't Work
- Treat `mc/` workspace folder as if it were the `valence/` repo root.

## Domain Notes
- `valence/` is a Rust workspace with root crate in `src/`, member crates in `crates/*`, and tools in `tools/*`.
- `cargo +nightly fmt --all` needed because `rustfmt.toml` uses unstable settings.
- Octet starter inventory on 2026-04-14 found low-noise counts in `valence_math` (0), `valence_lang` (0), `valence_ident` (2), and `valence_text` (2); keep `valence_server` and `valence_network` deferred until dedicated cleanup.
- A no-arg legacy Dylint run that included `valence_server_common` tripped a `dylint-driver` SIGSEGV while compiling transitive `valence_generated` on nightly-2026-03-21; keep that crate out of the starter default until toolchain or Dylint behavior changes.
