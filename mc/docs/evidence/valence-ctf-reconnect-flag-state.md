# Valence CTF reconnect flag-state compatibility evidence

## Scope

This evidence covers the bounded protocol-763 Valence CTF reconnect flag-state rail for the Stevenarella client. It runs one continuous owned local Valence CTF server, connects Stevenarella as `compatbot`, selects RED, picks up the BLUE flag, disconnects by timeout, reconnects with the same username, and requires server-side flag-return/state-coherence milestones.

This is not a claim of full CTF correctness, broad Minecraft compatibility, production load safety, or unbounded reconnect safety. It is a deterministic bounded receipt for the specified local fixture.

## Maintained command

```sh
cd /home/brittonr/git/mc
nix run .#mc-compat-valence-ctf-reconnect-flag-state
```

Dry-run/check fixture:

```sh
cd /home/brittonr/git/mc
nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-valence-ctf-reconnect-flag-state-dry-run --no-link -L
```

## Live receipt

- Receipt: `target/mc-compat-reconnect-flag-state/reconnect-flag-state.json`
- BLAKE3: `4d848af56b25ad4b3c466863bac5b2052adbbc1c59e2b2164bfb2a696c225cb3`
- Status: `pass`
- Scenario: `reconnect-flag-state`
- Client milestones: `protocol_detected`, `join_game`, `render_tick`, `team_red`, `flag_pickup`, `reconnect_session`
- Server milestones: `server_username_seen`, `server_flag_pickup`, `server_flag_disconnect_return`, `server_reconnect_state_coherent`
- Forbidden patterns include panic/protocol/decode failures and unexpected flag capture/score milestones.

## Observed server correlation

The Valence log recorded:

```text
MC-COMPAT-MILESTONE flag_pickup username=compatbot carrier_team=Red flag_team=Blue
MC-COMPAT-MILESTONE flag_disconnect_return carrier=compatbot flag_team=blue reason=client_disconnect score_unchanged=true
MC-COMPAT-MILESTONE reconnect_state_coherent username=compatbot team=Red reconnect_session=2 red_flag_held=false blue_flag_held=false
```

The receipt recorded two same-username client log paths and `client_server_correlation.passed=true`.
