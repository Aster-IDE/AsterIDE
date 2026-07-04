#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
version_file="${repo_root}/Version.toml"

if [[ ! -f "${version_file}" ]]; then
  echo "Version.toml not found at ${version_file}" >&2
  exit 1
fi

version="$(grep -E '^version[[:space:]]*=\s*"[^"]+"' "${version_file}" | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')"

if [[ -z "${version}" ]]; then
  echo "Could not parse version from ${version_file}" >&2
  exit 1
fi

echo "${version}"
