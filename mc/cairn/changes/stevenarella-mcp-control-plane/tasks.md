# Tasks

- [ ] [serial] Define pure Stevenarella control command, response, error, key, and mouse-button types with positive and negative parser fixtures. r[mc_compatibility.stevenarella_mcp_control.contract]
- [ ] [depends:contract] Add native-only MCP transport wiring for stdio and loopback TCP, with stdout-clean stdio mode and token-gated non-loopback rejection. r[mc_compatibility.stevenarella_mcp_control.transport]
- [ ] [depends:transport] Add a main-thread command queue and drain commands at a deterministic frame boundary without mutating `Game`, `Server`, winit, or GL from MCP worker threads. r[mc_compatibility.stevenarella_mcp_control.main_thread_queue]
- [ ] [depends:main_thread_queue] Implement status, connect, disconnect, key, look, mouse, use-item, attack, and chat tools by reusing existing Stevenarella internal methods. r[mc_compatibility.stevenarella_mcp_control.tools]
- [ ] [depends:tools] Add focused positive and negative tests for invalid key/button names, disconnected operations, stdout-clean stdio mode, non-loopback bind rejection, and fake queued-command drain behavior. r[mc_compatibility.stevenarella_mcp_control.validation]
- [ ] [depends:validation] Record reviewable command/test/Cairn validation output under `docs/evidence/` with BLAKE3 manifests before archiving. r[mc_compatibility.stevenarella_mcp_control.artifacts]
