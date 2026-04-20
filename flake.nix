{
  description = "AsterIDE Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "asteride-dev-shell";

          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy

            rust-analyzer
            pkg-config
          ];
        };
      });
}
