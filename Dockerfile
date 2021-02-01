# building in rust
# - compiling server
# - compiling wasm
FROM rust:1.49 AS rust-build

COPY ./client /app/client
COPY ./config /app/config
COPY ./mighty /app/mighty
COPY ./server /app/server
COPY ./types  /app/types

RUN cargo install --root /app/build --path /app/server \
 && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh \
 && wasm-pack build --release --out-dir /app/public/js/pkg --out-name index /app/client

# building in sass
# - compiling sass files
FROM node:15.7 AS node-build

COPY                   ./public                  /app/public
COPY                   ./package.json            /app
COPY                   ./webpack.config.js       /app
COPY --from=rust-build /app/public/js/pkg/*.js   /app/public/js/pkg
COPY --from=rust-build /app/public/js/pkg/*.wasm /app/public/js/pkg

WORKDIR /app

RUN npm install \
 && npx sass /app/public/res/scss/style.scss /app/public/res/css/style.css --no-source-map \
 && npx webpack --env=docker \
 && rm -rf /app/public/res/scss /app/public/js

# minifying css & js files
FROM python:3.9 AS python-build

COPY --from=node-build /app/public        /app/public
COPY                   ./requirements.txt /app
COPY                   ./minify_files.py  /app

RUN python3 -m pip install -r /app/requirements.txt \
 && python3 /app/minify_files.py --path /app/public --remove

# main container
FROM ubuntu:focal

# labels
LABEL maintainer="Jaeyong Sung <jaeyong0201@gmail.com>"
LABEL org.label-schema.name="buttercrab/web-mighty"
LABEL org.label-schema.description="Mighty Card Game in Online"

ENV SERVE_PATH="/app/public"

COPY --from=rust-build   /app/build/bin /app/bin
COPY --from=python-build /app/public    /app/public

RUN apt-get update \
 && apt-get install -y --no-install-recommends libssl-dev=1.1.1f-1ubuntu2.1 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/app/bin/server"]