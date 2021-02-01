// import * as wasm from "../pkg";
//
// let main;
//
// await (async function () {
//     await wasm.run();
//
//     main = new wasm.Main();
// })();
//
// let ws = {
//     main
// };
//
// export default ws;

let ws = import ("../pkg/index").then(wasm => {
    wasm.run()

    return {
        main: new wasm.Main()
    };
});

export default ws;