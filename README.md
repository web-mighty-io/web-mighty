# Mighty Card Game

[![Github Action](https://img.shields.io/github/workflow/status/web-mighty-io/web-mighty/build?logo=github&logoColor=white&style=flat-square)](https://github.com/web-mighty-io/web-mighty/actions)
[![Codecov](https://img.shields.io/codecov/c/github/web-mighty-io/web-mighty?logo=codecov&logoColor=white&style=flat-square)](https://codecov.io/gh/web-mighty-io/web-mighty)
[![Codacy](https://img.shields.io/codacy/grade/ee071f0f9621405ebada9b2c7180f703?logo=codacy&style=flat-square)](https://app.codacy.com/gh/web-mighty-io/web-mighty)
[![LICENSE](https://img.shields.io/github/license/web-mighty-io/web-mighty?logo=Open%20Source%20Initiative&logoColor=white&style=flat-square)](https://github.com/web-mighty-io/web-mighty/blob/master/LICENSE)

## Run your own server

Docker command to start right away. 

```shell script
docker run -e HOST="0.0.0.0" -e POSTGRES__HOST="" -p 8080:80 -d buttercrab/web-mighty
```

Go to [localhost](http://localhost:8080)

## Manually start your server

1.  [install rust](https://www.rust-lang.org/tools/install)
2.  [install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
3.  [install sass](https://sass-lang.com/install)
4.  [install bulma](https://bulma.io/): download latest version and unzip to `static/res/bulma`
5.  convert `static/res/scss/style.scss` to `static/res/css/style.css` using `sass`
6.  [install](https://www.postgresql.org/download/) and start your postgresql server
7.  run `wasm-pack build --target web --release -d staic/res/pkg public`
8.  run `cargo install --root build --path server`
9.  make `server.toml` based from `server.sample.toml`
10. run `./build/bin/server`