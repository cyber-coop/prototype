##### BUILDER #####
FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/prototype
COPY . .
RUN cargo install --path .

##### RUNNER #####
FROM debian:buster-slim

LABEL author="Lola Rigaut-Luczak <me@laflemme.lol>"
LABEL description="Custom node that allow indexing blocks and transactions from block chains."

COPY --from=builder /usr/local/cargo/bin/prototype /usr/local/bin/prototype

RUN apt-get update && rm -rf /var/lib/apt/lists/*

# default env
ENV NETWORK "dogecoin"
ENV TESTNET "true"
ENV RUST_LOG "prototype=info"

CMD prototype ${NETWORK} ${TESTNET} 