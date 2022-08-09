#!/bin/sh

set -e

KEYCLOAK_RUST_VERSION=`grep "version = " Cargo.toml | head -n1 | cut -d"\"" -f 2`
KEYCLOAK_VERSION=`echo ${KEYCLOAK_RUST_VERSION} | cut -f 1,2 -d"."`
KEYCLOAK_RUST_MAJOR_VERSION=${KEYCLOAK_VERSION}
GITHUB_ISSUE=`git branch --show-current | grep -oh "\d*"`

echo "Enter cargo version [${KEYCLOAK_RUST_VERSION}]: \c"
read keycloak_rust_version
[ -n "$keycloak_rust_version" ] && KEYCLOAK_RUST_VERSION=$keycloak_rust_version

echo "Enter Keycloak REST API version [${KEYCLOAK_VERSION}]: \c"
read keycloak_version
[ -n "$keycloak_version" ] && KEYCLOAK_VERSION=$keycloak_version

echo "Enter GitHub issue [${GITHUB_ISSUE}]: \c"
read github_issue
[ -n "$github_issue" ] && GITHUB_ISSUE=$github_issue

echo "cargo version: ${KEYCLOAK_RUST_VERSION} rest: ${KEYCLOAK_VERSION} issue: ${GITHUB_ISSUE}"

echo "Continue [Y]? \c"
read CONFIRM
case "$CONFIRM" in
  [Nn]*)
    exit 0
    ;;
  *)
esac

KEYCLOAK_REST_DOCS=https://www.keycloak.org/docs-api/${KEYCLOAK_VERSION}/rest-api/index.html
echo "Download and update docs from ${KEYCLOAK_REST_DOCS} [Y]? \c"
read CONFIRM

case "$CONFIRM" in
  [Yy]*|"")
    curl ${KEYCLOAK_REST_DOCS} -o docs/rest-api.html
    git diff docs/rest-api.html || true
    echo "Docs updated."
    ;;
  *) echo "No docs updated."
esac

echo Updating documentation...
export KEYCLOAK_VERSION KEYCLOAK_RUST_MAJOR_VERSION
handlebars-magic templates .
echo Documentation updated.
git diff README.md || true
echo "Continue [Y]? \c"
read CONFIRM
case "$CONFIRM" in
  [Nn]*)
    exit 0
    ;;
  *)
esac

echo Generating types...
git checkout -- src/types.rs src/rest/rest.rs
cargo run --example gen -- types > src/types2.rs
cargo run --example gen -- rest > src/rest/rest2.rs
mv src/types2.rs src/types.rs
mv src/rest/rest2.rs src/rest/generated_rest.rs
echo Types generated.
echo Format and build
cargo fmt
cargo build

echo Showing final difference...
git diff || true

echo "Commit changes to git [Y]? \c"
read CONFIRM
case "$CONFIRM" in
  [Nn]*)
    exit 0
    ;;
  *)
esac

git add .
git commit -am"Keycloak Admin REST API v${KEYCLOAK_RUST_VERSION}"
git tag -f -a -m "Release ${KEYCLOAK_RUST_VERSION}" -m "" -m "Changes:" -m "" -m "- Keycloak Admin REST API v${KEYCLOAK_VERSION} #${GITHUB_ISSUE}" v${KEYCLOAK_RUST_VERSION}

git log
git tag --list -n10 v${KEYCLOAK_RUST_VERSION}

echo "Publish to crates.io [Y]? \c"
read CONFIRM
case "$CONFIRM" in
  [Nn]*)
    exit 0
    ;;
  *)
esac

cargo publish