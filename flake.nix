{
  description = "AsterIDE — a simple text editor written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";

    # pleme-io substrate provides the reusable eframe/egui GUI build kit that
    # nix/{buildPackage,devShell}.nix consume — the single source of the
    # X11 + wayland + vulkan + GL (Linux) / apple-sdk (macOS) native-dep
    # surface an eframe app needs. Tracks main; follows our nixpkgs so the
    # whole input set resolves against one nixpkgs.
    substrate = {
      url = "github:pleme-io/substrate";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      treefmt-nix,
      substrate,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        # eframe/egui GUI build kit from substrate.
        eframe = import "${substrate}/lib/build/rust/eframe.nix" { inherit pkgs; };

        asteride = import ./nix/buildPackage.nix {
          inherit pkgs eframe;
          src = self;
        };

        asterideApp = {
          type = "app";
          program = "${asteride}/bin/asteride";
        };
      in
      {
        packages = {
          inherit asteride;
          default = asteride;
        };

        devShells = {
          asteride = import ./nix/devShell.nix { inherit pkgs eframe; };
          default = import ./nix/devShell.nix { inherit pkgs eframe; };
        };

        apps = {
          asteride = asterideApp;
          default = asterideApp;
        };

        formatter = import ./nix/formatter.nix {
          inherit pkgs;
          inputs = { inherit treefmt-nix; };
        };
      }
    );
}
