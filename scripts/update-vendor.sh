#!/bin/bash
# Script to update vendored DSRs dependency

set -e

echo "🔄 Updating vendored DSRs dependency..."

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Get the latest SHA from DSRs repo
echo -e "${YELLOW}Fetching latest DSRs version...${NC}"
LATEST_SHA=$(curl -s https://api.github.com/repos/krypticmouse/DSRs/commits/main | jq -r '.sha' | head -c 7)
echo -e "Latest SHA: ${GREEN}$LATEST_SHA${NC}"

# Check current SHA
if [ -f vendor/.git_sha ]; then
    CURRENT_SHA=$(cat vendor/.git_sha)
    echo -e "Current SHA: ${GREEN}$CURRENT_SHA${NC}"
else
    echo -e "${YELLOW}No SHA file found, treating as first install${NC}"
    CURRENT_SHA="unknown"
fi

if [ "$LATEST_SHA" == "$CURRENT_SHA" ] && [ "$1" != "--force" ]; then
    echo -e "${GREEN}✅ Already up to date!${NC}"
    exit 0
fi

# Create backup
echo -e "${YELLOW}Creating backup of current vendor directory...${NC}"
if [ -d vendor ]; then
    mv vendor vendor.backup
fi

# Clone latest DSRs
echo -e "${YELLOW}Cloning latest DSRs...${NC}"
git clone --depth 1 https://github.com/krypticmouse/DSRs.git vendor-tmp

# Copy to vendor (excluding .git)
echo -e "${YELLOW}Copying files to vendor directory...${NC}"
mkdir -p vendor
cp -r vendor-tmp/* vendor/ 2>/dev/null || true
cp -r vendor-tmp/.[^.]* vendor/ 2>/dev/null || true
rm -rf vendor/.git

# Save the SHA
echo "$LATEST_SHA" > vendor/.git_sha

# Fix Rust edition issues for compatibility
echo -e "${YELLOW}Fixing Rust edition compatibility...${NC}"
find vendor -name "Cargo.toml" -exec sed -i.bak 's/edition = "2024"/edition = "2021"/g' {} \;
find vendor -name "*.bak" -delete

# Fix let-chain syntax in macros if needed
if [ -f vendor/crates/dsrs-macros/src/lib.rs ]; then
    echo -e "${YELLOW}Fixing let-chain syntax...${NC}"
    # This is a simplified fix - in reality Claude would do this more carefully
    sed -i.bak 's/if let Ok(nv) = syn::parse2::<MetaNameValue>(tokens)$/if let Ok(nv) = syn::parse2::<MetaNameValue>(tokens) {/g' vendor/crates/dsrs-macros/src/lib.rs
    sed -i.bak 's/&& nv\.path\.is_ident("desc")$/if nv.path.is_ident("desc") {/g' vendor/crates/dsrs-macros/src/lib.rs
    sed -i.bak 's/&& let syn::Expr::Lit/if let syn::Expr::Lit/g' vendor/crates/dsrs-macros/src/lib.rs
    find vendor -name "*.bak" -delete
fi

# Clean up
echo -e "${YELLOW}Cleaning up...${NC}"
rm -rf vendor-tmp

# Test that it builds
echo -e "${YELLOW}Testing build...${NC}"
if cargo check; then
    echo -e "${GREEN}✅ Build successful!${NC}"
    rm -rf vendor.backup
else
    echo -e "${RED}❌ Build failed! Restoring backup...${NC}"
    rm -rf vendor
    if [ -d vendor.backup ]; then
        mv vendor.backup vendor
    fi
    exit 1
fi

echo -e "${GREEN}✨ Successfully updated DSRs from $CURRENT_SHA to $LATEST_SHA${NC}"
echo -e "${YELLOW}Don't forget to commit these changes!${NC}"