#!/bin/sh
set -e
cd "$SRCROOT/.."

NIX_BIN=""
for candidate in \
  /run/current-system/sw/bin/nix \
  /nix/var/nix/profiles/default/bin/nix \
  /usr/local/bin/nix \
  "$(command -v nix 2>/dev/null)"
do
  if [ -x "$candidate" ]; then
    NIX_BIN="$candidate"
    break
  fi
done

if [ -z "$NIX_BIN" ]; then
  echo "error: could not locate nix binary" >&2
  exit 1
fi

if [ "$1" = "Release" ]; then
  "$NIX_BIN" develop --command cargo build --release
else
  "$NIX_BIN" develop --command cargo build
fi
