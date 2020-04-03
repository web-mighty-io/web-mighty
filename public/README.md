# Webassembly 

## How to build

```shell script
# Installing wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Building wasm source
wasm-pack build --target web

# Static serve (for just debug)
python3 -m http.server 8080
``` 

and navigate to [http://localhost:8080/static/index.html](http://localhost:8080/static/index.html)