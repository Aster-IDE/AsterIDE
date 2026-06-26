#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/../.." && pwd)"
cd "${repo_root}"

app_name="${APP_NAME:-AsterIDE}"
bundle_path="${BUNDLE_PATH:-result/Applications/${app_name}.app}"
build_dir="${BUILD_DIR:-build/dmg}"
output_dir="${OUTPUT_DIR:-build}"
output_dmg="${OUTPUT_DMG:-${output_dir}/${app_name}.dmg}"
tmp_dmg="${TMPDIR:-/private/tmp}/${app_name}.$$.dmg"
version="${VERSION:-$(bash "${script_dir}/../read-version.sh")}"

remove_staging_dir() {
    if [[ -d "${build_dir}" ]]; then
        chmod -R u+w "${build_dir}" 2>/dev/null || true
        rm -rf "${build_dir}"
    fi
}

if ! command -v hdiutil >/dev/null 2>&1; then
    echo "hdiutil was not found. Run this on macOS." >&2
    exit 1
fi

if [[ ! -d "${bundle_path}" ]]; then
    echo "Could not find ${bundle_path}. Run 'nix build' or 'just build-nix' first." >&2
    exit 1
fi

if [[ -z "${version}" ]]; then
    echo "Could not determine the package version. Set VERSION explicitly and try again." >&2
    exit 1
fi

remove_staging_dir
mkdir -p "${build_dir}" "${output_dir}"

ditto "${bundle_path}" "${build_dir}/${app_name}.app"
xattr -cr "${build_dir}/${app_name}.app" 2>/dev/null || true

chmod u+w "${output_dir}" 2>/dev/null || true
rm -f "${output_dmg}"
rm -f "${tmp_dmg}"

hdiutil create -volname "${app_name}" \
    -srcfolder "${build_dir}" \
    -ov \
    -format UDRW \
    "${tmp_dmg}"

hdiutil convert "${tmp_dmg}" \
    -format UDZO \
    -imagekey zlib-level=9 \
    -o "${output_dmg}"

rm -f "${tmp_dmg}"

remove_staging_dir
echo "Created ${output_dmg}"
