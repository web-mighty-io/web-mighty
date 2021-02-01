let ws = import ("../pkg/index").then((wasm) => {
    wasm.run();

    return {
        main: new wasm.Main()
    };
});

export default ws;