import ws from "./modules/ws.js";

let ws_ = await ws;

ws_.main.on("start", function () {
    alert("Hello");
})
