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
  llvmPackages_22
}:
let
  inherit (inputs) fenix crane;

  toolchain =
    with fenix.packages.${stdenv.system};
    combine [
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
    llvmPackages_22.clang
  ];

  commonArgs = {
    pname = "asteride";
    version = "2.0.0";
    src = craneLib.cleanCargoSource ../.;
    strictDeps = true;
    nativeBuildInputs = buildDeps;
    buildInputs = buildDeps;
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;

    meta = with lib; {
      description = "A Simple Text Editor written in Rust.";
      homepage = "https://asteride.dev";
      license = licenses.asl20;
      maintainers = [ "playfairs <root@playfairs.cc>" "Invra <identificationsucks@gmail.com>" ];
    };
  }
)
