#!/bin/bash
rm -rf ~/../host/web/tneubauer.xyz/files/resume
mkdir ~/../host/web/tneubauer.xyz/files/resume
mkdir ~/../host/web/tneubauer.xyz/files/resume/pkg

cp wasm-app/index.html ~/../host/web/tneubauer.xyz/files/resume
cp wasm-app/styles.css ~/../host/web/tneubauer.xyz/files/resume
cp wasm-app/favicon.ico ~/../host/web/tneubauer.xyz/files/resume
cp wasm-app/pkg/bundle.js ~/../host/web/tneubauer.xyz/files/resume/pkg/
cp wasm-app/pkg/wasm_app_bg.wasm ~/../host/web/tneubauer.xyz/files/resume/pkg/
cp ./TroyNeubauer-Resume.pdf ~/../host/web/tneubauer.xyz/files/resume/
chmod 777 ~/../host/web/tneubauer.xyz/files/resume/*
