# Design: Stevenarella protocol version source of truth

## Context

Protocol name parsing and packet-translation dispatch are small compared with packet definitions, but they are high-impact compatibility surfaces. A typed source of truth can make supported versions and reuse relationships auditable without changing the public API.

## Decisions

### 1. Make version metadata explicit

**Choice:** Represent supported protocol rows with canonical name, aliases, numeric protocol id, translation module, and optional reuse/fallback relationship.

**Rationale:** Reviewers can inspect protocol support without reading match arms.

### 2. Preserve current Rust API

**Choice:** Keep `protocol_name_to_protocol_version` and `translate_internal_packet_id_for_version` as public compatibility functions while moving their tables behind generated or validated metadata.

**Rationale:** Client call sites and tests should not need a broad API migration.

### 3. Validate generated surfaces

**Choice:** Add a checker that rejects stale generated dispatch tables, duplicate aliases, missing translation modules, unknown fallback targets, and mismatched protocol ids.

**Rationale:** Data-driving only helps if drift is caught before runtime.

### 4. Keep packet support claims bounded

**Choice:** Adding a row or alias remains a table/dispatch change unless paired packet-boundary evidence explicitly claims packet behavior.

**Rationale:** Version metadata must not overclaim compatibility.

## Risks / Trade-offs

- Introducing generation can add workflow friction; keep checked-in outputs deterministic and provide a focused freshness check.
- Existing panic behavior for unknown inputs may be undesirable but should be preserved first.
- Per-version packet modules may still contain hand-authored translation logic; this change only owns dispatch metadata unless explicitly broadened.
