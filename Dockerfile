FROM debian:buster-slim

RUN apt update && apt install -y --no-install-recommends curl && \
  curl -sSL https://git.io/JcGER | bash && \
  apt clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
