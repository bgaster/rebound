#!/bin/sh
create-dmg \
  --volname "Rebound Installer" \
  --volicon "icons/128x128.icns" \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --icon "Rebound.app" 200 190 \
  --hide-extension "Rebound.app" \
  --app-drop-link 600 185 \
  "target/release/bundle/osx/Rebound-Installer.dmg" \
  "target/release/bundle/osx/"