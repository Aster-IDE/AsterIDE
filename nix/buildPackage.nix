{
  lib,
  pkgs,
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
  craneLib = inputs.crane.mkLib pkgs;

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
