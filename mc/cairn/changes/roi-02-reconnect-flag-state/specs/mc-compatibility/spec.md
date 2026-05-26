# Delta: Reconnect while holding or touching flag compatibility rail

## Requirements

### Requirement: Continuous Server

r[mc_compatibility.reconnect_flag_state.continuous_server] The system MUST run the reconnect flag-state scenario against one continuous owned Valence CTF server.

#### Scenario: Continuous Server evidence is required

r[mc_compatibility.reconnect_flag_state.continuous_server.scenario]
- GIVEN the `Reconnect while holding or touching flag compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.reconnect_flag_state.continuous_server`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Flag State Correlation

r[mc_compatibility.reconnect_flag_state.flag_state_correlation] The system MUST correlate pre-disconnect flag pickup or touch state with post-reconnect server/client state and reject stale owner or phantom-score evidence.

#### Scenario: Flag State Correlation evidence is required

r[mc_compatibility.reconnect_flag_state.flag_state_correlation.scenario]
- GIVEN the `Reconnect while holding or touching flag compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.reconnect_flag_state.flag_state_correlation`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Client Reconnect Evidence

r[mc_compatibility.reconnect_flag_state.client_reconnect_evidence] The system MUST record both first-session and second-session Stevenarella login/join/render/team milestones plus the relevant flag-state observations.

#### Scenario: Client Reconnect Evidence evidence is required

r[mc_compatibility.reconnect_flag_state.client_reconnect_evidence.scenario]
- GIVEN the `Reconnect while holding or touching flag compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.reconnect_flag_state.client_reconnect_evidence`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Receipt Gate

r[mc_compatibility.reconnect_flag_state.receipt_gate] The system MUST provide a maintained dry-run gate and live receipt with explicit non-claims for full CTF compatibility and unbounded reconnect safety.

#### Scenario: Receipt Gate evidence is required

r[mc_compatibility.reconnect_flag_state.receipt_gate.scenario]
- GIVEN the `Reconnect while holding or touching flag compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.reconnect_flag_state.receipt_gate`
- AND the receipt or documentation states scoped non-claims where the proof is bounded
