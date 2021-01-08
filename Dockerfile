# building in rust
# - compiling server
# - compiling wasm
FROM rust:latest AS rust-build

# labels
LABEL maintainer="Jaeyong Sung <jaeyong0201@gmail.com>"
LABEL org.label-schema.name="buttercrab/web-mighty"
LABEL org.label-schema.description="Mighty Card Game in Online"

COPY . /app
RUN cargo install --root /app/build --path /app/server --features https
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN wasm-pack build --target web -d /app/static/res/pkg /app/public

# building in sass
# - compiling sass files
FROM node:latest AS node-build

COPY ./static /app/static
RUN npm install -g sass
ADD https://github.com/jgthms/bulma/releases/download/0.9.1/bulma-0.9.1.zip /app/static/res
RUN unzip /app/static/res/bulma-0.9.1.zip -d /app/static/res
RUN rm /app/static/res/bulma-0.9.1.zip
RUN sass /app/static/res/scss/style.scss /app/static/res/css/style.css

# minifying css & js files
FROM python:latest AS python-build

COPY --from=node-build /app/static /app/static
COPY ./requirements.txt /app
COPY ./minify_files.py /app
RUN python3 -m pip install -r /app/requirements.txt
RUN python3 /app/minify_files.py --path /app/static --remove

# main container
FROM ubuntu:latest

COPY --from=rust-build /app/build/bin/server /app/bin
COPY --from=rust-build /app/static/res/pkg/*.js /app/static/res/pkg
COPY --from=rust-build /app/static/res/pkg/*.wasm /app/static/res/pkg
COPY --from=python-build /app/static /app/static
RUN apt-get update && apt-get install libssl-dev -y

# for postgresql server
EXPOSE 5432

ENTRYPOINT ["/app/bin/server"]