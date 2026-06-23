# Design: Define repository check tiers

## Context

Not every change should run the full evidence matrix, but every change needs a defensible validation choice. Tiers can name intent and reduce over/under-testing.

## Decisions

### 1. Name tiers by review purpose

**Choice:** Define tiers around outcomes: fast source validation, generated freshness, evidence validation, component behavior, live/manual evidence, and archive closeout.

**Rationale:** Developers choose checks based on what changed and what evidence is needed.

### 2. Map existing checks first

**Choice:** Start by classifying current flake checks, apps, manual commands, and Cairn gates rather than inventing many new wrappers.

**Rationale:** The taxonomy should reflect the real repo.

### 3. Keep live/manual checks opt-in

**Choice:** Live rails and expensive/manual evidence runs are named but not made default unless an existing gate already requires them.

**Rationale:** Some checks require local services, time, or human review.

### 4. Record validation rationale in tasks

**Choice:** Cairn closeout should cite the tier(s) run and why they are sufficient for the touched scope.

**Rationale:** Reviewers can evaluate validation coverage without guessing.

## Risks / Trade-offs

- Too many tiers can confuse users; mitigate with a small required set and examples.
- Tier wrappers can drift from underlying checks; mitigate with generated or evaluated inventories.
- Naming tiers can be mistaken for stronger guarantees; mitigate with explicit non-claims.
