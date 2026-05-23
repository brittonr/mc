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
                --set CMAKE_POLICY_VERSION_MINIMUM 3.5
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

      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = [
            self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
            cairn.packages.${pkgs.stdenv.hostPlatform.system}.cairn
            octet.packages.${pkgs.stdenv.hostPlatform.system}.cargo-octet
            pkgs.nickel
          ];
          shellHook = ''
            echo "mc compat shell: use 'mc-compat-runner --dry-run' or 'nix run .#mc-compat-smoke -- --run'"
            echo "OnixResearch tools are pinned over SSH: cairn, cargo-octet"
          '';
        };
      });
    };
}
