set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]
build_dir := "build"

build-nix:
    @just sync-version
    nix build

sync-version:
    ./scripts/sync-version.sh

clean:
    rm -rf build
    rm -f result

create-mac-pkg:
    @just sync-version
    @just build-nix
    ./scripts/macos/build-pkg.sh

create-mac-dmg:
    @just sync-version
    @just build-nix
    ./scripts/macos/build-dmg.sh

create-mac-release:
    @just create-mac-dmg
