#!/bin/bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

if [[ "$(uname -s)" != "Darwin" ]]; then
    echo "macOS packaging requires macOS, sips, and iconutil." >&2
    exit 1
fi

TARGET_DIR="${CARGO_TARGET_DIR:-$PROJECT_ROOT/target}"
RELEASE_DIR="$TARGET_DIR/aarch64-apple-darwin/release"
APP_DIR="$RELEASE_DIR/Daozai.app"
APP_STAGING="$RELEASE_DIR/Daozai.app.staging.$$.tmp"
TEMP_DIR="$(mktemp -d)"
ICONSET_DIR="$TEMP_DIR/Daozai.iconset"
ICON_SOURCE="$PROJECT_ROOT/assets/daozai-logo.png"

cleanup() {
    rm -rf "$APP_STAGING" "$TEMP_DIR"
}
trap cleanup EXIT

MACOSX_DEPLOYMENT_TARGET=11.0 cargo build --locked --release --bin daozai --target aarch64-apple-darwin

EXECUTABLE="$RELEASE_DIR/daozai"
if [[ ! -x "$EXECUTABLE" ]]; then
    echo "Release executable not found at $EXECUTABLE" >&2
    exit 1
fi

VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)"
if [[ -z "$VERSION" ]]; then
    echo "Could not read package version from Cargo.toml." >&2
    exit 1
fi

mkdir -p "$RELEASE_DIR" "$ICONSET_DIR" \
    "$APP_STAGING/Contents/MacOS" "$APP_STAGING/Contents/Resources"
cp "$EXECUTABLE" "$APP_STAGING/Contents/MacOS/daozai"
chmod 755 "$APP_STAGING/Contents/MacOS/daozai"

create_icon() {
    local filename="$1"
    local pixels="$2"
    local artwork_pixels=$((pixels * 9 / 10))
    local artwork="$TEMP_DIR/$filename"
    sips -z "$artwork_pixels" "$artwork_pixels" "$ICON_SOURCE" --out "$artwork" >/dev/null
    sips -p "$pixels" "$pixels" "$artwork" --out "$ICONSET_DIR/$filename" >/dev/null
}

create_icon icon_16x16.png 16
create_icon icon_16x16@2x.png 32
create_icon icon_32x32.png 32
create_icon icon_32x32@2x.png 64
create_icon icon_128x128.png 128
create_icon icon_128x128@2x.png 256
create_icon icon_256x256.png 256
create_icon icon_256x256@2x.png 512
create_icon icon_512x512.png 512
create_icon icon_512x512@2x.png 1024
iconutil -c icns "$ICONSET_DIR" -o "$APP_STAGING/Contents/Resources/Daozai.icns"

cat > "$APP_STAGING/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>zh_CN</string>
    <key>CFBundleExecutable</key>
    <string>daozai</string>
    <key>CFBundleIconFile</key>
    <string>Daozai.icns</string>
    <key>CFBundleIdentifier</key>
    <string>com.renly1994.daozai</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Daozai</string>
    <key>CFBundleDisplayName</key>
    <string>岛仔</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
</dict>
</plist>
PLIST

plutil -lint "$APP_STAGING/Contents/Info.plist" >/dev/null
rm -rf "$APP_DIR"
mv "$APP_STAGING" "$APP_DIR"
echo "$APP_DIR"
