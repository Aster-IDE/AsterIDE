{
  lib,
  stdenv,
  rustPlatform,
  cargo,
  rustc,
}:
# IMPORTANT: broken, need to change it to get a binary.
stdenv.mkDerivation (finalAttrs: {
  pname = "asteride";
  version = (fromTOML (builtins.readFile ../Version.toml)).version;
  src = ../.;

  __noChroot = true;

  nativeBuildInputs = [
    cargo
    rustc
    rustPlatform.cargoSetupHook
  ];

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ../Cargo.lock;
  };

  buildPhase = ''
    runHook preBuild

    cargo build --release --offline --target-dir target

    xcodebuild \
      -project macApp/macApp.xcodeproj \
      -target asteride \
      -configuration Release \
      -derivedDataPath build/DerivedData \
      CODE_SIGNING_ALLOWED=NO \
      build

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall
    mkdir -p $out/Applications
    cp -R build/DerivedData/Build/Products/Release/asteride.app $out/Applications/
    mkdir -p $out/bin
    ln -sf $out/Applications/asteride.app/Contents/MacOS/asteride $out/bin/asteride
    runHook postInstall
  '';

  meta = with lib; {
    description = "A Simple Text Editor written in Rust.";
    homepage = "https://github.com/playfairs/AsterIDE";
    license = licenses.gpl3;
    maintainers = [ ];
    platforms = platforms.darwin;
  };
})
