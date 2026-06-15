##### BUILDER #####
FROM rustlang/rust:nightly AS builder

WORKDIR /usr/src/prototype
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/eth-prototype/target \
    cargo install --path .

##### RUNNER #####
FROM debian:trixie-slim

LABEL author="Lola Rigaut-Luczak <me@laflemme.lol>"
LABEL description="Custom node that allow indexing blocks and transactions from block chains."

COPY --from=builder /usr/local/cargo/bin/prototype /usr/local/bin/prototype

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# default env
ENV NETWORK "dogecoin_testnet"
ENV RUST_LOG "prototype=info"

ENTRYPOINT ["/bin/sh", "-c", "exec prototype ${NETWORK}"]