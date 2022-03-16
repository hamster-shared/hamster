#!/bin/bash
PACKAGE_VERSION=1.0.2
IMAGEID="hamstershare/hamster:$PACKAGE_VERSION"
echo "hamstershare/hamster:$PACKAGE_VERSION ..."
## docker run -it --rm -v $PWD:/app -w /app -v $PWD/.cargo/config:/root/.cargo/config paritytech/ci-linux:363245ca-20210706 cargo build --release
docker build -t $IMAGEID .
