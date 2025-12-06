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
    Reads 'day' and 'SESSION' from .env file in parent directory.

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

if [[ -z "${SESSION:-}" ]]; then
    echo "Error: 'SESSION' must be set in .env file"
    echo "Example: SESSION=your_session_token"
    exit 1
fi

# Remove any leading zeros and validate day is a number
if [[ ! "$DAY" =~ ^[0-9]+$ ]]; then
    echo "Error: DAY must be a number (e.g., DAY=1 or DAY=01)"
    exit 1
fi

DAY_NUMBER=$((10#$DAY))
# Format day with leading zero for directory name (day_01, day_02, etc.)
DAY_DIR=$(printf "day_%02d" "$DAY_NUMBER")


URL="https://adventofcode.com/2025/day/${DAY_NUMBER}/input"
echo "sending to \`$URL\`"

if command -v curl &> /dev/null; then
    INPUT_DATA=$(curl -sS -H "Cookie: session=$SESSION" "$URL")
    CURL_EXIT=$?
else
    echo "Error: curl not found. Please ensure Git Bash is properly installed."
    exit 1
fi

if [[ $CURL_EXIT -ne 0 ]]; then
    echo "Error: Failed to fetch data from $URL"
    exit 1
fi

# Write inputs
for FILENAME in input1.txt input2.txt; do
    FILE_PATH="$CWD/$DAY_DIR/$FILENAME"

    mkdir -p "$CWD/$DAY_DIR"
    printf "%s" "$INPUT_DATA" > "$FILE_PATH"

    if [[ $? -ne 0 ]]; then
        echo "Error: Failed to write to $FILE_PATH"
        exit 1
    fi

    echo "wrote $FILE_PATH"
done