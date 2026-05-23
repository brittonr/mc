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
          }/bin/mc-compat-runner --dry-run --server-backend paper --client-dir "$PWD/fake-stevenarella" > dry-run.log
          grep -Fq "start Paper server" dry-run.log
          grep -Fq "would run Rust protocol status probe" dry-run.log
          grep -Fq "would run Stevenarella under xvfb-run" dry-run.log
          mkdir -p "$out"
          cp dry-run.log "$out/"
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
          grep -Fq -- "--client-dir PATH" help.log
          grep -Fq "CLIENT_DIR" help.log
          grep -Fq -- "--valence-repo PATH" help.log
          grep -Fq "no inherited Wayland socket" help.log
          mkdir -p "$out"
          cp help.log "$out/"
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
          ];
          shellHook = ''
            echo "mc compat shell: use 'mc-compat-runner --dry-run' or 'nix run .#mc-compat-smoke -- --run'"
            echo "OnixResearch tools are pinned over SSH: cairn, cargo-octet"
          '';
        };
      });
    };
}
