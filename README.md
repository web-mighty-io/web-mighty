# Mighty Card Game

[![Github Action](https://img.shields.io/github/workflow/status/web-mighty-io/web-mighty/build?style=flat-square)](https://github.com/web-mighty-io/web-mighty/actions)
[![Codecov](https://img.shields.io/codecov/c/github/web-mighty-io/web-mighty?style=flat-square)](https://codecov.io/gh/web-mighty-io/web-mighty)
[![LICENSE](https://img.shields.io/github/license/web-mighty-io/web-mighty?style=flat-square)](https://github.com/web-mighty-io/web-mighty/blob/master/LICENSE)

## Run your own server

Docker command to start right away.

```shell script
docker run -p 80:80 -p 5432:5432 -d buttercrab/web-mighty
```

## Manually start your server

1. [install rust](https://www.rust-lang.org/tools/install)
1. [install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
1. [install sass](https://sass-lang.com/install)
1. [install bulma](https://bulma.io/): download latest version and unzip to `static/res/bulma`
1. convert `static/res/scss/style.scss` to `static/res/css/style.css` using `sass`
1. [install](https://www.postgresql.org/download/) and start your postgresql server
1. run `cd public && wasm-pack build --target web --release`
1. run `cargo install --root build --path server`
1. make `server.toml` based from `server.sample.toml`
1. run `./build/bin/server`