FROM alpine:latest

RUN apk add --update curl ca-certificates

ADD install /install 
RUN sh ./install