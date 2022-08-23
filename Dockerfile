FROM paritytech/ci-linux:bfd0dd38-20220507 as builder
WORKDIR /app
COPY . /app
RUN cargo build --release

## ImageBuild
FROM debian:11

RUN apt-get update && \
  apt-get install -y openssl && \
  rm /var/lib/apt/ -rf && \
  rm /var/cache/apt/ -rf

WORKDIR /opt/ttchain/

#COPY ./docker/run.sh /opt/run.sh
COPY --from=builder  /app/target/release/node-template /opt/ttchain/node-template

#CMD /opt/run.sh