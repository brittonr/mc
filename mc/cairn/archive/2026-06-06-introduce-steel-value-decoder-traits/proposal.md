# Proposal: Introduce Steel value decoder traits

## Why

`tools/mc-compat-runner/src/runtime_config.rs` repeats typed Steel export decoding across `required_string`, `required_string_list`, `required_u32`, and `required_f64`. The repeated shape makes diagnostics and future config fields drift-prone. A small decoding trait can keep Rust-owned typing explicit while reducing boilerplate.

## What Changes

- Add a pure `FromSteelValue` or equivalent decoder trait for values accepted by the runtime configuration loader.
- Replace repeated required-value helpers with one generic typed helper that preserves path-aware diagnostics.
- Keep the restricted Steel literal subset, sandbox checks, supported exports, range checks, mutability policy, and snapshot schema unchanged.
- Add positive and negative tests for every decoded type and malformed/missing/wrong-type export case.

## Impact

- **Files**: `tools/mc-compat-runner/src/runtime_config.rs` and focused runtime config tests.
- **Testing**: valid default config decoding, missing export, wrong type, malformed literal, range-invalid value, sandbox violation regression, reload planning tests, and Cairn validation/gates.