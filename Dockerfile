# building in rust
# - compiling server
# - compiling wasm
FROM rust:1.53.0 AS rust-build

COPY ./client /app/client
COPY ./config /app/config
COPY ./mighty /app/mighty
COPY ./server /app/server
COPY ./types  /app/types

RUN cargo install --root /app/build --path /app/server \
 && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh \
 && wasm-pack build --release --out-dir /app/public/src/js/pkg --out-name index /app/client

# building in webpack
FROM node:15.8 AS node-build

COPY                   ./public                  /app/public
COPY                   ./package.json            /app
COPY                   ./webpack.config.js       /app
COPY --from=rust-build /app/public/src/js/pkg/*.js   /app/public/src/js/pkg/
COPY --from=rust-build /app/public/src/js/pkg/*.wasm /app/public/src/js/pkg/

WORKDIR /app

RUN npm install -g npm \
 && npm install \
 && npx webpack --env=docker \
 && rm -rf /app/public/src

# main container
FROM ubuntu:focal

# labels
LABEL maintainer="Jaeyong Sung <jaeyong0201@gmail.com>"
LABEL org.label-schema.name="buttercrab/web-mighty"
LABEL org.label-schema.description="Mighty Card Game in Online"

ENV SERVE_PATH="/app/public"

COPY --from=rust-build /app/build/bin /app/bin
COPY --from=node-build /app/public    /app/public

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
            ca-certificates=20210119~20.04.1 \
            libssl-dev=1.1.1f-1ubuntu2.1 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/app/bin/server"]