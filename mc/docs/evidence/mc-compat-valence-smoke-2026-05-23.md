# MC compatibility Valence smoke evidence — 2026-05-23

## Scope

This records the refreshed live Stevenarella → Valence compatibility smoke for the local `/home/brittonr/git/mc` harness.

## Inputs

- Workspace commit: `09b9071`
- Command:
  ```sh
  nix run .#mc-compat-smoke -- --run --server-backend valence --receipt target/mc-compat-smoke-valence.json
  ```
- Client: `/home/brittonr/git/mc/stevenarella`
- Server backend: `valence`
- Valence revision: `8ad9c85`
- Valence worktree: `/tmp/valence-compat-758`
- Valence example: `terrain`
- Expected Minecraft version/protocol: `1.18.2` / `758`
- Headless isolation: Xvfb/X11/software GL, no inherited Wayland socket

## Result

- Receipt status: `pass`
- Receipt mode: `run`
- Server status probe: protocol `758`
- Client exit code: `124`
- Client classification: `timeout-success-evidence`
- Matched success pattern: `Detected server protocol version`
- Client log path from this run: `/tmp/mc-compat-client.1779514835613.log`
- Receipt path: `target/mc-compat-smoke-valence.json`
- Receipt BLAKE3: `24923f2adec8212f86ba5fd8d0aa7ddf4f6cd778b73ae5d53cf5195ff911ece6`

Relevant live client evidence printed during the run:

```text
Detected server protocol version 758
Dimension type: { ... "effects": String("overworld"), "min_y": Int(-64), "height": Int(384), ... }
```

Post-run harness status:

```text
paper_container=absent
valence_pid=absent
valence_worktree=/tmp/valence-compat-758 exists=true
valence_target_dir=/tmp/valence-compat-758-target exists=true
valence_log=/tmp/mc-compat-valence.log exists=true
client_logs=1
client_log=/tmp/mc-compat-client.1779514835613.log
```

## Non-claims

This is a bounded compatibility smoke receipt. It proves the harness ran with the selected inputs and observed connection/world-load evidence. It does not claim Minecraft correctness or semantic equivalence.
