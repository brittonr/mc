# Octet reviewed baselines

These baselines capture pre-existing findings after enabling every lint from the repo-pinned Octet input at `deny` level for the owned mc Rust workspaces.

Owner: mc workspace maintainers.

Rationale: `cargo-octet --write-baseline` only writes when the underlying cargo run succeeds. With every lint denied, current findings make cargo exit with code 101 before Octet can emit its built-in baseline. The repo-owned aggregate checker therefore stores reviewed stable IDs here and rejects any new stable ID reported by the pinned `cargo-octet` runner.

Removal condition: fix accepted findings by improving code quality, rerun the aggregate gate, and regenerate the affected baseline with fewer or zero accepted stable IDs.

Baselines:

- `compat-runner.reviewed-baseline.json` for `compat/runner`
- `stevenarella.reviewed-baseline.json` for `clients/stevenarella`
- `valence.reviewed-baseline.json` for `servers/valence`

Run the aggregate gate from the repository root:

```sh
rustc --edition=2021 tools/check_octet_monorepo.rs -o target/check-octet-monorepo
OCTET_SOURCE_DIR=$(nix eval --raw --impure --expr '(builtins.getFlake (toString ./.)).inputs.octet.outPath')
target/check-octet-monorepo --root . --octet-source "$OCTET_SOURCE_DIR" --run-octet
```
