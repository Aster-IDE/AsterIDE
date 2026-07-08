{
  pkgs,
  eframe,
  src,
}:
# Builds the AsterIDE Rust binary (`asteride`, crates/core) via substrate's
# eframe/egui build kit. Version is read from the workspace Cargo.toml
# ([workspace.package]); native deps (X11/wayland/vulkan/GL on Linux,
# apple-sdk on macOS) come from the kit.
#
# NOTE: the macOS `.app` / `.dmg` / `.pkg` bundles are intentionally NOT built
# here — they require the full Xcode toolchain (xcodebuild), which is not in
# nixpkgs and cannot run inside the Nix sandbox. Those live in the justfile
# (`just create-mac-app` / `create-mac-installers`), which the CI job drives
# via `nix develop -c just ci-publish-macos`. This derivation is the portable
# binary that the Xcode project embeds and that Linux/Windows/FreeBSD ship.
eframe.mkPackage {
  pname = "asteride";
  inherit src;

  meta = with pkgs.lib; {
    description = "A simple text editor written in Rust";
    homepage = "https://github.com/Aster-IDE/AsterIDE";
    license = licenses.gpl3Only;
    mainProgram = "asteride";
    platforms = platforms.unix;
  };
}
