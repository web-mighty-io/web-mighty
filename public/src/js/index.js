import supported from "./modules/wasm.js";

if (!supported) {
    window.location.replace("/wasm-not-supported");
}

import "../scss/index.scss";

window.onload = function () {
    console.log("Hello");
};
