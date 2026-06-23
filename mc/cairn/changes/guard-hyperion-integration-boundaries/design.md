# Design: Guard Hyperion integration boundaries

## Context

The recommended integration path is selective: proxy ideas, byte-backed protocol work, packet compose APIs, chunk cache concepts, spatial helpers, tools, and optional plugins. The rejected path is wholesale merger. Future work needs a durable guardrail so implementation Cairns do not silently broaden scope.

## Decisions

### 1. Classify source before porting

**Choice:** Every future Hyperion integration Cairn records whether each source is adopted, ported, used as reference, or rejected.

**Rationale:** Reviewers can see whether code is being copied or only informing a Valence-native implementation.

### 2. Forbid core gameplay imports without evidence

**Choice:** Bedwars logic, custom combat mechanics, and game-mode policy cannot enter Valence core. Gameplay semantics require optional plugin scope and reference/compat evidence.

**Rationale:** Valence should stay modular and not inherit Hyperion-specific game design.

### 3. Require nightly/unsafe audit

**Choice:** Nightly-only or unsafe-heavy code requires a separate audit before adoption; otherwise the idea must be reimplemented with stable, reviewed Valence-owned code.

**Rationale:** Performance shortcuts should not bypass Valence safety and maintenance expectations.

### 4. Keep non-claims explicit

**Choice:** Integration Cairns must state what evidence does and does not prove, especially for production scale, vanilla parity, and Hyperion compatibility.

**Rationale:** Boundary discipline prevents roadmap notes from becoming unsupported claims.

## Risks / Trade-offs

- Extra review steps can slow integration; mitigate with reusable inventory and templates.
- Some useful Hyperion code may need substantial rewrite; acceptable for stable Valence ownership.
- Boundary docs can drift; require validation or checklist use in each integration Cairn.
