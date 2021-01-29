# building in rust
# - compiling server
# - compiling wasm
FROM rust:1.49 AS rust-build

COPY ./config /app/config
COPY ./mighty /app/mighty
COPY ./public /app/public
COPY ./server /app/server
COPY ./types  /app/types

RUN cargo install --root /app/build --path /app/server
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN wasm-pack build --target web -d /app/static/res/pkg /app/public

# building in sass
# - compiling sass files
FROM node:15.7 AS node-build

COPY ./static /app/static
RUN npm install -g sass
ADD https://github.com/jgthms/bulma/releases/download/0.9.1/bulma-0.9.1.zip /app/static/res
RUN unzip /app/static/res/bulma-0.9.1.zip -d /app/static/res
RUN rm /app/static/res/bulma-0.9.1.zip
RUN sass /app/static/res/scss/style.scss /app/static/res/css/style.css
RUN rm -rf /app/static/res/bulma /app/static/res/scss

# minifying css & js files
FROM python:3.9 AS python-build

COPY --from=node-build /app/static /app/static
COPY --from=rust-build /app/static/res/pkg/*.js /app/static/res/pkg
COPY --from=rust-build /app/static/res/pkg/*.wasm /app/static/res/pkg
COPY ./requirements.txt /app
COPY ./minify_files.py /app
RUN python3 -m pip install -r /app/requirements.txt
RUN python3 /app/minify_files.py --path /app/static --remove

# main container
FROM ubuntu:focal

# labels
LABEL maintainer="Jaeyong Sung <jaeyong0201@gmail.com>"
LABEL org.label-schema.name="buttercrab/web-mighty"
LABEL org.label-schema.description="Mighty Card Game in Online"

ENV SERVE_PATH="/app/static"

COPY --from=rust-build /app/build/bin /app/bin
COPY --from=python-build /app/static /app/static
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev=1.1.1f-1ubuntu2.1 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/app/bin/server"]