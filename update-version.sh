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

# Update metainfo.xml.in - prepend a new release stub; never touch older entries
echo "  - data/me.rueegger.bootmate.metainfo.xml.in"
CURRENT_DATE=$(date +%Y-%m-%d)
METAINFO="data/me.rueegger.bootmate.metainfo.xml.in"

if grep -q "<release version=\"$NEW_VERSION\"" "$METAINFO"; then
    echo "    (release $NEW_VERSION already present — leaving changelog untouched)"
else
    python3 - "$NEW_VERSION" "$CURRENT_DATE" "$METAINFO" <<'PYEOF'
import sys
version, date, path = sys.argv[1], sys.argv[2], sys.argv[3]
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
marker = '<releases>'
idx = content.find(marker)
if idx == -1:
    sys.exit(f"<releases> tag not found in {path}")
insert_at = idx + len(marker)
stub = (
    f"\n    <release version=\"{version}\" date=\"{date}\">\n"
    f"      <description>\n"
    f"        <p>TODO: short summary</p>\n"
    f"        <ul>\n"
    f"          <li>TODO: changelog entry</li>\n"
    f"        </ul>\n"
    f"      </description>\n"
    f"    </release>"
)
with open(path, 'w', encoding='utf-8') as f:
    f.write(content[:insert_at] + stub + content[insert_at:])
PYEOF
    echo "    (prepended release $NEW_VERSION stub — fill in the <li> entries)"
fi

echo ""
echo "Version updated to $NEW_VERSION successfully!"
echo ""
echo "Modified files:"
echo "  - meson.build"
echo "  - Cargo.toml"
echo "  - po/en.po"
echo "  - po/de.po"
echo "  - data/me.rueegger.bootmate.metainfo.xml.in (new release stub prepended)"
echo ""
echo "Next steps:"
echo "  1. Fill in the TODO changelog entries in data/me.rueegger.bootmate.metainfo.xml.in"
echo "  2. Review changes: git diff"
echo "  3. Commit: git add . && git commit -m 'Bump version to $NEW_VERSION'"
