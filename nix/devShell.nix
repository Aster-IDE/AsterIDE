{
  pkgs,
  eframe,
}:
# The dev shell is AsterIDE's canonical build path: CI runs
# `nix develop -c just ci-publish-macos`, the macApp Xcode "Run Script" phase
# runs `nix develop -c cargo build` (see macApp/nix-build.sh), and the justfile
# targets assume it. It layers the project's packaging tools on top of
# substrate's eframe/egui native-dep surface (Rust toolchain + rust-analyzer +
# X11/wayland/vulkan/GL on Linux, apple-sdk on macOS, pkg-config, and
# LD_LIBRARY_PATH wired for the wgpu/vulkan/GL loaders on Linux).
eframe.mkDevShell {
  extraPackages =
    [ pkgs.just ]
    # create-dmg is a macOS-only packaging tool used by `just create-mac-dmg`;
    # gating it to Darwin keeps `nix develop` working on Linux.
    ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.create-dmg ];
}
