import init, * as wasm from "../pkg/client.js";

let main;

await (async function () {
    await init();
    await wasm.run();

    main = new wasm.Main();
})();

let ws = {
    main
};

export default ws;
