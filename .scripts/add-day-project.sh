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

usage() {
    cat << EOF
Usage: $0 [options]

Options:
    -h, --help       Show this help message

Configuration:
    Reads 'day' from .env file in parent directory.

    Example .env file:
        DAY=1
        SESSION=your_session_token

EOF
    exit 1
}

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            usage
            ;;
        *)
            echo "Error: Unknown option $1"
            usage
            ;;
    esac
done

# Check for required environment variables
if [[ -z "${DAY:-}" ]]; then
    echo "Error: 'DAY' must be set in .env file"
    echo "Example: DAY=1"
    exit 1
fi

# Remove any leading zeros and validate day is a number
if [[ ! "$DAY" =~ ^[0-9]+$ ]]; then
    echo "Error: day must be a number (e.g., DAY=1 or DAY=01)"
    exit 1
fi

DAY_NUMBER=$((10#$DAY))
# Format day with leading zero for directory name (day_01, day_02, etc.)
DAY_DIR=$(printf "day_%02d" "$DAY_NUMBER")

cargo generate --path "${CWD}/.daily_template" --name "${DAY_DIR}"
