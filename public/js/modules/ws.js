let ws = import ("../pkg/index.js").then((wasm) => {
    wasm.run();

    return {
        main: new wasm.Main()
    };
});

export default ws;