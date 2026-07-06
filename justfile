set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]
version := `cargo xtask`
build_dir := "build"

build-clean:
    rm -rf {{build_dir}}

ci-publish-macos:
    @just build-clean
    @just create-mac-app
    @just create-mac-installers
    @just clean-mac-build-artifacts


create-mac-app:
    cargo build --release

    SKIP_CARGO_BUILD=1 xcodebuild \
      -project macApp/macApp.xcodeproj \
      -target AsterIDE \
      -configuration Release \
      CODE_SIGNING_ALLOWED=NO \
      MARKETING_VERSION={{version}} \
      CURRENT_PROJECT_VERSION={{version}} \
      build

    codesign --force --deep --sign - macApp/build/Release/AsterIDE.app
    mkdir -p {{build_dir}}
    ditto macApp/build/Release/AsterIDE.app {{build_dir}}/AsterIDE.app
    xattr -cr {{build_dir}}/AsterIDE.app

create-mac-installers:
    @just create-mac-pkg
    @just create-mac-dmg
    @just create-mac-tar

create-mac-pkg:
    pkgbuild --root {{build_dir}}/AsterIDE.app \
      --install-location "/Applications/AsterIDE.app" \
      --identifier dev.playfairs.asteride \
      --version {{version}} \
      {{build_dir}}/AsterIDE.pkg

create-mac-dmg:
    create-dmg \
      --volname "AsterIDE" \
      --window-size 500 300 \
      --icon-size 96 \
      --icon "AsterIDE.app" 125 150 \
      --app-drop-link 375 150 \
      "{{build_dir}}/AsterIDE.dmg" \
      "{{build_dir}}/AsterIDE.app"

create-mac-tar:
    tar -czf {{build_dir}}/AsterIDE.app.tar.gz -C {{build_dir}} AsterIDE.app

clean-mac-build-artifacts:
    rm -r {{build_dir}}/AsterIDE.app
