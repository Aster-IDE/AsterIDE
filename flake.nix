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
    { nixpkgs, ... }@inputs:
    let
      forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
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
          asteride = pkgs.callPackage ./nix/devShell.nix { inherit inputs; };
          default = asteride;
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        rec {
          asteride = pkgs.callPackage ./nix/buildPackage.nix { inherit inputs;};
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
