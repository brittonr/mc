{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    octet = {
      url = "git+ssh://git@github.com/OnixResearch/octet.git";
      # Don't follow nixpkgs or rust-overlay — the Octet check
      # needs its own pinned nightly toolchain and nixpkgs version.
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, octet, ... }:
  flake-utils.lib.eachSystem
    [ "x86_64-linux" "aarch64-linux" ]
    (system:
    let
      overlays = [ (import rust-overlay)  ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);

      appNativeBuildInputs = with pkgs; [
          pkg-config
      ];
      appBuildInputs = with pkgs; [
          rust rust-analyzer
          udev alsa-lib vulkan-loader wayland
          libx11 libxcursor libxi libxrandr
          libxkbcommon wayland
      ];
    in
    rec
    {
        devShell = pkgs.mkShell {
            nativeBuildInputs = appNativeBuildInputs;
            buildInputs = appBuildInputs;
            shellHook = ''
                export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath appBuildInputs}"
            '';
        };

        checks.parkour-smoke-receipt = pkgs.runCommand "valence-parkour-smoke-receipt-check" {
          nativeBuildInputs = with pkgs; [ bash coreutils git gnugrep iproute2 jq shellcheck ];
        } ''
          cp -R ${./.} source
          chmod -R +w source
          cd source

          shellcheck scripts/smoke-parkour.sh

          receipt="$TMPDIR/parkour-smoke-receipt.json"
          bash scripts/smoke-parkour.sh --dry-run --receipt "$receipt"
          jq -e '
            .schema == "valence.parkour-smoke.receipt.v1"
            and .example == "parkour"
            and .port == 25565
            and .status == "passed"
            and .reason == "dry_run"
            and .dry_run == true
            and .live_smoke == false
            and .claims_client_compat == false
            and .claims_semantic_correctness == false
          ' "$receipt" >/dev/null

          mkdir -p "$out"
          cp "$receipt" "$out/parkour-smoke-receipt.json"
        '';

        checks.octet = octet.lib.mkConsumerCheck {
          inherit system;
          src = let
            # Use a simple filter that keeps Cargo sources + dylint.toml
            isCargoSource = path: type:
              let baseName = builtins.baseNameOf path; in
              type == "directory"
              || baseName == "Cargo.toml"
              || baseName == "Cargo.lock"
              || baseName == "dylint.toml"
              || baseName == "README.md"
              || pkgs.lib.hasSuffix ".rs" baseName
              || pkgs.lib.hasSuffix ".json" baseName
              || pkgs.lib.hasSuffix ".dat" baseName;
          in pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = isCargoSource;
          };
          packages = [ "valence_math" "valence_lang" "valence_ident" "valence_text" "valence_weather" "valence_world_border" "valence_boss_bar" ];
          cargoLock = ./Cargo.lock;
          nativeBuildInputs = with pkgs; [ pkg-config stdenv.cc ];
          buildInputs = with pkgs; [
            udev alsa-lib vulkan-loader wayland
            libx11 libxcursor libxi libxrandr
            libxkbcommon
          ];
        };
    });
}
