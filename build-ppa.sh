#!/bin/bash
# Script to build and upload Boot Mate to Launchpad PPA

set -e

VERSION="1.2.0"
MAINTAINER="Samuel Rüegger <samuel@rueegger.me>"
PPA="ppa:rueegger/bootmate"

# Ubuntu releases to build for
RELEASES=("noble" "oracular" "plucky")
RELEASE_NAMES=("24.04 LTS" "25.10 STS" "26.04 (dev)")

echo "================================================"
echo "Boot Mate PPA Build Script"
echo "================================================"
echo "Version: $VERSION"
echo "PPA: $PPA"
echo "Releases: ${RELEASES[@]}"
echo "================================================"
echo ""

# Check for required tools
echo "Checking required tools..."
MISSING_TOOLS=()

if ! command -v debuild &> /dev/null || ! command -v dch &> /dev/null; then
    MISSING_TOOLS+=("devscripts")
fi

if ! command -v dput &> /dev/null; then
    MISSING_TOOLS+=("dput")
fi

if ! command -v dh &> /dev/null; then
    MISSING_TOOLS+=("debhelper")
fi

if ! command -v dpkg-buildpackage &> /dev/null; then
    MISSING_TOOLS+=("dpkg-dev")
fi

if [ ${#MISSING_TOOLS[@]} -gt 0 ]; then
    echo "ERROR: Missing required tools!"
    echo "Please install: sudo apt install ${MISSING_TOOLS[@]}"
    exit 1
fi

echo "✓ All required tools found"
echo ""

# Create and setup PPA build directory
echo "Setting up PPA build directory..."
rm -rf ppa
mkdir -p ppa
cd ppa
echo "✓ Created ppa/ directory"
echo ""

# Check if dput is configured
if [ ! -f ~/.dput.cf ]; then
    echo "WARNING: ~/.dput.cf not found!"
    echo "Creating dput configuration..."
    cat > ~/.dput.cf <<EOF
[bootmate-ppa]
fqdn = ppa.launchpad.net
method = ftp
incoming = ~rueegger/ubuntu/bootmate/
login = anonymous
allow_unsigned_uploads = 0
EOF
    echo "✓ Created ~/.dput.cf"
    echo ""
fi

# Clean any previous build artifacts in ppa directory
echo "Cleaning previous builds..."
rm -f *.deb *.dsc *.changes *.buildinfo *.tar.* *.upload
echo "✓ Cleaned"
echo ""

# Create orig.tar.xz from git
echo "Creating orig.tar.xz from git repository..."
git archive --format=tar --prefix=bootmate-${VERSION}/ HEAD | xz > bootmate_${VERSION}.orig.tar.xz
if [ $? -ne 0 ]; then
    echo "ERROR: Failed to create orig.tar.xz"
    exit 1
fi
echo "✓ Created ppa/bootmate_${VERSION}.orig.tar.xz"
echo ""

# Extract the source tree for building
echo "Extracting source tree..."
tar -xf bootmate_${VERSION}.orig.tar.xz
echo "✓ Extracted"
echo ""

# Build for each release
for i in "${!RELEASES[@]}"; do
    RELEASE="${RELEASES[$i]}"
    RELEASE_NAME="${RELEASE_NAMES[$i]}"

    echo "================================================"
    echo "Building for Ubuntu ${RELEASE_NAME} (${RELEASE})"
    echo "================================================"

    # Create a fresh extract for this release
    rm -rf bootmate-${VERSION}
    tar -xf bootmate_${VERSION}.orig.tar.xz

    # Copy debian directory into the extracted source
    echo "Copying debian directory..."
    cp -r ../debian bootmate-${VERSION}/
    cd bootmate-${VERSION}

    # Update changelog for this release
    export DEBFULLNAME="Samuel Rüegger"
    export DEBEMAIL="samuel@rueegger.me"

    # Create changelog entry for this release
    dch -v "${VERSION}-0ubuntu1~${RELEASE}1" -D "${RELEASE}" "New upstream release ${VERSION}" 2>/dev/null || true

    # Build source package
    echo "Building source package..."
    debuild -S -sa -d

    if [ $? -ne 0 ]; then
        echo "ERROR: Failed to build source package for ${RELEASE}"
        exit 1
    fi

    # Move back to ppa directory
    cd ..

    echo "✓ Source package built for ${RELEASE}"
    echo ""
done

echo "================================================"
echo "Build Complete!"
echo "================================================"
echo ""
echo "Generated source packages:"
ls -lh *.changes
echo ""
echo "Next steps:"
echo "1. Review the changes files above"
echo "2. Upload to PPA (from the ppa/ directory):"
echo ""
for RELEASE in "${RELEASES[@]}"; do
    echo "   dput ppa:rueegger/bootmate bootmate_${VERSION}-0ubuntu1~${RELEASE}1_source.changes"
done
echo ""
echo "Or upload all at once:"
echo "   cd ppa && for file in bootmate_${VERSION}-*.changes; do dput ppa:rueegger/bootmate \$file; done"
echo ""
echo "3. Monitor builds at: https://launchpad.net/~rueegger/+archive/ubuntu/bootmate/+packages"
