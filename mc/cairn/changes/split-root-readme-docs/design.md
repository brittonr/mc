# Design: Split root README into focused documentation

## Context

The README is the primary entrypoint but currently contains detailed command lists for many maintained scenarios. Those lists are useful but crowd out the layout overview and are partly derived from structured scenario data.

## Decisions

### 1. Keep README as an index and quickstart

**Choice:** Root README should describe purpose, layout, fastest dry-run/live command examples, evidence caveats, and links to focused docs.

**Rationale:** New readers should understand the repo before encountering every scenario row.

### 2. Move deep references under docs

**Choice:** Create focused docs for scenarios, commands, evidence workflow, config, and verification tiers as needed.

**Rationale:** Smaller docs map better to ownership and generated sections.

### 3. Generate machine-owned command blocks

**Choice:** Scenario command blocks should be generated or checked against the manifest when the output is stable and bounded.

**Rationale:** Command lists should not drift from flake wrappers and scenario metadata.

### 4. Preserve evidence caveats

**Choice:** Non-claim language and evidence interpretation remain human-authored and visible from README links.

**Rationale:** Documentation cleanup must not weaken claim boundaries.

## Risks / Trade-offs

- Moving content can break bookmarks; mitigate with a README link map.
- Generated docs can hide important caveats; mitigate by keeping caveats outside generated blocks.
- Splitting docs can scatter information; mitigate with an explicit docs index.
