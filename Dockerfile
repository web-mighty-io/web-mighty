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

COPY --from=rust-build /app /app
RUN npm install -g sass
ADD https://github.com/jgthms/bulma/releases/download/0.9.1/bulma-0.9.1.zip /app/static/res
RUN unzip /app/static/res/bulma-0.9.1.zip -d /app/static/res
RUN rm /app/static/res/bulma-0.9.1.zip
RUN sass /app/static/res/scss/style.scss /app/static/res/css/style.css

# main container
FROM ubuntu:latest

COPY --from=node-build /app/build/bin /app/build/bin
COPY --from=node-build /app/static /app/static

# for postgresql server
EXPOSE 5432
# for http server
EXPOSE 8080
# for https server
EXPOSE 8443

ENTRYPOINT ["/app/build/bin/server"]