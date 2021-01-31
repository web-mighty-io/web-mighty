import init, * as wasm from "../pkg/public.js";

await (async function () {
    await init();

    document.addEventListener('mousemove', myListener, false);

})();
