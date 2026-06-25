# Design: Isolate runner JSON and evidence rendering

## Context

The runner intentionally has a small dependency footprint, but low-level JSON and evidence rendering should not live in the top-level process shell. Whether the implementation adopts typed `serde` structs or retains a dependency-light parser, schema logic needs an explicit module boundary and negative tests.

## Decisions

### 1. Define schema-owned data types

Receipt, failure bundle, typed-event artifact, latency/jitter, public-server safety, MCP control, frame artifact, and scenario-evidence structures should live in evidence modules with explicit render/parse contracts.

### 2. Keep IO in shell

Evidence modules should parse/render in-memory strings and typed structs. Reading/writing files, computing file hashes, creating directories, and printing diagnostics remain shell responsibilities.

### 3. Preserve existing schemas

The current receipt schema and legacy marker fields are compatibility surfaces. The first extraction should keep byte-level output stable where practical or document harmless ordering/format differences only if consumers permit them.

### 4. Prefer typed validation over stringly helpers

Parsing should return typed results or diagnostics naming the field/schema. String helper functions can remain private to the evidence module if dependency constraints block `serde`.

## Risks / Trade-offs

- Introducing `serde` increases dependencies but reduces parser risk; a separate implementation decision should weigh that.
- Byte-for-byte receipt stability may constrain formatting; schema compatibility matters more than cosmetic rewrites.
- Evidence code must not silently widen compatibility claims while moving structs.
