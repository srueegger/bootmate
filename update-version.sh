#!/bin/bash
# Script to update version across all project files

set -e

if [ -z "$1" ]; then
    echo "Usage: ./update-version.sh <new-version>"
    echo "Example: ./update-version.sh 1.2.0"
    exit 1
fi

NEW_VERSION="$1"

# Validate version format (X.Y.Z)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Version must be in format X.Y.Z (e.g., 1.2.0)"
    exit 1
fi

echo "Updating version to $NEW_VERSION..."

# Update meson.build
echo "  - meson.build"
sed -i "s/version: '[0-9]\+\.[0-9]\+\.[0-9]\+'/version: '$NEW_VERSION'/" meson.build

# Update Cargo.toml
echo "  - Cargo.toml"
sed -i "s/^version = \"[0-9]\+\.[0-9]\+\.[0-9]\+\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update po files
echo "  - po/en.po"
sed -i "s/Project-Id-Version: bootmate [0-9]\+\.[0-9]\+\.[0-9]\+/Project-Id-Version: bootmate $NEW_VERSION/" po/en.po

echo "  - po/de.po"
sed -i "s/Project-Id-Version: bootmate [0-9]\+\.[0-9]\+\.[0-9]\+/Project-Id-Version: bootmate $NEW_VERSION/" po/de.po

# Update metainfo.xml.in - only update the first (newest) release version
echo "  - data/ch.srueegger.bootmate.metainfo.xml.in"
# Get current date in YYYY-MM-DD format
CURRENT_DATE=$(date +%Y-%m-%d)
# Update only the first occurrence of release version
sed -i "0,/<release version=\"[0-9]\+\.[0-9]\+\.[0-9]\+\"/s/<release version=\"[0-9]\+\.[0-9]\+\.[0-9]\+\" date=\"[^\"]*\">/<release version=\"$NEW_VERSION\" date=\"$CURRENT_DATE\">/" data/ch.srueegger.bootmate.metainfo.xml.in

# Update RELEASE.md
echo "  - RELEASE.md"
sed -i "s/\*\*Aktuelle Version:\*\* [0-9]\+\.[0-9]\+\.[0-9]\+/**Aktuelle Version:** $NEW_VERSION/" RELEASE.md

# Update PPA files (if they exist)
if [ -f "build-ppa.sh" ]; then
    echo "  - build-ppa.sh"
    sed -i "s/^VERSION=\"[0-9]\+\.[0-9]\+\.[0-9]\+\"/VERSION=\"$NEW_VERSION\"/" build-ppa.sh
fi

if [ -f "debian/changelog" ]; then
    echo "  - debian/changelog"
    # Update only the first line (newest version) in debian/changelog
    # Format: bootmate (VERSION-0ubuntu1~noble1) noble; urgency=medium
    sed -i "1s/bootmate ([0-9]\+\.[0-9]\+\.[0-9]\+-/bootmate ($NEW_VERSION-/" debian/changelog
    # Update date on line with the maintainer
    sed -i "0,/ -- /s/ -- \(.*\)  .*/ -- \1  $(date -R)/" debian/changelog
fi

echo ""
echo "Version updated to $NEW_VERSION successfully!"
echo ""
echo "Modified files:"
echo "  - meson.build"
echo "  - Cargo.toml"
echo "  - po/en.po"
echo "  - po/de.po"
echo "  - data/ch.srueegger.bootmate.metainfo.xml.in (version and date)"
echo "  - RELEASE.md"
if [ -f "build-ppa.sh" ]; then
    echo "  - build-ppa.sh"
fi
if [ -f "debian/changelog" ]; then
    echo "  - debian/changelog (version and timestamp)"
fi
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff"
echo "  2. Add release notes to data/ch.srueegger.bootmate.metainfo.xml.in"
echo "  3. Commit: git add . && git commit -m 'Bump version to $NEW_VERSION'"
