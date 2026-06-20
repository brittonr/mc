# Proposal: Extract mc-compat harness planning core

## Why

The runner mixes scenario planning, backend selection, client session planning, receipt expectations, cleanup behavior, and process orchestration in a large imperative shell. Many regressions can be tested without launching Valence, Paper, Xvfb, Docker, or Stevenarella if the plan is represented as pure data first.

## What Changes

- Extract pure planning cores for server startup, client sessions, scenario expectations, receipt paths, artifact collection, and cleanup actions.
- Keep command execution, filesystem mutation, process management, Docker calls, and environment mutation in thin shell code.
- Add positive and negative tests for plan generation across dry-run, live, matrix, reconnect, multi-client, Paper, Valence, cleanup, and failure paths.
- Preserve existing CLI arguments, receipt names, scenario semantics, and evidence non-claims.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, possible new runner modules, focused runner tests, scenario-manifest checker if surfaces move, evidence docs/manifests, and Cairn artifacts.
- **Testing**: baseline runner tests before refactor, post-refactor runner tests, plan-core positive/negative fixtures, dry-run checks, scenario manifest checks, and Cairn gates/validation.
- **Non-claims**: this is architecture and testability work only; it does not promote new compatibility evidence or change gameplay behavior.
