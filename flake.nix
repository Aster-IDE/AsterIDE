{
  description = "AsterIDE Development Environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };
  outputs =
    { nixpkgs, treefmt-nix, ... }:
    let
      forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
      inputs = {
        inherit nixpkgs treefmt-nix;
      };
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            config.allowUnfreePredicate =
              pkg:
              builtins.elem (nixpkgs.lib.getName pkg) [
                "vscode"
              ];
          };
        in
        rec {
          asteride = pkgs.callPackage ./nix/devShell.nix { };
          default = asteride;
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        rec {
          asteride = pkgs.callPackage ./nix/buildPackage.nix { };
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
    };
}
