# Tasks

- [x] [serial] Define the generated-surface contract, generated-block ownership markers, runtime no-Nickel boundary, and non-claim scope. r[mc_compatibility.generated_harness_surfaces.contract]
  Evidence: docs/evidence/generated-harness-surfaces-2026-06-20.run.log; docs/evidence/generated-harness-surfaces-2026-06-20.b3
- [x] [depends:contract] Add a pure manifest-to-surface generator core with positive and negative fixtures for valid rows, missing required fields, unsupported migration states, duplicate generated names, and unsafe output paths. r[mc_compatibility.generated_harness_surfaces.generator]
  Evidence: docs/evidence/generated-harness-surfaces-2026-06-20.run.log; docs/evidence/generated-harness-surfaces-2026-06-20.b3
- [x] [depends:generator] Generate checked-in Rust scenario tables and any stable wrapper metadata from the manifest while preserving existing scenario semantics. r[mc_compatibility.generated_harness_surfaces.rust_tables]
  Evidence: docs/evidence/generated-harness-surfaces-2026-06-20.run.log; docs/evidence/generated-harness-surfaces-2026-06-20.b3
- [x] [depends:rust_tables] Add generated README/index blocks only where output is stable, bounded, and clearly marked as machine-owned. r[mc_compatibility.generated_harness_surfaces.docs_blocks]
  Evidence: docs/evidence/generated-harness-surfaces-2026-06-20.run.log; docs/evidence/generated-harness-surfaces-2026-06-20.b3
- [x] [depends:docs_blocks] Add a Nix freshness check that regenerates outputs and fails on checked-in drift. r[mc_compatibility.generated_harness_surfaces.freshness]
  Evidence: docs/evidence/generated-harness-surfaces-2026-06-20.run.log; docs/evidence/generated-harness-surfaces-2026-06-20.b3
- [x] [depends:freshness] Run generator tests, runner tests, scenario-manifest checks, maintained dry-run aggregate, evidence manifest checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.generated_harness_surfaces.validation]
  Evidence: docs/evidence/generated-harness-surfaces-2026-06-20.run.log; docs/evidence/generated-harness-surfaces-maintained-dry-runs-2026-06-20.run.log; docs/evidence/generated-harness-surfaces-evidence-manifest-2026-06-20.run.log; docs/evidence/generated-harness-surfaces-2026-06-20.b3; docs/evidence/generated-harness-surfaces-maintained-dry-runs-2026-06-20.b3; docs/evidence/generated-harness-surfaces-evidence-manifest-2026-06-20.b3
