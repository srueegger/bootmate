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

# Cargo requires 3-component semver, append .0 for X.Y versions
if [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+$ ]]; then
    CARGO_VERSION="${NEW_VERSION}.0"
else
    CARGO_VERSION="$NEW_VERSION"
fi

CURRENT_DATE=$(date +%Y-%m-%d)

# All edits are done by Python, writing the files in place. 'sed -i' differs
# between GNU and BSD/macOS and fails on some mounted file systems.
python3 - "$NEW_VERSION" "$CARGO_VERSION" "$CURRENT_DATE" <<'PYEOF'
import re
import sys

version, cargo_version, date = sys.argv[1:4]
VERSION_RE = r"[0-9]+\.[0-9]+(?:\.[0-9]+)?"


def replace_once(path, pattern, replacement):
    with open(path, encoding="utf-8") as f:
        content = f.read()
    new_content, count = re.subn(pattern, replacement, content, count=1, flags=re.MULTILINE)
    if count != 1:
        sys.exit(f"Error: version pattern not found in {path}")
    with open(path, "w", encoding="utf-8") as f:
        f.write(new_content)
    print(f"  - {path}")


replace_once("meson.build", rf"version: '{VERSION_RE}'", f"version: '{version}'")
replace_once("Cargo.toml", rf'^version = "{VERSION_RE}"', f'version = "{cargo_version}"')
for po in ("po/en.po", "po/de.po"):
    replace_once(
        po,
        rf"Project-Id-Version: bootmate {VERSION_RE}",
        f"Project-Id-Version: bootmate {version}",
    )

# metainfo.xml.in - prepend a new release stub; never touch older entries
path = "data/me.rueegger.bootmate.metainfo.xml.in"
with open(path, encoding="utf-8") as f:
    content = f.read()
print(f"  - {path}")
if f'<release version="{version}"' in content:
    print(f"    (release {version} already present - leaving changelog untouched)")
else:
    marker = "<releases>"
    idx = content.find(marker)
    if idx == -1:
        sys.exit(f"Error: <releases> tag not found in {path}")
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
    with open(path, "w", encoding="utf-8") as f:
        f.write(content[:insert_at] + stub + content[insert_at:])
    print(f"    (prepended release {version} stub - fill in the <li> entries)")
PYEOF

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
echo "  2. Build once (this updates the version in Cargo.lock)"
echo "  3. Review changes: git diff"
echo "  4. Commit: git add . && git commit -m 'Bump version to $NEW_VERSION'"
