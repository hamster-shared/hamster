## ImageBuild
FROM debian:11

RUN apt-get update && \
  apt-get install -y openssl && \
  rm /var/lib/apt/ -rf && \
  rm /var/cache/apt/ -rf

WORKDIR /opt/ttchain/

#COPY ./docker/run.sh /opt/run.sh
ADD ./target/release/node-template /opt/ttchain/node-template

#CMD /opt/run.sh
