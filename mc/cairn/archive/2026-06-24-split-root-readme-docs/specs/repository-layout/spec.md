# repository-layout Change Spec: Root README split

## Requirements

### Requirement: README scope

r[repository_layout.readme_split.scope] The root README SHOULD serve as a concise quickstart and navigation index instead of the canonical home for every scenario command and evidence workflow detail.

#### Scenario: README remains navigable

r[repository_layout.readme_split.scope.quickstart]
- GIVEN a new developer opens the root README
- WHEN they read the first sections
- THEN they can identify the repository purpose, main component layout, fastest dry-run/live commands, key evidence caveats, and links to deeper docs
- AND they are not required to scan every maintained scenario row to find the basic workflow.

### Requirement: Scenario command docs

r[repository_layout.readme_split.scenario_docs] Detailed scenario command listings SHOULD live in focused docs or generated indexes that are checked against scenario metadata.

#### Scenario: Scenario command listing stays fresh

r[repository_layout.readme_split.scenario_docs.fresh]
- GIVEN a scenario wrapper, manifest row, or generated command changes
- WHEN documentation freshness checks run
- THEN stale scenario command docs are reported
- AND human-authored prose outside generated blocks is preserved.

### Requirement: Evidence workflow docs

r[repository_layout.readme_split.evidence_docs] Evidence workflow details and non-claim boundaries MUST remain visible from README links after content is moved into focused docs.

#### Scenario: Evidence caveats remain discoverable

r[repository_layout.readme_split.evidence_docs.caveats]
- GIVEN a reader follows README links for evidence workflow
- WHEN they inspect the focused evidence docs
- THEN durable evidence location, BLAKE3 manifest expectations, non-claims, and Cairn task citation rules remain documented
- AND generated command blocks do not overwrite human evidence interpretation.

### Requirement: Config and verification docs

r[repository_layout.readme_split.config_verification_docs] Config and verification details MAY move from README into focused docs only if README keeps stable entry links and commands remain checkable.

#### Scenario: Verification command remains findable

r[repository_layout.readme_split.config_verification_docs.findable]
- GIVEN a developer needs to run repository verification
- WHEN they read the README and linked docs
- THEN focused verification docs list the relevant fast, generated, evidence, and full gates
- AND README links point to those docs.

### Requirement: Documentation checks

r[repository_layout.readme_split.checks] The README split SHOULD include checks for generated doc freshness, command parity, and broken local documentation links when practical.

#### Scenario: Broken moved link fails

r[repository_layout.readme_split.checks.links]
- GIVEN README links to a moved docs page or generated command index
- WHEN the docs check runs
- THEN missing local docs or stale generated command blocks are reported
- AND archive is blocked until docs are refreshed.

### Requirement: README split validation

r[repository_layout.readme_split.validation] The README split MUST record docs checks, generated freshness checks, selected command parity, and Cairn gates before archive.

#### Scenario: Documentation closeout is reviewable

r[repository_layout.readme_split.validation.log]
- GIVEN root README content has been split into focused docs
- WHEN the change is archived
- THEN reviewable logs show docs checks, generated freshness checks, selected command dry-runs, Cairn proposal/design/tasks gates, and Cairn validation.
