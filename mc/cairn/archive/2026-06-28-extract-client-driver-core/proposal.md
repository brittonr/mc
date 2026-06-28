# Proposal: Extract mc-compat client-driver functional core

## Why

`client_driver::run_client` mixes process execution, dry-run evidence, multi-client orchestration, log combination, scenario evaluation, projectile checks, server-correlation checks, and classification. Much of that logic can be tested without Xvfb, Stevenarella, Valence, Paper, filesystem reads, or process exits.

## What Changes

- Extract pure client run planning, combined-output construction, scenario evaluation selection, projectile evidence selection, and client-run classification cores.
- Keep process spawning, log file reads/writes, sleeps, server restarts, and stdout/stderr in thin shell functions.
- Add positive and negative tests for dry-run, single-client, reconnect, multi-client, projectile, server-correlation, and failure classifications.
- Preserve existing evidence fields, receipt classification strings, log paths, usernames, and non-claims.

## Impact

- **Files**: `compat/runner/src/client_driver.rs`, evidence modules, planning modules, focused tests, and Cairn artifacts.
- **Testing**: baseline client-driver tests, positive/negative pure-core fixtures, dry-run checks, runner tests, Cairn gates, and Cairn validation.
- **Non-claims**: client-driver architecture only; no new live compatibility evidence or gameplay semantic change.
