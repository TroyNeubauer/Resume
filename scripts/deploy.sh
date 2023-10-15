#!/usr/bin/env bash
set -e
rm -rf dist

# Ensure that a `trunk serve` instance doesn't step on us
pkill trunk | true

echo "Building resume..."
nix-shell --command "trunk build"

echo "Generating pdf..."
nix-shell --command "./scripts/generate_pdf.sh"

cp TroyNeubauer-Resume.pdf dist/

ls -lah dist
echo "Uploading new resume..."
scp -r dist/* troy@10.222.0.1:/tmp/resume_data
# /www/public/resume

echo "Making new resume public..."
ssh troy@10.222.0.1 "chown -R troy:www-data /tmp/resume_data && chmod -R u=rxw,g=xr,o=xr /tmp/resume_data && rm -rf /www/public/resume/ && mv /tmp/resume_data /www/public/resume/ && rm -rf /tmp/resume_data"

echo "Updated resume successfully"
