{ writeShellScriptBin, google-chrome, python3 }:
# google chrome doesnt play nicely with nix sandbox so use basic script instead
writeShellScriptBin "generate_pdf" ''
    if [ $# -eq 2 ]; then
        export OUTFILE=$1
        export SRC=$2
    else
      echo "Usage: generate_pdf <output pdf path> <server files path>"
      exit 1
    fi

    set -x
    echo "Running web server against $SRC"
    ${python3}/bin/python3 -m http.server 8081 -d $SRC &
    PID=$!

    sleep 1
    echo "Starting chrome"

    # Workaround for headless mode
    export LIBVA_DRIVER_NAME="vdpau";

    timeout 5 ${google-chrome}/bin/google-chrome-stable \
        --headless --disable-gpu --no-sandbox \
        --disable-crash-reporter --disable-breakpad \
        --disable-background-timer-throttling \
        --disable-default-apps --disable-extensions --disable-hang-monitor \
        --disable-popup-blocking --disable-prompt-on-repost --disable-sync \
        --disable-translate --disable-client-side-phishing-detection \
        --disable-databases --disable-local-storage \
        --run-all-compositor-stages-before-draw \
        --print-to-pdf=$OUTFILE \
        http://localhost:8081/resume

    kill "$PID"
''
