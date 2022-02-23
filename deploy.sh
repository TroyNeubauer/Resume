#!/bin/bash
trunk build --public-url "/resume" --features print
rm -rf /home/host/web/tneubauer.xyz/files/resume
mkdir /home/host/web/tneubauer.xyz/files/resume

cp ./dist/* /home/host/web/tneubauer.xyz/files/resume
chmod 777 /home/host/web/tneubauer.xyz/files/resume/*
