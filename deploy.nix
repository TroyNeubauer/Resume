{ writeShellScriptBin, wasm, yq }:
writeShellScriptBin "deploy" ''
  if output=$(git status --porcelain) && [ -z "$output" ]; then
    echo "Working dir is clean, proceeding..."
  else
    echo "Error: Repo has uncommited changes"
    echo "Commit and try again"
    exit 1
  fi

  DIR=$(mktemp -d)

  OUTFILE=$(cat resume_data.yaml | ${yq}/bin/yq .name | sed "s/ //g" | xargs printf "%s-Resume.pdf\n")

  generate_pdf $DIR/$OUTFILE
  cp -r ${wasm}/* $DIR

  echo "Build resume:"
  tree $DIR

  echo "Uploading new resume..."
  scp -r $DIR/* troy@10.222.0.1:/tmp/resume_data
  # /www/public/resume

  echo "Making new resume public..."
  ssh troy@10.222.0.1 bash -c "set -x && chown -R troy:www-data /tmp/resume_data && chmod -R u=rxw,g=xr,o=xr /tmp/resume_data && rm -rf /www/public/resume/ && mv /tmp/resume_data /www/public/resume/ && rm -rf /tmp/resume_data"

  echo "Updated resume successfully"
''
