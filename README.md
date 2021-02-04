<h1 align="center">Mighty Card Game</h1>

<p align="center">
<a href="https://github.com/web-mighty-io/web-mighty/actions"><img src="https://img.shields.io/github/workflow/status/web-mighty-io/web-mighty/build?logo=github&logoColor=white&style=flat-square" alt="Github Action"/></a>
<a href="https://hub.docker.com/r/buttercrab/web-mighty"><img src="https://img.shields.io/github/workflow/status/web-mighty-io/web-mighty/deploy?label=docker&logo=docker&style=flat-square" alt="Github Action"/></a>
<a href="https://codecov.io/gh/web-mighty-io/web-mighty"><img src="https://img.shields.io/codecov/c/github/web-mighty-io/web-mighty?logo=codecov&logoColor=white&style=flat-square" alt="Codecov"/></a>
<a href="https://app.codacy.com/gh/web-mighty-io/web-mighty"><img src="https://img.shields.io/codacy/grade/ee071f0f9621405ebada9b2c7180f703?logo=codacy&style=flat-square" alt="Codacy"/></a>
<a href="https://github.com/web-mighty-io/web-mighty/blob/master/LICENSE"><img src="https://img.shields.io/github/license/web-mighty-io/web-mighty?logo=Open%20Source%20Initiative&logoColor=white&style=flat-square" alt="MIT License"/></a>
</p>

## Run server in docker

Start your postgresql server at `0.0.0.0:5432`. Then run:

```shell script
docker run -e HOST="0.0.0.0" 
           -e POSTGRES__HOST="host.docker.internal" 
           -e POSTGRES__USER="<postgres username>"
           -e POSTGRES__PASSWORD="<postgres password>"
           -p 8080:80 -d buttercrab/web-mighty
```

Go to [localhost:8080](http://localhost:8080) and have fun!

## Manually start your server

1.  [install rust](https://www.rust-lang.org/tools/install)
2.  [install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
3.  [install npm](https://www.npmjs.com/get-npm)
4.  [install](https://www.postgresql.org/download/) and start your postgres server
5.  make `server.toml` based from `server.sample.toml`
6.  run `npm i && npm run build`
7.  run `./build/bin/server`

## For developing

1.  do 1 ~ 5 from above
2.  run `npm run build-watch`
3.  run `./build/bin/server`
4.  run `npx webpack -w` in other terminal

For every change, webpack will rebuild your js code and server will catch changes.