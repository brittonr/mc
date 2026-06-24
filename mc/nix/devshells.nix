{
  self,
  pkgs,
  lib,
  cairn,
  octet,
}:
let
  sharedTools = import ./shared-tools.nix { inherit pkgs lib; };
  inherit (sharedTools)
    guiLibs
    pkgConfigPath
    libraryPath
    cmakePolicyVersionMinimum
    softwareGlEnabled
    ;
in
{
  default = pkgs.mkShell {
    packages =
      with pkgs;
      [
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
        clippy
        shellcheck
        nickel
        steel
        git
        coreutils
        xvfb-run
        xauth
        python3
        b3sum
        docker-client
      ]
      ++ guiLibs;

    OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
    PKG_CONFIG_PATH = pkgConfigPath;
    LD_LIBRARY_PATH = libraryPath;
    LIBRARY_PATH = libraryPath;
    RUSTC_WRAPPER = "";
    CMAKE_POLICY_VERSION_MINIMUM = cmakePolicyVersionMinimum;
    WINIT_UNIX_BACKEND = "x11";
    LIBGL_ALWAYS_SOFTWARE = softwareGlEnabled;

    shellHook = ''
      echo "mc compat shell: use 'mc-compat-runner --dry-run' or 'nix run .#mc-compat-smoke -- --run'"
      echo "Stevenarella dev env: cargo/rustc/xvfb-run/OpenSSL/fontconfig/freetype/libxcb paths are available"
      echo "OnixResearch tools are pinned over SSH: cairn, cargo-octet"
      echo "Steel is available for repo scripts, e.g. steel scripts/ralph-drain-cairns.scm --self-test"
    '';
  };
}
