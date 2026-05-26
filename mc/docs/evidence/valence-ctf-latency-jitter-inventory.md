# Valence CTF bounded latency/jitter inventory evidence

## Scope

This evidence covers the bounded protocol-763 Valence CTF latency/jitter tolerance rail for Stevenarella by reusing the maintained `inventory-interaction` semantic scenario. The wrapper records bounded perturbation parameters and fails closed through the existing semantic milestone oracle without requiring privileged network mutation.

This is a local compatibility fixture only. It does **not** claim WAN safety, adversarial network safety, packet loss coverage beyond the recorded zero-loss setting, production load tolerance, full inventory correctness, full CTF correctness, or broad Minecraft compatibility.

## Maintained command

```bash
cd /home/brittonr/git/mc
nix run .#mc-compat-valence-ctf-latency-jitter-inventory
```

Deterministic dry-run gate:

```bash
cd /home/brittonr/git/mc
nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-valence-ctf-latency-jitter-inventory-dry-run --no-link -L
```

## Live receipt

- Receipt path: `target/mc-compat-latency-jitter/latency-jitter-inventory.json`
- BLAKE3: `a4a407fb1ac3aceae06faeacb794891ff8411c8ac86470c651c89b37b6c7f33d`
- Schema: `mc.compat.scenario.receipt.v2`
- Scenario: `inventory-interaction`
- Status: `pass`
- Client classification: `timeout-success-evidence`
- Valence example: `ctf`
- Protocol/version: `763` / `1.20.1`
- Client username: `compatbot`

## Perturbation receipt fields

The live receipt records:

- `latency_jitter_tolerance.selected: true`
- `mechanism: bounded-client-cadence`
- `target_rail: inventory-interaction`
- `delay_ms: 80`
- `jitter_ms: 30`
- `loss_percent: 0`
- `timeout_secs: 180`
- `hygiene_status: bounded-local-fixture`
- `privileged_network_mutation_required: false`
- `fail_closed_when_unavailable: true`
- `claims_wan_safety: false`
- `claims_adversarial_network_safety: false`

## Semantic evidence preserved

The perturbation rail did not weaken the reused inventory oracle. The live receipt still requires and observes all client milestones:

- `protocol_detected`
- `join_game`
- `render_tick`
- `team_red`
- `inventory_slot_update`
- `inventory_sword_slot`
- `inventory_wool_slot`
- `inventory_drop_sent`
- `inventory_pickup_seen`
- `inventory_click_sent`
- `inventory_open_container_seen`
- `inventory_container_click_sent`
- `inventory_block_place_sent`

It also requires and observes server-side Valence correlation:

- `server_username_seen`
- `server_inventory_hotbar_select`
- `server_inventory_drop`
- `server_inventory_pickup`
- `server_inventory_click`
- `server_inventory_open_container`
- `server_inventory_container_click`
- `server_block_place`

The receipt had no missing client milestones, no missing server milestones, no forbidden-pattern matches, and `triage.suggested_boundary: none`.

## Verification run

Commands run for this slice:

```bash
cd /home/brittonr/git/mc/tools/mc-compat-runner
nix shell nixpkgs#cargo nixpkgs#rustc nixpkgs#gcc -c cargo test

cd /home/brittonr/git/mc
nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-valence-ctf-latency-jitter-inventory-dry-run --no-link -L
nix run .#mc-compat-valence-ctf-latency-jitter-inventory
python3 -m json.tool target/mc-compat-latency-jitter/latency-jitter-inventory.json >/tmp/latency-jitter-receipt.pretty.json
nix shell nixpkgs#b3sum -c b3sum target/mc-compat-latency-jitter/latency-jitter-inventory.json
grep -E 'panic|UnexpectedEof|protocol mismatch|decode error|Decode|thread .* panicked' /tmp/mc-compat-client.compatbot.1779768364370.log /tmp/mc-compat-valence.log || true
```

The remote builder warning for `ssh-ng://root@10.10.10.1` during the Nix dry-run gate was non-blocking; Nix fell back locally and the check exited 0.
