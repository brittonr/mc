# Design: Stevenarella model core extraction

## Context

The model module owns both data-loading shells and deterministic model decisions. Extracting the deterministic pieces will make block model changes safer and help isolate rendering/resource side effects.

## Decisions

### 1. Split resource/model/geometry decisions

**Choice:** Separate resource reference parsing, model path normalization, state-model selection, multipart rules, raw-model inheritance, geometry/face planning, and light/biome calculations.

**Rationale:** Each decision has independent fixtures and failure modes.

### 2. Keep external data access in shells

**Choice:** Resource manager reads, JSON decoding, texture lookup, random model choice, and renderer allocation remain outside pure cores.

**Rationale:** Model decisions should be tested with in-memory raw summaries.

### 3. Preserve public model behavior

**Choice:** Existing public types and model selection behavior remain stable through compatibility adapters when modules move.

**Rationale:** Render and world call sites should not need a broad migration.
