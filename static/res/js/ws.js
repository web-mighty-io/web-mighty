import "../pkg/public.js";

let main;

await (async function () {
    await run();

    main = new Main();
})();

export let ws = {
    main
};