{
  lib,
  pkgs,
  stdenv,
  inputs,
  pkg-config,
  libglvnd,
  freetype,
  fontconfig,
  libX11,
  libxcb,
  libxcb-wm,
  libxcursor,
  libxkbcommon,
}:
let
  inherit (inputs)fenix crane;

  toolchain = with fenix.packages.${stdenv.system}; combine [
    latest.toolchain
  ];
  
  craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

  buildDeps = [
    pkg-config
    libglvnd
    freetype
    fontconfig
    libX11
    libxcb
    libxcb-wm
    libxcursor
    libxkbcommon
  ];

  commonArgs = {
    pname = "asteride";
    version = "0.1.0";
    src = craneLib.cleanCargoSource ../.;
    strictDeps = true;
    nativeBuildInputs = buildDeps;
    buildInputs = buildDeps;
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (commonArgs // {
  inherit cargoArtifacts;

  meta = with lib; {
    description = "A Simple Text Editor written in Rust.";
    homepage = "https://github.com/playfairs/AsterIDE";
    license = licenses.gpl3;
    maintainers = [ "Invra <identificationsucks@gmail.com>" ];
  };
})
