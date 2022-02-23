#!/usr/bin/env bash
set -e

trunk build

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." && pwd )"
cd "$DIR"

if [[ "$OSTYPE" == "darwin"* ]]; then
    shopt -s expand_aliases
    alias chrome="/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome"
fi

if ! [[ $(google-chrome-stable --version) ]]; then
  echo "No 'chrome' command found for running headless PDF generation" >&2
  exit 1
fi

OUTFILE=$(cat resume_data.yaml \
  | grep -E "^name:\s.+$" \
  | sed "s/name: //g" \
  | sed "s/ //g" \
  | xargs printf "%s-Resume.pdf\n")


trunk serve --port 8081 &
PID=$!

sleep 1

{
    google-chrome-stable --headless --disable-gpu \
        --run-all-compositor-stages-before-draw \
        --print-to-pdf="$OUTFILE" \
        http://localhost:8081

} || {
    printf ""
}

kill "$PID"
