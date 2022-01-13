#!/bin/bash
REGISTRY="registry.onecloud.newtouch.com"
PACKAGE_VERSION=1.0.0
IMAGEID="$REGISTRY/ttc/ttchain:$PACKAGE_VERSION"
echo "Building $REGISTRY/ttc/ttchain:$PACKAGE_VERSION ..."
docker run -it --rm -v $PWD:/app -w /app -v $PWD/.cargo/config:/root/.cargo/config paritytech/ci-linux:363245ca-20210706 cargo build --release
docker build -t $IMAGEID .
