{
  description = "AsterIDE — a simple text editor written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix.url = "github:numtide/treefmt-nix";

    # pleme-io substrate provides the reusable eframe/egui GUI build kit — the
    # single source of the X11 + wayland + vulkan + GL (Linux) / apple-sdk
    # (macOS) native-dependency surface an eframe app needs. Tracks main;
    # follows our nixpkgs so the whole input set resolves against one nixpkgs.
    substrate = {
      url = "github:pleme-io/substrate";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, treefmt-nix, substrate, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];

      imports = [ treefmt-nix.flakeModule ];

      perSystem =
        { pkgs, ... }:
        let
          # eframe/egui GUI build kit from substrate.
          eframe = import "${substrate}/lib/build/rust/eframe.nix" { inherit pkgs; };

          # The AsterIDE binary (crates/core). Version is read from the
          # workspace Cargo.toml ([workspace.package]); the X11/wayland/vulkan/
          # GL (Linux) / apple-sdk (macOS) native deps come from the kit.
          #
          # The macOS .app / .dmg / .pkg bundles are NOT built here — they need
          # the full Xcode toolchain (xcodebuild), which is not in nixpkgs and
          # cannot run in the Nix sandbox. Those stay in the justfile
          # (`just create-mac-app` / `create-mac-installers`), driven by CI via
          # `nix develop -c just ci-publish-macos`. This derivation is the
          # portable binary the Xcode project embeds and that the Linux/
          # Windows/FreeBSD jobs ship.
          asteride = eframe.mkPackage {
            pname = "asteride";
            src = inputs.self;
            meta = with pkgs.lib; {
              description = "A simple text editor written in Rust";
              homepage = "https://github.com/Aster-IDE/AsterIDE";
              license = licenses.gpl3Only;
              mainProgram = "asteride";
              platforms = platforms.unix;
            };
          };

          asterideApp = {
            type = "app";
            program = "${asteride}/bin/asteride";
          };
        in
        {
          packages.asteride = asteride;
          packages.default = asteride;

          # AsterIDE's canonical build path: CI runs
          # `nix develop -c just ci-publish-macos`, macApp/nix-build.sh runs
          # `nix develop -c cargo build`. Layers `just` (+ macOS create-dmg) on
          # the kit's toolchain + native libs + LD_LIBRARY_PATH.
          devShells.default = eframe.mkDevShell {
            extraPackages =
              [ pkgs.just ]
              ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.create-dmg ];
          };

          apps.asteride = asterideApp;
          apps.default = asterideApp;

          # `nix fmt` — treefmt-nix flake-module (Nix + TOML + Rust).
          # flakeCheck = false: `nix flake check` should not gate on formatting
          # the whole Rust tree; `nix fmt` is available on demand.
          treefmt = {
            flakeCheck = false;
            projectRootFile = "flake.nix";
            programs = {
              nixfmt.enable = true;
              taplo.enable = true;
              rustfmt.enable = true;
            };
            settings.formatter.rustfmt.options = [
              "--config"
              "condense_wildcard_suffixes=true,tab_spaces=2,imports_layout=vertical"
              "--style-edition"
              "2024"
            ];
          };
        };
    };
}
