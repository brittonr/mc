# Hyperion integration boundaries

Use this boundary before any Hyperion-to-Valence implementation work. It classifies sources, blocks unsafe core merges, and keeps gameplay semantics behind optional plugin or reference-evidence gates. This document is governance evidence only; it does not implement a Valence feature or claim Hyperion compatibility.

## Inventory template

Every future integration Cairn that uses Hyperion code or concepts MUST include an inventory row for each inspected source before implementation.

| Field | Required content |
| --- | --- |
| `source_path` | Hyperion path, crate/module/function, or design note inspected |
| `classification` | One of `adopt`, `port`, `reference`, or `reject` |
| `owner` | Person/change responsible for the decision |
| `reason` | Why the classification is safe for Valence |
| `valence_target` | Valence crate/plugin/example/docs target, or `none` for rejected/reference-only work |
| `safety_notes` | Stable Rust, unsafe audit, threading/runtime, allocation, and API compatibility notes |
| `evidence` | Tests, docs, receipts, benchmark, or oracle evidence required before archive |
| `non_claims` | Production scale, vanilla parity, Hyperion compatibility, default behavior, and safety claims left unsupported |

## Classification rules

| Classification | Allowed use | Required boundary |
| --- | --- | --- |
| `adopt` | Directly use a small, audited, license-compatible implementation detail | Requires stable Rust compatibility, explicit unsafe/nightly audit, positive and negative tests, and Valence-owned API review |
| `port` | Reimplement the idea in Valence-owned types and style | Requires pure core plus thin shell, positive and negative tests, and evidence that defaults are preserved |
| `reference` | Use Hyperion only to inform a design or checklist | Requires no copied code, no default behavior change, and explicit non-claims |
| `reject` | Do not use the source for current Valence work | Requires reason and, when useful, a safer replacement or future audit action |

## Forbidden core-merge categories

Integration work MUST NOT merge these categories into Valence core:

- Bedwars-specific game logic, maps, scoring, shops, teams, or event policy.
- Full Hyperion runtime replacement, scheduler replacement, networking runtime swap, or ECS architecture replacement.
- Custom combat as Valence core behavior without separate accepted Valence scope and vanilla/reference evidence.
- Unaudited nightly-only, unsafe-heavy, or architecture-specific code copied directly into Valence core.
- Load/stress tooling that can target public servers without explicit authorization, redaction, and bounded safety controls.

Forbidden sources may still be classified as `reference` when they only motivate a Valence-owned design and no code or unsupported behavior claim is copied.

## Gameplay and optional-plugin boundary

Gameplay semantics inspired by Hyperion MAY be implemented only as optional plugins, examples, or reference notes unless a separate accepted Valence scope and reference evidence justify core behavior.

Before archive, gameplay integration work MUST state:

- whether Valence core behavior changes;
- whether the behavior is an optional plugin, example, fixture, or docs-only reference;
- paired reference evidence required for vanilla/parity claims;
- fallback/default behavior when the plugin is disabled;
- non-claims for broad combat, GUI, anti-cheat, production scale, security, and Hyperion compatibility.

Combat, GUI helpers, anti-cheat observations, Bedwars gameplay, projectile semantics, and CTF-specific behavior default to optional plugin/example/reference scope.

## Review gate checklist

Future Hyperion integration Cairns SHOULD cite this checklist before archive.

- [ ] Inventory rows exist for all inspected Hyperion sources.
- [ ] Each row has one classification: `adopt`, `port`, `reference`, or `reject`.
- [ ] Forbidden core-merge categories are rejected or reference-only.
- [ ] Optional gameplay/plugin boundaries are explicit.
- [ ] Positive and negative tests are listed for adopted or ported code.
- [ ] Nightly/unsafe-heavy code has an audit or is not copied.
- [ ] Default Valence behavior is unchanged unless the Cairn explicitly scopes a change.
- [ ] Production-scale, vanilla-parity, Hyperion-compatibility, default-behavior, and safety claims are supported by evidence or explicitly recorded as non-claims.
- [ ] Reviewable logs and BLAKE3 manifests live under `docs/evidence/`.

## Positive and negative examples

### Positive: reference-only routing idea

| Field | Example |
| --- | --- |
| `source_path` | `hyperion/crates/.../broadcast.rs` design notes |
| `classification` | `reference` |
| `valence_target` | `docs` or a Valence-owned pure planner design |
| `reason` | Routing vocabulary informs a Valence API, but no Hyperion code is copied |
| `evidence` | Checker output, planner tests if implemented later |
| `non_claims` | No production-scale, default-behavior, or Hyperion-compatibility claim |

This passes because it records the source, does not copy code, and leaves broad claims as non-claims.

### Positive: port stable pure math helper

| Field | Example |
| --- | --- |
| `source_path` | Hyperion geometry helper used as algorithm reference |
| `classification` | `port` |
| `valence_target` | Valence-owned pure math module or optional plugin helper |
| `reason` | Reimplemented with Valence types, stable Rust, and deterministic tests |
| `evidence` | Positive and negative fixtures for boundary/NaN/zero-vector cases |
| `non_claims` | No vanilla combat parity or broad projectile correctness claim |

This passes only if the implementation is Valence-owned and tested without hidden runtime coupling.

### Negative: direct Bedwars import

| Field | Example |
| --- | --- |
| `source_path` | `hyperion/events/bedwars/...` |
| `classification` | `adopt` |
| `valence_target` | Valence core |
| `reason` | Copy Bedwars behavior into default server logic |

This fails because Bedwars-specific game logic is a forbidden core merge.

### Negative: unaudited unsafe runtime copy

| Field | Example |
| --- | --- |
| `source_path` | Hyperion runtime/networking internals with unsafe/nightly coupling |
| `classification` | `adopt` |
| `valence_target` | Valence core runtime |
| `reason` | Performance shortcut without separate audit |

This fails because runtime replacement and unaudited unsafe-heavy code are forbidden in Valence core.
