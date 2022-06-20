#!/bin/bash
set -ex;
#cargo build --release

cp scripts/ci/docker/substrate.Dockerfile target/release/Dockerfile

CI_COMMIT_SHA="$(git rev-parse HEAD)"
IMAGE_REPOSITORY="registry.onecloud.newtouch.com"
IMAGE_NAMESPACE="authright"
IMAGE_NAME="authright-substrate"
VERSION="$(date -u '+%Y%m%dT%H%M%S')"

pushd target/release

docker build -t $IMAGE_REPOSITORY/$IMAGE_NAMESPACE/$IMAGE_NAME:$VERSION \
  --build-arg VCS_REF="$CI_COMMIT_SHA" \
  --build-arg BUILD_DATE=$VERSION \
 .

