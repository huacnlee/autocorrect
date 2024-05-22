FROM alpine:latest

RUN apk add --no-cache curl ca-certificates

ADD install /install 
RUN sh ./install