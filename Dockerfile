FROM debian:buster-slim

RUN apt update && apt install -y --no-install-recommends curl ca-certificates

ADD install /install 
RUN ./install

RUN apt clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
