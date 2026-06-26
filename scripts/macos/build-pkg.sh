#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/../.." && pwd)"
cd "${repo_root}"

app_name="${APP_NAME:-AsterIDE}"
bundle_path="${BUNDLE_PATH:-result/Applications/${app_name}.app}"
build_dir="${BUILD_DIR:-build/pkg}"
payload_dir="${build_dir}/Payload"
output_dir="${OUTPUT_DIR:-build}"
output_pkg="${OUTPUT_PKG:-${output_dir}/${app_name}.pkg}"
pkg_identifier="${PKG_IDENTIFIER:-dev.playfairs.asteride}"
tmp_pkg="${TMPDIR:-/private/tmp}/${app_name}.$$.pkg"
version="${VERSION:-$(bash "${script_dir}/../read-version.sh")}"

remove_staging_dir() {
    if [[ -d "${build_dir}" ]]; then
        chmod -R u+w "${build_dir}" 2>/dev/null || true
        rm -rf "${build_dir}"
    fi
}

if ! command -v pkgbuild >/dev/null 2>&1; then
    echo "pkgbuild was not found. Run this on macOS with Xcode Command Line Tools installed." >&2
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
mkdir -p "${payload_dir}/Applications" "${output_dir}"

ditto "${bundle_path}" "${payload_dir}/Applications/${app_name}.app"
xattr -cr "${payload_dir}/Applications/${app_name}.app" 2>/dev/null || true

chmod u+w "${output_dir}" 2>/dev/null || true
rm -f "${output_pkg}"
rm -f "${tmp_pkg}"
pkgbuild \
    --root "${payload_dir}" \
    --install-location "/" \
    --identifier "${pkg_identifier}" \
    --version "${version}" \
    --ownership recommended \
    "${tmp_pkg}"

mv "${tmp_pkg}" "${output_pkg}"

remove_staging_dir
echo "Created ${output_pkg}"
