import websocket from "./modules/websocket.js";

let ws = (await websocket)(["main"]);

ws.main.on("start", function () {
    alert("Hello");
})
