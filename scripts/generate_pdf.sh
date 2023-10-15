#!/usr/bin/env bash
set -e

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

if ! [[ $(python --version) ]]; then
  echo "No 'python' command found for running headless PDF generation" >&2
  exit 1
fi

OUTFILE=$(cat resume_data.yaml \
  | grep -E "^name:\s.+$" \
  | sed "s/name: //g" \
  | sed "s/ //g" \
  | xargs printf "%s-Resume.pdf\n")

rm -rf "$OUTFILE"

cd dist/
rm -rf resume
mkdir resume
cp -r * resume/ > /dev/null || true
python -m http.server 8081 &
PID=$!
cd ..

sleep 1

{
    google-chrome-stable --headless --disable-gpu \
        --run-all-compositor-stages-before-draw \
        --print-to-pdf="$OUTFILE" \
        http://localhost:8081/resume

} || {
    printf ""
}

kill "$PID"
rm -rf dist/resume
