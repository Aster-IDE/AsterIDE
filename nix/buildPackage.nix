{
  rustPlatform
}:
rustPlatform.buildRustPackage {
  pname = "asteride";
  version = "0.1.0";

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };
}
