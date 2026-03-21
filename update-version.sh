#!/bin/bash
# Script to update version across all project files

set -e

if [ -z "$1" ]; then
    echo "Usage: ./update-version.sh <new-version>"
    echo "Example: ./update-version.sh 50.0"
    echo "Example: ./update-version.sh 1.2.0"
    exit 1
fi

NEW_VERSION="$1"

# Validate version format (X.Y or X.Y.Z)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+(\.[0-9]+)?$ ]]; then
    echo "Error: Version must be in format X.Y or X.Y.Z (e.g., 50.0 or 1.2.0)"
    exit 1
fi

echo "Updating version to $NEW_VERSION..."

# Update meson.build
echo "  - meson.build"
sed -i "s/version: '[0-9]\+\.[0-9]\+\(\.[0-9]\+\)\?'/version: '$NEW_VERSION'/" meson.build

# Update Cargo.toml (requires 3-component semver, append .0 for X.Y versions)
echo "  - Cargo.toml"
if [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+$ ]]; then
    CARGO_VERSION="${NEW_VERSION}.0"
else
    CARGO_VERSION="$NEW_VERSION"
fi
sed -i "s/^version = \"[0-9]\+\.[0-9]\+\(\.[0-9]\+\)\?\"/version = \"$CARGO_VERSION\"/" Cargo.toml

# Update po files
echo "  - po/en.po"
sed -i "s/Project-Id-Version: bootmate [0-9]\+\.[0-9]\+\(\.[0-9]\+\)\?/Project-Id-Version: bootmate $NEW_VERSION/" po/en.po

echo "  - po/de.po"
sed -i "s/Project-Id-Version: bootmate [0-9]\+\.[0-9]\+\(\.[0-9]\+\)\?/Project-Id-Version: bootmate $NEW_VERSION/" po/de.po

# Update metainfo.xml.in - only update the first (newest) release version
echo "  - data/me.rueegger.bootmate.metainfo.xml.in"
CURRENT_DATE=$(date +%Y-%m-%d)
sed -i "0,/<release version=\"[0-9]\+\.[0-9]\+\(\.[0-9]\+\)\?\"/s/<release version=\"[0-9]\+\.[0-9]\+\(\.[0-9]\+\)\?\" date=\"[^\"]*\">/<release version=\"$NEW_VERSION\" date=\"$CURRENT_DATE\">/" data/me.rueegger.bootmate.metainfo.xml.in

echo ""
echo "Version updated to $NEW_VERSION successfully!"
echo ""
echo "Modified files:"
echo "  - meson.build"
echo "  - Cargo.toml"
echo "  - po/en.po"
echo "  - po/de.po"
echo "  - data/me.rueegger.bootmate.metainfo.xml.in (version and date)"
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff"
echo "  2. Add release notes to data/me.rueegger.bootmate.metainfo.xml.in"
echo "  3. Commit: git add . && git commit -m 'Bump version to $NEW_VERSION'"
