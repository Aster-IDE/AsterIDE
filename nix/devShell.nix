{
  mkShell,
  lib,
  libx11,
  libice,
  libsm,
  libxi,
  libxrandr,
  stdenv,
  expat,
  fontconfig,
  freetype,
  libGL,
  vulkan-loader,
  wayland,
  libxkbcommon,
  libxkbcommon_8,
  pkg-config,
  libxcb,
  xcbutil,
  libxcursor,
  cargo,
  rustfmt,
  clippy,
  clang,
  rust-analyzer,
  glib,
  vscode,
  create-dmg
}:
mkShell rec {
  meta.license = lib.licenses.unlicense;
  runtimeLibs = lib.optionals stdenv.isLinux [
    expat
    fontconfig
    freetype
    libGL
    vulkan-loader
    wayland
    libxkbcommon

    libx11
    libice
    libsm
    libxi
    libxrandr
    libxcursor
    libxcb
    xcbutil
  ];

  buildInputs = [
    cargo
    rustfmt
    clippy
    rust-analyzer
    pkg-config
    clang
  ] ++ lib.optionals stdenv.isLinux [
    glib
    vscode
  ] ++ lib.optionals stdenv.isDarwin [
    create-dmg
  ];

  nativeBuildInputs = lib.optionals stdenv.isLinux [
    pkg-config
    libxcb
    xcbutil
    libxkbcommon
    libxkbcommon_8
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath runtimeLibs;
}
