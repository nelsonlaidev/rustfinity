#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Usage: x.sh <challenge name>" >&2
  exit 1
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
challenge="$root/$1"

if [ ! -d "$challenge" ]; then
  echo "Challenge not found: $1" >&2
  exit 1
fi

cd "$challenge"

if ! cargo test; then
  echo "Tests failed, not submitting." >&2
  exit 1
fi

rustfinity submit
