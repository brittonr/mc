# Proposal: Survival biome and dimension state parity rail

## Why

The survival coverage matrix still lists biome/dimension as missing. A narrow environment-state rail can prove that Stevenarella receives and reports one configured biome or dimension state against both backends before any broad world-generation claim exists.

## What Changes

- Add a bounded protocol-763 `survival-biome-dimension-state` survival scenario for one configured environment-state observation, either a fixed biome sample or a bounded fixture-driven dimension transition, with explicit normalized state fields.
- Add a Stevenarella probe path that can join the configured survival fixture, observe the environment state required by the selected subrail, optionally perform the bounded transition action, and emit client milestones for the decoded state.
- Add paired Paper and Valence fixture instrumentation: Paper and Valence fixtures must select the same subrail, publish the same expected environment identifiers, and log normalized state observations without relying on broad worldgen equivalence.
- Add deterministic checker coverage that rejects missing reference evidence, missing environment identifiers, mismatched biome/dimension value, ambiguous subrail selection, stale child revisions, and Valence-only evidence.
- Promote only the `biome/dimension` survival coverage matrix row after paired evidence passes.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, Stevenarella probe code, `valence/examples/survival_compat.rs`, `tools/paper-survival-fixture/`, row checker, survival matrix/current bundle docs, and `docs/evidence/` artifacts.
- **Testing**: runner unit tests, checker positive and negative fixtures, paired Paper/Valence dry-run or live receipts, BLAKE3 evidence manifests, and Cairn validation/gates.
- **Non-claims**: world-generation parity, all biomes, all dimensions, portal mechanics breadth, lighting/weather parity, structure generation, full survival compatibility, broad vanilla parity, and production readiness.
