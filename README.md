# Mighty Card Game

[![Github Action](https://img.shields.io/github/workflow/status/buttercrab/web-mighty/build?style=flat-square)](https://github.com/buttercrab/web-mighty/actions)
[![Codecov](https://img.shields.io/codecov/c/github/buttercrab/web-mighty?style=flat-square)](https://codecov.io/gh/buttercrab/web-mighty)
[![LICENSE](https://img.shields.io/github/license/buttercrab/web-mighty?style=flat-square)](https://github.com/buttercrab/web-mighty/blob/master/LICENSE)

## Run your own server

Docker command to start right away.
You can use `server.docker.toml` to configure based on docker.

1. If you want https:
    ```shell script
    docker run -v <your_dir_to_conf_and_pem>:/app/conf -e CONFIG='/app/conf/server.toml' 
   -p 80:80 -p 443:443 -p 5432:5432 -d buttercrab/web-mighty
    ```

1. If you want http:
    ```shell script
    docker run  -e PORT=80 
   -p 80:80 -p 5432:5432 -d buttercrab/web-mighty
    ```

## Manually start your server

1. [install rust](https://www.rust-lang.org/tools/install)

1. [install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

1. [install sass](https://sass-lang.com/install)

1. [install bulma](https://bulma.io/): download latest version and unzip to `static/res/bulma`

1. convert `static/res/scss/style.scss` to `static/res/css/style.css` using `sass`

1. [install](https://www.postgresql.org/download/) and start your postgresql server

1. run `cd public && wasm-pack build --target web --release`

1. build server

    1) if you want https:

       run `cargo install --features https --root build --path server`

    1) if you want only http:

       run `cargo install --root build --path server`

1. make `server.toml` based from `server.sample.toml`

1. run `./build/bin/server`