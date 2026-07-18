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
  inherit (inputs) fenix crane;

  toolchain =
    with fenix.packages.${stdenv.system};
    combine [
      minimal.toolchain
    ];

  craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

  buildDeps = [
    pkg-config
  ]
  ++ lib.optionals stdenv.isLinux [
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
    version = "2.0.0";
    src = craneLib.path ../.;
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
      maintainers = [
        "playfairs <root@playfairs.cc>"
        "Invra <identificationsucks@gmail.com>"
      ];
    };
  }
)
