#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
version="$(bash "${script_dir}/read-version.sh")"

update_section_version() {
  local file="$1"
  local section="$2"
  local in_section=0
  local replaced=0
  local output=()

  while IFS= read -r line; do
    if [[ "${line// /}" == "${section// /}" ]]; then
      in_section=1
      output+=("${line}")
      continue
    fi

    if [[ $in_section -eq 1 && $replaced -eq 0 && ${line// /} == version=* ]]; then
      output+=("version = \"${version}\"")
      replaced=1
      continue
    fi

    if [[ $in_section -eq 1 && ${line:0:1} == "[" && ${line} != "${section}" ]]; then
      in_section=0
    fi

    output+=("${line}")
  done <"${file}"

  if [[ $replaced -eq 1 ]]; then
    printf '%s\n' "${output[@]}" >"${file}"
  fi
}

update_bundle_version() {
  local file="$1"
  local in_bundle=0
  local replaced=0
  local output=()

  while IFS= read -r line; do
    if [[ "${line// /}" == "[package.metadata.bundle]" ]]; then
      in_bundle=1
      output+=("${line}")
      continue
    fi

    if [[ $in_bundle -eq 1 && $replaced -eq 0 && ${line// /} == version=* ]]; then
      output+=("version = \"${version}\"")
      replaced=1
      continue
    fi

    if [[ $in_bundle -eq 1 && ${line:0:1} == "[" && ${line} != "[package.metadata.bundle]" ]]; then
      in_bundle=0
    fi

    output+=("${line}")
  done <"${file}"

  if [[ $replaced -eq 1 ]]; then
    printf '%s\n' "${output[@]}" >"${file}"
  fi
}

cd "${repo_root}"

update_section_version "${repo_root}/Cargo.toml" "[workspace.package]"
for manifest in "${repo_root}"/crates/*/Cargo.toml; do
  update_section_version "${manifest}" "[package]"
  if [[ "${manifest}" == "${repo_root}/crates/core/Cargo.toml" ]]; then
    update_section_version "${manifest}" "[package.metadata.bundle]"
  fi
 done

echo "Synced version ${version} into Cargo.toml manifests"
