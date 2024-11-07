{ writeShellScriptBin, yq }:
writeShellScriptBin "deploy" ''
  if output=$(git status --porcelain) && [ -z "$output" ]; then
    echo "Working dir is clean, proceeding..."
  else
    echo "Error: Repo has uncommited changes"
    echo "Commit and try again"
    exit 1
  fi

  WASM=$(nix build .#wasm --print-out-paths)

  DIR=$(mktemp -d)

  OUTFILE=$(cat resume_data.yaml | ${yq}/bin/yq .name | sed "s/ //g" | xargs printf "%s-Resume.pdf\n")

  generate_pdf $DIR/$OUTFILE $WASM
  cp -r $WASM/resume/* $DIR

  echo ""
  echo "Build resume:"
  tree $DIR

  echo ""
  echo "Uploading new resume..."
  scp -r $DIR/* troy@10.222.0.1:/tmp/resume_data

  echo ""
  echo "Making new resume public..."
  ssh troy@10.222.0.1 "chmod -R u=rxw,g=xr,o=xr /tmp/resume_data && rm -rf /www/troy/public/resume/ && cp -r /tmp/resume_data /www/troy/public/resume && echo "Updated resume successfully"" 
''
