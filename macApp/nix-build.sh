#!/bin/sh
set -e
cd "$SRCROOT/.."

NIX="/run/current-system/sw/bin/nix"

if [ "$1" = "Release" ]; then
  exec $NIX develop --command cargo build --release
else
  exec $NIX develop --command cargo build
fi
