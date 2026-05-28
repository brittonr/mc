# Tasks

- [ ] [serial] Add production/network safety matrix with separate owned-local, public, WAN, and adversarial rows. r[mc_compatibility.production_network_safety.matrix]
- [ ] [serial] Add checker failures for missing authorization, target ownership, bounds, telemetry, or non-claims. r[mc_compatibility.production_network_safety.gate]
- [ ] [depends:matrix] Add bounded owned-local load receipt requirements and negative tests for over-limit configs. r[mc_compatibility.production_network_safety.owned_local]
- [ ] [depends:matrix] Add WAN/jitter/loss receipt requirements that fail closed when perturbation tooling is unavailable. r[mc_compatibility.production_network_safety.wan]
- [ ] [depends:matrix] Add adversarial-network safety requirements and require explicit oracle/human approval before promotion. r[mc_compatibility.production_network_safety.adversarial]
- [ ] [depends:gate] Update current evidence docs so existing loopback receipts remain non-production evidence. r[mc_compatibility.production_network_safety.nonclaims]
