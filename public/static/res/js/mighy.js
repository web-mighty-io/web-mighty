import init, * as wasm from '../../pkg/public.js';

async function run() {
    await init();

    wasm.greet();
}

run();