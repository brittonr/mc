# Proposal: Modularize Valence CTF compatibility fixture

## Why

`servers/valence/examples/ctf.rs` combines arena setup, runtime config, Steel arrow policy loading, flag/scoring rules, inventory probes, combat/projectile probes, team assignment, schedule contracts, milestone formatting, and tests in one large example file. That makes bounded compatibility probes harder to review and encourages new gameplay or evidence logic to accumulate in the example shell.

## What Changes

- Split the CTF fixture into focused modules for runtime config, arena construction, team/flag/scoring rules, inventory probes, combat/projectile probes, schedule contracts, and milestone formatting.
- Move non-trivial rule and probe decisions into pure cores that return explicit decisions for Bevy systems to apply.
- Keep Bevy ECS queries, resource mutation, packet/event emission, filesystem reads, and logging in thin system shells.
- Preserve existing CTF fixture behavior, env flags, milestone vocabulary, schedule contracts, receipt evidence boundaries, and non-claims.
- Add positive and negative tests for extracted rule/probe cores and module boundary checks.

## Impact

- **Files**: `servers/valence/examples/ctf.rs`, new `servers/valence/examples/ctf/*` or fixture-core modules, focused tests, docs if ownership is documented, and Cairn artifacts.
- **Testing**: baseline CTF/example checks, focused positive and negative fixture-core tests, affected mc-compat dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: fixture architecture only; this does not promote new CTF correctness, public-server safety, or broad Minecraft compatibility evidence.
