{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    octet = {
      url = "git+file:///home/brittonr/git/octet";
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
              || pkgs.lib.hasSuffix ".json" baseName;
          in pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = isCargoSource;
          };
          packages = [ "valence_math" "valence_lang" "valence_ident" "valence_text" ];
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
