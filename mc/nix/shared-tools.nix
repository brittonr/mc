{ pkgs, lib }:
let
  cmakePolicyVersionMinimum = "3.5";
  softwareGlEnabled = "1";
  xvfbAutoEnabled = "1";

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

  editableCargoTools = with pkgs; [
    cargo
    rustc
    gcc
    gnumake
    pkg-config
    cmake
    git
    coreutils
    xvfb-run
    xauth
  ];

  pkgConfigPath = lib.makeSearchPathOutput "dev" "lib/pkgconfig" [
    pkgs.openssl
    pkgs.fontconfig
    pkgs.freetype
    pkgs.expat
  ];
  runtimePath = lib.makeBinPath nativeTools;
  libraryPath = lib.makeLibraryPath guiLibs;
in
{
  inherit
    cmakePolicyVersionMinimum
    softwareGlEnabled
    xvfbAutoEnabled
    nativeTools
    guiLibs
    editableCargoTools
    pkgConfigPath
    runtimePath
    libraryPath
    ;
}
