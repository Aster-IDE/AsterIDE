set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]
build_dir := "build"

build-nix:
    nix build

clean:
    rm -rf build
    rm -f result

create-mac-pkg:
    @just build-nix
    ./scripts/macos/build-pkg.sh

create-mac-dmg:
    @just build-nix
    ./scripts/macos/build-dmg.sh

create-mac-release:
    @just create-mac-dmg
