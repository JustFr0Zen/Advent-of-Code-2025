#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CWD="$(dirname "$SCRIPT_DIR")"

# Load ENV file
if [[ -f "$CWD/.env" ]]; then
    set -a
    source "$CWD/.env"
    set +a
elif [[ -f .env ]]; then
    set -a
    source .env
    set +a
fi


echo "${SESSION}" > "${CWD}/.aoc_tiles/session.cookie"
echo "Cookie Saved Successfully"
