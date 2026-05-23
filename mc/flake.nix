{
  description = "Rust-on-Rust Minecraft compatibility smoke harness for Stevenarella and Valence";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    cairn.url = "git+ssh://git@github.com/onixresearch/cairn.git";
    octet.url = "git+ssh://git@github.com/onixresearch/octet.git";
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
      cairn,
      octet,
    }:
    let
      eachSystem = f: nixpkgs.lib.genAttrs (import systems) (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = eachSystem (
        pkgs:
        let
          lib = pkgs.lib;
          nativeTools = with pkgs; [
            cargo
            rustc
            gcc
            gnumake
            pkg-config
            cmake
            rustfmt
            shellcheck
            nickel
            git
            coreutils
            xvfb-run
            xauth
            python3
            b3sum
            docker-client
          ];
          guiLibs = with pkgs; [
            openssl
            freetype
            fontconfig
            expat
            libxcb
            libx11
            libxkbcommon
            wayland
            libxcursor
            libxi
            libxrandr
            mesa
            libGL
          ];
          pkgConfigPath = lib.makeSearchPathOutput "dev" "lib/pkgconfig" [
            pkgs.openssl
            pkgs.fontconfig
            pkgs.freetype
            pkgs.expat
          ];
          runtimePath = lib.makeBinPath nativeTools;
          libraryPath = lib.makeLibraryPath guiLibs;
          mc-compat-runner = pkgs.rustPlatform.buildRustPackage {
            pname = "mc-compat-runner";
            version = "0.1.0";
            src = ./tools/mc-compat-runner;
            cargoLock.lockFile = ./tools/mc-compat-runner/Cargo.lock;
            nativeBuildInputs = [ pkgs.makeWrapper ];
            postInstall = ''
              wrapProgram "$out/bin/mc-compat-runner" \
                --prefix PATH : ${lib.escapeShellArg runtimePath} \
                --prefix PKG_CONFIG_PATH : ${lib.escapeShellArg pkgConfigPath} \
                --prefix LIBRARY_PATH : ${lib.escapeShellArg libraryPath} \
                --prefix LD_LIBRARY_PATH : ${lib.escapeShellArg libraryPath} \
                --set OPENSSL_INCLUDE_DIR ${lib.escapeShellArg "${pkgs.openssl.dev}/include"} \
                --set OPENSSL_LIB_DIR ${lib.escapeShellArg "${pkgs.openssl.out}/lib"} \
                --set RUSTC_WRAPPER "" \
                --set CMAKE_POLICY_VERSION_MINIMUM 3.5 \
                --set WINIT_UNIX_BACKEND x11 \
                --set LIBGL_ALWAYS_SOFTWARE 1
            '';
            meta = {
              description = "Hardened Stevenarella/Valence compatibility smoke runner";
              mainProgram = "mc-compat-runner";
            };
          };
        in
        {
          inherit mc-compat-runner;
          cairn = cairn.packages.${pkgs.stdenv.hostPlatform.system}.cairn;
          cargo-octet = octet.packages.${pkgs.stdenv.hostPlatform.system}.cargo-octet;
          octet = octet.packages.${pkgs.stdenv.hostPlatform.system}.octet;
          default = mc-compat-runner;
        }
      );

      apps = eachSystem (pkgs: {
        mc-compat-smoke = {
          type = "app";
          program = "${
            self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
          }/bin/mc-compat-runner";
          meta.description = "Run the hardened Stevenarella/Valence compatibility smoke.";
        };
        default = self.apps.${pkgs.stdenv.hostPlatform.system}.mc-compat-smoke;
      });

      checks = eachSystem (pkgs: {
        mc-compat-runner = self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner;
        mc-compat-dry-run = pkgs.runCommand "mc-compat-dry-run" { } ''
          mkdir -p fake-stevenarella
          printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
          ${
            self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
          }/bin/mc-compat-runner --dry-run --server-backend paper --client-dir "$PWD/fake-stevenarella" --receipt smoke-receipt.json > dry-run.log
          grep -Fq "start Paper server" dry-run.log
          grep -Fq "would run Rust protocol status probe" dry-run.log
          grep -Fq "would run Stevenarella under xvfb-run" dry-run.log
          grep -Fq '"schema": "mc.compat.smoke.receipt.v1"' smoke-receipt.json
          grep -Fq '"cairn_contract": "mc.compat.smoke.receipt.v1"' smoke-receipt.json
          grep -Fq '"octet_producer_surface": "tools/mc-compat-runner/src/main.rs"' smoke-receipt.json
          grep -Fq '"claims_correctness": false' smoke-receipt.json
          grep -Fq '"claims_semantic_equivalence": false' smoke-receipt.json
          grep -Fq '"wayland_socket_inherited": false' smoke-receipt.json
          mkdir -p "$out"
          cp dry-run.log smoke-receipt.json "$out/"
        '';
        mc-compat-compare-receipts = pkgs.runCommand "mc-compat-compare-receipts" { } ''
                    write_receipt() {
                      backend="$1"
                      protocol="$2"
                      port="$3"
                      path="$4"
                      cat > "$path" <<EOF
          {
            "schema": "mc.compat.smoke.receipt.v1",
            "status": "pass",
            "mode": "run",
            "dry_run": false,
            "contract": {
              "claims_correctness": false,
              "claims_semantic_equivalence": false
            },
            "server": {
              "backend": "$backend",
              "version": "1.18.2",
              "protocol": $protocol,
              "port": $port
            },
            "client": {
              "headless_isolation": {
                "xvfb": true,
                "x11_backend": true,
                "software_gl": true,
                "wayland_socket_inherited": false
              },
              "classification": "timeout-success-evidence",
              "matched_success_pattern": "Detected server protocol version"
            },
            "error": null
          }
          EOF
                    }
                    write_receipt paper 758 25566 paper.json
                    write_receipt valence 758 25565 valence.json
                    ${
                      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
                    }/bin/mc-compat-runner --compare-receipts paper.json valence.json > compare.log
                    grep -Fq "receipt comparison passed" compare.log

                    write_receipt valence 759 25565 bad-valence.json
                    if ${
                      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
                    }/bin/mc-compat-runner --compare-receipts paper.json bad-valence.json > bad.log 2>&1; then
                      echo "expected mismatched protocol comparison to fail" >&2
                      cat bad.log >&2
                      exit 1
                    fi
                    grep -Fq "receipt protocol mismatch" bad.log
                    mkdir -p "$out"
                    cp paper.json valence.json compare.log bad.log "$out/"
        '';
        mc-compat-run-matrix =
          pkgs.runCommand "mc-compat-run-matrix" { nativeBuildInputs = [ pkgs.git ]; }
            ''
              mkdir -p fake-stevenarella fake-valence matrix
              printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
              git -C fake-valence init
              git -C fake-valence config user.email mc-compat@example.invalid
              git -C fake-valence config user.name mc-compat
              printf '%s\n' fake > fake-valence/README.md
              git -C fake-valence add README.md
              git -C fake-valence commit -m init
              ${
                self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
              }/bin/mc-compat-runner --run-matrix --dry-run --receipt-dir "$PWD/matrix" --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > matrix.log
              grep -Fq "matrix passed" matrix.log
              grep -Fq '"backend": "paper"' matrix/paper.json
              grep -Fq '"port": 25566' matrix/paper.json
              grep -Fq '"backend": "valence"' matrix/valence.json
              grep -Fq '"port": 25565' matrix/valence.json
              grep -Fq '"classification": "dry-run"' matrix/paper.json
              grep -Fq '"classification": "dry-run"' matrix/valence.json
              mkdir -p "$out"
              cp -r matrix matrix.log "$out/"
            '';
        mc-compat-missing-client = pkgs.runCommand "mc-compat-missing-client" { } ''
          if ${
            self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
          }/bin/mc-compat-runner --dry-run --server-backend paper --client-dir "$PWD/no-such-stevenarella" > missing.log 2>&1; then
            echo "expected missing Stevenarella checkout to fail" >&2
            cat missing.log >&2
            exit 1
          fi
          grep -Fq "Stevenarella checkout not found" missing.log
          grep -Fq -- "--client-dir/CLIENT_DIR" missing.log
          mkdir -p "$out"
          cp missing.log "$out/"
        '';
        mc-compat-missing-valence = pkgs.runCommand "mc-compat-missing-valence" { } ''
          mkdir -p fake-stevenarella
          printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
          if ${
            self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
          }/bin/mc-compat-runner --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/no-such-valence" > missing.log 2>&1; then
            echo "expected missing Valence checkout to fail" >&2
            cat missing.log >&2
            exit 1
          fi
          grep -Fq "Valence checkout not found" missing.log
          grep -Fq -- "--valence-repo/VALENCE_REPO" missing.log
          mkdir -p "$out"
          cp missing.log "$out/"
        '';
        mc-compat-help = pkgs.runCommand "mc-compat-help" { } ''
          ${
            self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
          }/bin/mc-compat-runner --help > help.log
          grep -Fq -- "--server-backend valence|paper" help.log
          grep -Fq -- "--config PATH" help.log
          grep -Fq -- "--client-dir PATH" help.log
          grep -Fq -- "--receipt PATH" help.log
          grep -Fq -- "--receipt-dir DIR" help.log
          grep -Fq -- "--run-matrix" help.log
          grep -Fq -- "--status-only" help.log
          grep -Fq -- "--status" help.log
          grep -Fq -- "--cleanup" help.log
          grep -Fq -- "--apply" help.log
          grep -Fq -- "--stop" help.log
          grep -Fq -- "--compare-receipts PAPER_RECEIPT VALENCE_RECEIPT" help.log
          grep -Fq "SMOKE_RECEIPT" help.log
          grep -Fq "SMOKE_RECEIPT_DIR" help.log
          grep -Fq "MC_COMPAT_CONFIG" help.log
          grep -Fq "CLIENT_DIR" help.log
          grep -Fq -- "--valence-repo PATH" help.log
          grep -Fq "no inherited Wayland socket" help.log
          mkdir -p "$out"
          cp help.log "$out/"
        '';
        mc-compat-nickel-config =
          pkgs.runCommand "mc-compat-nickel-config" { nativeBuildInputs = [ pkgs.nickel ]; }
            ''
              nickel typecheck ${./config/mc-compat/default.ncl}
              nickel export ${./config/mc-compat/default.ncl} > exported.json
              cmp exported.json ${./config/mc-compat/generated/default.json}

              mkdir -p fake-stevenarella
              printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
              ${
                self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
              }/bin/mc-compat-runner --config ${./config/mc-compat/generated/default.json} --dry-run --server-backend paper --client-dir "$PWD/fake-stevenarella" > config-dry-run.log
              grep -Fq "start Paper server" config-dry-run.log
              grep -Fq "protocol 758" config-dry-run.log
              mkdir -p "$out"
              cp exported.json config-dry-run.log "$out/"
            '';
        mc-compat-receipt-contract = pkgs.runCommand "mc-compat-receipt-contract" { } ''
          mkdir -p fake-stevenarella
          printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
          ${
            self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
          }/bin/mc-compat-runner --dry-run --server-backend paper --client-dir "$PWD/fake-stevenarella" --receipt smoke-receipt.json > smoke.log
          ${cairn.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn --help > cairn-help.log
          ${
            octet.packages.${pkgs.stdenv.hostPlatform.system}.cargo-octet
          }/bin/cargo-octet fingerprint --check --output-format json ${./tools/mc-compat-runner/src/main.rs} > octet-fingerprint.json
          grep -Fq '"schema": "mc.compat.smoke.receipt.v1"' smoke-receipt.json
          grep -Fq '"cairn_contract": "mc.compat.smoke.receipt.v1"' smoke-receipt.json
          grep -Fq '"octet_producer_surface": "tools/mc-compat-runner/src/main.rs"' smoke-receipt.json
          grep -Fq '"headless_isolation"' smoke-receipt.json
          grep -Fq 'agent-receipt validate' cairn-help.log
          grep -Fq '"schema_version": 1' octet-fingerprint.json
          mkdir -p "$out"
          cp smoke.log smoke-receipt.json cairn-help.log octet-fingerprint.json "$out/"
        '';
        mc-compat-valence-evidence =
          pkgs.runCommand "mc-compat-valence-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/mc-compat-valence-smoke-2026-05-23.receipt.json}
              note=${./docs/evidence/mc-compat-valence-smoke-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.smoke.receipt.v1")
          assert_eq("status", receipt["status"], "pass")
          assert_eq("mode", receipt["mode"], "run")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("server.backend", receipt["server"]["backend"], "valence")
          assert_eq("server.version", receipt["server"]["version"], "1.18.2")
          assert_eq("server.protocol", receipt["server"]["protocol"], 758)
          assert_eq("client.classification", receipt["client"]["classification"], "timeout-success-evidence")
          assert_eq("client.matched_success_pattern", receipt["client"]["matched_success_pattern"], "Detected server protocol version")
          assert_eq("headless.xvfb", receipt["client"]["headless_isolation"]["xvfb"], True)
          assert_eq("headless.x11_backend", receipt["client"]["headless_isolation"]["x11_backend"], True)
          assert_eq("headless.software_gl", receipt["client"]["headless_isolation"]["software_gl"], True)
          assert_eq("headless.wayland_socket_inherited", receipt["client"]["headless_isolation"]["wayland_socket_inherited"], False)
          assert_eq("contract.claims_correctness", receipt["contract"]["claims_correctness"], False)
          assert_eq("contract.claims_semantic_equivalence", receipt["contract"]["claims_semantic_equivalence"], False)

          required_note_fragments = [
              "Stevenarella → Valence",
              "Receipt status: `pass`",
              "Server status probe: protocol `758`",
              "Client classification: `timeout-success-evidence`",
              "Matched success pattern: `Detected server protocol version`",
              "This is a bounded compatibility smoke receipt.",
          ]
          for fragment in required_note_fragments:
              if fragment not in note:
                  raise SystemExit(f"evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        mc-compat-valence-boundary-evidence =
          pkgs.runCommand "mc-compat-valence-boundary-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/mc-compat-valence-boundary-2026-05-23.receipt.json}
              note=${./docs/evidence/mc-compat-valence-boundary-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.valence-boundary.receipt.v1")
          assert_eq("status", receipt["status"], "pass")
          assert_eq("mode", receipt["mode"], "boundary_probe")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.minecraft_version", receipt["valence"]["minecraft_version"], "1.20.1")
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.status_probe.observed_protocol", receipt["valence"]["status_probe"]["observed_protocol"], 763)
          assert_eq("valence.live_parkour_smoke.status", receipt["valence"]["live_parkour_smoke"]["status"], "passed")
          assert_eq("valence.live_parkour_smoke.claims_client_compat", receipt["valence"]["live_parkour_smoke"]["claims_client_compat"], False)
          assert_eq("stevenarella.default_protocol", receipt["stevenarella"]["default_protocol"], 758)
          assert_eq("stevenarella.supports_valence_current_protocol", receipt["stevenarella"]["supports_valence_current_protocol"], False)
          assert_eq("boundary.proven_path_protocol", receipt["boundary"]["proven_path_protocol"], 758)
          assert_eq("boundary.current_valence_protocol", receipt["boundary"]["current_valence_protocol"], 763)
          assert_eq("boundary.protocol_gap", receipt["boundary"]["protocol_gap"], 5)
          assert_eq("boundary.update_stevenarella_required_for_current_valence", receipt["boundary"]["update_stevenarella_required_for_current_valence"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_stevenarella_763_support", receipt["contract"]["claims_stevenarella_763_support"], False)

          required_note_fragments = [
              "Current Valence main is **not** the same compatibility target",
              "Valence advertised protocol: `763`",
              "Stevenarella default/highest supported protocol: `758`",
              "Status probe for expected protocol `758`: failed as expected",
              "Stevenarella is updated to support protocol `763`",
              "Valence is pinned/translated back to protocol `758`",
          ]
          for fragment in required_note_fragments:
              if fragment not in note:
                  raise SystemExit(f"boundary evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-handshake-evidence =
          pkgs.runCommand "stevenarella-valence-763-handshake-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-handshake-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-handshake-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-handshake.receipt.v1")
          assert_eq("status", receipt["status"], "pass")
          assert_eq("mode", receipt["mode"], "current_valence_763_handshake_probe")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.status_probe_passed", receipt["valence"]["status_probe_passed"], True)
          assert_eq("stevenarella.claims_full_1_20_1_protocol_support", receipt["stevenarella"]["claims_full_1_20_1_protocol_support"], False)
          assert_eq("client.exit_code", receipt["client_probe"]["exit_code"], 124)
          assert_eq("client.matched_success_pattern", receipt["client_probe"]["matched_success_pattern"], "Detected server protocol version 763")
          assert_eq("verification.unit_tests_status", receipt["verification"]["unit_tests_status"], "pass")
          assert_eq("contract.claims_current_valence_initial_handshake", receipt["contract"]["claims_current_valence_initial_handshake"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)

          required_note_fragments = [
              "Detected server protocol version 763",
              "not** full 1.20.1 protocol support",
              "Client log BLAKE3",
              "Receipt BLAKE3",
          ]
          for fragment in required_note_fragments:
              if fragment not in note:
                  raise SystemExit(f"handshake evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-packet-boundary-evidence =
          pkgs.runCommand "stevenarella-valence-763-packet-boundary-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-packet-boundary-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-packet-boundary-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-packet-boundary.receipt.v1")
          assert_eq("status", receipt["status"], "pass")
          assert_eq("mode", receipt["mode"], "current_valence_763_offline_login_packet_trace")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.connection_mode", receipt["valence"]["connection_mode"], "Offline")
          assert_eq("first packet", receipt["valence"]["first_captured_packets"][0]["semantic"], "SetCompressionS2C")
          assert_eq("second packet", receipt["valence"]["first_captured_packets"][1]["semantic"], "LoginSuccessS2C")
          assert_eq("boundary wire id", receipt["boundary"]["first_mismatch"]["wire_id"], "0x28")
          assert_eq("boundary valence semantic", receipt["boundary"]["first_mismatch"]["valence_763_semantic"], "GameJoinS2C")
          assert_eq("boundary stevenarella semantic", receipt["boundary"]["first_mismatch"]["stevenarella_758_alias_semantic"], "TradeList_WithRestock")
          assert_eq("trace packet count", receipt["trace"]["packet_count"], 20)
          assert_eq("devshell validated", receipt["devshell_fix"]["validated"], True)
          assert_eq("verification.runner build", receipt["verification"]["mc_compat_runner_build_status"], "pass")
          assert_eq("contract.claims_current_valence_login_packet_boundary", receipt["contract"]["claims_current_valence_login_packet_boundary"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)

          required_note_fragments = [
              "play/clientbound/0x28",
              "GameJoinS2C",
              "TradeList_WithRestock",
              "Devshell/flake repair",
              "Receipt BLAKE3",
          ]
          for fragment in required_note_fragments:
              if fragment not in note:
                  raise SystemExit(f"packet-boundary evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-join-game-patch-evidence =
          pkgs.runCommand "stevenarella-valence-763-join-game-patch-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-join-game-patch-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-join-game-patch-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-join-game-patch.receipt.v1")
          assert_eq("status", receipt["status"], "pass")
          assert_eq("mode", receipt["mode"], "protocol_763_join_game_mapping_patch")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("patched wire id", receipt["boundary"]["patched_boundary"]["wire_id"], "0x28")
          assert_eq("patched valence semantic", receipt["boundary"]["patched_boundary"]["valence_763_semantic"], "GameJoinS2C")
          assert_eq("patched internal", receipt["boundary"]["patched_boundary"]["stevenarella_763_internal"], "JoinGame_WorldNames_IsHard_SimDist")
          assert_eq("patched status", receipt["boundary"]["patched_boundary"]["status"], "mapped")
          assert_eq("next wire id", receipt["boundary"]["next_boundary"]["wire_id"], "0x10")
          assert_eq("next valence semantic", receipt["boundary"]["next_boundary"]["valence_763_semantic"], "CommandTreeS2CPacket")
          assert_eq("next stevenarella semantic", receipt["boundary"]["next_boundary"]["stevenarella_758_alias_semantic"], "ClearTitles")
          assert_eq("tests", receipt["verification"]["steven_protocol_tests"], "pass")
          assert_eq("fmt", receipt["verification"]["cargo_fmt_check"], "pass")
          assert_eq("contract.claims_protocol_763_join_game_mapping", receipt["contract"]["claims_protocol_763_join_game_mapping"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)

          required_note_fragments = [
              "play/clientbound/0x28",
              "JoinGame_WorldNames_IsHard_SimDist",
              "play/clientbound/0x10",
              "CommandTreeS2CPacket",
              "ClearTitles",
              "Receipt BLAKE3",
          ]
          for fragment in required_note_fragments:
              if fragment not in note:
                  raise SystemExit(f"join-game patch evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-command-tree-update-evidence =
          pkgs.runCommand "stevenarella-valence-763-command-tree-update-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-command-tree-update-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-command-tree-update-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-command-tree-update.receipt.v1")
          assert_eq("status", receipt["status"], "pass")
          assert_eq("mode", receipt["mode"], "protocol_763_command_tree_mapping_update")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("updated wire id", receipt["boundary"]["updated_boundary"]["wire_id"], "0x10")
          assert_eq("updated valence semantic", receipt["boundary"]["updated_boundary"]["valence_763_semantic"], "CommandTreeS2CPacket")
          assert_eq("updated internal", receipt["boundary"]["updated_boundary"]["stevenarella_763_internal"], "DeclareCommands")
          assert_eq("previous inherited semantic", receipt["boundary"]["updated_boundary"]["previous_stevenarella_758_alias_semantic"], "ClearTitles")
          assert_eq("updated status", receipt["boundary"]["updated_boundary"]["status"], "mapped")
          assert_eq("prior mapped wire id", receipt["boundary"]["still_mapped_from_prior_update"]["wire_id"], "0x28")
          assert_eq("next wire id", receipt["boundary"]["next_boundary"]["wire_id"], "0x64")
          assert_eq("next valence semantic", receipt["boundary"]["next_boundary"]["valence_763_semantic"], "GameMessageS2CPacket")
          assert_eq("next stevenarella semantic", receipt["boundary"]["next_boundary"]["stevenarella_758_alias_semantic"], "EntityProperties_VarIntVarInt")
          assert_eq("tests", receipt["verification"]["steven_protocol_tests"], "pass")
          assert_eq("fmt", receipt["verification"]["cargo_fmt_check"], "pass")
          assert_eq("contract.claims_protocol_763_command_tree_mapping", receipt["contract"]["claims_protocol_763_command_tree_mapping"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)

          required_note_fragments = [
              "play/clientbound/0x10",
              "CommandTreeS2CPacket",
              "DeclareCommands",
              "play/clientbound/0x64",
              "EntityProperties_VarIntVarInt",
              "Receipt BLAKE3",
          ]
          for fragment in required_note_fragments:
              if fragment not in note:
                  raise SystemExit(f"command-tree update evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-game-message-update-evidence =
          pkgs.runCommand "stevenarella-valence-763-game-message-update-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-game-message-update-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-game-message-update-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-game-message-update.receipt.v1")
          assert_eq("status", receipt["status"], "pass")
          assert_eq("mode", receipt["mode"], "protocol_763_game_message_mapping_update")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("updated wire id", receipt["boundary"]["updated_boundary"]["wire_id"], "0x64")
          assert_eq("updated valence semantic", receipt["boundary"]["updated_boundary"]["valence_763_semantic"], "GameMessageS2CPacket")
          assert_eq("updated internal", receipt["boundary"]["updated_boundary"]["stevenarella_763_internal"], "ServerMessage_Position")
          assert_eq("previous inherited semantic", receipt["boundary"]["updated_boundary"]["previous_stevenarella_758_alias_semantic"], "EntityProperties_VarIntVarInt")
          assert_eq("updated status", receipt["boundary"]["updated_boundary"]["status"], "mapped")
          assert_eq("prior first mapped wire id", receipt["boundary"]["still_mapped_from_prior_updates"][0]["wire_id"], "0x28")
          assert_eq("prior second mapped wire id", receipt["boundary"]["still_mapped_from_prior_updates"][1]["wire_id"], "0x10")
          assert_eq("next wire id", receipt["boundary"]["next_boundary"]["wire_id"], "0x69")
          assert_eq("next valence semantic", receipt["boundary"]["next_boundary"]["valence_763_semantic"], "AdvancementUpdateS2CPacket")
          assert_eq("next stevenarella semantic", receipt["boundary"]["next_boundary"]["stevenarella_758_alias_semantic"], None)
          assert_eq("next status", receipt["boundary"]["next_boundary"]["status"], "unmapped_by_758_fallback")
          assert_eq("tests", receipt["verification"]["steven_protocol_tests"], "pass")
          assert_eq("fmt", receipt["verification"]["cargo_fmt_check"], "pass")
          assert_eq("contract.claims_protocol_763_game_message_mapping", receipt["contract"]["claims_protocol_763_game_message_mapping"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
          assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)

          required_note_fragments = [
              "play/clientbound/0x64",
              "GameMessageS2CPacket",
              "ServerMessage_Position",
              "EntityProperties_VarIntVarInt",
              "play/clientbound/0x69",
              "AdvancementUpdateS2CPacket",
              "Receipt BLAKE3",
          ]
          for fragment in required_note_fragments:
              if fragment not in note:
                  raise SystemExit(f"game-message update evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-observed-boundaries-drain-evidence =
          pkgs.runCommand "stevenarella-valence-763-observed-boundaries-drain-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-observed-boundaries-drain-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-observed-boundaries-drain-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-observed-boundaries-drain.receipt.v1")
          assert_eq("status", receipt["status"], "pass")
          assert_eq("mode", receipt["mode"], "protocol_763_observed_boundaries_mapping_drain")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "4c5e89d")
          assert_eq("boundary count", len(receipt["boundaries"]), 14)
          assert_eq("prior boundary count", len(receipt["prior_mapped_boundaries"]), 3)
          assert_eq("trace first unmapped", receipt["verification"]["trace_first_unmapped"], None)
          assert_eq("fmt", receipt["verification"]["cargo_fmt_check"], "pass")
          assert_eq("tests", receipt["verification"]["steven_protocol_tests"], "pass")
          assert_eq("trace", receipt["verification"]["valence_ctf_trace"], "pass")
          assert_eq("headless", receipt["verification"]["headless_probe"], "pass")
          assert_eq("contract.claims_observed_valence_ctf_boundaries_mapped", receipt["contract"]["claims_observed_valence_ctf_boundaries_mapped"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
          assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
          assert_eq("contract.claims_all_minecraft_1_20_1_packets", receipt["contract"]["claims_all_minecraft_1_20_1_packets"], False)

          required = {
              "0x69": "Advancements",
              "0x58": "ScoreboardObjective",
              "0x51": "ScoreboardDisplay",
              "0x5b": "UpdateScore_VarInt",
              "0x4d": "SetCurrentHotbarSlot",
              "0x14": "WindowSetSlot_State",
              "0x3a": "PlayerInfo",
              "0x57": "UpdateHealth",
              "0x52": "EntityMetadata",
              "0x6a": "EntityProperties_VarIntVarInt",
              "0x1c": "EntityStatus",
              "0x34": "PlayerAbilities",
              "0x6e": "Tags_Nested",
              "0x24": "ChunkData_AndLight",
          }
          actual = {item["wire_id"]: item["stevenarella_763_internal"] for item in receipt["boundaries"]}
          assert_eq("boundary map", actual, required)

          for fragment in [
              "play/clientbound/0x69",
              "AdvancementUpdateS2CPacket",
              "ChunkData_AndLight",
              "Receipt BLAKE3",
              "Does not prove every protocol 763 packet is mapped",
          ]:
              if fragment not in note:
                  raise SystemExit(f"drain evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-gameplay-smoke-evidence =
          pkgs.runCommand "stevenarella-valence-763-gameplay-smoke-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-gameplay-smoke-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-gameplay-smoke-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-gameplay-smoke.receipt.v1")
          assert_eq("status", receipt["status"], "pass")
          assert_eq("mode", receipt["mode"], "protocol_763_bounded_stevenarella_valence_ctf_runtime_smoke")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "4c5e89d")
          assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
          assert_eq("debug status", receipt["artifacts"]["debug_status"]["content"], "exit=124")
          assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
          assert_eq("debug protocol", receipt["artifacts"]["debug_log"]["Detected server protocol version 763"], 1)
          assert_eq("probe panic count", receipt["artifacts"]["probe_log"]["panicked"], 0)
          assert_eq("debug panic count", receipt["artifacts"]["debug_log"]["panicked"], 0)
          assert_eq("probe unmapped count", receipt["artifacts"]["probe_log"]["unmapped"], 0)
          assert_eq("debug unmapped count", receipt["artifacts"]["debug_log"]["unmapped"], 0)
          assert_eq("first semantic failure", receipt["verification"]["first_semantic_runtime_failure"], None)
          assert_eq("bounded smoke claim", receipt["contract"]["claims_bounded_runtime_smoke_survived_after_protocol_detection"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
          assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
          assert_eq("contract.claims_in_world_gameplay_success", receipt["contract"]["claims_in_world_gameplay_success"], False)

          for fragment in [
              "Detected server protocol version 763",
              "bounded runtime smoke",
              "exit=124",
              "Does not prove semantic packet parser correctness",
              "Does not prove in-world gameplay success",
              "Receipt BLAKE3",
          ]:
              if fragment not in note:
                  raise SystemExit(f"gameplay smoke evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-instrumented-login-milestones-evidence =
          pkgs.runCommand "stevenarella-valence-763-instrumented-login-milestones-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-instrumented-login-milestones-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-instrumented-login-milestones-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-instrumented-login-milestones.receipt.v1")
          assert_eq("status", receipt["status"], "fail_at_first_play_read_after_login_success")
          assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_login_milestones")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "10e4562")
          assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=101")
          assert_eq("backtrace status", receipt["artifacts"]["backtrace_status"]["content"], "exit=101")
          assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
          assert_eq("login compression", receipt["artifacts"]["probe_log"]["login_compression"], 1)
          assert_eq("login success", receipt["artifacts"]["probe_log"]["login_success"], 1)
          assert_eq("join game", receipt["artifacts"]["probe_log"]["join_game"], 0)
          assert_eq("first chunk", receipt["artifacts"]["probe_log"]["first_chunk_data"], 0)
          assert_eq("render tick", receipt["artifacts"]["probe_log"]["render_tick_with_player"], 0)
          assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 1)
          assert_eq("first failure", receipt["verification"]["first_observed_runtime_failure"], "UnexpectedEof while reading first play-state packet after login_success")
          assert_eq("login success claim", receipt["contract"]["claims_login_success_reached"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
          assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
          assert_eq("contract.claims_in_world_gameplay_success", receipt["contract"]["claims_in_world_gameplay_success"], False)

          for fragment in [
              "MC-COMPAT-MILESTONE login_success",
              "UnexpectedEof while reading the first play-state packet",
              "Does not prove in-world gameplay success",
              "Receipt BLAKE3",
          ]:
              if fragment not in note:
                  raise SystemExit(f"instrumented login evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-post-login-play-state-evidence =
          pkgs.runCommand "stevenarella-valence-763-post-login-play-state-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-post-login-play-state-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-post-login-play-state-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-post-login-play-state.receipt.v1")
          assert_eq("status", receipt["status"], "advanced_past_login_success_to_join_game_render_tick_next_failure_entity_metadata_parser_panic")
          assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_post_login_play_state")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "51d22f2")
          assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
          assert_eq("backtrace status", receipt["artifacts"]["backtrace_status"]["content"], "exit=124")
          assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
          assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
          assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
          assert_eq("join game", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game"], 2)
          assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
          assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
          assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked"], 1)
          assert_eq("first failure", receipt["verification"]["first_observed_runtime_failure"], "panic in EntityMetadata metadata string parser: FromUtf8Error at protocol/src/protocol/mod.rs:281")
          assert_eq("join game claim", receipt["contract"]["claims_join_game_reached"], True)
          assert_eq("render tick claim", receipt["contract"]["claims_render_tick_with_player_reached"], True)
          assert_eq("unexpected eof claim", receipt["contract"]["claims_unexpected_eof_fixed_for_this_probe"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
          assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
          assert_eq("contract.claims_chunk_streaming_success", receipt["contract"]["claims_chunk_streaming_success"], False)

          for fragment in [
              "MC-COMPAT-MILESTONE join_game_763_shape",
              "MC-COMPAT-MILESTONE render_tick_with_player",
              "UnexpectedEof",
              "EntityMetadata metadata parser panic",
              "Does not prove full Minecraft 1.20.1 compatibility",
              "Receipt BLAKE3",
          ]:
              if fragment not in note:
                  raise SystemExit(f"post-login evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-entity-metadata-evidence =
          pkgs.runCommand "stevenarella-valence-763-entity-metadata-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-entity-metadata-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-entity-metadata-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-entity-metadata.receipt.v1")
          assert_eq("status", receipt["status"], "bounded_probe_timeout_after_entity_metadata_parser_fix")
          assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_entity_metadata")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "b2a6358")
          assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
          assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
          assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
          assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
          assert_eq("first chunk", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE first_chunk_data"], 1)
          assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
          assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
          assert_eq("from utf8", receipt["artifacts"]["probe_log"]["FromUtf8Error"], 0)
          assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked at"], 0)
          assert_eq("parse failure", receipt["artifacts"]["probe_log"]["failed to parse packet"], 0)
          assert_eq("entity metadata claim", receipt["contract"]["claims_entity_metadata_boundary_cleared_for_this_probe"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
          assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
          assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)

          for fragment in [
              "EntityMetadata",
              "MC-COMPAT-MILESTONE first_chunk_data",
              "MC-COMPAT-MILESTONE render_tick_with_player",
              "FromUtf8Error",
              "Does not prove full Minecraft 1.20.1 compatibility",
              "Receipt BLAKE3",
          ]:
              if fragment not in note:
                  raise SystemExit(f"entity metadata evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-extended-gameplay-evidence =
          pkgs.runCommand "stevenarella-valence-763-extended-gameplay-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-extended-gameplay-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-extended-gameplay-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-extended-gameplay.receipt.v1")
          assert_eq("status", receipt["status"], "bounded_300s_timeout_no_logged_runtime_failure")
          assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_extended_gameplay")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "b2a6358")
          assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
          assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
          assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
          assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
          assert_eq("first chunk", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE first_chunk_data"], 1)
          assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
          assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
          assert_eq("from utf8", receipt["artifacts"]["probe_log"]["FromUtf8Error"], 0)
          assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked at"], 0)
          assert_eq("parse failure", receipt["artifacts"]["probe_log"]["failed to parse packet"], 0)
          assert_eq("disconnect", receipt["artifacts"]["probe_log"]["Disconnect"], 0)
          assert_eq("no runtime failure claim", receipt["contract"]["claims_no_logged_runtime_failure_in_this_probe"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
          assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
          assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)

          for fragment in [
              "bounded 300-second Valence `ctf` probe",
              "MC-COMPAT-MILESTONE first_chunk_data",
              "MC-COMPAT-MILESTONE render_tick_with_player",
              "No next concrete runtime boundary appeared",
              "Does not prove full Minecraft 1.20.1 compatibility",
              "Receipt BLAKE3",
          ]:
              if fragment not in note:
                  raise SystemExit(f"extended gameplay evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-active-gameplay-evidence =
          pkgs.runCommand "stevenarella-valence-763-active-gameplay-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-active-gameplay-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-active-gameplay-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-active-gameplay.receipt.v1")
          assert_eq("status", receipt["status"], "bounded_180s_timeout_active_movement_no_logged_runtime_failure")
          assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_active_gameplay")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "05a382b")
          assert_eq("active probe env", receipt["stevenarella"]["active_probe_env"], "MC_COMPAT_ACTIVE_PROBE=1")
          assert_eq("probe duration", receipt["probe"]["duration_seconds"], 180)
          assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
          assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
          assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
          assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
          assert_eq("first chunk", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE first_chunk_data"], 1)
          assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
          assert_eq("active start", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_input_start"], 1)
          assert_eq("active jump release", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_jump_release"], 1)
          assert_eq("active turn", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_input_turn"], 1)
          assert_eq("active stop", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_input_stop"], 1)
          assert_eq("position look sent", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_position_look_sent"], 1)
          assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
          assert_eq("from utf8", receipt["artifacts"]["probe_log"]["FromUtf8Error"], 0)
          assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked at"], 0)
          assert_eq("parse failure", receipt["artifacts"]["probe_log"]["failed to parse packet"], 0)
          assert_eq("disconnect", receipt["artifacts"]["probe_log"]["Disconnect"], 0)
          assert_eq("no runtime failure claim", receipt["contract"]["claims_no_logged_runtime_failure_in_this_probe"], True)
          assert_eq("movement packet claim", receipt["contract"]["claims_active_movement_packet_sent"], True)
          assert_eq("contract.claims_team_selection", receipt["contract"]["claims_team_selection"], False)
          assert_eq("contract.claims_combat_semantics", receipt["contract"]["claims_combat_semantics"], False)
          assert_eq("contract.claims_inventory_semantics", receipt["contract"]["claims_inventory_semantics"], False)
          assert_eq("contract.claims_death_respawn_semantics", receipt["contract"]["claims_death_respawn_semantics"], False)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
          assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
          assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)

          for fragment in [
              "bounded 180-second Valence `ctf` probe",
              "MC_COMPAT_ACTIVE_PROBE=1",
              "MC-COMPAT-MILESTONE active_probe_position_look_sent",
              "No `UnexpectedEof`, `FromUtf8Error`, parser panic",
              "does not prove full Minecraft 1.20.1 compatibility",
              "Receipt BLAKE3",
          ]:
              if fragment not in note:
                  raise SystemExit(f"active gameplay evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-team-interaction-evidence =
          pkgs.runCommand "stevenarella-valence-763-team-interaction-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-team-interaction-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-team-interaction-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-team-interaction.receipt.v1")
          assert_eq("status", receipt["status"], "bounded_180s_timeout_team_interaction_probe_no_logged_runtime_failure")
          assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_team_interaction")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "ca62c2c")
          assert_eq("active probe env", receipt["stevenarella"]["active_probe_env"], "MC_COMPAT_ACTIVE_PROBE=1")
          assert_eq("team probe env", receipt["stevenarella"]["team_probe_env"], "MC_COMPAT_TEAM_PROBE=1")
          assert_eq("probe duration", receipt["probe"]["duration_seconds"], 180)
          assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
          assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
          assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
          assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
          assert_eq("first chunk", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE first_chunk_data"], 1)
          assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
          assert_eq("position look sent", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_position_look_sent"], 1)
          assert_eq("team enter", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_enter_red_portal"], 1)
          assert_eq("team hold", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_hold_red_portal"], 1)
          assert_eq("hotbar", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_select_hotbar_slot"], 1)
          assert_eq("use item", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_use_item_sent"], 1)
          assert_eq("team chat not observed", receipt["artifacts"]["probe_log"]["You are on team"], 0)
          assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
          assert_eq("from utf8", receipt["artifacts"]["probe_log"]["FromUtf8Error"], 0)
          assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked at"], 0)
          assert_eq("parse failure", receipt["artifacts"]["probe_log"]["failed to parse packet"], 0)
          assert_eq("disconnect", receipt["artifacts"]["probe_log"]["Disconnect"], 0)
          assert_eq("no runtime failure claim", receipt["contract"]["claims_no_logged_runtime_failure_in_this_probe"], True)
          assert_eq("team packet claim", receipt["contract"]["claims_team_probe_packets_sent"], True)
          assert_eq("contract.claims_team_selection", receipt["contract"]["claims_team_selection"], False)
          assert_eq("contract.claims_team_selection_chat_observed", receipt["contract"]["claims_team_selection_chat_observed"], False)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
          assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
          assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)

          for fragment in [
              "bounded 180-second Valence `ctf` probe",
              "MC_COMPAT_TEAM_PROBE=1",
              "MC-COMPAT-MILESTONE team_probe_use_item_sent",
              "`You are on team`: `0`",
              "does not prove team selection semantics",
              "Receipt BLAKE3",
          ]:
              if fragment not in note:
                  raise SystemExit(f"team interaction evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-team-selection-evidence =
          pkgs.runCommand "stevenarella-valence-763-team-selection-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-team-selection-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-team-selection-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-team-selection.receipt.v1")
          assert_eq("status", receipt["status"], "bounded_180s_timeout_team_selection_chat_observed_no_logged_runtime_failure")
          assert_eq("mode", receipt["mode"], "protocol_763_stevenarella_valence_ctf_team_selection_semantic_probe")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("valence.committed_changes", receipt["valence"]["committed_changes"], False)
          assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "4c891eb")
          assert_eq("active probe env", receipt["stevenarella"]["active_probe_env"], "MC_COMPAT_ACTIVE_PROBE=1")
          assert_eq("team probe env", receipt["stevenarella"]["team_probe_env"], "MC_COMPAT_TEAM_PROBE=1")
          assert_eq("probe duration", receipt["probe"]["duration_seconds"], 180)
          assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
          assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
          assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
          assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
          assert_eq("first chunk", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE first_chunk_data"], 1)
          assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
          assert_eq("position look sent", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_position_look_sent"], 1)
          assert_eq("team enter", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_enter_red_portal"], 1)
          assert_eq("team hold", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_hold_red_portal"], 1)
          assert_eq("hotbar", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_select_hotbar_slot"], 1)
          assert_eq("use item", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_use_item_sent"], 1)
          assert_eq("team chat observed", receipt["artifacts"]["probe_log"]["You are on team"], 1)
          assert_eq("red team chat observed", receipt["artifacts"]["probe_log"]["You are on team RED"], 1)
          assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
          assert_eq("from utf8", receipt["artifacts"]["probe_log"]["FromUtf8Error"], 0)
          assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked at"], 0)
          assert_eq("parse failure", receipt["artifacts"]["probe_log"]["failed to parse packet"], 0)
          assert_eq("short read", receipt["artifacts"]["probe_log"]["Failed to read all of packet"], 0)
          assert_eq("bad packet id", receipt["artifacts"]["probe_log"]["bad packet id"], 0)
          assert_eq("disconnect", receipt["artifacts"]["probe_log"]["Disconnect"], 0)
          assert_eq("contract.claims_team_selection_chat_observed", receipt["contract"]["claims_team_selection_chat_observed"], True)
          assert_eq("no runtime failure claim", receipt["contract"]["claims_no_logged_runtime_failure_in_this_probe"], True)
          assert_eq("team packet claim", receipt["contract"]["claims_team_probe_packets_sent"], True)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
          assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)

          for fragment in [
              "bounded 180-second headless Stevenarella probe",
              "MC_COMPAT_TEAM_PROBE=1",
              "You are on team RED!",
              "does **not** prove full Minecraft 1.20.1 compatibility",
              "Receipt BLAKE3",
          ]:
              if fragment not in note:
                  raise SystemExit(f"team selection evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        stevenarella-valence-763-combat-death-evidence =
          pkgs.runCommand "stevenarella-valence-763-combat-death-evidence" { nativeBuildInputs = [ pkgs.b3sum pkgs.python3 ]; }
            ''
              receipt=${./docs/evidence/stevenarella-valence-763-combat-death-2026-05-23.receipt.json}
              note=${./docs/evidence/stevenarella-valence-763-combat-death-2026-05-23.md}

              python3 - "$receipt" "$note" <<'PY'
          import json
          import pathlib
          import sys

          receipt_path = pathlib.Path(sys.argv[1])
          note_path = pathlib.Path(sys.argv[2])
          receipt = json.loads(receipt_path.read_text())
          note = note_path.read_text()

          def assert_eq(name, actual, expected):
              if actual != expected:
                  raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

          assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-combat-death.receipt.v1")
          assert_eq("status", receipt["status"], "bounded_180s_two_client_combat_probe_damage_and_death_health_observed_no_logged_runtime_failure")
          assert_eq("mode", receipt["mode"], "protocol_763_stevenarella_valence_ctf_combat_death_semantic_probe")
          assert_eq("dry_run", receipt["dry_run"], False)
          assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
          assert_eq("valence.example", receipt["valence"]["example"], "ctf")
          assert_eq("valence.committed_changes", receipt["valence"]["committed_changes"], False)
          assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "2804c81")
          assert_eq("active probe env", receipt["stevenarella"]["active_probe_env"], "MC_COMPAT_ACTIVE_PROBE=1")
          assert_eq("team probe env", receipt["stevenarella"]["team_probe_env"], "MC_COMPAT_TEAM_PROBE=1")
          assert_eq("combat probe env", receipt["stevenarella"]["combat_probe_env"], "MC_COMPAT_COMBAT_PROBE=1")
          assert_eq("probe duration", receipt["probe"]["duration_seconds"], 180)
          assert_eq("red probe status", receipt["artifacts"]["red_probe_status"]["content"], "exit=124")
          assert_eq("blue probe status", receipt["artifacts"]["blue_probe_status"]["content"], "exit=124")
          combined = receipt["artifacts"]["combined_probe_log"]
          assert_eq("protocol detections", combined["Detected server protocol version 763"], 2)
          assert_eq("login successes", combined["MC-COMPAT-MILESTONE login_success"], 2)
          assert_eq("join game shapes", combined["MC-COMPAT-MILESTONE join_game_763_shape"], 2)
          assert_eq("first chunks", combined["MC-COMPAT-MILESTONE first_chunk_data"], 2)
          assert_eq("render ticks", combined["MC-COMPAT-MILESTONE render_tick_with_player"], 2)
          assert_eq("red team chat observed", combined["You are on team RED"], 1)
          assert_eq("blue team chat observed", combined["You are on team BLUE"], 1)
          assert_eq("combat move", combined["MC-COMPAT-MILESTONE combat_probe_move_near_blue_spawn"], 1)
          assert_eq("attacks", combined["MC-COMPAT-MILESTONE combat_probe_attack_sent"], 6)
          assert_eq("health 16", combined["update_health health=16.0"], 1)
          assert_eq("health 12", combined["update_health health=12.0"], 1)
          assert_eq("health 8", combined["update_health health=8.0"], 1)
          assert_eq("health 4", combined["update_health health=4.0"], 1)
          assert_eq("health 0", combined["update_health health=0.0"], 1)
          assert_eq("death observed", combined["combat_probe_death_observed"], 2)
          assert_eq("death message observed", combined["combat_probe_death_message"], 0)
          assert_eq("unexpected eof", combined["UnexpectedEof"], 0)
          assert_eq("from utf8", combined["FromUtf8Error"], 0)
          assert_eq("panic count", combined["panicked at"], 0)
          assert_eq("parse failure", combined["failed to parse packet"], 0)
          assert_eq("short read", combined["Failed to read all of packet"], 0)
          assert_eq("bad packet id", combined["bad packet id"], 0)
          assert_eq("disconnect", combined["Disconnect"], 0)
          assert_eq("claims both teams", receipt["contract"]["claims_both_teams_selected"], True)
          assert_eq("claims attacks", receipt["contract"]["claims_attack_packets_sent"], True)
          assert_eq("claims health decreased", receipt["contract"]["claims_victim_health_decreased"], True)
          assert_eq("claims death health", receipt["contract"]["claims_victim_death_health_observed"], True)
          assert_eq("no death message claim", receipt["contract"]["claims_death_message_observed"], False)
          assert_eq("no respawn claim", receipt["contract"]["claims_respawn_observed"], False)
          assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
          assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
          assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)
          assert_eq("contract.claims_full_combat_correctness", receipt["contract"]["claims_full_combat_correctness"], False)

          for fragment in [
              "Bounded two-client headless Stevenarella probe",
              "MC_COMPAT_COMBAT_PROBE=1",
              "You are on team BLUE",
              "You are on team RED",
              "update_health health=0.0",
              "does **not** prove full Minecraft 1.20.1 compatibility",
              "Receipt BLAKE3",
          ]:
              if fragment not in note:
                  raise SystemExit(f"combat/death evidence note missing fragment: {fragment}")
          PY

              b3=$(b3sum "$receipt" | cut -d' ' -f1)
              grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
              mkdir -p "$out"
              cp "$receipt" "$note" "$out/"
              printf '%s\n' "$b3" > "$out/receipt.b3"
            '';
        onixresearch-ssh-tools = pkgs.runCommand "onixresearch-ssh-tools" { } ''
          ${cairn.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn --help > cairn-help.log
          ${
            octet.packages.${pkgs.stdenv.hostPlatform.system}.cargo-octet
          }/bin/cargo-octet --help > cargo-octet-help.log
          grep -Fq "usage: cairn" cairn-help.log
          grep -Fq "Octet operator commands" cargo-octet-help.log
          mkdir -p "$out"
          cp cairn-help.log cargo-octet-help.log "$out/"
        '';
      });

      devShells = eachSystem (
        pkgs:
        let
          lib = pkgs.lib;
          guiLibs = with pkgs; [
            openssl
            freetype
            fontconfig
            expat
            libxcb
            libx11
            libxkbcommon
            wayland
            libxcursor
            libxi
            libxrandr
            mesa
            libGL
          ];
          pkgConfigPath = lib.makeSearchPathOutput "dev" "lib/pkgconfig" [
            pkgs.openssl
            pkgs.fontconfig
            pkgs.freetype
            pkgs.expat
          ];
          libraryPath = lib.makeLibraryPath guiLibs;
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              self.packages.${stdenv.hostPlatform.system}.mc-compat-runner
              cairn.packages.${stdenv.hostPlatform.system}.cairn
              octet.packages.${stdenv.hostPlatform.system}.cargo-octet
              cargo
              rustc
              gcc
              gnumake
              pkg-config
              cmake
              mold
              rustfmt
              shellcheck
              nickel
              git
              coreutils
              xvfb-run
              xauth
              python3
              b3sum
              docker-client
            ] ++ guiLibs;

            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            PKG_CONFIG_PATH = pkgConfigPath;
            LD_LIBRARY_PATH = libraryPath;
            LIBRARY_PATH = libraryPath;
            RUSTC_WRAPPER = "";
            CMAKE_POLICY_VERSION_MINIMUM = "3.5";
            WINIT_UNIX_BACKEND = "x11";
            LIBGL_ALWAYS_SOFTWARE = "1";

            shellHook = ''
              echo "mc compat shell: use 'mc-compat-runner --dry-run' or 'nix run .#mc-compat-smoke -- --run'"
              echo "Stevenarella dev env: cargo/rustc/xvfb-run/OpenSSL/fontconfig/freetype/libxcb paths are available"
              echo "OnixResearch tools are pinned over SSH: cairn, cargo-octet"
            '';
          };
        }
      );
    };
}
