{
  description = "AsterIDE Development Environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/26.05";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };
  outputs =
    { nixpkgs, self, ... }@inputs:
    let
      forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    in
    {
      overlays.default = final: prev: {
        asteride = final.callPackage ./nix/buildPackage.nix { inherit inputs; };
      };

      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        rec {
          asteride = pkgs.callPackage ./nix/devShell.nix { inherit inputs; };
          default = asteride;
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ self.overlays.default ];
          };
        in
        rec {
          asteride = pkgs.asteride;
          default = asteride;
        }
      );

      formatter = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        import ./nix/formatter.nix { inherit pkgs inputs; }
      );

      homeModules.asteride =
        {
          config,
          lib,
          pkgs,
          ...
        }:
        import ./nix/homeModules.nix {
          inherit config lib pkgs;
          asteride-pkg = self.packages.${pkgs.system}.asteride;
        };
    };
}
