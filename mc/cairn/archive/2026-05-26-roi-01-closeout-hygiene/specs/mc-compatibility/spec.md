# Delta: Closeout hygiene for drained ROI evidence

## Requirements

### Requirement: Archive Date

r[mc_compatibility.roi_01_closeout_hygiene.archive_date] The system MUST use a real dated ROI-05 archive directory instead of the placeholder 1970 date.

#### Scenario: Archive Date evidence is required

r[mc_compatibility.roi_01_closeout_hygiene.archive_date.scenario]
- GIVEN `Closeout hygiene for drained ROI evidence` is drained
- WHEN the evidence and checks are reviewed
- THEN `archive_date` is satisfied by a tracked artifact or deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Matrix Parent

r[mc_compatibility.roi_01_closeout_hygiene.matrix_parent] The acceptance matrix MUST identify the combat knockback parent evidence commit as `bb1400d`.

#### Scenario: Matrix Parent evidence is required

r[mc_compatibility.roi_01_closeout_hygiene.matrix_parent.scenario]
- GIVEN `Closeout hygiene for drained ROI evidence` is drained
- WHEN the evidence and checks are reviewed
- THEN `matrix_parent` is satisfied by a tracked artifact or deterministic check
- AND scoped non-claims are preserved where relevant
