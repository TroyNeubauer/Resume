OUTFILE=$(cat resume_data.yaml | yq .name | sed "s/ //g" | xargs printf "%s-Resume.pdf\n")

cd dist/
rm -rf resume
mkdir resume
cp -r * resume/ > /dev/null || true
python3 -m http.server 9000 -d XXX &
PID=$!
cd ..

sleep 1

{
    LIBVA_DRIVER_NAME="vdpau" timeout 5 google-chrome-stable \
        --headless --disable-gpu --no-sandbox \
        --disable-databases --disable-local-storage \
        --disable-crash-reporter --disable-breakpad \
        --run-all-compositor-stages-before-draw \
        --print-to-pdf="$OUTFILE" \
        http://localhost:8081/resume

} || {
    printf ""
}

kill "$PID"
